import { apiClient } from '@/src/shared/api/client';
import type { AuthResponse, LoginRequest, RegisterRequest } from '../types';

// 모든 API 함수는 명명된 타입 사용. 인라인 타입(Promise<{ field: Type }>) 금지.
export async function register(req: RegisterRequest): Promise<AuthResponse> {
  return apiClient.post('/api/v1/auth/register', req);
}

export async function login(req: LoginRequest): Promise<AuthResponse> {
  return apiClient.post('/api/v1/auth/login', req);
}
