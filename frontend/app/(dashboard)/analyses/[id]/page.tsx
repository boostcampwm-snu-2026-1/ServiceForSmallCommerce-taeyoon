'use client';

import Link from 'next/link';
import { useParams } from 'next/navigation';
import { useAnalysisPolling } from '@/src/features/analysis/hooks/useAnalysisPolling';
import { InsightReport } from '@/src/features/analysis/components/InsightReport';
import { AnalysisProgress } from '@/src/features/analysis/components/AnalysisProgress';
import { Card } from '@/src/shared/components/ui';
import type { Analysis, AnalysisResult } from '@/src/features/analysis/types';

function BackLink() {
  return (
    <Link href="/dashboard" className="text-sm text-brand-600 hover:underline">
      ← 대시보드로 돌아가기
    </Link>
  );
}

function formatCompletedAt(iso: string): string {
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return iso;
  return date.toLocaleString('ko-KR');
}

function ResultHeader({
  result,
  completedAt,
}: {
  result: AnalysisResult;
  completedAt: string | null;
}) {
  const totalReviews = result.products.reduce(
    (sum, product) => sum + product.total_reviews,
    0,
  );
  return (
    <Card className="p-5">
      <p className="font-semibold text-gray-900">
        {result.products.length}개 상품 · 총 {totalReviews}개 리뷰 분석 완료
      </p>
      {completedAt && (
        <p className="mt-1 text-sm text-gray-500">
          {formatCompletedAt(completedAt)} 생성
        </p>
      )}
    </Card>
  );
}

function ErrorView({ message }: { message: string }) {
  return (
    <Card className="border-red-200 bg-red-50 p-6">
      <p className="font-medium text-red-800">분석에 실패했습니다.</p>
      <p className="mt-1 text-sm text-red-700">{message}</p>
    </Card>
  );
}

function CompletedView({ analysis }: { analysis: Analysis }) {
  if (!analysis.result) return null;
  return (
    <div className="flex flex-col gap-6">
      <ResultHeader result={analysis.result} completedAt={analysis.completed_at} />
      <InsightReport result={analysis.result} />
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
    content = <AnalysisProgress status="pending" />;
  } else if (analysis.status === 'failed') {
    content = <ErrorView message={analysis.error ?? '알 수 없는 오류가 발생했습니다.'} />;
  } else if (analysis.status === 'completed' && analysis.result) {
    content = <CompletedView analysis={analysis} />;
  } else {
    content = <AnalysisProgress status={analysis.status} />;
  }

  return (
    <div className="flex flex-col gap-6">
      <BackLink />
      {content}
    </div>
  );
}
