# Luật đặt cách chấm — rút ra từ 396 phát hiện của vòng phản biện Realm 1

Vòng phản biện đầu tiên trên 150 bài Realm 1 tìm ra 396 phát hiện. **30 trong
đó là lỗ chấm điểm**, và chúng chia làm hai nửa đều nhau, hai nửa ấy hỏng
ngược chiều nhau. Tệp này ghi ba luật rút ra, để 1.400 bài còn lại không phải
học lại bằng cách hỏng.

Đây là hướng dẫn cho **người và agent viết bài**, không phải một cổng. Ba luật
này đã thử làm thành cổng và đo là không đáng — lý do ghi ở `README.md` cùng
thư mục.

---

## Luật 1 — Chấm một cảnh thì không phải chấm

Sáu ca trong vòng ấy là bài chạy trên **đúng một bộ dữ liệu**, và với bộ ấy thì
nhiều câu trả lời sai cho ra cùng màn hình như câu đúng.

Ví dụ nặng nhất: bài dạy `else` thuộc về `if` nào, chấm bằng một ngày duy nhất.
Điền `if True:` vào chỗ đáng lẽ là `else:` — ĐẬU. `if co_ghi_so:` — ĐẬU.
`if tien < 500000:` — ĐẬU. Không câu nào là `else`.

> **Trước khi chốt một khối chấm, hãy liệt kê ba cách điền HỤT mà bạn đoán
> người học sẽ viết, rồi thử từng cách trên dữ liệu của bài. Cách nào cũng ra
> cùng màn hình với lời giải đúng thì bộ dữ liệu ấy chưa đủ.**

Cách chữa rẻ nhất gần như luôn là **thêm một cảnh thứ hai**, không phải viết
thêm assert.

## Luật 2 — Đừng đặt `min` theo lời giải mẫu

Mười ba ca là cách chấm đánh trượt một lời giải **đúng**. Với người học đó là
lớp lỗi tệ nhất: họ viết đúng, máy nói sai, và câu báo trượt còn chỉ sai chỗ.
Không có đường nào đi tiếp, vì họ không sai.

Cả mười ba đều gãy ở cùng một chỗ: luật `static` đặt `min` bằng **số lần lời
giải MẪU** chạm vào một cái tên.

- `uses-name target: tien, min: 1` — người học viết
  `return gia_mot_to * so_to` thay vì đi qua biến `tien`. Đúng, và trượt.
- `uses-fstring min: 2` — người học viết `print("nhãn:", ten)`, đúng lối bài
  T1.1 đã dạy. Đúng, và trượt.
- `uses-name target: __name__, min: 2` — chỗ trống 1 đã gán cái tên ấy vào một
  biến, nên chỗ trống 2 dùng lại biến. Đúng, và trượt.

> **Đặt `min` theo thứ BÀI thật sự đòi hỏi, không theo số lần lời giải mẫu
> chạm vào cái tên. Trước khi chốt con số, nghĩ ra MỘT cách viết đúng khác và
> đếm lại trên nó.**

Nếu không nghĩ ra cách viết đúng nào khác, thường là luật ấy không cần thiết —
`assert` chấm theo KẾT QUẢ đã đủ, và nó không bắt người học đi bằng đúng một
con đường.

## Luật 3 — Đề bài và cổng phải nói cùng một điều

Một bài viết đề mở — "điền vào đó thứ mà `len` đo được" — rồi cổng chỉ nhận
`str`. `len(f"{tien}")` ra đúng kết quả, qua hết assert, và trượt.

> **Đề bài hỏi rộng thì cổng phải nhận rộng. Cổng hẹp thì đề bài phải nêu đích
> danh công cụ.** Và khi cổng cố ý hẹp, câu `onFail` nên nói thẳng rằng cách
> kia cũng đúng, chỉ là bài này đang dạy cái này.

---

## Ba câu tự hỏi trước khi nộp một khối chấm

1. Ba cách điền hụt tôi đoán được, có cách nào ra cùng màn hình với lời giải
   đúng không?
2. Một cách viết đúng KHÁC lời giải mẫu, có qua được mọi tầng không?
3. Đề bài và `onFail` có nói cùng một điều với cổng không?

Câu thứ hai là câu bị bỏ qua nhiều nhất — mười ba trên mười ba.
