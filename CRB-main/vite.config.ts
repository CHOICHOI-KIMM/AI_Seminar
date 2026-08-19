import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    port: 5175,
    strictPort: true,
    // Tauri 공식 권장: Vite 가 src-tauri/target/ 안의 cargo 빌드 산출물(계속 재생성되는 .dll)을
    // 감시하면 EBUSY (파일 잠금 충돌) 발생. src-tauri/ 통째로 감시 제외.
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
})
