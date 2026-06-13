import { Badge, Card, type BadgeProps } from '@/src/shared/components/ui';
import type { AnalysisResult, ProductSummary } from '@/src/features/analysis/types';

interface InsightReportProps {
  result: AnalysisResult;
}

const SEVERITY_VARIANT: Record<string, NonNullable<BadgeProps['variant']>> = {
  high: 'danger',
  medium: 'warning',
  low: 'neutral',
};

function severityVariant(severity: string): NonNullable<BadgeProps['variant']> {
  return SEVERITY_VARIANT[severity] ?? 'neutral';
}

function Section({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <Card className="p-6">
      <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
      {description ? (
        <p className="mt-1 text-sm text-gray-500">{description}</p>
      ) : null}
      <div className="mt-4">{children}</div>
    </Card>
  );
}

function Methodology({ products }: { products: ProductSummary[] }) {
  const totalReviews = products.reduce((sum, p) => sum + p.total_reviews, 0);
  const productCount = products.length;
  return (
    <Card className="border-brand-100 bg-brand-50 p-5">
      <details>
        <summary className="flex cursor-pointer items-center gap-2 font-medium text-gray-900">
          <Badge variant="info">info</Badge>
          분석 방법 · 데이터 출처
        </summary>
        <div className="mt-3 flex flex-col gap-2 text-sm text-gray-600">
          <p>
            쿠팡 공개 리뷰를 자동 수집해, 한국어 문맥 기준으로 AI가 불만·강점 표현을
            분류·집계했습니다.
          </p>
          <p className="font-medium text-gray-800">
            분석 표본: 총 {totalReviews}개 리뷰 (상품 {productCount}개)
          </p>
          <p>표본이 많을수록 패턴 신뢰도가 높아집니다.</p>
        </div>
      </details>
    </Card>
  );
}

function Disclaimer() {
  return (
    <div className="border-t border-gray-100 pt-4 text-xs text-gray-500">
      본 분석은 공개 리뷰를 AI가 자동 분류·집계한 결과로 참고용 인사이트입니다. 중요한
      의사결정 전 원문 리뷰를 함께 확인하시기 바랍니다.
    </div>
  );
}

function EmptyHint() {
  return <p className="text-sm text-gray-500">데이터 없음</p>;
}

function RatingDistribution({ product }: { product: ProductSummary }) {
  const total = Object.values(product.rating_distribution).reduce(
    (sum, n) => sum + n,
    0,
  );
  return (
    <div className="flex flex-col gap-3">
      <div className="flex items-baseline justify-between gap-2">
        <h3 className="text-sm font-medium text-gray-900">{product.product_name}</h3>
        <span className="text-xs text-gray-500">
          평점 {product.avg_rating} · 리뷰 {product.total_reviews}개
        </span>
      </div>
      <div className="flex flex-col gap-1.5">
        {[5, 4, 3, 2, 1].map((star) => {
          const count = product.rating_distribution[String(star)] ?? 0;
          const pct = total > 0 ? (count / total) * 100 : 0;
          return (
            <div key={star} className="flex items-center gap-2">
              <span className="w-8 text-xs text-gray-600">{star}점</span>
              <div className="h-3 flex-1 overflow-hidden rounded-full bg-gray-100">
                <div
                  className="h-full rounded-full bg-amber-400"
                  style={{ width: `${pct}%` }}
                  aria-label={`${star}점 ${count}개`}
                />
              </div>
              <span className="w-10 text-right text-xs text-gray-500">{count}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

export function InsightReport({ result }: InsightReportProps) {
  const { products, insights } = result;
  const improvements = insights.improvement_points
    .slice()
    .sort((a, b) => a.rank - b.rank)
    .slice(0, 3);

  return (
    <div className="flex flex-col gap-6">
      <Methodology products={products} />

      <Section
        title="고쳐야 할 것 TOP 3"
        description="경쟁사 리뷰의 불만 패턴 중 개선 효과가 큰 순서입니다. 위에서부터 손대면 경쟁 우위를 빠르게 확보할 수 있습니다."
      >
        {improvements.length === 0 ? (
          <EmptyHint />
        ) : (
          <ol className="flex flex-col gap-4">
            {improvements.map((point) => (
              <li key={point.rank} className="flex gap-3">
                <span className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-brand-600 text-sm font-bold text-white">
                  {point.rank}
                </span>
                <div>
                  <p className="font-medium text-gray-900">{point.title}</p>
                  <p className="mt-0.5 text-sm text-gray-600">{point.detail}</p>
                </div>
              </li>
            ))}
          </ol>
        )}
      </Section>

      <Section
        title="경쟁사 약점"
        description="경쟁사가 반복적으로 지적받는 지점입니다. 내 상품이 파고들 기회입니다."
      >
        {insights.competitor_weaknesses.length === 0 ? (
          <EmptyHint />
        ) : (
          <ul className="flex flex-col gap-3">
            {insights.competitor_weaknesses.map((weakness, i) => (
              <li key={i} className="rounded-md border border-gray-100 bg-gray-50 p-3">
                <p className="font-medium text-gray-900">{weakness.title}</p>
                <p className="mt-0.5 text-sm text-gray-600">{weakness.opportunity}</p>
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section
        title="구매 결정 요인"
        description="고객이 구매를 결정할 때 실제로 언급한 핵심 요소입니다. 상세페이지·마케팅 메시지에 활용하세요."
      >
        {insights.purchase_drivers.length === 0 ? (
          <EmptyHint />
        ) : (
          <ul className="flex flex-wrap gap-2">
            {insights.purchase_drivers.map((driver, i) => (
              <li
                key={i}
                className="rounded-full bg-brand-50 px-3 py-1 text-sm text-brand-700"
              >
                {driver}
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section
        title="반복 불만 / 긍정 요인"
        description="리뷰에 자주 등장한 표현을 빈도순으로 집계했습니다."
      >
        <div className="grid gap-6 sm:grid-cols-2">
          <div>
            <h3 className="mb-2 text-sm font-medium text-gray-900">반복 불만</h3>
            {insights.top_complaints.length === 0 ? (
              <EmptyHint />
            ) : (
              <ul className="flex flex-col gap-2">
                {insights.top_complaints.map((c, i) => (
                  <li key={i} className="flex items-center justify-between gap-2">
                    <span className="text-sm text-gray-800">{c.text}</span>
                    <span className="flex items-center gap-2">
                      <Badge variant={severityVariant(c.severity)}>{c.severity}</Badge>
                      <span className="text-xs text-gray-500">{c.count}회</span>
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div>
            <h3 className="mb-2 text-sm font-medium text-gray-900">긍정 요인</h3>
            {insights.top_positives.length === 0 ? (
              <EmptyHint />
            ) : (
              <ul className="flex flex-col gap-2">
                {insights.top_positives.map((p, i) => (
                  <li key={i} className="flex items-center justify-between gap-2">
                    <span className="text-sm text-gray-800">{p.text}</span>
                    <span className="text-xs text-gray-500">{p.count}회</span>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>
      </Section>

      <Section
        title="평점 분포"
        description="수집된 리뷰의 별점 분포입니다."
      >
        {products.length === 0 ? (
          <EmptyHint />
        ) : (
          <div className="flex flex-col gap-6">
            {products.map((product) => (
              <RatingDistribution key={product.url} product={product} />
            ))}
          </div>
        )}
      </Section>

      <Disclaimer />
    </div>
  );
}
