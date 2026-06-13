import React from 'react';
import { cn } from './cn';

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: 'neutral' | 'info' | 'success' | 'warning' | 'danger';
}

const BASE = 'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium';

const VARIANT: Record<NonNullable<BadgeProps['variant']>, string> = {
  neutral: 'bg-gray-100 text-gray-700',
  info: 'bg-brand-50 text-brand-700',
  success: 'bg-green-100 text-green-800',
  warning: 'bg-amber-100 text-amber-800',
  danger: 'bg-red-100 text-red-800',
};

export function Badge({ variant = 'neutral', className, ...props }: BadgeProps) {
  return <span className={cn(BASE, VARIANT[variant], className)} {...props} />;
}
