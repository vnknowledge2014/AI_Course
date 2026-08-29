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

## Một cổng KHÔNG nên dựng: đối chiếu khẳng định Python trong văn xuôi

Sau ba vòng phản biện R1, 14/52 lỗi CHẶN thuộc lớp "sai sự thật về Python".
Nhìn qua thì đó là lớp máy làm được: bài viết `<biểu thức>` cho `<kết quả>`,
máy chạy thử rồi đối chiếu. Đã dựng thử và đo trên cả 302 bài.

**Kết quả: 79 khẳng định kiểm được, 19 báo lệch — cả 19 đều là báo oan.**

- "cho", "ra", "là", "thành" là những chữ quá phổ biến trong tiếng Việt, nên
  biểu thức chính quy vơ luôn những cặp không có quan hệ tính toán nào: mạch
  Toán viết `9/12 + 8/12 = 17/12` bằng ký hiệu phân số, không phải Python.
- `"Phở bò"` với `'Phở bò'` chỉ khác kiểu dấu nháy.
- Nhiều câu nói về KIỂU chứ không về giá trị: "`50000 * 10` cho một `int`".
- Nhiều câu nói về phép BIẾN ĐỔI: "làm tròn `45000.7` ra `45001`".

Và điều quyết định: đối chiếu ngược lại 14 lỗi CHẶN có thật, cổng này **không
bắt được ca nào**. Vì chúng không có hình dạng `biểu thức → hằng số`:

- "`20.1 * 1000` lại **thừa** một chút" — không có kết quả nào để so;
- "`ten, tien = ...` cho `too many values to unpack (expected 2, got 3)`" —
  vế trái là câu lệnh, vế phải là thông báo lỗi;
- "phép chia luôn cho ra số có phần lẻ" — một luật, không một phép tính;
- "`not` lật mỗi cái tên sát bên phải" — một luật về cú pháp.

Mười bốn lỗi ấy là những khẳng định **phát biểu bằng lời** về hành vi của máy.
Muốn kiểm chúng thì phải hiểu câu tiếng Việt, chứ không phải chạy một biểu
thức. Đó là việc của vòng phản biện, và tới giờ vẫn chưa có cách nào rẻ hơn.

Ghi lại đây để lần sau đừng dựng lại: một cổng bắt 0 lỗi thật và sinh 19 báo
oan thì tệ hơn không có cổng nào — người ta sẽ tắt nó, và tắt rồi thì nó cũng
không bắt được ca thật nào nữa.
