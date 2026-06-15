import type { AnalysisStatus } from '@/src/features/analysis/types';

const PROGRESS_LABEL: Record<AnalysisStatus, string> = {
  pending: '분석 대기 중',
  crawling: '리뷰 수집 중',
  analyzing: 'AI 분석 중',
  completed: '분석 완료',
  failed: '분석 실패',
};

type StepKey = 'crawling' | 'analyzing' | 'completed';

interface StepDef {
  key: StepKey;
  label: string;
}

const STEPS: StepDef[] = [
  { key: 'crawling', label: '리뷰 수집' },
  { key: 'analyzing', label: 'AI 분석' },
  { key: 'completed', label: '분석 완료' },
];

type StepState = 'done' | 'current' | 'upcoming';

function stepStateFor(stepIndex: number, status: AnalysisStatus): StepState {
  // 'pending' 은 아직 어떤 단계도 시작되지 않음 → 모두 upcoming
  const order: Record<StepKey, number> = { crawling: 0, analyzing: 1, completed: 2 };

  if (status === 'pending') {
    return 'upcoming';
  }

  const currentIndex =
    status === 'completed' ? order.completed : order[status as StepKey];

  if (status === 'completed') {
    // 완료 시 모든 단계 done
    return 'done';
  }
  if (stepIndex < currentIndex) return 'done';
  if (stepIndex === currentIndex) return 'current';
  return 'upcoming';
}

function CheckIcon() {
  return (
    <svg
      viewBox="0 0 20 20"
      fill="currentColor"
      className="h-5 w-5"
      aria-hidden="true"
    >
      <path
        fillRule="evenodd"
        d="M16.7 5.3a1 1 0 010 1.4l-7.5 7.5a1 1 0 01-1.4 0l-3.5-3.5a1 1 0 111.4-1.4l2.8 2.79 6.8-6.79a1 1 0 011.4 0z"
        clipRule="evenodd"
      />
    </svg>
  );
}

function StepIndicator({ state, index }: { state: StepState; index: number }) {
  if (state === 'done') {
    return (
      <span className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-brand-600 text-white shadow-sm shadow-brand-600/30 transition-colors duration-300">
        <CheckIcon />
      </span>
    );
  }
  if (state === 'current') {
    return (
      <span className="relative flex h-11 w-11 shrink-0 items-center justify-center rounded-full border-2 border-brand-600 bg-brand-50 text-sm font-semibold text-brand-700 transition-colors duration-300">
        <span className="absolute inset-0 animate-ping rounded-full border-2 border-brand-400/60" aria-hidden="true" />
        <span className="relative">{index + 1}</span>
      </span>
    );
  }
  return (
    <span className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full border-2 border-gray-200 bg-gray-50 text-sm font-semibold text-gray-400 transition-colors duration-300">
      {index + 1}
    </span>
  );
}

export function AnalysisProgress({ status }: { status: AnalysisStatus }) {
  const isActive = status !== 'completed' && status !== 'failed';

  return (
    <div className="rounded-2xl border border-gray-200 bg-white p-8 shadow-sm">
      <ol className="flex items-start">
        {STEPS.map((step, index) => {
          const state = stepStateFor(index, status);
          const isLast = index === STEPS.length - 1;
          const connectorDone = stepStateFor(index + 1, status) !== 'upcoming';
          return (
            <li
              key={step.key}
              className={isLast ? 'flex flex-col items-center' : 'flex flex-1 items-center'}
            >
              <div className="flex flex-col items-center gap-2">
                <StepIndicator state={state} index={index} />
                <span
                  className={
                    state === 'upcoming'
                      ? 'text-xs font-medium text-gray-400 transition-colors duration-300'
                      : 'text-xs font-semibold text-brand-700 transition-colors duration-300'
                  }
                >
                  {step.label}
                </span>
              </div>
              {!isLast && (
                <div
                  className={`mx-2 mt-[22px] h-0.5 flex-1 rounded-full transition-colors duration-500 ${
                    connectorDone ? 'bg-brand-600' : 'bg-gray-200'
                  }`}
                  aria-hidden="true"
                />
              )}
            </li>
          );
        })}
      </ol>

      {isActive && (
        <div className="mt-8 flex flex-col items-center gap-3 rounded-xl bg-brand-50/60 px-4 py-6 text-center">
          <div
            className="h-8 w-8 animate-spin rounded-full border-4 border-brand-100 border-t-brand-600"
            role="status"
            aria-label="로딩 중"
          />
          <p className="font-semibold text-brand-800">{PROGRESS_LABEL[status]}</p>
          <p className="text-sm text-gray-500">분석 중입니다. 잠시만 기다려 주세요.</p>
        </div>
      )}
    </div>
  );
}
