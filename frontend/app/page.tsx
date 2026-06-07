'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuthStore } from '@/src/features/auth/store';

/**
 * 루트 진입점. 별도 랜딩 화면 없이 인증 상태에 따라 리다이렉트한다.
 * - 로그인 상태(토큰 보유) → /dashboard
 * - 비로그인 → /login
 *
 * 토큰은 zustand persist(localStorage)에서 복원되지만, SSR/CSR 간
 * 하이드레이션 미스매치를 피하려고 mount 이후에만 판별·이동한다.
 */
export default function Home() {
  const router = useRouter();
  const token = useAuthStore((s) => s.token);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    if (mounted) {
      router.replace(token ? '/dashboard' : '/login');
    }
  }, [mounted, token, router]);

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50">
      <span className="text-sm text-gray-500">로딩 중...</span>
    </div>
  );
}
