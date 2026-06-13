import Link from 'next/link';
import { Container } from '@/src/shared/components/ui';

export function Footer() {
  return (
    <footer className="bg-brand-950 text-brand-100">
      <Container className="py-12">
        <div className="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-5">
          <div className="lg:col-span-2">
            <span className="text-lg font-semibold text-white">
              Coupang Review AI
            </span>
            <p className="mt-3 max-w-xs text-sm text-brand-300">
              쿠팡 경쟁 상품 리뷰를 AI가 분석해 개선 인사이트를 제공합니다.
            </p>
          </div>

          <div>
            <h2 className="text-sm font-semibold text-white">제품</h2>
            <ul className="mt-3 space-y-2 text-sm">
              <li>
                <Link href="/#features" className="hover:text-white">
                  기능
                </Link>
              </li>
              <li>
                <Link href="/#pricing" className="hover:text-white">
                  요금
                </Link>
              </li>
            </ul>
          </div>

          <div>
            <h2 className="text-sm font-semibold text-white">회사</h2>
            <ul className="mt-3 space-y-2 text-sm">
              <li>
                <Link href="/#about" className="hover:text-white">
                  소개
                </Link>
              </li>
            </ul>
          </div>

          <div>
            <h2 className="text-sm font-semibold text-white">법적</h2>
            <ul className="mt-3 space-y-2 text-sm">
              <li>
                <Link href="/terms" className="hover:text-white">
                  이용약관
                </Link>
              </li>
              <li>
                <Link href="/privacy" className="hover:text-white">
                  개인정보 처리방침
                </Link>
              </li>
            </ul>
          </div>
        </div>

        <div className="mt-8 text-sm">
          <a
            href="mailto:support@coupang-review-ai.example"
            className="hover:text-white"
          >
            문의: support@coupang-review-ai.example
          </a>
        </div>

        <div className="mt-8 border-t border-brand-800 pt-6">
          <p className="text-sm text-brand-300">
            © 2026 Coupang Review AI. All rights reserved.
          </p>
        </div>
      </Container>
    </footer>
  );
}
