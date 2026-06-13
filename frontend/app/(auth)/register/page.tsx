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
    <main className="min-h-screen lg:grid lg:grid-cols-2">
      <section className="hidden flex-col justify-between bg-gradient-to-br from-brand-800 to-brand-950 p-12 text-white lg:flex">
        <span className="inline-flex items-center gap-2">
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect width="24" height="24" rx="6" className="fill-white/15" />
            <path
              d="M7 12.5L10.5 16L17 8.5"
              stroke="white"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
          <span className="font-semibold text-white">Coupang Review AI</span>
        </span>
        <div className="max-w-md">
          <h2 className="text-3xl font-bold leading-tight">리뷰 속에 답이 있습니다</h2>
          <p className="mt-4 text-brand-100">
            경쟁 상품의 쿠팡 리뷰를 AI가 분석해, 무엇을 고치면 이기는지 알려드립니다.
          </p>
          <ul className="mt-8 flex flex-col gap-3 text-brand-100">
            <li>✓ 최근 리뷰 수백 건 자동 수집</li>
            <li>✓ 불만·강점 패턴 자동 추출</li>
            <li>✓ 실행 가능한 개선 우선순위 제시</li>
          </ul>
        </div>
        <p className="text-sm text-brand-200">© Coupang Review AI</p>
      </section>
      <div className="flex min-h-screen items-center justify-center bg-brand-50/40 p-6 lg:min-h-0">
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
      </div>
    </main>
  );
}
