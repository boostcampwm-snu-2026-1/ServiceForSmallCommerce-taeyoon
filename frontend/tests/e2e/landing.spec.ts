import { test, expect } from '@playwright/test';

/**
 * 랜딩 페이지 스모크 테스트.
 *
 * 미인증 방문자가 루트(/)에 접속하면 랜딩 페이지가 렌더되고,
 * 핵심 카피·CTA·주요 섹션이 노출되는지 확인한다. 공개 페이지이므로
 * 백엔드 없이 프론트 dev 서버만으로 검증 가능하다.
 */
test.describe('랜딩 페이지', () => {
  test('히어로 카피와 가입 CTA가 노출된다', async ({ page }) => {
    await page.goto('/');

    // 히어로 헤드라인
    await expect(
      page.getByRole('heading', {
        name: '리뷰가 알려주는 경쟁사를 이기는 법',
        level: 1,
      })
    ).toBeVisible();

    // 가입/로그인 진입점 (네비바 + 히어로에 복수 존재)
    await expect(
      page.getByRole('link', { name: '로그인' }).first()
    ).toBeVisible();
    await expect(
      page.getByRole('button', { name: '무료로 시작' }).first()
    ).toBeVisible();
  });

  test('주요 섹션 헤딩이 모두 렌더된다', async ({ page }) => {
    await page.goto('/');

    for (const name of ['핵심 가치', '작동 방식', '신뢰할 수 있는 분석', '요금제']) {
      await expect(page.getByRole('heading', { name, level: 2 })).toBeVisible();
    }
  });

  test('가입 CTA가 회원가입 페이지로 연결된다', async ({ page }) => {
    await page.goto('/');

    await page
      .getByRole('main')
      .getByRole('link', { name: '무료로 시작하기' })
      .first()
      .click();

    await expect(page).toHaveURL(/\/register$/);
  });
});
