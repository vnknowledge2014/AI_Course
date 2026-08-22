import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  // Tauri phục vụ file từ đĩa nên mọi đường dẫn phải tương đối.
  base: './',
  server: { port: 5273, strictPort: true },
  build: {
    target: 'es2022',
    outDir: 'dist',
    // Trình biên dịch `typescript` là 3,6 MB và KHÔNG tách nhỏ được — nó là
    // một đơn vị. Nhưng nó nằm ở chunk riêng, nạp lười, chỉ khi có bài
    // TypeScript thật sự chạy: bundle chính vẫn 77 KB, nên người học Python
    // không trả giá cho một thứ họ không dùng.
    //
    // Nâng ngưỡng cảnh báo thay vì tắt nó: 4 MB vẫn kêu nếu có chunk nào khác
    // phình ra ngoài dự tính.
    chunkSizeWarningLimit: 4000,
  },
});
