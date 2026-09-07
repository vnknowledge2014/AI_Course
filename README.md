# Byte Academy

Học liệu tương tác dạy lập trình từ số 0 → thành thạo — 10 realm (`R0`–`R9`), 3 ngôn ngữ
(Python/TypeScript/Rust), 7 nền tảng (macOS/Windows/Linux/Android/iOS/Chrome/Firefox). Xem
[`MASTERPLAN.md`](MASTERPLAN.md) để có đặc tả đầy đủ (kiến trúc, quyết định kỹ thuật, phạm vi nội dung).

**Trạng thái:** toàn bộ MASTERPLAN R0–R9 đã hoàn tất — 1521 bài `.lesson.md`, `tools/cong.sh` (15 cổng
kiểm chất lượng) xanh.

## Cấu trúc repo

Repo này gồm **hai lớp**, phục vụ cùng một mục tiêu (dạy lập trình) nhưng khác vai trò:

### 1. Nền tảng khoá học (lớp chính, đang phát triển tích cực)

| Thư mục | Vai trò |
|---|---|
| `content/` | Nguồn nội dung — `*.lesson.md` (markdown + frontmatter + directive), tổ chức theo `content/curriculum/thu-tu.yaml`. `content/legacy/` là 159 chương đọc-thêm (Tier-C), sinh tự động từ `fp/*_Books` (gitignored, xem mục fp/ bên dưới). |
| `apps/byte` | App học tập — Svelte 5 + Vite, đóng gói qua Tauri 2 cho desktop/mobile. Chạy 100% offline sau khi tải, không backend bắt buộc. |
| `packages/` | Thư viện dùng chung (pnpm workspace): `content-schema`/`content-compiler` (biên dịch `.lesson.md` → JSON), `exec-core`/`exec-python`/`exec-typescript`/`exec-rust` (ba engine chấm bài chạy đúng runtime học viên dùng), `ui`, `simdisk` (mô phỏng block device cho Realm 6), `mcp-kit` (type MCP/A2A cho Realm 9). |
| `crates/` | `byte-rust` — interpreter Rust tự viết (biên dịch WASM) dùng để chấm bài Rust; `byte-rust-conformance` — đối chiếu engine này với `rustc` thật. |
| `tools/` | Pipeline kiểm chất lượng nội dung — `cong.sh` chạy ~15 cổng (sư phạm, đồ thị tiền đề, số học, bí mật, mutation-testing, chạy lời giải thật qua đúng engine app dùng, v.v.). |
| `docs/` | ADR và nhật ký quyết định kỹ thuật của quá trình xây dựng nền tảng. |
| `.phan-bien/` | Output vòng phản biện/QA tự động cho nội dung (gitignored). |

Lệnh cơ bản: `pnpm install`, `pnpm --filter @byte/app noi-dung` (biên dịch nội dung), `pnpm --filter @byte/app dev`
(chạy app), `bash tools/cong.sh` (chạy toàn bộ cổng kiểm tra), `bash tools/cong.sh --nhanh` (bỏ qua các cổng chậm).

### 2. `fp/` — nguồn sách + tiền thân của app

`fp/{Python,Rust,TypeScript}_Books/` là bộ sách nguồn (Domain-Driven Design + Functional Programming) —
nguồn READ-ONLY cho `content/legacy/` (di trú bằng `tools/di_tru_sach.py`) và cho corpus đối chiếu
`rustc` (`tools/trich_rust.py` → `crates/byte-rust-conformance/corpus/`).

`fp/game` ("Code Odyssey") là **nguyên mẫu ban đầu** — ứng dụng đọc-và-chạy-sách kiểu Swift Playgrounds,
tiền thân trực tiếp truyền cảm hứng cho `apps/byte`. Sau khi đánh giá (xem `MASTERPLAN.md` §1: bản build
rỗng, chạy code học viên trên main thread không an toàn), sản phẩm được xây lại từ đầu với kiến trúc
vững hơn (Worker cô lập, Tauri, chấm bài tất định) dưới tên **Byte Academy**. `fp/game` được giữ lại làm
tài liệu lịch sử/tham chiếu thiết kế, không phải một phần của pnpm workspace.

`fp/.agent/`, `fp/AGENTS.md`, `fp/omni.config.yaml` là cấu hình cho một hệ agent khác (OmniUltraAgent/
Antigravity) dùng khi làm việc trực tiếp trong `fp/` — không liên quan Claude Code, không ảnh hưởng build
byte-academy.

### 3. Kho tham chiếu tri thức AI ("LLM Wiki")

`resource.md` (danh sách nguồn), `repos/` (clone các repo AI liên quan), `references/` (bài báo/tài liệu
kỹ thuật), `wiki/` (tổng hợp tri thức do LLM sinh ra) — đây là **lớp tham chiếu sâu** dùng để tra cứu chi
tiết, rút kiến thức và tìm ý cải tiến/mở rộng nội dung MASTERPLAN (không phải một sản phẩm độc lập tách
biệt khỏi Byte Academy). Vận hành qua 3 script:

- `clone_repos.py` — đọc `resource.md`, clone/pull mọi repo về `repos/` (chỉ local, không commit nội dung con).
- `check_repos.py` — kiểm tính toàn vẹn `repos/` so với `resource.md`.
- `auto_ingest.py` — đọc `repos/`, tóm tắt và cập nhật `wiki/`.

Quy tắc thao tác wiki nằm trong [`AGENTS.md`](AGENTS.md). Vòng ingest gần nhất dừng ở 2026-06-18 (tạm
gác lại trong lúc dồn lực hoàn tất MASTERPLAN R0–R9) — chạy lại `auto_ingest.py` bất cứ khi nào cần đào
sâu một chủ đề để cải tiến bài học.

## Bắt đầu từ đâu

- Muốn hiểu/sửa nội dung khoá học hay app → đọc `MASTERPLAN.md`, rồi `content/curriculum/thu-tu.yaml`.
- Muốn tra cứu sâu một chủ đề AI để làm giàu bài học → dùng `wiki/index.md`, hoặc clone thêm nguồn mới
  qua `resource.md` + `clone_repos.py` + `auto_ingest.py`.
- Muốn hiểu nguồn gốc thiết kế app học-qua-chơi → xem `fp/game/` và phần "SỰ THẬT NỀN" trong `MASTERPLAN.md`.
