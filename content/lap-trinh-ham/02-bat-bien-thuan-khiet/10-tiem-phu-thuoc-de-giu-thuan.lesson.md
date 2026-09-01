---
id: lap-trinh-ham.bat-bien-thuan-khiet.tiem-phu-thuoc-de-giu-thuan
title: "Tiêm phụ thuộc (dependency injection) để giữ hàm thuần"
summary: "Thay vì hàm TỰ GỌI random/datetime.now() bên trong, TRUYỀN chúng vào như một tham số — hàm giờ thuần thật sự: với cùng đối số VÀ cùng tham số phụ thuộc, luôn cùng kết quả. Cái không tất định bị đẩy RA NGOÀI, không biến mất."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.dependency-injection]
requires: [fp.io-is-side-effect]
concepts: [fp.dependency-injection]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Hàm cần biết giờ hiện tại. Vẫn có cách giữ nó thuần — không cần né
tránh nhu cầu đó, chỉ cần đừng tự đi lấy giờ.
::::

::::explain{#tiem-thay-vi-tu-goi}
Bài trước để lại một tình huống thật: một hàm chào theo buổi
(sáng/trưa/tối) cần biết giờ hiện tại để hoạt động đúng. Cách viết quen
thuộc là tự gọi `datetime.now()` ngay bên trong hàm — nhưng bài 8 đã
chỉ ra, làm vậy hàm mất tính đoán trước được ngay lập tức.

Lối ra không phải "đừng biết giờ", mà là "đừng TỰ ĐI LẤY giờ". Thay vì
hàm tự gọi nguồn không tất định, người GỌI hàm truyền nguồn đó vào như
một THAM SỐ — một hàm khác, được gọi ĐÚNG lúc cần, do bên ngoài quyết
định là hàm nào. Kỹ thuật này gọi là **tiêm phụ thuộc** (dependency
injection): thay vì hàm tự dựng lấy thứ nó cần, thứ đó được TIÊM (truyền
vào) từ ngoài.

```python
def chao_theo_gio(ten, lay_thoi_diem):
    return f"Chào {lay_thoi_diem()}, {ten}!"
```

`chao_theo_gio` không hề gọi `datetime.now()`. Nó nhận một THAM SỐ THỨ
HAI — `lay_thoi_diem`, một hàm không đối số, trả về một chuỗi mô tả thời
điểm — rồi gọi CHÍNH tham số đó. Giờ `chao_theo_gio` lại thuần theo đúng
định nghĩa: với CÙNG `ten` và CÙNG `lay_thoi_diem`, luôn CÙNG kết quả.
Sự không tất định không biến mất — nó chỉ bị ĐẨY RA NGOÀI, tới tay
người gọi hàm. Test gọi hàm với một hàm giả cố định (`lambda: "buổi
sáng"`), lúc nào cũng cùng chuỗi ấy; sản phẩm thật gọi hàm với
`datetime.now`, có giờ thật.
::::

::::example{#mot-ham-hai-cach-goi}
Byte gọi cùng một `chao_theo_gio`, nhưng tiêm vào hai `lay_thoi_diem`
khác nhau — một cho môi trường kiểm thử, một mô phỏng buổi tối thật.

```python title=readonly
def chao_theo_gio(ten, lay_thoi_diem):
    return f"Chào {lay_thoi_diem()}, {ten}!"

# Kiểm thử: hàm giả, luôn trả về đúng một chuỗi cố định
gio_gia = lambda: "buổi sáng"

# "Sản phẩm": một hàm khác, mô phỏng đọc giờ thật lúc này là buổi tối
gio_that = lambda: "buổi tối"

print(chao_theo_gio("Lan", gio_gia))
print(chao_theo_gio("Lan", gio_gia))
print(chao_theo_gio("Mai", gio_that))
```

```text title=readonly
Chào buổi sáng, Lan!
Chào buổi sáng, Lan!
Chào buổi tối, Mai!
```

Hai lời gọi đầu dùng CÙNG `ten` và CÙNG `gio_gia` — kết quả giống hệt
nhau, đúng như một hàm thuần phải có. `chao_theo_gio` bản thân nó không
đổi giữa hai dòng; chỉ có THAM SỐ tiêm vào ở dòng ba đổi
(`gio_that` thay vì `gio_gia`), nên kết quả đổi theo — đúng và chỉ đúng
theo cách một hàm thuần được phép đổi kết quả: đổi vì đối số đổi, không
phải vì thời điểm gọi đổi.
::::

::::predict{#doan-goi-lambda-khac-nhau commitOnce}
`tinh_gia_km` dưới đây tính giá một chuyến xe — số ki-lô-mét nhân đơn
giá, và đơn giá được TIÊM vào chứ không hardcode trong hàm.

**Trước khi bấm chạy**, bạn đoán hai dòng in ra gì?

```python
def tinh_gia_km(so_km, lay_don_gia):
    return so_km * lay_don_gia()

gia_thuong = lambda: 12000
gia_gio_cao_diem = lambda: 18000

print(tinh_gia_km(10, gia_thuong))
print(tinh_gia_km(10, gia_gio_cao_diem))
```

:::opt{correct}
`120000` rồi `180000` — cùng `so_km=10`, chỉ khác hàm đơn giá được tiêm
vào, nên kết quả đổi đúng theo hàm đó
:::

:::opt
`120000` rồi `120000` — `tinh_gia_km` là một hàm THUẦN, mà hàm thuần
phải luôn ra cùng kết quả cho dù gọi thế nào
::why
Gần đúng ở việc bạn nhớ đúng luật "hàm thuần, cùng đối số, cùng kết
quả" — luật đó có thật.

Chỗ lệch: hai lời gọi ở đây KHÔNG cùng đối số. `lay_don_gia` là MỘT
tham số của `tinh_gia_km`, và nó đổi giữa hai dòng (`gia_thuong` rồi
`gia_gio_cao_diem`) — đổi tham số thì kết quả được PHÉP đổi theo, đó
đúng là cách một hàm thuần "phản ứng" với input khác nhau, không phải
một ngoại lệ phá luật.
::
:::

:::opt
Máy dừng lại báo lỗi, vì `lay_don_gia` không phải một số nên không nhân
được với `so_km`
::why
Gần đúng ở việc bạn để ý `lay_don_gia` không phải một con số ngay trong
lời gọi hàm — nó là một HÀM.

Chỗ lệch: thân `tinh_gia_km` không nhân `so_km` với `lay_don_gia` (cái
hàm) — nó nhân với `lay_don_gia()` (KẾT QUẢ của việc GỌI hàm đó, có dấu
ngoặc tròn). `gia_thuong()` chạy ra `12000`, một số nguyên bình thường,
nhân với `so_km` hợp lệ hoàn toàn.
::
:::

:::opt
`180000` rồi `120000` — Python tính từ dưới lên, nên dòng `print` cuối
chạy trước
::why
Gần đúng ở việc hai kết quả có xuất hiện trong bài, chỉ là bạn đảo thứ
tự.

Chỗ lệch: Python không có quy tắc "tính từ dưới lên". Các câu lệnh chạy
TUẦN TỰ từ trên xuống, đúng thứ tự viết trong file — dòng
`print(tinh_gia_km(10, gia_thuong))` luôn chạy VÀ in ra TRƯỚC dòng
`print(tinh_gia_km(10, gia_gio_cao_diem))` phía dưới nó.
::
:::
::::

::::code{#loi-chao-tiem-gio}
Cửa hàng cần một hàm chào khách theo buổi. Mặc định (không truyền tham
số thứ hai) hàm chào "buổi sáng" — dùng cho lúc chưa nối được đồng hồ
thật. Điền đúng MỘT chỗ trống để hàm THẬT SỰ dùng tham số được tiêm vào,
không tự bịa ra buổi nào cả.

```python title=starter
def chao_theo_gio(ten, lay_thoi_diem=lambda: "buổi sáng"):
    return f"Chào {___}, {ten}!"

print(chao_theo_gio("Lan"))
print(chao_theo_gio("Mai", lambda: "buổi tối"))
```

```python title=solution
def chao_theo_gio(ten, lay_thoi_diem=lambda: "buổi sáng"):
    return f"Chào {lay_thoi_diem()}, {ten}!"

print(chao_theo_gio("Lan"))
print(chao_theo_gio("Mai", lambda: "buổi tối"))
```

```python title=test
assert chao_theo_gio("Lan") == "Chào buổi sáng, Lan!", "không truyền lay_thoi_diem thì phải dùng đúng mặc định 'buổi sáng'"
assert chao_theo_gio("Mai", lambda: "buổi tối") == "Chào buổi tối, Mai!", "truyền lay_thoi_diem khác thì kết quả phải đổi THEO đúng hàm đó — đừng bỏ qua tham số"
assert chao_theo_gio("An", lambda: "buổi trưa") == chao_theo_gio("An", lambda: "buổi trưa"), "cùng ten VÀ cùng lay_thoi_diem thì hai lần gọi phải ra cùng kết quả — đây chính là hàm thuần"
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên trong f-string, ngay vị trí phải điền phần "thời điểm" của lời chào. Đừng gõ tay chuỗi "buổi sáng" — nó phải đến từ tham số lay_thoi_diem.
- kind: strategy
  body: lay_thoi_diem là một hàm KHÔNG đối số, được truyền vào (hoặc lấy mặc định). Muốn lấy chuỗi nó trả về, phải GỌI nó — thêm dấu ngoặc tròn rỗng ngay sau tên, không chỉ nhắc tên suông.
- kind: one-line
  body: "Điền `lay_thoi_diem()` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi lay_thoi_diem() — không hardcode chuỗi "buổi sáng"/"buổi tối", không import datetime để tự lấy giờ thay vì dùng tham số được tiêm vào
  requireAst:
  - kind: uses-call, target: lay_thoi_diem, min: 1
  - kind: no-import
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Chào buổi sáng, Lan!\nChào buổi tối, Mai!\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`chao_theo_gio` không hề biết giờ thật là mấy giờ — và đó chính xác là
điều làm nó đáng tin. Ai gọi nó mới là người quyết định giờ nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa TIÊM `lay_thoi_diem` vào thay vì để hàm tự gọi `datetime.now()`
— `chao_theo_gio` giờ thuần thật, kiểm được bằng `lambda` cố định. Nhưng
làm sao BIẾT CHẮC một hàm bất kỳ — không phải hàm bạn vừa viết, mà một
hàm ai đó khác đưa cho bạn — có thuần hay không, mà không phải đọc từng
dòng thân hàm bằng mắt?

Bài sau đưa ra một phép kiểm nhanh.
::::

::::checkpoint{mastery=0.8}
::::
