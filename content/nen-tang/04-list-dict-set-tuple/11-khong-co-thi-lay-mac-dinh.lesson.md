---
id: nen-tang.list-dict-set-tuple.khong-co-thi-lay-mac-dinh
title: Không có thì lấy mặc định
summary: "`.get(khoá, dự_phòng)` là lối hỏi có mang sẵn câu trả lời phòng khi khoá vắng mặt — sổ không có thì nhận về con số bạn chọn, chương trình không dừng."
locale: vi
track: nen-tang
module: list-dict-set-tuple
order: 11
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.dict-get-default]
requires: [core.dict, err.key-error, core.string-method, core.none, core.fstring, core.variable, core.accumulator]
concepts: [core.so-tra-cuu, core.gia-tri-canh]
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
Hỏi mình mà mang sẵn theo một câu trả lời dự phòng, thì mình không phải kêu.
::::

::::explain{#hoi-kem-mot-cau-tra-loi-du-phong}
Bài trước kết ở một chỗ khó xử. Bản báo cáo cuối tháng có những dòng cố định,
tháng nào cũng in đủ — và một trong số đó là *sửa nhà*, thứ mà tháng này cuốn
sổ không hề có khoá. Tra nó bằng ngoặc vuông thì chương trình chết đứng, trong
khi thứ bạn muốn in ra chỉ là `Sửa nhà: 0 đồng`.

Chú ý một điều: con số `0` ấy là **quyết định của bạn**, không phải sự thật
nằm trong sổ. Bài trước đã nói vì sao máy không tự đưa ra nó — máy không phân
biệt được "nhóm này tiêu hết 0 đồng" với "bạn gõ nhầm tên nhóm". Nhưng ở đây
thì bạn biết rõ mình đang hỏi gì và muốn gì, nên bạn nói ra quyết định ấy.

Python cho bạn một lối hỏi thứ hai, và nó nhận **hai** thứ: khoá cần tra, và
câu trả lời dự phòng dùng khi khoá vắng mặt.

```python title=readonly
chi.get("sửa nhà", 0)
```

Đọc thành lời: *tra sổ `chi` bằng khoá `"sửa nhà"`; có thì đưa giá trị của
khoá ấy ra, không có thì đưa ra `0`.*

Cách viết này không có gì mới về cú pháp. Dấu chấm sau một giá trị rồi tới một
cái tên kèm ngoặc — đó đúng là **phương thức**, thứ bạn đã dùng với chuỗi.
`.get` chỉ là một phương thức nữa, lần này của cuốn sổ tra cứu.

Điểm đáng nhớ nhất: `.get` **không bao giờ dừng chương trình**. Khoá có thì
nó đưa giá trị thật; khoá không có thì nó đưa thứ bạn đã chuẩn bị sẵn. Cả hai
đường đều đi tiếp.
::::

::::example{#ba-dong-du-mat}
Vẫn cuốn sổ bốn khoá của Byte, mà bản báo cáo cần ba dòng cố định — và một
trong ba dòng ấy không có khoá tương ứng trong sổ:

```python title=readonly
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

print(f"Ăn sáng: {chi.get('ăn sáng', 0)} đồng")
print(f"Đổ xăng: {chi.get('đổ xăng', 0)} đồng")
print(f"Sửa nhà: {chi.get('sửa nhà', 0)} đồng")
```

Máy in ra:

```text
Ăn sáng: 85000 đồng
Đổ xăng: 120000 đồng
Sửa nhà: 0 đồng
```

Ba dòng, không một lời báo lỗi. Hai dòng đầu lấy con số thật từ trong sổ; dòng
thứ ba lấy con số dự phòng mà chính bạn viết ra.

Và có một chuyện **không** xảy ra, đáng nhìn ngang với chuyện đã xảy ra: sau
ba dòng ấy, cuốn sổ `chi` vẫn còn đúng bốn khoá như lúc đầu. Khoá `"sửa nhà"`
không tự mọc ra chỉ vì có người hỏi tới nó. `.get` là một lối **đọc** — nó
nhìn vào sổ rồi trả lời, nó không viết gì vào sổ cả.

> **Chỗ dễ vấp.** Thứ thứ hai trong ngoặc không phải phần trang trí. Viết
> `chi.get("sửa nhà")` — bỏ trống chỗ dự phòng — thì máy vẫn không dừng, nhưng
> thứ nó đưa ra là `None`, cái tên Realm 0 đặt cho *chưa có gì*. Và `None` thì
> không cộng được vào một con số: dòng tính tổng ở cuối bản báo cáo sẽ nổ
> `TypeError`, cách xa dòng hỏi cả chục dòng, nên rất khó lần ra. Đã hỏi bằng
> `.get` thì viết luôn con số dự phòng ngay tại chỗ hỏi.
::::

::::predict{#doan-ba-dong commitOnce}
Byte trộn cả hai lối tra vào cùng một đoạn: hai dòng đầu hỏi bằng `.get`, dòng
cuối hỏi bằng ngoặc vuông.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

print(chi.get("ăn sáng", 0))
print(chi.get("sửa nhà", 0))
print(chi["sửa nhà"])
```

:::opt{correct}
In `85000`, rồi in `0`, rồi dừng lại với `KeyError: 'sửa nhà'`
:::

:::opt
In `85000`, rồi `0`, rồi `0` lần nữa
::why
Gần đúng ở chỗ bạn theo dõi hai dòng đầu chính xác từng con số. Chỗ lệch nằm
ở việc `.get` đã dạy được cho ai.

`.get` không sửa cuốn sổ và cũng không sửa cách cặp ngoặc vuông làm việc. Nó
chỉ là một lối hỏi khác, và cái dự phòng `0` chỉ có hiệu lực trong đúng lần
hỏi có viết nó ra. Dòng cuối dùng lối hỏi cũ, không mang theo dự phòng nào,
nên nó cư xử đúng như bài trước: khoá không có thì dừng chương trình.
::
:::

:::opt
In `85000`, rồi `0`, rồi `None`
::why
Gần đúng ở chỗ bạn nhớ rằng có một lối tra cho ra `None` khi khoá vắng mặt —
và điều đó có thật: `chi.get("sửa nhà")` viết thiếu chỗ dự phòng sẽ cho ra
đúng `None`.

Chỗ lệch là dòng cuối không phải lối tra ấy. Nó là `chi["sửa nhà"]`, cặp ngoặc
vuông trần. Cặp ngoặc vuông không có nhánh dự phòng nào để rơi vào, kể cả một
nhánh `None`, nên nó dừng chương trình.
::
:::

:::opt
Dừng ngay ở dòng thứ hai, vì khoá `"sửa nhà"` không có trong sổ
::why
Gần đúng ở chỗ bạn nhìn thấy đúng vấn đề: khoá `"sửa nhà"` không nằm trong
cuốn sổ, và ở bài trước một khoá vắng mặt đúng là đã làm chương trình dừng.

Chỗ lệch nằm ở lối hỏi của dòng thứ hai. Nó có mang theo một câu trả lời dự
phòng, và đó chính là lý do `.get` tồn tại: khoá vắng mặt là tình huống nó
được chuẩn bị sẵn để xử lý, chứ không phải tình huống làm nó bó tay. Dòng thứ
hai in ra `0` rồi đi tiếp bình thường.
::
:::
::::

::::code{#ba-dong-bao-cao}
Bản báo cáo phải in đủ ba dòng cố định, tháng nào cũng vậy, kể cả tháng không
phát sinh nhóm nào đó. Dòng cuối cộng cả ba con số lại.

Dòng `an_sang` đã viết sẵn cho bạn. Hai chỗ trống còn lại dùng **đúng lối hỏi
của dòng ấy** — cả hai đều phải hỏi cuốn sổ, không được chép sẵn con số vào.

```python title=starter
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

an_sang = chi.get("ăn sáng", 0)
do_xang = ___
sua_nha = ___

print(f"Ăn sáng: {an_sang} đồng")
print(f"Đổ xăng: {do_xang} đồng")
print(f"Sửa nhà: {sua_nha} đồng")
print(f"Tổng ba dòng: {an_sang + do_xang + sua_nha} đồng")
```

```python title=solution
chi = {
    "ăn sáng": 85000,
    "sửa xe": 500000,
    "biếu bà": 300000,
    "đổ xăng": 120000,
}

an_sang = chi.get("ăn sáng", 0)
do_xang = chi.get("đổ xăng", 0)
sua_nha = chi.get("sửa nhà", 0)

print(f"Ăn sáng: {an_sang} đồng")
print(f"Đổ xăng: {do_xang} đồng")
print(f"Sửa nhà: {sua_nha} đồng")
print(f"Tổng ba dòng: {an_sang + do_xang + sua_nha} đồng")
```

```python title=test
# Hai chỗ trống, hai vai khác nhau: một chỗ hỏi khoá CÓ trong sổ, một chỗ hỏi
# khoá KHÔNG có. Câu thứ ba canh phần dễ quên nhất — hỏi xong thì cuốn sổ phải
# còn nguyên như trước khi hỏi.
assert do_xang == 120000, "sổ này có ghi nhóm đổ xăng 120000 đồng, nên chỗ trống ấy phải lấy ra con số đang nằm trong sổ chứ không phải con số dự phòng"
assert sua_nha == 0, "sổ này không có khoá `sửa nhà`, nên phép hỏi phải cho về con số dự phòng 0 thay vì dừng chương trình"
assert chi.get("sửa nhà", -1) == -1, "hỏi bằng .get là chỉ đọc: sau hai dòng trên, khoá `sửa nhà` vẫn phải KHÔNG có mặt trong sổ — hỏi lại nó với dự phòng -1 mà nhận về -1 nghĩa là sổ chưa hề mọc thêm khoá nào"
```

:::hints
- kind: attention
  body: Dòng `an_sang` ngay phía trên hai chỗ trống là bản mẫu đầy đủ: tên cuốn sổ, dấu chấm, tên phương thức, rồi hai thứ trong ngoặc. Hai chỗ trống chỉ khác nó ở đúng cái khoá.
- kind: strategy
  body: Cả hai dòng đều hỏi cùng một cuốn sổ và đều mang theo con số dự phòng 0. Khoá `đổ xăng` có trong sổ nên con số dự phòng sẽ không được dùng tới; khoá `sửa nhà` không có nên nó sẽ được dùng. Cùng một lối viết lo được cả hai trường hợp, đó là lý do bạn không phải hỏi trước xem khoá nào có.
- kind: one-line
  body: 'Hai dòng lần lượt là `chi.get("đổ xăng", 0)` và `chi.get("sửa nhà", 0)`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Ăn sáng: 85000 đồng\nĐổ xăng: 120000 đồng\nSửa nhà: 0 đồng\nTổng ba dòng: 205000 đồng\s*$
- tier: output
  expect: Tổng ba dòng: 205000 đồng
- tier: static
  onFail: hai chỗ trống phải HỎI cuốn sổ bằng `.get`, không được chép sẵn con số vào
  requireAst:
  # `min: 3` vì khung đã có sẵn MỘT lời gọi `.get` (dòng `an_sang`). Hai chỗ
  # trống điền đúng là lời gọi thứ hai và thứ ba. Chép thẳng con số `120000`
  # vào chỗ trống thì số lời gọi đứng nguyên ở 1 và luật này vỡ.
  - kind: uses-call, target: get, min: 3
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba dòng đủ mặt, sổ vẫn còn bốn khoá. Mình chỉ đọc, mình không viết gì vào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai bài vừa rồi bạn chỉ mới **đọc** cuốn sổ tra cứu: đọc bằng ngoặc vuông thì
khoá lạ làm chương trình dừng, đọc bằng `.get` thì khoá lạ cho ra con số dự
phòng. Cả hai lối đều để cuốn sổ y nguyên như lúc chưa ai hỏi.

Mà sổ tra cứu thì không phải chỉ để đọc. Cuốn sổ chi tiêu không phải cuốn duy
nhất bạn đang giữ: bảng giá của quán phở — thứ mà cuối mạch Hàm còn là hai dãy
`ten_mon` và `gia_mon` đi song song, và làm bạn khốn khổ vì chúng lệch nhau —
bây giờ viết được thành một cuốn sổ tra cứu, tra tên món ra giá món:

```python title=readonly
gia = {
    "Phở bò": 45000,
    "Lẩu gà": 60000,
    "Trà đá": 5000,
}
```

Quán tăng giá phở. Sửa **một** khoá trong sổ tra cứu thì viết gì — chẳng lẽ
dựng lại cả sổ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
