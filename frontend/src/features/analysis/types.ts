// ── Domain models ─────────────────────────────────────────────────────────────
export type AnalysisStatus =
  | 'pending'
  | 'crawling'
  | 'analyzing'
  | 'completed'
  | 'failed';

export interface ProductSummary {
  url: string;
  product_name: string;
  total_reviews: number;
  avg_rating: number;
  rating_distribution: Record<string, number>;
}

export interface Insights {
  top_complaints: { text: string; count: number; severity: string }[];
  top_positives: { text: string; count: number }[];
  improvement_points: { rank: number; title: string; detail: string }[];
  competitor_weaknesses: { title: string; opportunity: string }[];
  purchase_drivers: string[];
}

export interface AnalysisResult {
  products: ProductSummary[];
  insights: Insights;
}

export interface Analysis {
  id: string;
  status: AnalysisStatus;
  urls: string[];
  result: AnalysisResult | null;
  error: string | null;
  created_at: string;
  completed_at: string | null;
}

// ── Request types ─────────────────────────────────────────────────────────────
export interface CreateAnalysisRequest {
  urls: string[];
  review_limit: number;
}

// ── Response types ────────────────────────────────────────────────────────────
export interface CreateAnalysisResponse {
  analysis_id: string;
  status: AnalysisStatus;
  created_at: string;
}

export interface ListAnalysesResponse {
  analyses: Pick<Analysis, 'id' | 'status' | 'urls' | 'created_at'>[];
  total: number;
  page: number;
  per_page: number;
}
