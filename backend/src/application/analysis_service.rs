use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::{Analysis, AnalysisResult, AnalysisStatus, ProductSummary};
use crate::domain::ports::ai_analyzer::AiAnalyzer;
use crate::domain::ports::analysis_repository::AnalysisRepository;
use crate::domain::ports::crawler::{CoupangCrawler, ProductReviews};
use crate::error::{AppError, AppResult};

/// 분석 작업 유스케이스 포트.
#[async_trait]
pub trait AnalysisService: Send + Sync {
    /// 분석 작업을 생성하고 백그라운드 파이프라인을 시작한다.
    /// 생성 시점의 (pending) Analysis 를 즉시 반환한다.
    async fn create_analysis(
        &self,
        user_id: Uuid,
        urls: Vec<String>,
        review_limit: i32,
    ) -> AppResult<Analysis>;

    /// 소유자 스코프 단건 조회. 없거나 소유권이 다르면 `NotFound`.
    async fn get_analysis(&self, id: Uuid, user_id: Uuid) -> AppResult<Analysis>;

    /// 소유자별 분석 목록(페이지네이션). 반환: (항목, 전체 개수).
    async fn list_analyses(
        &self,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<Analysis>, i64)>;
}

/// repo + crawler + analyzer 를 조합한 기본 구현.
pub struct StandardAnalysisService {
    repo: Arc<dyn AnalysisRepository>,
    crawler: Arc<dyn CoupangCrawler>,
    analyzer: Arc<dyn AiAnalyzer>,
}

impl StandardAnalysisService {
    pub fn new(
        repo: Arc<dyn AnalysisRepository>,
        crawler: Arc<dyn CoupangCrawler>,
        analyzer: Arc<dyn AiAnalyzer>,
    ) -> Self {
        Self {
            repo,
            crawler,
            analyzer,
        }
    }

    /// 백그라운드 분석 파이프라인 (순수 분리, 테스트에서 직접 await 가능).
    ///
    /// crawling → analyzing → completed 순으로 상태를 갱신하며,
    /// 어느 단계든 실패하면 `set_error` 로 status=failed 로 마킹하고 종료한다.
    /// 패닉을 일으키지 않는다.
    async fn process_analysis(
        repo: Arc<dyn AnalysisRepository>,
        crawler: Arc<dyn CoupangCrawler>,
        analyzer: Arc<dyn AiAnalyzer>,
        id: Uuid,
        urls: Vec<String>,
        review_limit: i32,
    ) {
        // 1. crawling
        if let Err(e) = repo.update_status(id, AnalysisStatus::Crawling).await {
            // 상태 갱신 실패는 더 진행해도 의미가 없으므로 에러 기록 후 종료.
            let _ = repo.set_error(id, &e.to_string()).await;
            return;
        }

        // 2. 각 url 크롤링
        let mut product_reviews: Vec<ProductReviews> = Vec::with_capacity(urls.len());
        for url in &urls {
            match crawler.fetch_reviews(url, review_limit as u32).await {
                Ok(pr) => product_reviews.push(pr),
                Err(e) => {
                    let _ = repo.set_error(id, &e.to_string()).await;
                    return;
                }
            }
        }

        // 3. 결정론적 통계 집계
        let products: Vec<ProductSummary> = product_reviews
            .iter()
            .map(Self::aggregate_summary)
            .collect();

        // 4. analyzing
        if let Err(e) = repo.update_status(id, AnalysisStatus::Analyzing).await {
            let _ = repo.set_error(id, &e.to_string()).await;
            return;
        }

        // 5. AI 분석
        let insights = match analyzer.analyze(&product_reviews).await {
            Ok(i) => i,
            Err(e) => {
                let _ = repo.set_error(id, &e.to_string()).await;
                return;
            }
        };

        // 6. 결과 저장
        let result = AnalysisResult { products, insights };
        if let Err(e) = repo.set_result(id, &result).await {
            let _ = repo.set_error(id, &e.to_string()).await;
        }
    }

