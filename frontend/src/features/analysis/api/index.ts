import { apiClient } from '@/src/shared/api/client';
import type {
  Analysis,
  CreateAnalysisRequest,
  CreateAnalysisResponse,
  ListAnalysesResponse,
} from '../types';

export async function createAnalysis(
  req: CreateAnalysisRequest,
): Promise<CreateAnalysisResponse> {
  return apiClient.post('/api/v1/analyses', req);
}

export async function getAnalysis(analysisId: string): Promise<Analysis> {
  return apiClient.get(`/api/v1/analyses/${analysisId}`);
}

export async function listAnalyses(): Promise<ListAnalysesResponse> {
  return apiClient.get('/api/v1/analyses');
}
