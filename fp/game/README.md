# Code Odyssey

Ứng dụng đọc–và–chạy cho bộ giáo trình **Domain-Driven FP** trong `fp/`.
Mỗi chương markdown được biên dịch thành một "book" JSON có phân trang, kèm
code editor chạy được ngay trong trình duyệt.

## Ngăn xếp công nghệ

| Thành phần | Công nghệ |
|---|---|
| UI | **Svelte 5** (runes: `$props`, `$state`) |
| Bundler | Vite 8 (Rolldown) |
| Editor | CodeMirror 6 — highlight cho Python / Rust / TypeScript |
| Chạy Python | **Pyodide** (CPython biên dịch sang WASM), tải từ `public/pyodide/` |
| Chạy TypeScript | **Sucrase** transform rồi eval trong trình duyệt |
| Đồ hoạ | PixiJS 8 |
| Lưu tiến độ | Dexie (IndexedDB) |
| Render markdown | `marked` + `dompurify` |

> Đây **không** phải project React. Nếu bạn thấy tài liệu nhắc tới React ở đâu
> đó, đó là tàn dư từ template Vite ban đầu.

## Bắt đầu

```bash
npm install
npm run download-pyodide   # tải runtime Pyodide vào public/ (chỉ cần 1 lần)
npm run ingest             # markdown trong fp/*_Books/ -> src/data/books/*.json
npm run dev
```

## Scripts

| Lệnh | Việc nó làm |
|---|---|
| `npm run dev` | Dev server có HMR |
| `npm run build` | `tsc -b` rồi `vite build` → `dist/` |
| `npm run preview` | Xem thử bản build |
| `npm run ingest` | **Sinh lại toàn bộ dữ liệu sách từ markdown** |
| `npm run download-pyodide` | Tải Pyodide runtime về `public/pyodide/` |

## Pipeline ingest

`scripts/ingest-v3.cjs` quét ba thư mục sách và sinh ra một JSON mỗi chương.

```
fp/Python_Books/**/chapter_*.md      ──┐
fp/Rust_Books/**/chapter_*.md        ──┼──> src/data/books/<prefix>-ch<NN>.json
fp/TypeScript_Books/**/chapter_*.md  ──┘         + src/data/manifest.json
```

- Prefix: `py-` / `rs-` / `ts-`
- ID lấy từ tên file: `chapter_27b_...` → `py-ch27b`
- Chỉ nhận file khớp `chapter_<số>[chữ]_*.md`; outline, preface và appendix bị bỏ qua
- Thư mục part được ánh xạ sang "World" 0–7 trong `PART_TO_WORLD`

> ⚠️ **Chạy lại `npm run ingest` mỗi khi bạn thêm hoặc đổi số chương.** Dữ liệu
> trong `src/data/` là dữ liệu sinh ra, không phải nguồn — sửa markdown, đừng
> sửa JSON.

### Thêm một Part mới

Khi bổ sung `part_9_...`, phải cập nhật **bốn** chỗ trong `scripts/ingest-v3.cjs`:
`PART_TO_WORLD`, `WORLD_NAMES`, `WORLD_COLORS`, `WORLD_EMOJIS`. Thiếu bất kỳ chỗ
nào thì chương mới sẽ rơi vào World 1 mà không báo lỗi.

## Cấu trúc thư mục

```
src/
  App.svelte              # shell + điều hướng
  components/
    CodeEditor.svelte     # CodeMirror, chọn mode theo book.language
    Markdown.svelte       # marked + dompurify
  screens/
    Lesson.svelte         # trình đọc theo trang + khung chạy code
  engine/
    runner.ts             # Pyodide (Python) và Sucrase (TypeScript)
    factory.ts
  state/
    db.ts                 # Dexie: tiến độ đọc, cache manifest
  data/                   # ⚠️ SINH RA — do npm run ingest tạo
    manifest.json
    books/*.json
scripts/
  ingest-v3.cjs
  download-pyodide.sh
```

## Ghi chú

- Dev server đặt header `Cross-Origin-Opener-Policy` / `Cross-Origin-Embedder-Policy`
  vì Pyodide cần `SharedArrayBuffer`. Khi deploy, host cũng phải trả hai header này.
- `pyodide` bị loại khỏi `optimizeDeps` — nó được nạp lúc chạy từ `/pyodide/`,
  không đi qua bundler.
- Bundle hiện ~1 MB (PixiJS + CodeMirror). Muốn giảm thì code-split màn hình game.
