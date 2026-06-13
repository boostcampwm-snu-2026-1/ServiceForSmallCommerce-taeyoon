import Link from 'next/link';
import { Button, Card, Badge, Container } from '@/src/shared/components/ui';
import { Footer } from '@/src/shared/components/Footer';
import { Wordmark } from '@/src/shared/components/Wordmark';

interface ValueItem {
  icon: string;
  title: string;
  description: string;
}

interface StepItem {
  step: string;
  title: string;
  description: string;
}

interface TrustItem {
  title: string;
  description: string;
}

interface PlanItem {
  name: string;
  price: string;
  period: string;
  features: string[];
  recommended?: boolean;
}

const VALUES: ValueItem[] = [
  {
    icon: '🔁',
    title: '반복 불만 패턴 자동 추출',
    description:
      '수백 건의 리뷰에서 반복적으로 등장하는 불만을 AI가 묶어, 흩어진 목소리를 한눈에 보이는 패턴으로 정리합니다.',
  },
  {
    icon: '🎯',
    title: '경쟁사 약점 = 내 기회',
    description:
      '경쟁 상품이 받는 비판을 우리 제품의 차별화 포인트로 환산해, 무엇을 강조하면 이기는지 알려드립니다.',
  },
  {
    icon: '🏆',
    title: '실행 가능한 개선 우선순위 TOP',
    description:
      '단순 통계가 아니라 "지금 무엇부터 고쳐야 하는지" 우선순위가 매겨진 액션 리스트를 제공합니다.',
  },
];

const STEPS: StepItem[] = [
  {
    step: '1',
    title: 'URL 입력',
    description: '분석하고 싶은 경쟁 상품의 쿠팡 URL을 붙여넣습니다.',
  },
  {
    step: '2',
    title: '리뷰 자동 수집',
    description: '공개된 최근 리뷰 수백 건을 자동으로 모읍니다.',
  },
  {
    step: '3',
    title: 'AI 분석',
    description: '한국어 리뷰를 LLM이 분류하고 집계해 패턴을 찾아냅니다.',
  },
  {
    step: '4',
    title: '개선 리포트',
    description: '우선순위가 매겨진 실행 가능한 개선 리포트를 받습니다.',
  },
];

const TRUST: TrustItem[] = [
  {
    title: '데이터 출처와 수집',
    description:
      '쿠팡 공개 리뷰를 기반으로, 최근 리뷰 수백 건을 자동으로 수집합니다. 비공개 정보나 개인 식별 정보는 다루지 않습니다.',
  },
  {
    title: 'AI 분석 방법론',
    description:
      '한국어 리뷰를 LLM이 주제별로 분류하고 집계해 반복되는 패턴으로 구조화합니다. 단순 키워드 카운팅이 아니라 맥락 기반 분석입니다.',
  },
  {
    title: '보안과 개인정보',
    description:
      '전송 구간은 HTTPS로 암호화되며, 분석 데이터는 계정 단위로만 보관됩니다. 다른 사용자와 데이터가 섞이지 않습니다.',
  },
];

const PLANS: PlanItem[] = [
  {
    name: '무료',
    price: '₩0',
    period: '',
    features: ['월 3회 분석', '리뷰 50개 수집', '기본 개선 리포트'],
  },
  {
    name: '스타터',
    price: '₩19,900',
    period: '/월',
    features: ['월 30회 분석', '리뷰 200개 수집', '상세 개선 리포트'],
  },
  {
    name: '프로',
    price: '₩49,900',
    period: '/월',
    features: ['무제한 분석', '리뷰 500개 수집', 'PDF 내보내기'],
    recommended: true,
  },
];

function Navbar() {
  return (
    <header className="border-b border-gray-100 bg-white">
      <Container className="flex h-16 items-center justify-between">
        <Wordmark />
        <nav className="flex items-center gap-2">
          <Link href="/login">
            <Button variant="outline" size="sm">
              로그인
            </Button>
          </Link>
          <Link href="/register">
            <Button variant="primary" size="sm">
              무료로 시작
            </Button>
          </Link>
        </nav>
      </Container>
    </header>
  );
}

function Hero() {
  return (
    <section className="bg-gradient-to-b from-brand-950 to-brand-800 text-white">
      <Container className="py-20 text-center sm:py-28">
        <h1 className="mx-auto max-w-3xl text-4xl font-bold leading-tight sm:text-5xl">
          리뷰가 알려주는 경쟁사를 이기는 법
        </h1>
        <p className="mx-auto mt-6 max-w-2xl text-lg text-brand-100">
          경쟁 상품의 쿠팡 리뷰를 AI가 분석해, 데이터 나열이 아니라 &lsquo;무엇을
          고치면 이기는지&rsquo;까지 알려드립니다.
        </p>
        <div className="mt-10 flex flex-col items-center justify-center gap-3 sm:flex-row">
          <Link href="/register">
            <Button variant="primary" size="lg">
              무료로 시작하기
            </Button>
          </Link>
          <Link href="/login">
            <Button
              variant="outline"
              size="lg"
              className="border-white/40 bg-transparent text-white hover:bg-white/10"
            >
              로그인
            </Button>
          </Link>
        </div>
      </Container>
    </section>
  );
}

