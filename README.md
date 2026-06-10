# AI Course & LLM Wiki

Dự án này là một tập hợp các tài nguyên, mã nguồn, bài báo và ghi chú liên quan đến Khoa học Trí tuệ Nhân tạo, Kiến trúc Agentic, và LLMs.

Đặc biệt, hệ thống này được quản lý và bảo trì bởi các AI Agent thông qua khái niệm **LLM Wiki**.

## Cấu trúc dự án

- `resource.md`: Danh sách trung tâm chứa tất cả các nguồn dữ liệu (Video, GitHub Repos, Bài viết, Khóa học).
- `repos/`: Thư mục lưu trữ mã nguồn của các dự án AI được clone về (bị bỏ qua trên GitHub gốc để tránh quá tải, được quản lý như dạng gitlinks).
- `references/`: Nơi lưu trữ các bài báo, bài viết kỹ thuật.
- `wiki/`: Kho tri thức được các LLM Agent tự động sinh ra, phân tích và tổ chức. Các file `.md` tại đây chứa tóm tắt về từng repository và các chủ đề liên quan (LLMs, Agents, RAG, Transformers).
- `AGENTS.md`: File cấu hình Schema quy định cách các LLM thao tác với hệ thống wiki này.

## Tự động hóa

Dự án chứa các script Python để hỗ trợ quản lý tự động:
- `clone_repos.py`: Dùng để đọc `resource.md` và tự động clone/pull cập nhật toàn bộ các kho mã nguồn về `repos/`.
- `check_repos.py`: Kiểm tra tính toàn vẹn và đồng bộ của các kho đã tải về so với danh sách `resource.md`.
- `auto_ingest.py`: Script do LLM thiết lập để tự động đọc tài liệu, trích xuất tóm tắt và cập nhật cấu trúc mục lục (`wiki/index.md`) cùng nhật ký (`wiki/log.md`).

## Cách sử dụng LLM Wiki

Thay vì đọc thủ công, bạn sử dụng một Agent (ví dụ: Antigravity/Claude/ChatGPT) đọc thư mục `wiki/` để tra cứu chéo, so sánh kiến trúc giữa các dự án hoặc sinh ra các bài viết tổng hợp mới dựa trên thông tin đã được ingest.