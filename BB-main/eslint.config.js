import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores(['dist']),
  // ── 경계 규칙 (BB Plan §3.6.5.6) ───────────────────────────────────
  //  `components/**` (TRB 잔존) 이 `bb/**` (BB 전용) 를 import 하는 것을 막는다.
  //  잔존물이 BB 전용물에 의존하기 시작하면 나중에 `components/` 를 통째로
  //  지울 수 없다 — 의존 방향이 한쪽이어야 §3.6.4.6 의 일괄 정리가 가능하다.
  //  역방향(`bb/**` → `components/**`)은 **허용**한다: 공통 유틸
  //  (`PlotWithCopy`·`plotlyDefaults`·`DetailTable`) 재사용이 최소 변경의 전제다.
  {
    files: ['src/components/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': ['error', {
        patterns: [
          {
            group: ['**/bb/*', '**/bb/**', '@/bb/*', '@/bb/**'],
            message: 'components/** 는 bb/** 를 import 할 수 없다 (BB Plan §3.6.5.6). 역방향만 허용된다.',
          },
        ],
      }],
    },
  },
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
])
