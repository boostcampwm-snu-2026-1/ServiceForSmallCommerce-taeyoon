import { useAuthStore } from '@/src/features/auth/store';

const API_BASE = process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:8080';

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  // 인증 토큰이 있으면 Authorization 헤더로 첨부한다.
  // (백엔드 AuthUser 추출기는 `Authorization: Bearer <token>` 가 없으면 401)
  const token = useAuthStore.getState().token;
  const authHeader: Record<string, string> = token
    ? { Authorization: `Bearer ${token}` }
    : {};

  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...authHeader, ...init?.headers },
    ...init,
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(err.error ?? res.statusText);
  }
  return res.json() as Promise<T>;
}

export const apiClient = {
  get: <T>(path: string) => request<T>(path),
  post: <T>(path: string, body: unknown) =>
    request<T>(path, { method: 'POST', body: JSON.stringify(body) }),
};
