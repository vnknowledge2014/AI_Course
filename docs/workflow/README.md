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

## Một cổng nữa KHÔNG nên dựng: "đáp án đúng có bị chặn oan không"

Vòng phản biện Realm 1 tìm ra **mười ba** ca cách chấm đánh trượt một lời giải
ĐÚNG — nhiều thứ nhì sau "mâu thuẫn bài trước". Với người học đó là lớp lỗi tệ
nhất: họ viết đúng, máy nói sai, câu báo trượt còn chỉ sai chỗ.

Điểm chung của cả mười ba: một luật `static` đặt theo **số lần lời giải MẪU**
chạm vào một cái tên, chứ không theo thứ bài thật sự đòi hỏi.

Nghe rất hợp để giao cho máy. Cổng đột biến hỏi một chiều — "đáp án sai có bị
chặn không"; cổng này hỏi chiều ngược lại. Đã dựng thử (`kiem_cham_oan.mjs` +
`noi_thang.py`, xem lịch sử git), đo trên cả 302 bài, rồi **bỏ**.

### Bản đầu tiên báo oan 18 ca, và bằng đúng cái bệnh nó đi chữa

Cách đo: viết lại lời giải mẫu bằng một phép giữ nguyên hành vi (đổi `str(x)`
thành f-string, bỏ một biến bắc cầu dùng đúng một lần…), đòi output y hệt và
mọi `assert` vẫn đạt, rồi mới hỏi tầng `static`.

Nó báo 18 ca. Soi lại một ca thì lộ ra chuyện này:

```python
sang_byte = 12          # ← DÒNG KHUNG, người học không xoá được
sang_an = 7
tong_sang = ___         # ← chỗ duy nhất họ điền
```

Phép "bỏ biến bắc cầu" của cổng đã xoá `sang_byte = 12` đi rồi kết luận cách
chấm sai. Nhưng **người học chỉ điền được vào chỗ trống**; họ không nộp được
bản viết lại ấy. Mười tám ca đều là hiện vật của việc tôi quên mất điều đó.

Một cổng dựng ra để bắt "con số xanh không đo thứ nó nói", tự nó cho ra một
con số đỏ không đo thứ nó nói.

### Sửa hàng rào xong thì cổng gần như mù

Thêm hàng rào đúng — bản viết lại phải giữ nguyên mọi dòng khung không chứa
`___` — thì con số đi từ 18 xuống **1**, trên **176 bước** có tầng `static`.
Tức 175 bước cổng không nói được gì, và ca duy nhất còn lại là ca đã cố ý siết
(bài `muon-cat-so-phai-doi-thanh-chu`: đề bài nay nêu đích danh `str`, và lời
báo trượt nói thẳng rằng f-string cũng đúng, chỉ là bài đang dạy `str`).

Lý do nó mù: chỗ trống của học liệu này gần như luôn là một **biểu thức ngắn**,
không phải một câu lệnh có biến để bỏ. Mấy phép biến đổi giữ-nguyên-hành-vi
đơn giản hầu như không áp được vào chỗ ấy.

Ship nó là thêm một dấu tích xanh thứ mười sáu, và dấu ấy nghĩa là "đã kiểm 1
bước". Đúng thứ cả dự án đi bắt.

### Vậy lớp lỗi ấy giao cho ai

Vẫn cho vòng phản biện. Nhưng có một luật rẻ tiền rút ra được, đáng viết vào
hiến chương thay vì viết thành cổng:

> Đặt `min` cho một luật `static` theo thứ **bài thật sự đòi hỏi**, không theo
> số lần lời giải mẫu chạm vào cái tên ấy. Trước khi chốt con số, hãy nghĩ ra
> MỘT cách viết đúng khác và đếm lại trên nó.

Mười ba ca có thật đều gãy ở đúng bước "nghĩ ra một cách viết đúng khác".

Luật ấy cùng hai luật nữa rút từ 30 lỗ chấm điểm của vòng R1 đã viết thành
`docs/workflow/luat-cham-diem.md`, để 1.400 bài còn lại không phải học lại
bằng cách hỏng.

## Cái xanh giả tôi tự dựng lên — bản biên dịch cũ (2026-08-30)

`tools/kiem_ma_bai_hoc.mjs` đọc `dist/content/*.json`, **không** đọc
`.lesson.md`. Trong `cong.sh` thì vô hại vì bước biên dịch chạy ngay trước nó.
Chạy tay một mình sau khi sửa bài thì nó chấm bản dịch cũ và báo xanh — về một
bài học không còn tồn tại.

Tôi dính đúng bẫy ấy suốt một phiên: sửa bài, chạy cổng, thấy xanh, đi tiếp.
Chỗ lộ ra là lúc thử đột biến — bỏ hẳn luật chấm của một bài mà cổng **vẫn**
xanh. Nếu không thử đột biến thì cái xanh ấy còn sống rất lâu.

Nay cổng so `mtime` của `content/**/*.lesson.md` với `dist/content/*.json` và
**từ chối chạy** nếu nguồn mới hơn. Thà đỏ vì chưa biên dịch còn hơn xanh vì
đo nhầm bản.

Luật rút ra: **một cổng đọc bản dẫn xuất phải tự kiểm bản ấy còn tươi.** Cổng
nào cũng vậy, không riêng cổng này.

## Sáu truy vấn AST khai mà chưa cài (2026-08-30)

`PyAstKind` trong `content-schema` khai `recursion`, `frozen-dataclass`,
`no-mutation`, `pure-fn`, `no-global`, `uses-generator` cho Realm 4 (FP).
Không tên nào có nhánh xử lý trong `kiem-ast.ts`.

Trước đây viết một trong sáu tên ấy vào bài thì nó đếm được **0**:
- trong `requireAst` → luật trượt trên chính lời giải, `cong.sh` bắt được;
- trong `forbidAst` → 0 đúng bằng thứ luật cấm muốn thấy, nên nó **ĐẬU**, và
  đậu mãi mãi. Một luật canh gác không canh gì cả.

Tôi tự viết `assigns-name` (tên thật là `gan-ten`) và suýt cất nó đi như một
luật đang canh gác. Nay `kiemAst` kiểm tên truy vấn **trước** mọi `try/catch`
và ném nếu tên không có thật — ai soạn bài FP đầu tiên sẽ phải cài truy vấn
trước khi dùng nó, đúng thứ tự.

Chỗ đáng nhớ về khối `catch`: nó quy **mọi** lỗi về "mã người học không phân
tích được". Với một `kind` viết sai thì kết luận ấy đổ tội nhầm người — code
người học không sao, luật chấm mới là thứ hỏng, mà họ đọc được thông báo còn
người soạn bài thì không.
