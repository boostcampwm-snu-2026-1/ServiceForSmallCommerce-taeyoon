import { useQuery } from '@tanstack/react-query';
import { getAnalysis } from '@/src/features/analysis/api';
import type { AnalysisStatus } from '@/src/features/analysis/types';

/**
 * 폴링 간격 결정 (순수 함수).
 * completed/failed 상태면 폴링 중단(false), 그 외(진행 중/미정)는 2초.
 */
export function pollInterval(status: AnalysisStatus | undefined): number | false {
  return status === 'completed' || status === 'failed' ? false : 2000;
}

export function useAnalysisPolling(analysisId: string) {
  return useQuery({
    queryKey: ['analysis', analysisId],
    queryFn: () => getAnalysis(analysisId),
    refetchInterval: (query) => pollInterval(query.state.data?.status),
    enabled: !!analysisId,
  });
}
