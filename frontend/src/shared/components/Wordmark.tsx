import { cn } from './ui/cn';

export function Wordmark({ className }: { className?: string }) {
  return (
    <span className={cn('inline-flex items-center gap-2', className)}>
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        aria-hidden="true"
        xmlns="http://www.w3.org/2000/svg"
      >
        <rect width="24" height="24" rx="6" className="fill-brand-600" />
        <path
          d="M7 12.5L10.5 16L17 8.5"
          stroke="white"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
      <span className="font-semibold text-gray-900">
        Coupang Review <span className="text-brand-600">AI</span>
      </span>
    </span>
  );
}
