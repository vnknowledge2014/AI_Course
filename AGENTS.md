# LLM Wiki Schema & Guidelines cho dự án AI_Course

Đây là cấu hình để duy trì wiki tri thức dự án thông qua LLMs (như Antigravity hoặc các Agent khác). 
LLM đóng vai trò duy trì cấu trúc nội dung, cập nhật kiến thức liên tục từ Raw Sources vào Wiki.

## Cấu trúc lưu trữ (Architecture)

### 1. Raw Sources (Nguồn thô)
- **Vị trí:** Thư mục `repos/` (chứa các git repository về AI) và `references/` (bài báo, slide, link tham khảo khác), file `resource.md`.
- **Luật:** KHÔNG ĐƯỢC CHỈNH SỬA Raw Sources. Chúng là dữ liệu đầu vào (immutable). Chỉ sử dụng lệnh đọc (ví dụ: `cat`, `grep_search`, `view_file`) để trích xuất thông tin.

### 2. The Wiki (Kho tri thức)
- **Vị trí:** Thư mục `wiki/`
- **Luật:** LLM hoàn toàn kiểm soát và chỉnh sửa khu vực này. Mọi trang wiki đều là file markdown `.md`.
- Các loại file wiki bao gồm:
  - `wiki/index.md`: Mục lục liệt kê mọi thứ.
  - `wiki/log.md`: Nhật ký hoạt động thao tác wiki.
  - Entity Pages: (ví dụ: `wiki/LLM.md`, `wiki/RAG.md`) 
  - Source Summaries: (ví dụ: `wiki/Summary_Repo_ABC.md`)

## Quy trình vận hành (Operations)

### 1. Ingest (Nạp dữ liệu)
Khi người dùng yêu cầu đọc một tài liệu mới hoặc một repo mới:
1. Đọc và phân tích thông tin từ Raw Source tương ứng.
2. Thảo luận/Trích xuất các ý chính, khái niệm AI nổi bật.
3. Tạo hoặc cập nhật trang tổng hợp (Summary Page) trong thư mục `wiki/`.
4. Tìm và cập nhật các trang Concepts/Entities có liên quan. Nếu khái niệm mới, tạo file `.md` mới cho khái niệm đó. Cập nhật link dẫn chéo (cross-references).
5. Thêm một mục (entry) ghi lại file/concept mới vào `wiki/index.md`.
6. Ghi log sự kiện Nạp Dữ Liệu vào `wiki/log.md` theo format chronical. (VD: `## [YYYY-MM-DD] ingest | Đọc repo AI-For-Beginners`)

### 2. Query (Truy vấn)
Khi người dùng hỏi về kiến thức AI trong dự án:
1. LUÔN tìm kiếm và ưu tiên đọc từ thư mục `wiki/` (đặc biệt là đọc qua `wiki/index.md` trước để tra cứu file liên quan).
2. Tổng hợp câu trả lời, trích dẫn link đến file markdown trong wiki.
3. Nếu phát hiện thấy insight mới hoặc sự so sánh thú vị qua câu trả lời, đề xuất lưu trữ/thêm trực tiếp thành một file wiki mới để tích luỹ kiến thức (compounding).

### 3. Lint & Health-check (Dọn dẹp, kiểm tra sức khoẻ wiki)
- Định kỳ, Agent nên quét qua các file `.md` trong thư mục `wiki/`.
- Kiểm tra các trang "mồ côi" (orphan pages) không có inbound links.
- Sửa chữa mâu thuẫn dữ liệu hoặc update theo chuẩn nguồn thô mới.

## Quy ước ghi file `wiki/log.md`
Mỗi log thao tác phải tuân thủ dạng:
`## [YYYY-MM-DD] <hành_động> | <Tiêu đề ngắn>`
Trong đó <hành_động> thường là: `ingest`, `query`, `lint`, `initialize`.

---
LLM hãy luôn tuân thủ nguyên tắc: **Con người ra lệnh và định hướng, LLM thực thi việc tóm tắt, tổng hợp, tạo link chéo và bảo trì wiki**.

## 4. Hướng dẫn Đồng bộ Đa người dùng / Đa thiết bị (Git Sync)
Để hệ thống LLM Wiki này hoạt động trơn tru trên nhiều máy khác nhau, Agent và Người dùng phải tuân thủ luồng làm việc sau:
1. **Khi Clone sang máy mới:** Thư mục `repos/` trên GitHub chỉ chứa "gitlinks" (subprojects), KHÔNG chứa mã nguồn bên trong. Người dùng hoặc Agent phải chạy `python3 clone_repos.py` để tải toàn bộ mã nguồn về máy local trước khi làm việc.
2. **Luôn Pull trước khi Ingest:** Trước khi chạy `auto_ingest.py` hoặc thêm log mới, luôn phải thực hiện `git pull --rebase` để đảm bảo file `wiki/log.md` và `wiki/index.md` lấy bản mới nhất, tránh conflict.
3. **Đường dẫn tương đối (Relative Paths):** Các Agent khi sinh ra file markdown mới tuyệt đối KHÔNG ĐƯỢC dùng đường dẫn tuyệt đối (như `/Volumes/...` hoặc `C:\...`). Hãy luôn dùng đường dẫn tương đối (ví dụ: `../../repos/Tên_Repo`) để link vẫn hoạt động trên mọi thiết bị.
