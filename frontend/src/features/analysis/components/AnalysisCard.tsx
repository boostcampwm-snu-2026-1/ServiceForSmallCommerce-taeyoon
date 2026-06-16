import Link from 'next/link';
import { Badge, type BadgeProps } from '@/src/shared/components/ui';
import type { Analysis, AnalysisStatus } from '@/src/features/analysis/types';

interface AnalysisCardProps {
  analysis: Pick<Analysis, 'id' | 'status' | 'urls' | 'my_url' | 'created_at'>;
}

const STATUS_LABEL: Record<AnalysisStatus, string> = {
  pending: '대기 중',
  crawling: '수집 중',
  analyzing: '분석 중',
  completed: '완료',
  failed: '실패',
};

const STATUS_VARIANT: Record<AnalysisStatus, NonNullable<BadgeProps['variant']>> = {
  pending: 'warning',
  crawling: 'warning',
  analyzing: 'warning',
  completed: 'success',
  failed: 'danger',
};

const STATUS_DOT: Record<AnalysisStatus, string> = {
  pending: 'bg-amber-400',
  crawling: 'bg-amber-400',
  analyzing: 'bg-amber-400',
  completed: 'bg-emerald-500',
  failed: 'bg-red-500',
};

function summarizeUrls(urls: string[]): string {
  if (urls.length === 0) return 'URL 없음';
  const [first, ...rest] = urls;
  return rest.length > 0 ? `${first} 외 ${rest.length}개` : first;
}

function summarizeAnalysis(
  myUrl: string | null,
  urls: string[],
): string {
  if (myUrl) {
    return `내 제품 vs 경쟁 ${urls.length}개`;
  }
  return summarizeUrls(urls);
}

function formatDate(iso: string): string {
  const d = new Date(iso);
  return Number.isNaN(d.getTime()) ? iso : d.toLocaleString('ko-KR');
}

export function AnalysisCard({ analysis }: AnalysisCardProps) {
  const isCompleted = analysis.status === 'completed';
  return (
    <Link
      href={`/analyses/${analysis.id}`}
      className="group block min-w-0 rounded-xl border border-gray-200 bg-white p-4 shadow-sm transition hover:-translate-y-0.5 hover:border-brand-300 hover:shadow-md"
    >
      <div className="flex items-center justify-between gap-2">
        <Badge variant={STATUS_VARIANT[analysis.status]} className="inline-flex items-center gap-1.5">
          <span className={`h-1.5 w-1.5 rounded-full ${STATUS_DOT[analysis.status]}`} aria-hidden="true" />
          {STATUS_LABEL[analysis.status]}
        </Badge>
        <span className="shrink-0 whitespace-nowrap text-xs text-gray-400">{formatDate(analysis.created_at)}</span>
      </div>
      <div className="mt-3 flex min-w-0 items-center gap-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth={1.8}
          strokeLinecap="round"
          strokeLinejoin="round"
          className="h-4 w-4 shrink-0 text-gray-400"
          aria-hidden="true"
        >
          <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
          <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
        </svg>
        <p className="truncate text-sm text-gray-800">
          {summarizeAnalysis(analysis.my_url, analysis.urls)}
        </p>
      </div>
      {isCompleted && (
        <p className="mt-3 text-right text-xs font-medium text-brand-600">
          결과 보기 <span className="transition group-hover:translate-x-0.5">→</span>
        </p>
      )}
    </Link>
  );
}
