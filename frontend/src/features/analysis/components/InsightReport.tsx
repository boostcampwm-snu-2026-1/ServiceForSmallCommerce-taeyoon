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

const SEVERITY_LABEL: Record<string, string> = {
  high: '높음',
  medium: '보통',
  low: '낮음',
};

function severityVariant(severity: string): NonNullable<BadgeProps['variant']> {
  return SEVERITY_VARIANT[severity] ?? 'neutral';
}

function severityLabel(severity: string): string {
  return SEVERITY_LABEL[severity] ?? severity;
}

// ── Accent system ─────────────────────────────────────────────────────────────
type Accent = 'brand' | 'amber' | 'emerald';

const ACCENT: Record<
  Accent,
  { border: string; chipBg: string; chipText: string }
> = {
  brand: {
    border: 'border-l-brand-500',
    chipBg: 'bg-brand-50',
    chipText: 'text-brand-600',
  },
  amber: {
    border: 'border-l-amber-400',
    chipBg: 'bg-amber-50',
    chipText: 'text-amber-600',
  },
  emerald: {
    border: 'border-l-emerald-400',
    chipBg: 'bg-emerald-50',
    chipText: 'text-emerald-600',
  },
};

function Section({
  title,
  description,
  accent,
  icon,
  children,
}: {
  title: string;
  description?: string;
  accent: Accent;
  icon: React.ReactNode;
  children: React.ReactNode;
}) {
  const a = ACCENT[accent];
  return (
    <div
      className={`rounded-xl border border-gray-200 border-l-4 ${a.border} bg-white p-6 shadow-sm transition-shadow hover:shadow-md`}
    >
      <div className="flex items-start gap-3">
        <span
          className={`flex h-9 w-9 shrink-0 items-center justify-center rounded-lg ${a.chipBg} ${a.chipText}`}
          aria-hidden="true"
        >
          {icon}
        </span>
        <div className="min-w-0">
          <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
          {description ? (
            <p className="mt-1 text-sm text-gray-500">{description}</p>
          ) : null}
        </div>
      </div>
      <div className="mt-5">{children}</div>
    </div>
  );
}

// ── Icons (inline SVG) ────────────────────────────────────────────────────────
function TargetIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="h-5 w-5" aria-hidden="true">
      <circle cx="12" cy="12" r="9" />
      <circle cx="12" cy="12" r="5" />
      <circle cx="12" cy="12" r="1.5" fill="currentColor" stroke="none" />
    </svg>
  );
}

function BoltIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="currentColor" className="h-5 w-5" aria-hidden="true">
      <path d="M13 2 4.5 13.5H11l-1 8.5L19.5 10H13l0-8z" />
    </svg>
  );
}

function CartCheckIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-5 w-5" aria-hidden="true">
      <path d="M9 11l2 2 4-4" />
      <path d="M3 4h2l1.6 9.6a1 1 0 0 0 1 .8h8.7a1 1 0 0 0 1-.8L20 7H6" />
      <circle cx="9" cy="20" r="1" />
      <circle cx="17" cy="20" r="1" />
    </svg>
  );
}

function ScaleIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-5 w-5" aria-hidden="true">
      <path d="M12 3v18M6 7h12M5 7l-2.5 5a3 3 0 0 0 5 0L5 7zM19 7l-2.5 5a3 3 0 0 0 5 0L19 7z" />
    </svg>
  );
}

function StarIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="currentColor" className="h-5 w-5" aria-hidden="true">
      <path d="M12 2.5l2.9 5.9 6.5.95-4.7 4.58 1.1 6.47L12 17.9l-5.8 3.05 1.1-6.47-4.7-4.58 6.5-.95L12 2.5z" />
    </svg>
  );
}

function InfoIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="h-4 w-4" aria-hidden="true">
      <circle cx="12" cy="12" r="9" />
      <path d="M12 11v5M12 8h.01" />
    </svg>
  );
}

