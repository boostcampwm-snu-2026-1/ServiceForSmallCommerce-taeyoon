import Link from 'next/link';
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

const STATUS_BADGE: Record<AnalysisStatus, string> = {
  pending: 'bg-amber-100 text-amber-800',
  crawling: 'bg-amber-100 text-amber-800',
  analyzing: 'bg-amber-100 text-amber-800',
  completed: 'bg-green-100 text-green-800',
  failed: 'bg-red-100 text-red-800',
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
      className="block rounded-lg border border-gray-200 bg-white p-4 shadow-sm transition hover:border-blue-400 hover:shadow"
    >
      <div className="flex items-center justify-between gap-2">
        <span
          className={`rounded-full px-2.5 py-0.5 text-xs font-medium ${STATUS_BADGE[analysis.status]}`}
        >
          {STATUS_LABEL[analysis.status]}
        </span>
        <span className="text-xs text-gray-500">{formatDate(analysis.created_at)}</span>
      </div>
      <p className="mt-2 truncate text-sm text-gray-800">{summarizeUrls(analysis.urls)}</p>
    </Link>
  );
}
