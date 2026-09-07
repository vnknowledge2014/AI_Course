# Sổ nợ nội dung — index

Mỗi file là kết quả một vòng phản biện thủ công (đọc hiểu nghĩa, không phải cổng
máy) trên một mạch bài học. Quy ước mức độ trong mọi file: **CHẶN** = sai sự
thật/mâu thuẫn/chấm hổng tới mức phải sửa trước khi ship; **nên-sửa** = đúng
sự thật nhưng diễn đạt/suy luận thiếu chặt; **nhỏ** = typo, nhịp câu.

## Trạng thái (số liệu lấy nguyên từ header từng file)

| File | Mạch | Phát hiện | CHẶN | Còn mở (nên-sửa + nhỏ) |
|---|---|---:|---:|---:|
| `R0.T1.md` | Máy tính nói gì | 43 | 8 — đã sửa hết | 35 |
| `R0.T2.md` | Ra lệnh cho Byte | 57 | 15 — đã sửa hết | 42 |
| `T1.1.md` | Giá trị, biến & kiểu | 82 | 20 — đã sửa hết | 60 |
| `T1.2.md` | Rẽ nhánh & lặp | 87 | 13 — đã sửa hết | 74 |
| `T1.3.md` | Hàm — viên gạch | 80 | 19 — đã sửa hết | 61 |
| `T1.4.md` | List, dict, set & tuple | 79 | 19 — đã sửa hết | 60 |
| `T1.5.md` | Chương trình thật | 68 | 15 — đã sửa hết | 53 |
| `T2.2.md` | Đại số & hàm số | 108 | **ĐÃ ĐÓNG TOÀN BỘ** | 0 |
| `cham-diem.md` | Đột biến cách chấm (475 mutant / 230 bước) | 21 mutant lọt | 1 đã sửa · 20 khai miễn trừ kèm lý-do | 0 |

**Tổng:** 109 lỗi mức CHẶN trên 704 phát hiện — tất cả đã xử lý xong. Còn lại
**385 phát hiện mức nên-sửa/nhỏ** là backlog biên tập có ý thức: chúng không
làm sai sự thật, không làm hỏng cách chấm; sửa chúng là cải tiến văn phong và
độ chặt của suy luận, từng bài một, có đối chiếu chéo (lịch sử cho thấy phát
hiện của agent đôi khi sai — ví dụ ca `20.1 * 1000` ở T1.1).

## Quy tắc đọc sổ

1. Header mỗi file mạch ghi trạng thái: cụm **"đã sửa hết"** sau số CHẶN nghĩa
   là mạch đó không còn lỗi chặn; con số "còn mở" là backlog biên tập.
2. Miễn trừ CỐ ĐỊNH không nằm ở đây mà nằm cạnh cổng máy tương ứng:
   `content/curriculum/dot-bien-bo-qua.yaml`, `doan-truoc-bo-qua.yaml`,
   `so-hoc-sai-co-y.yaml`, `su-that-the-gioi.yaml` — mỗi mục kèm lý-do riêng.
3. Khi xử lý xong một mạch tới cùng (như T2.2), đổi header thành **ĐÃ ĐÓNG**
   và cập nhật bảng trên.
