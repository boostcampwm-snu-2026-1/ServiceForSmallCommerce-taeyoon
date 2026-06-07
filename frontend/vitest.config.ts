import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/setup.ts'],
    include: ['src/**/*.test.{ts,tsx}', 'tests/unit/**/*.test.{ts,tsx}'],
    coverage: {
      provider: 'v8',
      include: ['src/features/**/*.{ts,tsx}'],
      exclude: [
        'src/features/**/*.test.{ts,tsx}',
        'src/features/**/types.ts',
        'src/features/**/index.ts',
      ],
      reporter: ['text', 'json-summary'],
      // 초기 세팅 시 threshold 없이 시작 → 테스트 작성 후 측정값으로 설정
      // thresholds: { statements: 95, branches: 90, functions: 90, lines: 95 },
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, '.'),
    },
  },
})
