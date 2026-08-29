---
id: nen-tang.gia-tri-bien-kieu.so-nguyen-va-so-thuc
title: Con số có cái đuôi .0
summary: Chỉ một số thực lẫn vào là cả phép tính đổi sang số thực, và cái đuôi ấy bám theo tới cuối dòng sổ.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.float-contagion]
requires: [core.float, core.division, core.arithmetic, core.type-of-value]
concepts: [core.kieu-gia-tri, core.so-thap-phan]
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
Cái đuôi chấm-không kia không tự rụng đâu. Nó còn đi theo bạn tới cuối dòng sổ.
::::

::::explain{#quyen-so-cua-byte}
Realm 0 khép lại bằng một cái máy biết hỏi khách rồi trả lời. Realm 1 mở ra
bằng một quyển sổ.

Sổ chi tiêu của Byte. Mỗi ngày vài dòng, mỗi dòng ba ô:

| tên khoản | số tiền | ghi chú |
|---|---|---|
| cà phê | 25000 | |
| phở bò | 90000 | đi ăn chung, 2 người |
| thịt bò | ? | mua 1.5 cân |

Ô ghi chú không phải chỗ trang trí. Với khoản đi ăn chung nó giữ **số người
chia**; với khoản đi chợ nó giữ **số cân**. Suốt chặng này, mọi thứ bạn học đều
rơi xuống đúng quyển sổ ấy — và tới bài cuối bạn in ra được một dòng sổ hoàn
chỉnh, thẳng cột, không thừa một ký tự nào.

Dòng thứ hai gây chuyện ngay hôm nay.

Chín mươi nghìn, hai người chia nhau. Ở Realm 0 bạn đã biết kết quả trông thế
nào: dấu `/` luôn cho ra một giá trị mang nhãn `float`, kể cả khi chia hết
không dư đồng nào. `90000 / 2` ra `45000.0`, không phải `45000`.

Chuyện đó bạn đã gặp rồi. Hôm nay là chuyện chưa gặp: **cái đuôi ấy không dừng
lại ở đó.**
::::

::::example{#duoi-khong-chiu-rung}
Byte chia tiền phở, rồi cộng thêm tiền gửi xe.

```python title=readonly
tien_ca_ban = 90000
so_nguoi = 2
phan_moi_nguoi = tien_ca_ban / so_nguoi
print(phan_moi_nguoi)

gui_xe = 5000
print(phan_moi_nguoi + gui_xe)
```

Máy in ra:

```text
45000.0
50000.0
```

Dòng đầu thì bạn đoán được. Dòng thứ hai mới lạ.

Nhìn kỹ dòng lệnh cuối: trong đó không có dấu `/` nào cả. Chỉ có một dấu cộng,
với hai số hạng — `phan_moi_nguoi` mang nhãn `float`, còn `gui_xe` là `5000`
viết trần, mang nhãn `int`. Cộng xong, kết quả vẫn đeo cái đuôi `.0`.

Bốn mươi lăm nghìn cộng năm nghìn ra năm mươi nghìn. Con số không sai một đồng.
Chỉ có cái nhãn là đi theo.
::::

::::predict{#hai-dong-in commitOnce}
Byte sắp chạy đoạn dưới. Hai dòng `print`, và cả hai đều chỉ dùng dấu cộng.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra hai con số nào?

```python
phan_moi_nguoi = 90000 / 2
gui_xe = 5000
print(phan_moi_nguoi + gui_xe)
print(gui_xe + 3000)
```

:::opt{correct}
50000.0 rồi 8000
:::

:::opt
50000.0 rồi 8000.0
::why
Gần đúng ở chỗ bạn nhận ra dòng đầu đã kéo một `float` vào cuộc, và bạn theo
nó tới cùng — dòng đầu bạn đoán trúng từng ký tự.

Chỗ lệch nằm ở câu hỏi "cái đuôi bám vào đâu". Nó bám vào **giá trị**, không
bám vào chương trình. Ở dòng cuối, `gui_xe` là `5000` và `3000` cũng viết trần
— hai số hạng đều mang nhãn `int`, và chúng chẳng dính dáng gì tới phép chia ở
trên. Máy tính riêng từng phép, nhìn riêng hai vế của phép đó.
::
:::

:::opt
50000 rồi 8000
::why
Gần đúng ở chỗ bạn tính đúng cả hai con số, và bạn suy nghĩ theo một luật rất
hợp lý: cộng thêm một số tròn thì phần lẻ vẫn bằng không, nên cái đuôi hết việc
và rụng đi.

Chỗ lệch: máy chọn nhãn theo **phép tính và hai vế của nó**, không chọn theo
kết quả tình cờ tròn hay lẻ. Đây đúng là chuyện bạn đã gặp ở Realm 0 — `90000 /
2` chia hết mà vẫn ra `45000.0` — chỉ khác là lần này nó xảy ra ở phép cộng.
::
:::

:::opt
Máy dừng lại và báo TypeError ở dòng in đầu tiên
::why
Gần đúng ở chỗ bạn nhớ một bài rất đáng nhớ của Realm 0: ghép một câu chữ với
một con số thì máy dừng hẳn, vì không có quy ước nào cho việc đó.

Chỗ lệch là ở chỗ `int` và `float` **đều là số**. Máy có sẵn quy ước cho việc
cộng hai loại số này với nhau, và quy ước ấy chính là bài hôm nay. `TypeError`
dành cho hai kiểu không đi cùng nhau được — chữ với số — chứ không dành cho hai
loại số.
::
:::
::::

::::explain{#mot-giot-muc}
Một ca nước trong, nhỏ vào đúng một giọt mực. Cả ca đổi màu. Và rót thêm bao
nhiêu nước trong nữa cũng không làm ca nước trong lại — cộng thêm, nhân thêm
bao nhiêu số nguyên cũng vậy. Muốn có nước trong thì phải có một cái vợt riêng,
chứ không phải rót thêm.

Con số trong Python cũng vậy:

> Trong một phép tính, chỉ cần **một** vế mang nhãn `float` là kết quả mang
> nhãn `float`. Chiều ngược lại thì không chắc: hai vế cùng `int` thường cho
> `int`, nhưng riêng dấu `/` **luôn** cho `float` — kể cả `90000 / 2`. Đó chính
> là đường giọt mực rơi vào sổ mà bạn không tự gõ dấu chấm nào.

Đặt cạnh nhau cho rõ:

| phép tính | kết quả | nhãn |
|---|---|---|
| `45000 + 5000` | `50000` | `int` |
| `90000 / 2` | `45000.0` | `float` |
| `45000.0 + 5000` | `50000.0` | `float` |
| `45000 * 2` | `90000` | `int` |
| `120000 * 1.5` | `180000.0` | `float` |
| `45000 - 0.5` | `44999.5` | `float` |

Ẩn dụ đã xong, giờ tới thuật ngữ: việc máy kéo `5000` lên thành `float` để cộng
được với `45000.0` gọi là **nâng kiểu** — tiếng Anh là *type promotion*. Đó là
từ để bạn tra cứu khi cần.

Một giọt mực rơi vào sổ của bạn bằng đúng hai đường:

- **Bạn tự gõ ra** một con số có dấu chấm: `can_thit = 1.5`.
- **Một phép chia `/` ở đâu đó phía trên** đã sinh ra nó, rồi bạn cộng, trừ,
  nhân tiếp lên kết quả ấy.

Đường thứ hai nguy hơn, vì dấu chấm không nằm ở dòng bạn đang nhìn. Nó nằm cách
đó ba dòng, và bạn phải nhớ ra.
::::

::::code{#hai-dong-so-cho}
Sổ chợ sáng nay có hai dòng.

- **Thịt bò**: mua `1.5` cân, giá `120000` một cân.
- **Rau**: một bó rau `5000` và một mớ hành `3000`.

Điền hai chỗ trống để máy in ra số tiền của từng dòng. Bài chấm cả hai dòng chứ
không riêng dòng nào — vì hai dòng này khác nhau đúng ở chỗ bài đang dạy: một
dòng có giọt mực, một dòng không.

```python title=starter
gia_mot_can = 120000
can_thit = 1.5
tien_thit = ___

bo_rau = 5000
mo_hanh = 3000
tien_rau = ___

print(tien_thit)
print(tien_rau)
```

```python title=solution
gia_mot_can = 120000
can_thit = 1.5
tien_thit = gia_mot_can * can_thit

bo_rau = 5000
mo_hanh = 3000
tien_rau = bo_rau + mo_hanh

print(tien_thit)
print(tien_rau)
```

```python title=test
# Chấm bằng HAI dòng sổ, và kiểm cả NHÃN chứ không chỉ con số.
#
# Vì sao phải kiểm nhãn: trong Python `180000.0 == 180000` cho `True`. Một
# phép so sánh bằng không phân biệt nổi `float` với `int` — mà đó đúng là thứ
# bài này dạy. Phải hỏi thẳng cái nhãn.
#
# Vì sao hỏi bằng `.__class__.__name__` chứ không bằng `type(x) is int`: bộ
# chấm chạy nhiều bài trong cùng một phiên Python, và bài 13 của chính track
# này dạy đè lên tên có sẵn (`int = 0`). Sau bài ấy, `int` không còn là cái
# kiểu nữa. Cách hỏi dưới đây không tra một cái tên có sẵn nào, nên nó đúng
# bất kể ai đã đè lên cái gì.
#
# Vì sao phải hai dòng chứ không một: một dòng thì không lộ ra chỗ hụt.
#   viết `gia_mot_can * can_thit` cho cả hai chỗ  → dòng rau sai con số;
#   điền một con số chết vào chỗ trống            → dòng kia sai;
#   viết `bo_rau + mo_hanh` cho cả hai chỗ        → dòng thịt sai;
#   dùng `/` thay cho `*` ở dòng thịt             → sai con số.
# Chỉ một dòng có `1.5` lẫn vào, nên chỉ một dòng được mang nhãn `float` —
# người học phải nhìn ra sự khác nhau ấy mới qua được cả hai câu assert cuối.
assert tien_thit == 180000.0, "tiền thịt phải là 180000.0"
assert tien_thit.__class__.__name__ == "float", "tiền thịt phải mang nhãn float vì có 1.5 lẫn vào phép nhân"
assert tien_rau == 8000, "tiền rau phải là 8000"
assert tien_rau.__class__.__name__ == "int", "tiền rau phải mang nhãn int vì cả hai số hạng đều là int"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trên hai dòng khác nhau, và hai dòng ấy không cùng một phép tính. Nhìn ba dòng ngay phía trên mỗi chỗ trống: một bên có `1.5`, một bên không có dấu chấm nào.
- kind: strategy
  body: Tiền thịt là giá một cân nhân với số cân. Tiền rau là hai món cộng lại. Cả hai phép đều viết được bằng những cái tên đã có sẵn phía trên, không cần gõ lại con số nào.
- kind: one-line
  body: "Viết `gia_mot_can * can_thit` vào chỗ trống thứ nhất và `bo_rau + mo_hanh` vào chỗ trống thứ hai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^180000\.0\n8000\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một lệnh in, hai cái đuôi khác nhau. Nhãn đi theo giá trị, không đi theo dòng lệnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Dòng sổ của bạn giờ ghi `180000.0`. Đưa quyển sổ ấy cho bác bán thịt xem, bác
sẽ hỏi lại chấm-không là cái gì. Không ai viết đuôi `.0` lên giấy, và không máy
tính tiền nào ở quán in ra `45000.0đ`.

Con số thì đúng. Chỉ cái nhãn là thừa.

Vậy muốn bảo máy kéo `180000.0` về lại một số nguyên — bỏ hẳn cái đuôi — thì gõ
gì? Bạn đã cầm sẵn một công cụ từ Realm 0 chuyên làm việc đổi kiểu, chỉ khác là
hồi đó bạn đặt nó lên một câu chữ chứ chưa đặt lên một con số có phần lẻ.

Và một câu nữa, khó hơn nhiều. Nếu con số không tròn trịa như `180000.0` mà là
`45000.7` thì sao? Cái công cụ ấy cho ra `45001`, hay cho ra `45000`?

Hai kết quả cách nhau đúng một đồng. Bài sau trả lời cả hai câu — và câu thứ
hai mới là câu đắt tiền.
::::

::::checkpoint{mastery=0.8}
::::