    /// 단일 상품 리뷰 → 결정론적 통계 요약(개수/평균/평점분포).
    /// 평점 분포는 "1".."5" 키를 항상 포함(미등장 평점은 0).
    fn aggregate_summary(pr: &ProductReviews) -> ProductSummary {
        let total_reviews = pr.reviews.len() as i32;
        let avg_rating = if pr.reviews.is_empty() {
            0.0
        } else {
            let sum: i64 = pr.reviews.iter().map(|r| r.rating as i64).sum();
            sum as f64 / pr.reviews.len() as f64
        };

        let mut rating_distribution: HashMap<String, i32> = HashMap::new();
        for star in 1..=5 {
            rating_distribution.insert(star.to_string(), 0);
        }
        for r in &pr.reviews {
            if (1..=5).contains(&r.rating) {
                *rating_distribution.entry(r.rating.to_string()).or_insert(0) += 1;
            }
        }

        ProductSummary {
            url: pr.url.clone(),
            product_name: pr.product_name.clone(),
            total_reviews,
            avg_rating,
            rating_distribution,
        }
    }
}

#[async_trait]
impl AnalysisService for StandardAnalysisService {
    async fn create_analysis(
        &self,
        user_id: Uuid,
        urls: Vec<String>,
        review_limit: i32,
    ) -> AppResult<Analysis> {
        // 입력 검증 (플랜 제한 강제 없음).
        if urls.is_empty() || urls.len() > 3 {
            return Err(AppError::BadRequest("urls 는 1~3개여야 합니다".into()));
        }
        if urls.iter().any(|u| u.trim().is_empty()) {
            return Err(AppError::BadRequest("빈 URL 은 허용되지 않습니다".into()));
        }
        if !(50..=500).contains(&review_limit) {
            return Err(AppError::BadRequest(
                "review_limit 은 50~500 사이여야 합니다".into(),
            ));
        }

        let analysis = self.repo.create(user_id, &urls, review_limit).await?;

        let repo = Arc::clone(&self.repo);
        let crawler = Arc::clone(&self.crawler);
        let analyzer = Arc::clone(&self.analyzer);
        let id = analysis.id;
        tokio::spawn(async move {
            Self::process_analysis(repo, crawler, analyzer, id, urls, review_limit).await;
        });

        Ok(analysis)
    }

    async fn get_analysis(&self, id: Uuid, user_id: Uuid) -> AppResult<Analysis> {
        let analysis = self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;
        if analysis.user_id != user_id {
            return Err(AppError::NotFound);
        }
        Ok(analysis)
    }

