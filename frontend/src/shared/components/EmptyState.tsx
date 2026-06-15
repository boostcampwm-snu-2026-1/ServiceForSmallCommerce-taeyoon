import React from 'react';
import { cn } from './ui';

export interface EmptyStateProps {
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: React.ReactNode;
  className?: string;
}

const DefaultIcon = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth={1.5}
    strokeLinecap="round"
    strokeLinejoin="round"
    className="h-6 w-6"
    aria-hidden="true"
  >
    <path d="M9 17H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6l2 2h6a2 2 0 0 1 2 2v3" />
    <path d="m15 18 2 2 4-4" />
  </svg>
);

export function EmptyState({ icon, title, description, action, className }: EmptyStateProps) {
  return (
    <div
      className={cn(
        'rounded-xl border border-dashed border-gray-300 bg-white px-6 py-12 text-center',
        className,
      )}
    >
      <div className="mb-4 flex justify-center">
        <span className="flex h-12 w-12 items-center justify-center rounded-full bg-brand-50 text-brand-500">
          {icon ?? DefaultIcon}
        </span>
      </div>
      <p className="font-semibold text-gray-900">{title}</p>
      {description && (
        <p className="mx-auto mt-1.5 max-w-sm text-sm leading-relaxed text-gray-500">{description}</p>
      )}
      {action && <div className="mt-5 flex justify-center">{action}</div>}
    </div>
  );
}