function Values() {
  return (
    <section id="features" className="bg-white">
      <Container className="py-20">
        <h2 className="text-center text-3xl font-bold text-gray-900">
          핵심 가치
        </h2>
        <div className="mt-12 grid grid-cols-1 gap-6 md:grid-cols-3">
          {VALUES.map((item) => (
            <Card key={item.title} className="p-6">
              <div className="text-3xl" aria-hidden="true">
                {item.icon}
              </div>
              <h3 className="mt-4 text-lg font-semibold text-gray-900">
                {item.title}
              </h3>
              <p className="mt-2 text-sm text-gray-600">{item.description}</p>
            </Card>
          ))}
        </div>
      </Container>
    </section>
  );
}

function HowItWorks() {
  return (
    <section className="bg-gray-50">
      <Container className="py-20">
        <h2 className="text-center text-3xl font-bold text-gray-900">
          작동 방식
        </h2>
        <div className="mt-12 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
          {STEPS.map((item) => (
            <div key={item.step} className="text-center sm:text-left">
              <span className="inline-flex h-10 w-10 items-center justify-center rounded-full bg-brand-600 text-base font-bold text-white">
                {item.step}
              </span>
              <h3 className="mt-4 text-lg font-semibold text-gray-900">
                {item.title}
              </h3>
              <p className="mt-2 text-sm text-gray-600">{item.description}</p>
            </div>
          ))}
        </div>
      </Container>
    </section>
  );
}

function Trust() {
  return (
    <section id="about" className="bg-white">
      <Container className="py-20">
        <h2 className="text-center text-3xl font-bold text-gray-900">
          신뢰할 수 있는 분석
        </h2>
        <p className="mx-auto mt-4 max-w-2xl text-center text-sm text-gray-600">
          어떻게 데이터를 모으고, 분석하고, 안전하게 다루는지 투명하게
          공개합니다.
        </p>
        <div className="mt-12 grid grid-cols-1 gap-6 md:grid-cols-3">
          {TRUST.map((item) => (
            <Card key={item.title} className="p-6">
              <h3 className="text-base font-semibold text-gray-900">
                {item.title}
              </h3>
              <p className="mt-2 text-sm text-gray-600">{item.description}</p>
            </Card>
          ))}
        </div>
      </Container>
    </section>
  );
}

function Pricing() {
  return (
    <section id="pricing" className="bg-gray-50">
      <Container className="py-20">
        <h2 className="text-center text-3xl font-bold text-gray-900">요금제</h2>
        <p className="mx-auto mt-4 max-w-2xl text-center text-sm text-gray-600">
          무료로 시작하고, 필요할 때 업그레이드하세요.
        </p>
        <div className="mt-12 grid grid-cols-1 gap-6 md:grid-cols-3">
          {PLANS.map((plan) => (
            <Card
              key={plan.name}
              className={
                plan.recommended
                  ? 'relative border-brand-600 p-6 shadow-md ring-1 ring-brand-600'
                  : 'p-6'
              }
            >
              {plan.recommended ? (
                <Badge variant="info" className="absolute right-6 top-6">
                  추천
                </Badge>
              ) : null}
              <h3 className="text-lg font-semibold text-gray-900">
                {plan.name}
              </h3>
              <p className="mt-4">
                <span className="text-3xl font-bold text-gray-900">
                  {plan.price}
                </span>
                <span className="text-sm text-gray-500">{plan.period}</span>
              </p>
              <ul className="mt-6 space-y-3 text-sm text-gray-600">
                {plan.features.map((feature) => (
                  <li key={feature} className="flex items-start gap-2">
                    <span className="text-brand-600" aria-hidden="true">
                      ✓
                    </span>
                    <span>{feature}</span>
                  </li>
                ))}
              </ul>
              <Link href="/register" className="mt-8 block">
                <Button
                  variant={plan.recommended ? 'primary' : 'outline'}
                  className="w-full"
                >
                  시작하기
                </Button>
              </Link>
            </Card>
          ))}
        </div>
      </Container>
    </section>
  );
}

function CtaBand() {
  return (
    <section className="bg-brand-600 text-white">
      <Container className="py-16 text-center">
        <h2 className="text-3xl font-bold">지금 경쟁사 리뷰를 분석해보세요</h2>
        <p className="mt-3 text-brand-100">
          가입은 무료입니다. 카드 등록 없이 바로 시작하세요.
        </p>
        <div className="mt-8">
          <Link href="/register">
            <Button
              variant="outline"
              size="lg"
              className="border-white bg-white text-brand-700 hover:bg-brand-50"
            >
              무료로 시작하기
            </Button>
          </Link>
        </div>
      </Container>
    </section>
  );
}

export function Landing() {
  return (
    <div className="flex min-h-screen flex-col">
      <Navbar />
      <main className="flex-1">
        <Hero />
        <Values />
        <HowItWorks />
        <Trust />
        <Pricing />
        <CtaBand />
      </main>
      <Footer />
    </div>
  );
}
