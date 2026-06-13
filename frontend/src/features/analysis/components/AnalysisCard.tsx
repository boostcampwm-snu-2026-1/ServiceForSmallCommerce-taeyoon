import Link from 'next/link';
import { Badge, type BadgeProps } from '@/src/shared/components/ui';
import type { Analysis, AnalysisStatus } from '@/src/features/analysis/types';

interface AnalysisCardProps {
  analysis: Pick<Analysis, 'id' | 'status' | 'urls' | 'created_at'>;
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

function summarizeUrls(urls: string[]): string {
  if (urls.length === 0) return 'URL 없음';
  const [first, ...rest] = urls;
  return rest.length > 0 ? `${first} 외 ${rest.length}개` : first;
}

function formatDate(iso: string): string {
  const d = new Date(iso);
  return Number.isNaN(d.getTime()) ? iso : d.toLocaleString('ko-KR');
}

export function AnalysisCard({ analysis }: AnalysisCardProps) {
  return (
    <Link
      href={`/analyses/${analysis.id}`}
      className="block rounded-lg border border-gray-200 bg-white p-4 shadow-sm transition hover:border-brand-400 hover:shadow"
    >
      <div className="flex items-center justify-between gap-2">
        <Badge variant={STATUS_VARIANT[analysis.status]}>{STATUS_LABEL[analysis.status]}</Badge>
        <span className="text-xs text-gray-500">{formatDate(analysis.created_at)}</span>
      </div>
      <p className="mt-2 truncate text-sm text-gray-800">{summarizeUrls(analysis.urls)}</p>
    </Link>
  );
}
