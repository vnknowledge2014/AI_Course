# `fp/` — nguồn sách + tiền thân của Byte Academy

Thư mục này gộp ba thứ không cùng vai trò. Đọc kỹ trước khi di chuyển/xoá bất cứ gì ở đây.

## 1. `Python_Books/`, `Rust_Books/`, `TypeScript_Books/` — nguồn sách (READ-ONLY)

Ba bộ sách Domain-Driven Design + Functional Programming, viết dưới dạng chương `chapter_*.md`.
Đây là **nguồn thô bất biến** cho hai luồng biên dịch tự động của Byte Academy:

- `tools/di_tru_sach.py` — di trú 159 chương sang `content/legacy/*.chapter.md` (Tier-C, đọc-thêm,
  không chấm điểm), build ra `apps/byte/public/thu-vien/*.json`.
- `tools/trich_rust.py` — trích snippet Rust biên dịch độc lập từ `Rust_Books/` vào
  `crates/byte-rust-conformance/corpus/`, dùng làm corpus đối chiếu `byte-rust` với `rustc` thật.

KHÔNG sửa tay các chương này để "cải thiện nội dung Byte Academy" — sửa ở `content/legacy/` sẽ bị
lệch nguồn lần di trú sau. Muốn sửa, sửa ở đây rồi chạy lại `tools/di_tru_sach.py`.

`Rust_Books/.venv/`, `Rust_Books/.pageindex_raw_data/`, `Rust_Books/.rig_raw_data/` là dữ liệu thô/
môi trường của công cụ scrape nội bộ (không phải nội dung sách) — đã gitignore, chỉ tồn tại local.

## 2. `game/` ("Code Odyssey") — nguyên mẫu, tiền thân trực tiếp của `apps/byte`

Ứng dụng Svelte đọc-và-chạy-sách trực tiếp trong trình duyệt, kiểu Swift Playgrounds trên macOS/iPadOS
— đây LÀ ý tưởng gốc dẫn tới Byte Academy. `MASTERPLAN.md` §1 ghi lại đánh giá kỹ thuật tại thời điểm
quyết định xây lại: bản build không có bài học (`db.ts` fetch sai đường dẫn ngoài dev server), chạy
code học viên bằng `new Function()` trên cùng luồng với app (không cô lập, chạm được DOM/Dexie/
localStorage), COOP/COEP chỉ có ở dev server nên `SharedArrayBuffer` không hoạt động ở bản đóng gói.

Từ đánh giá đó, sản phẩm được xây lại từ đầu dưới tên **Byte Academy** (`apps/byte`) với kiến trúc
cô lập bằng Worker + Tauri + chấm bài tất định. `game/` được giữ nguyên làm tài liệu thiết kế/tham
chiếu lịch sử — KHÔNG nằm trong `pnpm-workspace.yaml`, KHÔNG được build/test bởi `tools/cong.sh`.

## 3. `.agent/`, `AGENTS.md`, `omni.config.yaml` — cấu hình một hệ agent khác

Quy tắc/workflow cho OmniUltraAgent Kit và Antigravity IDE khi làm việc trực tiếp trong `fp/` — không
liên quan Claude Code, không ảnh hưởng build/test của Byte Academy hay của `game/`.
