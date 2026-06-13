import Link from 'next/link';
import { Container } from '@/src/shared/components/ui';
import { Footer } from '@/src/shared/components/Footer';

export default function TermsPage() {
  return (
    <div className="min-h-screen bg-white">
      <Container className="py-12">
        <h1 className="text-3xl font-bold text-gray-900">이용약관</h1>
        <p className="mt-2 text-sm text-gray-500">시행일: 2026년 6월 14일</p>

        <div className="mt-8 space-y-6 text-gray-700 leading-relaxed">
          <p>
            본 이용약관은 현재 서비스 준비 중인 상태로, 정식 출시 전 임시
            플레이스홀더 문서입니다. 실제 약관 내용은 서비스 정식 출시 시점에
            업데이트될 예정입니다.
          </p>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">제1조 (목적)</h2>
            <p className="mt-2">
              본 약관은 Coupang Review AI(이하 &ldquo;서비스&rdquo;)가 제공하는
              리뷰 분석 및 인사이트 제공 서비스의 이용 조건과 절차, 이용자와
              서비스 운영자의 권리·의무 및 책임 사항을 규정함을 목적으로 합니다.
            </p>
          </section>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">
              제2조 (약관의 효력 및 변경)
            </h2>
            <p className="mt-2">
              본 약관은 서비스 화면에 게시하거나 기타 방법으로 이용자에게
              공지함으로써 효력이 발생합니다. 운영자는 관련 법령을 위반하지 않는
              범위에서 약관을 변경할 수 있으며, 변경 시 적용일자 및 변경 사유를
              명시하여 사전에 공지합니다.
            </p>
          </section>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">
              제3조 (서비스의 제공)
            </h2>
            <p className="mt-2">
              서비스는 쿠팡 경쟁 상품의 공개 리뷰 데이터를 분석하여 개선
              인사이트를 제공합니다. 구체적인 기능 및 이용 범위는 추후 정식
              약관에서 상세히 안내됩니다.
            </p>
          </section>
        </div>

        <div className="mt-12">
          <Link href="/" className="text-brand-600 hover:text-brand-700">
            ← 홈으로 돌아가기
          </Link>
        </div>
      </Container>
      <Footer />
    </div>
  );
}
