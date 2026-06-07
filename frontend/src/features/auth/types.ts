// ── Domain models ─────────────────────────────────────────────────────────────
export type Plan = 'free' | 'starter' | 'pro';

export interface User {
  id: string;
  email: string;
  plan: Plan;
  created_at?: string;
}

// ── Request types ─────────────────────────────────────────────────────────────
export interface RegisterRequest {
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

// ── Response types (BE response structs의 FE 대응) ─────────────────────────────
export interface AuthResponse {
  token: string;
  user: User;
}
