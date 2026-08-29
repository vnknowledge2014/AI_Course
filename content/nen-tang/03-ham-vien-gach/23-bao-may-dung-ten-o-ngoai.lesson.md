---
id: nen-tang.ham-vien-gach.bao-may-dung-ten-o-ngoai
title: Bảo máy dùng đúng cái tên ở ngoài
summary: Một dòng `global` đặt trong thân hàm khiến phép gán trỏ thẳng vào cái tên ngoài kia, thay vì dựng một cái tên cục bộ mới.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.global-keyword]
requires: [core.local-assignment, core.global-variable, core.function-def, core.function-parameter, core.function-call, core.assignment, core.accumulator, core.fstring]
concepts: [core.ham, core.bien, core.pham-vi]
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
Hôm nay mình muốn viết thẳng lên tấm bảng. Phải xin phép một câu.
::::

::::explain{#mot-cau-xin-phep}
Byte muốn một cái tên `tong_doanh_thu` sống suốt cả ngày, và mỗi lần bán được
một tô thì nó lớn thêm. Việc cộng dồn ấy Byte muốn gói vào một hàm để chỗ nào
cũng gọi được.

Bài trước vừa chỉ ra vì sao cách viết thẳng tuột không chạy: phép gán trong
thân hàm luôn dựng một cái tên cục bộ mới, nên con số cộng dồn lớn lên trên một
tờ giấy nháp rồi bỏ đi cùng lượt gọi. Cuối ngày tấm bảng ngoài kia vẫn ghi 0.

Chỗ thiếu không phải là một phép gán khác. Phép gán đã đúng rồi. Chỗ thiếu là
máy **không biết** bạn đang nói tới cái tên nào, và nó có sẵn một luật để
đoán — bên trái dấu `=` thì dựng tên cục bộ. Luật ấy đúng cho hầu hết trường
hợp, nên muốn khác đi thì bạn phải nói ra.

Câu nói ra ấy là `global`:

```python title=readonly
global tong_doanh_thu
```

Đặt dòng đó ở **đầu thân hàm**, trước mọi dòng có nhắc tới cái tên. Nó không
tính toán gì, không dán tên lên giá trị nào — nó chỉ báo cho máy một chuyện,
đúng một chuyện:

> Trong hàm này, `tong_doanh_thu` nghĩa là cái tên ngoài kia. Đừng dựng tên mới.

Từ đó về sau, mọi phép gán cho `tong_doanh_thu` trong thân hàm đều ghi thẳng
vào tấm bảng trên tường. Không còn tờ nháp nào nữa.

Cái tên ngoài hàm ấy, bài trước đã gọi đúng tên nó: **biến toàn cục**. Và dòng
`global` là cách bạn nói với máy rằng lần này mình muốn đúng cái toàn cục đó.
::::

::::example{#ghi-thang-len-bang}
Sáng nay quán bán hai tô. Đoạn dưới cộng dồn tiền của cả hai vào một cái tên
sống ngoài hàm:

```python title=readonly
tong_doanh_thu = 0

def ghi_doanh_thu(tien):
    global tong_doanh_thu
    tong_doanh_thu = tong_doanh_thu + tien
    print(f"Ghi {tien} đồng — tổng đang là {tong_doanh_thu} đồng")

ghi_doanh_thu(45000)
ghi_doanh_thu(60000)
print(f"Cuối ngày: {tong_doanh_thu} đồng")
```

Máy in ra:

```text
Ghi 45000 đồng — tổng đang là 45000 đồng
Ghi 60000 đồng — tổng đang là 105000 đồng
Cuối ngày: 105000 đồng
```

Đi lại đúng đường máy đi:

- Dòng đầu dựng cái tên `tong_doanh_thu` ngoài mọi hàm, mang số 0.
- Lượt gọi thứ nhất mở ra. Dòng `global` báo với máy rằng trong hàm này cái tên
  ấy là cái tên ngoài kia. Dòng cộng đọc ra 0, cộng thêm 45000, rồi ghi kết quả
  thẳng lên tấm bảng.
- Hàm chạy xong, nhưng lần này không có tờ nháp nào bỏ đi cả. Con số ở lại.
- Lượt gọi thứ hai đọc ra **45000** chứ không phải 0 — vì lượt trước đã ghi vào
  đó thật. Cộng thêm 60000 rồi ghi lại: 105000.
- Dòng `print` cuối đứng ngoài mọi hàm. Nó đọc tấm bảng và thấy đúng con số mà
  hàm vừa ghi.

Cùng một cách viết `tong_doanh_thu = tong_doanh_thu + tien`, mà bài trước thì
không đi ra khỏi hàm còn bài này thì có. Khác nhau đúng một dòng, và dòng đó
không làm gì ngoài việc chỉ chỗ.
::::

::::predict{#doan-hai-ham commitOnce}
Byte viết hai hàm cùng đếm số tô đã bán: một hàm cho khách ngồi quầy, một hàm
cho khách mang về. Hai hàm giống nhau gần hết, chỉ khác một dòng.

**Trước khi bấm chạy**, bạn đoán dòng cuối in ra số mấy?

```python title=readonly
so_to_da_ban = 0

def ban_qua_quay(n):
    global so_to_da_ban
    so_to_da_ban = n

def ban_mang_ve(n):
    so_to_da_ban = n

ban_qua_quay(3)
ban_mang_ve(10)
print(so_to_da_ban)
```

:::opt{correct}
3
:::

:::opt
10
::why
Gần đúng ở chỗ bạn theo dõi thứ tự chạy rất chuẩn: `ban_mang_ve(10)` chạy sau,
và khi hai phép gán cùng nhắm vào một cái tên thì phép sau đè lên phép trước.
Suy luận ấy đúng — nếu cả hai phép gán thật sự nhắm vào cùng một cái tên.

Chỗ lệch là chúng không. `ban_mang_ve` thiếu dòng `global`, nên phép gán trong
nó quay về luật mặc định của bài trước: dựng một cái tên cục bộ mới, sống hết
lượt gọi rồi bỏ đi. Số 10 có thật, nó chỉ không bao giờ ra tới tấm bảng ngoài
kia. Cái tên ngoài đó vẫn giữ số 3 mà `ban_qua_quay` đã ghi.
::
:::

:::opt
13
::why
Gần đúng ở chỗ bạn đọc ra ý định của cả đoạn: hai lần bán thì tổng số tô phải
gộp lại, và 13 đúng là con số mà một cái sổ bán hàng tử tế phải cho ra.

Chỗ lệch nằm ở dấu `=`. Cả hai hàm đều viết `so_to_da_ban = n` — dán tên lên
một giá trị mới, không cộng thêm gì vào giá trị cũ. Muốn cộng dồn thì dòng ấy
phải đọc con số đang có rồi mới ghi lại, như `tong_doanh_thu` ở ví dụ trên. Ở
đây mỗi lời gọi chỉ đơn giản ghi đè.
::
:::

:::opt
0
::why
Gần đúng ở chỗ bạn giữ chắc kết luận của bài trước: hàm gán cho một cái tên
trùng với tên ngoài thì cái tên ngoài không suy suyển. Với `ban_mang_ve`, điều
đó vẫn đúng nguyên.

Chỗ lệch là `ban_qua_quay` có thêm một dòng mà bài trước chưa có. Dòng `global
so_to_da_ban` gỡ đúng cái luật ấy đi cho riêng hàm này — phép gán bên dưới nó
không dựng tên mới nữa mà ghi thẳng ra ngoài. Nên số 0 đã bị số 3 thay chỗ từ
lời gọi đầu tiên.
::
:::
::::

::::explain{#ba-chuyen-can-nho}
Ba chuyện đáng ghim lại về dòng `global`:

- **Nó nói về một cái tên, không phải về một giá trị.** Viết `global x` không
  dán gì lên `x`, không đọc `x` ra. Nó chỉ đổi cách máy hiểu chữ `x` trong
  đúng thân hàm này.
- **Nó đặt ở đầu thân hàm.** Đặt sau khi đã dùng cái tên thì máy đã lỡ hiểu
  theo cách cũ mất rồi, và bạn sẽ nhận về một thông báo lỗi thay vì một chương
  trình chạy. Cứ để nó làm dòng đầu tiên, ngay dưới `def` — chỗ ấy dễ nhìn cho
  cả người đọc.
- **Nó chỉ có hiệu lực trong một hàm.** Hàm khác muốn ghi ra cùng cái tên ấy
  thì phải tự khai lại dòng của mình, như `ban_qua_quay` với `ban_mang_ve` vừa
  rồi.

Còn một chuyện nữa, và nó không phải chi tiết kỹ thuật: kể từ lúc có dòng
`global`, hàm của bạn không còn tự lo lấy phần việc của mình nữa. Nó thò tay ra
ngoài. Chuyện đó **chạy được** — bạn vừa nhìn thấy — nhưng nó đổi một thứ khác
mà chưa ai nói tới. Giữ câu này trong đầu khi làm phần tiếp theo.
::::

::::code{#cong-don-ca-ngay}
Byte bán hai tô phở, cùng giá 45 nghìn, và muốn cái tên `tong_doanh_thu` ngoài
kia lớn lên thật sau mỗi lần ghi.

Đoạn dưới đã có đủ phép gán cộng dồn. Nó còn thiếu đúng câu nói với máy rằng
`tong_doanh_thu` trong hàm này là cái tên ngoài kia.

Hãy điền dòng còn thiếu ở đầu thân hàm.

```python title=starter
tong_doanh_thu = 0

def ghi_doanh_thu(tien):
    ___
    tong_doanh_thu = tong_doanh_thu + tien
    print(f"Ghi {tien} đồng — tổng đang là {tong_doanh_thu} đồng")

ghi_doanh_thu(45000)
ghi_doanh_thu(45000)
print(f"Cuối ngày: {tong_doanh_thu} đồng")
```

```python title=solution
tong_doanh_thu = 0

def ghi_doanh_thu(tien):
    global tong_doanh_thu
    tong_doanh_thu = tong_doanh_thu + tien
    print(f"Ghi {tien} đồng — tổng đang là {tong_doanh_thu} đồng")

ghi_doanh_thu(45000)
ghi_doanh_thu(45000)
print(f"Cuối ngày: {tong_doanh_thu} đồng")
```

```python title=test
# Hai tô cùng 45 nghìn, nên chỉ có một con số chứng minh được rằng lượt gọi thứ
# hai đọc ra thứ mà lượt đầu đã ghi: 90 nghìn. Hàm nào cộng dồn trên tờ nháp
# riêng của mình thì cái tên ngoài đây vẫn còn nguyên số 0.
assert tong_doanh_thu == 90000, "sau hai lần ghi 45 nghìn, cái tên ngoài hàm phải mang 90 nghìn — nghĩa là phép gán trong hàm đã ghi thẳng ra ngoài chứ không dựng một cái tên cục bộ mới"
```

:::hints
- kind: attention
  body: Chỗ trống là dòng đầu tiên của thân hàm, đứng trên cả dòng cộng dồn. Dòng cộng dồn thì đã đúng rồi, đừng sửa nó — thứ còn thiếu là một câu chỉ chỗ cho máy.
- kind: strategy
  body: Cần đúng một từ khoá đi kèm tên biến, và nó không có dấu `=` nào cả. Câu ấy nói với máy rằng trong hàm này, cái tên `tong_doanh_thu` là cái tên đang sống ngoài kia chứ không phải một cái tên mới.
- kind: one-line
  body: Viết `global tong_doanh_thu` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay dưới.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Ghi 45000 đồng — tổng đang là 45000 đồng\nGhi 45000 đồng — tổng đang là 90000 đồng\nCuối ngày: 90000 đồng\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tấm bảng trên tường lớn lên thật rồi. Không còn tờ nháp nào bỏ đi nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Chạy đúng ý bạn. Nhưng nhìn lại hai dòng vừa in ra mà xem — bạn gọi
`ghi_doanh_thu(45000)` hai lần với cùng một đối số, hai lần lại cho ra hai kết
quả khác nhau: lần đầu báo tổng 45 nghìn, lần sau báo 90 nghìn.

Cùng đầu vào mà khác đầu ra — vì sao?

Hãy so với `len("phở")` ở bài đầu mạch này: gọi bao nhiêu lần, gọi lúc nào,
nó cũng đưa ra đúng một con số. Hàm vừa viết thì không như vậy.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