function Methodology({ products }: { products: ProductSummary[] }) {
  const totalReviews = products.reduce((sum, p) => sum + p.total_reviews, 0);
  const productCount = products.length;
  return (
    <Card className="border-brand-100 bg-brand-50/70 p-5">
      <details>
        <summary className="flex cursor-pointer items-center gap-2 font-medium text-gray-900">
          <span className="flex h-6 w-6 items-center justify-center rounded-md bg-brand-100 text-brand-600">
            <InfoIcon />
          </span>
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
      <div className="flex items-center justify-between gap-2">
        <h3 className="min-w-0 break-words text-sm font-medium text-gray-900">
          {product.product_name}
        </h3>
        <span className="inline-flex shrink-0 items-center gap-1.5 rounded-full bg-amber-50 px-2.5 py-1 text-xs font-medium text-amber-700">
          <span className="text-amber-400">
            <svg viewBox="0 0 20 20" fill="currentColor" className="h-3.5 w-3.5" aria-hidden="true">
              <path d="M10 1.5l2.4 4.9 5.4.8-3.9 3.8.92 5.4L10 13.9 5.18 16.4l.92-5.4L2.2 7.2l5.4-.8L10 1.5z" />
            </svg>
          </span>
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
                  className="h-full rounded-full bg-amber-400 transition-all"
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
        accent="brand"
        icon={<TargetIcon />}
      >
        {improvements.length === 0 ? (
          <EmptyHint />
        ) : (
          <ol className="flex flex-col gap-3">
            {improvements.map((point) => (
              <li
                key={point.rank}
                className="flex gap-3 rounded-lg border border-gray-100 bg-gray-50/60 p-3 transition-colors hover:border-brand-200 hover:bg-brand-50/60"
              >
                <span className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-brand-600 text-sm font-bold text-white shadow-sm shadow-brand-600/30">
                  {point.rank}
                </span>
                <div className="min-w-0">
                  <p className="font-medium text-gray-900 break-words">{point.title}</p>
                  <p className="mt-0.5 text-sm leading-relaxed text-gray-600 break-words">
                    {point.detail}
                  </p>
                </div>
              </li>
            ))}
          </ol>
        )}
      </Section>

      <Section
        title="경쟁사 약점"
        description="경쟁사가 반복적으로 지적받는 지점입니다. 내 상품이 파고들 기회입니다."
        accent="amber"
        icon={<BoltIcon />}
      >
        {insights.competitor_weaknesses.length === 0 ? (
          <EmptyHint />
        ) : (
          <ul className="flex flex-col gap-3">
            {insights.competitor_weaknesses.map((weakness, i) => (
              <li
                key={i}
                className="rounded-lg border border-amber-100 bg-amber-50/50 p-3.5 transition-colors hover:bg-amber-50"
              >
                <p className="font-medium text-gray-900 break-words">{weakness.title}</p>
                <p className="mt-1 flex items-start gap-1.5 text-sm leading-relaxed text-amber-800">
                  <span className="mt-0.5 shrink-0 text-amber-500" aria-hidden="true">
                    <svg viewBox="0 0 20 20" fill="currentColor" className="h-4 w-4">
                      <path d="M10 1l2.5 5.5L18 7l-4 4 1 6-5-2.8L5 17l1-6-4-4 5.5-.5L10 1z" />
                    </svg>
                  </span>
                  <span className="min-w-0 break-words">{weakness.opportunity}</span>
                </p>
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section
        title="구매 결정 요인"
        description="고객이 구매를 결정할 때 실제로 언급한 핵심 요소입니다. 상세페이지·마케팅 메시지에 활용하세요."
        accent="emerald"
        icon={<CartCheckIcon />}
      >
        {insights.purchase_drivers.length === 0 ? (
          <EmptyHint />
        ) : (
          <ul className="flex flex-wrap gap-2">
            {insights.purchase_drivers.map((driver, i) => (
              <li
                key={i}
                className="inline-flex max-w-full items-center gap-1.5 rounded-full border border-emerald-200 bg-emerald-50 px-3 py-1 text-sm font-medium text-emerald-700"
              >
                <span className="shrink-0 text-emerald-500" aria-hidden="true">
                  <svg viewBox="0 0 20 20" fill="currentColor" className="h-3.5 w-3.5">
                    <path fillRule="evenodd" d="M16.7 5.3a1 1 0 010 1.4l-7.5 7.5a1 1 0 01-1.4 0l-3.5-3.5a1 1 0 111.4-1.4l2.8 2.79 6.8-6.79a1 1 0 011.4 0z" clipRule="evenodd" />
                  </svg>
                </span>
                {driver}
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section
        title="반복 불만 / 긍정 요인"
        description="리뷰에 자주 등장한 표현을 빈도순으로 집계했습니다."
        accent="brand"
        icon={<ScaleIcon />}
      >
        <div className="grid gap-4 sm:grid-cols-2">
          <div className="rounded-lg border border-red-100 bg-red-50/40 p-4">
            <h3 className="mb-3 flex items-center gap-1.5 text-sm font-semibold text-red-700">
              <span className="text-red-400" aria-hidden="true">
                <svg viewBox="0 0 20 20" fill="currentColor" className="h-4 w-4">
                  <path d="M10 2l8 14H2L10 2z" />
                </svg>
              </span>
              반복 불만
            </h3>
            {insights.top_complaints.length === 0 ? (
              <EmptyHint />
            ) : (
              <ul className="flex flex-col gap-2">
                {insights.top_complaints.map((c, i) => (
                  <li key={i} className="flex items-center justify-between gap-2">
                    <span className="min-w-0 break-words text-sm text-gray-800">{c.text}</span>
                    <span className="flex shrink-0 items-center gap-2">
                      <Badge variant={severityVariant(c.severity)}>
                        {severityLabel(c.severity)}
                      </Badge>
                      <span className="text-xs text-gray-500">{c.count}회</span>
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div className="rounded-lg border border-green-100 bg-green-50/40 p-4">
            <h3 className="mb-3 flex items-center gap-1.5 text-sm font-semibold text-green-700">
              <span className="text-green-500" aria-hidden="true">
                <svg viewBox="0 0 20 20" fill="currentColor" className="h-4 w-4">
                  <path fillRule="evenodd" d="M16.7 5.3a1 1 0 010 1.4l-7.5 7.5a1 1 0 01-1.4 0l-3.5-3.5a1 1 0 111.4-1.4l2.8 2.79 6.8-6.79a1 1 0 011.4 0z" clipRule="evenodd" />
                </svg>
              </span>
              긍정 요인
            </h3>
            {insights.top_positives.length === 0 ? (
              <EmptyHint />
            ) : (
              <ul className="flex flex-col gap-2">
                {insights.top_positives.map((p, i) => (
                  <li key={i} className="flex items-center justify-between gap-2">
                    <span className="min-w-0 break-words text-sm text-gray-800">{p.text}</span>
                    <span className="shrink-0 text-xs text-gray-500">{p.count}회</span>
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
        accent="amber"
        icon={<StarIcon />}
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
