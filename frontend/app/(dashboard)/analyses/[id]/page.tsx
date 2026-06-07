'use client';

import Link from 'next/link';
import { useParams } from 'next/navigation';
import { useAnalysisPolling } from '@/src/features/analysis/hooks/useAnalysisPolling';
import { InsightReport } from '@/src/features/analysis/components/InsightReport';
import type { AnalysisStatus } from '@/src/features/analysis/types';

const PROGRESS_LABEL: Record<AnalysisStatus, string> = {
  pending: '분석 대기 중',
  crawling: '리뷰 수집 중',
  analyzing: 'AI 분석 중',
  completed: '완료',
  failed: '실패',
};

function BackLink() {
  return (
    <Link href="/dashboard" className="text-sm text-blue-600 hover:underline">
      ← 대시보드로 돌아가기
    </Link>
  );
}

function ProgressView({ status }: { status: AnalysisStatus }) {
  return (
    <div className="rounded-lg border border-gray-200 bg-white p-8 text-center shadow-sm">
      <div
        className="mx-auto mb-4 h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-blue-600"
        role="status"
        aria-label="로딩 중"
      />
      <p className="font-medium text-gray-900">{PROGRESS_LABEL[status]}</p>
      <p className="mt-1 text-sm text-gray-500">분석 중입니다. 잠시만 기다려 주세요.</p>
      <div className="mx-auto mt-4 h-1.5 w-48 overflow-hidden rounded-full bg-gray-100">
        <div className="h-full w-1/2 animate-pulse rounded-full bg-blue-500" />
      </div>
    </div>
  );
}

function ErrorView({ message }: { message: string }) {
  return (
    <div className="rounded-lg border border-red-200 bg-red-50 p-6">
      <p className="font-medium text-red-800">분석에 실패했습니다.</p>
      <p className="mt-1 text-sm text-red-700">{message}</p>
    </div>
  );
}

export default function AnalysisResultPage() {
  const params = useParams<{ id: string }>();
  const id = params?.id ?? '';
  const { data: analysis, isLoading, isError } = useAnalysisPolling(id);

  let content: React.ReactNode;
  if (isError) {
    content = <ErrorView message="분석 정보를 불러오지 못했습니다." />;
  } else if (isLoading || !analysis) {
    content = <ProgressView status="pending" />;
  } else if (analysis.status === 'failed') {
    content = <ErrorView message={analysis.error ?? '알 수 없는 오류가 발생했습니다.'} />;
  } else if (analysis.status === 'completed' && analysis.result) {
    content = <InsightReport result={analysis.result} />;
  } else {
    content = <ProgressView status={analysis.status} />;
  }

  return (
    <div className="flex flex-col gap-6">
      <BackLink />
      {content}
    </div>
  );
}