    async fn list_analyses(
        &self,
        user_id: Uuid,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<Analysis>, i64)> {
        self.repo.list_by_user(user_id, page, per_page).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::gemini::MockAiAnalyzer;
    use crate::adapters::coupang::MockCoupangCrawler;
    use crate::domain::models::{Analysis, Review};
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// in-memory AnalysisRepository (Docker 불필요).
    #[derive(Default)]
    struct MockAnalysisRepository {
        store: Mutex<HashMap<Uuid, Analysis>>,
    }

    #[async_trait]
    impl AnalysisRepository for MockAnalysisRepository {
        async fn create(
            &self,
            user_id: Uuid,
            urls: &[String],
            review_limit: i32,
        ) -> AppResult<Analysis> {
            let analysis = Analysis {
                id: Uuid::new_v4(),
                user_id,
                urls: urls.to_vec(),
                review_limit,
                status: AnalysisStatus::Pending,
                result: None,
                error: None,
                created_at: Utc::now(),
                completed_at: None,
            };
            self.store
                .lock()
                .unwrap()
                .insert(analysis.id, analysis.clone());
            Ok(analysis)
        }

        async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Analysis>> {
            Ok(self.store.lock().unwrap().get(&id).cloned())
        }

        async fn list_by_user(
            &self,
            user_id: Uuid,
            _page: i64,
            _per_page: i64,
        ) -> AppResult<(Vec<Analysis>, i64)> {
            let items: Vec<Analysis> = self
                .store
                .lock()
                .unwrap()
                .values()
                .filter(|a| a.user_id == user_id)
                .cloned()
                .collect();
            let total = items.len() as i64;
            Ok((items, total))
        }

        async fn update_status(&self, id: Uuid, status: AnalysisStatus) -> AppResult<()> {
            if let Some(a) = self.store.lock().unwrap().get_mut(&id) {
                a.status = status;
            }
            Ok(())
        }

        async fn set_result(&self, id: Uuid, result: &AnalysisResult) -> AppResult<()> {
            if let Some(a) = self.store.lock().unwrap().get_mut(&id) {
                a.status = AnalysisStatus::Completed;
                a.result = Some(result.clone());
                a.completed_at = Some(Utc::now());
            }
            Ok(())
        }

        async fn set_error(&self, id: Uuid, error: &str) -> AppResult<()> {
            if let Some(a) = self.store.lock().unwrap().get_mut(&id) {
                a.status = AnalysisStatus::Failed;
                a.error = Some(error.to_string());
                a.completed_at = Some(Utc::now());
            }
            Ok(())
        }

        async fn count_this_month(&self, user_id: Uuid) -> AppResult<i64> {
            let count = self
                .store
                .lock()
                .unwrap()
                .values()
                .filter(|a| a.user_id == user_id)
                .count() as i64;
            Ok(count)
        }
    }

    /// 항상 에러를 반환하는 크롤러.
    struct FailingCrawler;

    #[async_trait]
    impl CoupangCrawler for FailingCrawler {
        async fn fetch_reviews(&self, _url: &str, _limit: u32) -> AppResult<ProductReviews> {
            Err(AppError::Internal(anyhow::anyhow!("크롤링 실패")))
        }
    }

    fn service(
        repo: Arc<MockAnalysisRepository>,
        crawler: Arc<dyn CoupangCrawler>,
    ) -> StandardAnalysisService {
        StandardAnalysisService::new(repo, crawler, Arc::new(MockAiAnalyzer::new()))
    }

    #[tokio::test]
    async fn process_analysis_completes_with_result() {
        let repo = Arc::new(MockAnalysisRepository::default());
        let user_id = Uuid::new_v4();
        let urls = vec!["https://www.coupang.com/vp/products/1".to_string()];
        let created = repo.create(user_id, &urls, 100).await.unwrap();

        StandardAnalysisService::process_analysis(
            Arc::clone(&repo) as Arc<dyn AnalysisRepository>,
            Arc::new(MockCoupangCrawler::new()),
            Arc::new(MockAiAnalyzer::new()),
            created.id,
            urls,
            100,
        )
        .await;

        let stored = repo.find_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(stored.status, AnalysisStatus::Completed);
        let result = stored.result.expect("result should exist");
        assert!(!result.products.is_empty());
        // MockAiAnalyzer 는 부정 리뷰가 있어 인사이트를 생성한다.
        assert!(!result.insights.top_complaints.is_empty());
    }

    #[tokio::test]
    async fn process_analysis_marks_failed_on_crawler_error() {
        let repo = Arc::new(MockAnalysisRepository::default());
        let user_id = Uuid::new_v4();
        let urls = vec!["https://www.coupang.com/vp/products/1".to_string()];
        let created = repo.create(user_id, &urls, 100).await.unwrap();

        StandardAnalysisService::process_analysis(
            Arc::clone(&repo) as Arc<dyn AnalysisRepository>,
            Arc::new(FailingCrawler),
            Arc::new(MockAiAnalyzer::new()),
            created.id,
            urls,
            100,
        )
        .await;

        let stored = repo.find_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(stored.status, AnalysisStatus::Failed);
        assert!(stored.error.is_some());
        assert!(stored.result.is_none());
    }

    #[test]
    fn aggregate_summary_computes_avg_and_distribution() {
        let pr = ProductReviews {
            url: "https://www.coupang.com/vp/products/1".to_string(),
            product_name: "테스트 상품".to_string(),
            reviews: vec![
                Review {
                    text: "a".into(),
                    rating: 5,
                },
                Review {
                    text: "b".into(),
                    rating: 5,
                },
                Review {
                    text: "c".into(),
                    rating: 3,
                },
                Review {
                    text: "d".into(),
                    rating: 1,
                },
            ],
        };

        let summary = StandardAnalysisService::aggregate_summary(&pr);
        assert_eq!(summary.total_reviews, 4);
        assert_eq!(summary.avg_rating, (5 + 5 + 3 + 1) as f64 / 4.0);
        assert_eq!(summary.rating_distribution.get("5"), Some(&2));
        assert_eq!(summary.rating_distribution.get("3"), Some(&1));
        assert_eq!(summary.rating_distribution.get("1"), Some(&1));
        assert_eq!(summary.rating_distribution.get("2"), Some(&0));
        assert_eq!(summary.rating_distribution.get("4"), Some(&0));
    }

    #[test]
    fn aggregate_summary_empty_reviews() {
        let pr = ProductReviews {
            url: "u".into(),
            product_name: "p".into(),
            reviews: vec![],
        };
        let summary = StandardAnalysisService::aggregate_summary(&pr);
        assert_eq!(summary.total_reviews, 0);
        assert_eq!(summary.avg_rating, 0.0);
        assert_eq!(summary.rating_distribution.get("1"), Some(&0));
    }

    #[tokio::test]
    async fn create_analysis_validates_input() {
        let repo = Arc::new(MockAnalysisRepository::default());
        let svc = service(Arc::clone(&repo), Arc::new(MockCoupangCrawler::new()));
        let user_id = Uuid::new_v4();

        // urls 0개
        assert!(matches!(
            svc.create_analysis(user_id, vec![], 100).await,
            Err(AppError::BadRequest(_))
        ));
        // urls 4개
        let four = vec![
            "https://www.coupang.com/vp/products/1".to_string(),
            "https://www.coupang.com/vp/products/2".to_string(),
            "https://www.coupang.com/vp/products/3".to_string(),
            "https://www.coupang.com/vp/products/4".to_string(),
        ];
        assert!(matches!(
            svc.create_analysis(user_id, four, 100).await,
            Err(AppError::BadRequest(_))
        ));
        // review_limit 49
        assert!(matches!(
            svc.create_analysis(
                user_id,
                vec!["https://www.coupang.com/vp/products/1".to_string()],
                49
            )
            .await,
            Err(AppError::BadRequest(_))
        ));
        // review_limit 501
        assert!(matches!(
            svc.create_analysis(
                user_id,
                vec!["https://www.coupang.com/vp/products/1".to_string()],
                501
            )
            .await,
            Err(AppError::BadRequest(_))
        ));
    }

    #[tokio::test]
    async fn get_analysis_enforces_ownership() {
        let repo = Arc::new(MockAnalysisRepository::default());
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        let created = repo
            .create(
                owner,
                &["https://www.coupang.com/vp/products/1".to_string()],
                100,
            )
            .await
            .unwrap();

        let svc = service(Arc::clone(&repo), Arc::new(MockCoupangCrawler::new()));
        assert!(svc.get_analysis(created.id, owner).await.is_ok());
        assert!(matches!(
            svc.get_analysis(created.id, other).await,
            Err(AppError::NotFound)
        ));
        assert!(matches!(
            svc.get_analysis(Uuid::new_v4(), owner).await,
            Err(AppError::NotFound)
        ));
    }
}
