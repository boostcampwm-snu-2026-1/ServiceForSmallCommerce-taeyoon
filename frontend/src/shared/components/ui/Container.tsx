import React from 'react';
import { cn } from './cn';

export function Container({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) {
  return <div className={cn('mx-auto w-full max-w-5xl px-4', className)} {...props} />;
}
