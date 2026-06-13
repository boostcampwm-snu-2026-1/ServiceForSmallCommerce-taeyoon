import Link from 'next/link';
import { Container } from '@/src/shared/components/ui';
import { Footer } from '@/src/shared/components/Footer';

export default function PrivacyPage() {
  return (
    <div className="min-h-screen bg-white">
      <Container className="py-12">
        <h1 className="text-3xl font-bold text-gray-900">개인정보 처리방침</h1>
        <p className="mt-2 text-sm text-gray-500">시행일: 2026년 6월 14일</p>

        <div className="mt-8 space-y-6 text-gray-700 leading-relaxed">
          <p>
            본 개인정보 처리방침은 현재 서비스 준비 중인 상태로, 정식 출시 전
            임시 플레이스홀더 문서입니다. 실제 내용은 서비스 정식 출시 시점에
            업데이트될 예정입니다.
          </p>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">
              1. 수집하는 개인정보 항목
            </h2>
            <p className="mt-2">
              서비스는 회원 가입 및 서비스 제공을 위해 이메일 주소 등 최소한의
              정보를 수집할 수 있습니다. 구체적인 수집 항목은 정식 출시 시점에
              명시됩니다.
            </p>
          </section>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">
              2. 개인정보의 이용 목적
            </h2>
            <p className="mt-2">
              수집한 개인정보는 회원 식별, 서비스 제공 및 운영, 고객 문의 응대
              등의 목적으로만 이용되며, 명시된 목적 외의 용도로는 사용되지
              않습니다.
            </p>
          </section>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">
              3. 개인정보의 보관 기간
            </h2>
            <p className="mt-2">
              개인정보는 수집 및 이용 목적이 달성된 후 관련 법령에 따른 보관
              기간을 준수하여 지체 없이 파기합니다. 구체적인 보관 기간은 정식
              방침에서 안내됩니다.
            </p>
          </section>

          <section>
            <h2 className="text-xl font-semibold text-gray-900">
              4. 이용자의 권리
            </h2>
            <p className="mt-2">
              이용자는 언제든지 자신의 개인정보에 대한 열람·정정·삭제 및 처리
              정지를 요청할 수 있습니다. 요청 방법은 정식 출시 시점에
              안내됩니다.
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
