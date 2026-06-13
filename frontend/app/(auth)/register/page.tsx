'use client';

import { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { register } from '@/src/features/auth/api';
import { useAuthStore } from '@/src/features/auth/store';
import { Button, Card, Input } from '@/src/shared/components/ui';
import { Wordmark } from '@/src/shared/components/Wordmark';

export default function RegisterPage() {
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      const res = await register({ email, password });
      useAuthStore.getState().setAuth(res.token, res.user);
      router.push('/dashboard');
    } catch (err) {
      setError(err instanceof Error ? err.message : '회원가입에 실패했습니다.');
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="flex min-h-screen items-center justify-center bg-brand-50/40 p-4">
      <Card className="w-full max-w-sm p-8">
        <Wordmark className="mb-6" />
        <h1 className="mb-6 text-2xl font-bold text-gray-900">회원가입</h1>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <label className="flex flex-col gap-1 text-sm font-medium text-gray-700">
            이메일
            <Input
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </label>
          <label className="flex flex-col gap-1 text-sm font-medium text-gray-700">
            비밀번호
            <Input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
              minLength={8}
            />
          </label>
          <p className="text-xs text-gray-500">비밀번호는 8자 이상이어야 합니다.</p>
          {error && <p className="text-sm text-red-600">{error}</p>}
          <Button type="submit" disabled={loading} className="mt-2">
            {loading ? '가입 중...' : '회원가입'}
          </Button>
        </form>
        <p className="mt-4 text-center text-sm text-gray-600">
          이미 계정이 있으신가요?{' '}
          <Link href="/login" className="font-medium text-brand-600 hover:underline">
            로그인
          </Link>
        </p>
      </Card>
    </main>
  );
}
