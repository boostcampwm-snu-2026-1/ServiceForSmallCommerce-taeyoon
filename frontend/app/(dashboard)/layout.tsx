'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuthStore } from '@/src/features/auth/store';
import { Button, Container } from '@/src/shared/components/ui';
import { Wordmark } from '@/src/shared/components/Wordmark';

export default function DashboardLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  const router = useRouter();
  const token = useAuthStore((s) => s.token);
  const logout = useAuthStore((s) => s.logout);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    if (mounted && !token) {
      router.replace('/login');
    }
  }, [mounted, token, router]);

  if (!mounted || !token) {
    return null;
  }

  function handleLogout() {
    logout();
    router.replace('/login');
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="sticky top-0 z-10 border-b border-gray-200 bg-white/80 backdrop-blur">
        <Container className="flex items-center justify-between py-3">
          <Wordmark />
          <Button variant="outline" size="sm" onClick={handleLogout}>
            로그아웃
          </Button>
        </Container>
      </header>
      <Container className="py-8">{children}</Container>
    </div>
  );
}
