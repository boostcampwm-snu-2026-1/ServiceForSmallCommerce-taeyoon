import type { AnalysisResult, ProductSummary } from '@/src/features/analysis/types';

interface InsightReportProps {
  result: AnalysisResult;
}

const SEVERITY_BADGE: Record<string, string> = {
  high: 'bg-red-100 text-red-800',
  medium: 'bg-amber-100 text-amber-800',
  low: 'bg-gray-100 text-gray-700',
};

function severityBadge(severity: string): string {
  return SEVERITY_BADGE[severity] ?? 'bg-gray-100 text-gray-700';
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
      <h2 className="mb-4 text-lg font-semibold text-gray-900">{title}</h2>
      {children}
    </section>
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
      <Section title="고쳐야 할 것 TOP 3">
        {improvements.length === 0 ? (
          <EmptyHint />
        ) : (
          <ol className="flex flex-col gap-4">
            {improvements.map((point) => (
              <li key={point.rank} className="flex gap-3">
                <span className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-blue-600 text-sm font-bold text-white">
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

      <Section title="경쟁사 약점">
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

      <Section title="구매 결정 요인">
        {insights.purchase_drivers.length === 0 ? (
          <EmptyHint />
        ) : (
          <ul className="flex flex-wrap gap-2">
            {insights.purchase_drivers.map((driver, i) => (
              <li
                key={i}
                className="rounded-full bg-blue-50 px-3 py-1 text-sm text-blue-700"
              >
                {driver}
              </li>
            ))}
          </ul>
        )}
      </Section>

      <Section title="반복 불만 / 긍정 요인">
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
                      <span
                        className={`rounded-full px-2 py-0.5 text-xs font-medium ${severityBadge(c.severity)}`}
                      >
                        {c.severity}
                      </span>
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

      <Section title="평점 분포">
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
    </div>
  );
}
