# Script điều phối agent

Bản chép của các workflow script đang dùng. Bản chạy thật nằm ở
`~/.claude/projects/…/workflows/scripts/` — **ngoài repo**, nên nó biến mất
cùng phiên làm việc.

Chép vào đây vì hai lý do đã trả giá thật:

1. **Chúng mang bài học, không chỉ mang mã.** Phần lớn dòng chú thích trong
   `viet-t23.js` là một lỗi có thật đã xảy ra: `module` chép nhầm từ track
   trước làm 34 bài suýt mang sai nhãn; `args` tới dưới dạng chuỗi JSON khiến
   một workflow bung ra 3776 "cụm" mỗi cụm ba chữ cái. Mất script là mất cả
   những chỗ ấy.
2. **Chúng là nơi rút ra được cách giảm 37% hao phí.** Xem phần dưới.

## Chim hoàng yến — vì sao mọi workflow ở đây thả một cụm trước

Trong một phiên làm việc, **4,24 triệu token bị đốt vào bốn lần chạy hỏng vì
hạn mức, sinh ra 0 bài** — 37% tổng token đã tiêu.

Cách hỏng luôn giống nhau: bung 10–22 agent cùng lúc, tất cả chết vì cùng một
lẽ, mỗi con kịp đốt ~71k token trước khi tắt. Mà cái lẽ ấy biết được từ con
đầu tiên.

Nên mọi workflow ở đây chạy **một cụm thăm dò** trước. Sống thì thả nốt; chết
thì `throw` ngay. Một lần hỏng từ 709k xuống còn ~71k token.

Khi cắt cụm đầu ra khỏi `pipeline`, nhớ cộng lại chỉ số (`i + 1`): các agent
đọc bài hàng xóm theo chỉ số, lệch một là mỗi bài nối nhầm chỗ.

## Agent chết KHÔNG phải là "không tìm thấy lỗi"

`pipeline` trả `null` cho agent chết. Viết `ket.filter(Boolean).flatMap(...)`
thì một vòng phản biện chết sạch trả về **mảng rỗng**, và workflow báo
`{tong: 0}` — trông y hệt "đã đọc 30 bài, không thấy lỗi nào".

Chuyện này đã xảy ra thật với vòng phản biện T1.1, trong đúng một workflow
sinh ra để đi tìm loại lỗi ấy. Nên đếm số `null` TRƯỚC khi lọc, và `throw`.
