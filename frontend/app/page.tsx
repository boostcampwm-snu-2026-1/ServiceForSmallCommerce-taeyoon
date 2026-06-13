'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuthStore } from '@/src/features/auth/store';
import { Landing } from '@/src/shared/components/Landing';

/**
 * 루트 진입점. 인증 상태에 따라 화면을 분기한다.
 * - 로그인 상태(토큰 보유) → /dashboard 로 리다이렉트
 * - 비로그인 → 랜딩 페이지(<Landing/>)를 렌더 (리다이렉트하지 않음)
 *
 * 토큰은 zustand persist(localStorage)에서 복원되지만, SSR/CSR 간
 * 하이드레이션 미스매치를 피하려고 mount 이후에만 판별한다.
 * 마운트 전에는 깜빡임을 막기 위해 빈 화면을 잠깐 보여주고,
 * 마운트 후 토큰이 없으면 랜딩을, 토큰이 있으면 대시보드로 보낸다.
 */
export default function Home() {
  const router = useRouter();
  const token = useAuthStore((s) => s.token);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    if (mounted && token) {
      router.replace('/dashboard');
    }
  }, [mounted, token, router]);

  // 인증된 사용자는 대시보드로 이동 중이므로 랜딩 대신 빈 화면을 잠깐 노출.
  if (mounted && token) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-gray-50">
        <span className="text-sm text-gray-500">로딩 중...</span>
      </div>
    );
  }

  // 미인증 방문자(또는 마운트 전)에게는 랜딩 페이지를 보여준다.
  return <Landing />;
}
