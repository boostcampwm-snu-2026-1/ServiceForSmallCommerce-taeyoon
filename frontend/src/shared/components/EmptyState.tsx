import React from 'react';
import { cn } from './ui';

export interface EmptyStateProps {
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: React.ReactNode;
  className?: string;
}

export function EmptyState({ icon, title, description, action, className }: EmptyStateProps) {
  return (
    <div
      className={cn(
        'rounded-lg border border-dashed border-gray-300 bg-white p-10 text-center',
        className,
      )}
    >
      {icon && <div className="mb-3 flex justify-center text-gray-400">{icon}</div>}
      <p className="font-medium text-gray-900">{title}</p>
      {description && <p className="mt-1 text-sm text-gray-500">{description}</p>}
      {action && <div className="mt-4 flex justify-center">{action}</div>}
    </div>
  );
}
