---
id: nen-tang.re-nhanh-va-lap.bo-qua-mot-luot-thoi
title: Bỏ qua đúng một lượt
summary: Một lệnh nói thẳng "lượt này bỏ, đi tiếp" — phần còn lại của thân vòng không chạy, nhưng vòng lặp thì vẫn sống.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ctrl.continue]
requires: [ctrl.break, ctrl.if-nested, ctrl.for-each, ctrl.if]
concepts: [ctrl.lap, ctrl.re-nhanh]
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
Ngày này quán nghỉ, không tiêu đồng nào. Mình bỏ qua, rồi đi tiếp ngày sau.
::::

::::explain{#mot-tang-thut-le-nua}
Bài trước để lại một câu hỏi. Sổ chi tiêu có những ngày quán nghỉ — bà chủ vẫn
ghi sổ, và ghi số `0`, nghĩa là *"ngày này đã tính rồi, tiêu đúng 0 đồng"*.

Đó là số `0` đúng nghĩa mà T1.1 đã chốt: **đã biết, và bằng không**. Một ngày
chưa ai chạm bút vào thì mới là `None`, và cuốn sổ này không có ngày nào như
thế — bà chủ ghi đủ bảy ngày.

Việc cần làm: điểm lại sổ, ngày nào có tiêu thì in ra, ngày nghỉ thì bỏ hẳn —
không in, không đếm — nhưng vẫn phải đi tiếp những ngày sau.

`break` không dùng được ở đây. `break` gấp sổ lại và đi hẳn; gặp ngày nghỉ đầu
tiên là bao nhiêu ngày sau đó mất trắng.

Cách bạn làm được ngay hôm nay là bọc **toàn bộ** phần còn lại của thân vòng vào
một câu `if`. Nó chạy đúng. Cái giá nằm ở chỗ khác.
::::

::::example{#boc-ca-than-vao-if}
Năm ngày đầu tháng, hai ngày trong đó quán nghỉ:

```python title=readonly
so_chi = [120000, 0, 95000, 0, 260000]
ngay = 0

for tien in so_chi:
    ngay = ngay + 1
    if tien != 0:
        print(f"Ngày {ngay}: {tien} đồng")
        if tien > 200000:
            print("   (ngày tiêu mạnh)")
```

Chạy lên, màn hình đúng ý:

```text
Ngày 1: 120000 đồng
Ngày 3: 95000 đồng
Ngày 5: 260000 đồng
   (ngày tiêu mạnh)
```

Đúng thì đúng. Nhưng đếm số tầng thụt lề mà dòng `print("   (ngày tiêu mạnh)")`
đang nằm: tầng một là thân `for`, tầng hai là thân `if tien != 0`, tầng ba là
thân `if tien > 200000`. Ba tầng, và chỉ **một** trong ba tầng ấy nói về việc
thật sự cần làm.

Bài "Lối rẽ nằm trong lối rẽ" đã cho bạn thấy đọc một dòng nằm sâu ba tầng mệt
tới đâu: muốn biết nó chạy trong trường hợp nào, mắt phải leo ngược lên từng
tầng một. Ở đây tầng giữa chẳng nói được gì đáng leo — nó chỉ nói *"ngày này có
ghi"*, một chuyện bạn đã biết từ dòng đầu.

Và sổ thật còn dài. Mỗi việc bạn thêm vào thân vòng — cộng dồn, đếm, in — đều
phải nằm sâu thêm một tầng chỉ vì cái vỏ ấy.
::::

::::example{#noi-thang-bo-luot-nay}
Python có một từ để nói thẳng *"lượt này bỏ, đi tiếp"*: **`continue`**. Tiếng
Anh nghĩa là *đi tiếp*, và nó đi tiếp đúng nghĩa đen.

Cùng việc đó, viết lại:

```python title=readonly
so_chi = [120000, 0, 95000, 0, 260000]
ngay = 0

for tien in so_chi:
    ngay = ngay + 1
    if tien == 0:
        continue
    print(f"Ngày {ngay}: {tien} đồng")
    if tien > 200000:
        print("   (ngày tiêu mạnh)")
```

Màn hình ra **y hệt** bốn dòng lúc nãy. Nhưng dòng `print("   (ngày tiêu
mạnh)")` đã lùi từ tầng ba về tầng hai, và dòng `print(f"Ngày {ngay}...")` lùi
từ tầng hai về tầng một — ngang hàng với `ngay = ngay + 1`, tức là ngang hàng
với **việc chính của mỗi lượt**.

Đọc thân vòng theo thứ tự từ trên xuống, nó nói đúng như bà chủ nói: *"Ngày
nghỉ thì bỏ. Còn lại thì ghi ra."*

Máy chạy dòng `continue` là nó **nhảy ngay lên đầu lượt kế tiếp**. Hai dòng
`print` nằm dưới không được chạy trong lượt đó. Nhưng vòng lặp vẫn sống nguyên:
`tien` lấy giá trị tiếp theo trong danh sách, và mọi thứ chạy lại từ đầu thân.

Để ý dòng `ngay = ngay + 1` được đặt **trên** `continue`. Đó không phải ngẫu
nhiên: số ngày phải tăng cả ở những lượt bị bỏ, nếu không thì ngày 3 sẽ bị gọi
nhầm thành ngày 2. Thứ gì cần chạy ở **mọi** lượt phải nằm trước `continue`.

Cùng một chỗ đứng — giữa thân vòng — nhưng hai lệnh làm hai việc khác hẳn:

- `break` cắt **cả vòng lặp**. Xong là ra ngoài, không lượt nào nữa.
- `continue` cắt **một lượt**. Xong là lên lượt kế tiếp, vòng vẫn chạy.
::::

::::predict{#doan-continue commitOnce}
Quán mở cửa mấy tiếng mỗi ngày; ngày nghỉ ghi `0`. Byte cộng số giờ mở cửa.
**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python title=readonly
gio_mo_cua = [7, 0, 8, 0, 7]
tong = 0

for gio in gio_mo_cua:
    if gio == 0:
        continue
    tong = tong + gio
    print(f"Cộng {gio}")

print(f"Tổng {tong}")
```

:::opt{correct}
Cộng 7, Cộng 8, Cộng 7, rồi Tổng 22
:::

:::opt
Cộng 7, rồi Tổng 7
::why
Gần đúng ở chỗ bạn nhận ra `continue` cũng là một lệnh cắt ngang đặt giữa thân
vòng, y như `break` — và nó cắt thật, ngay tại dòng đó.

Chỗ lệch nằm ở **cái bị cắt**. `break` cắt cả vòng lặp: gặp số `0` thứ nhất là
ra hẳn, ba số sau không ai ngó tới, và `tong` đứng lại ở 7 — đúng như bạn đoán.
`continue` chỉ cắt **lượt đang chạy**. Cắt xong nó lên lượt kế, nên `8` và `7`
vẫn được cộng.
::
:::

:::opt
Cộng 7, Cộng 0, Cộng 8, Cộng 0, Cộng 7, rồi Tổng 22
::why
Gần đúng ở chỗ bạn tính ra `tong` bằng 22 — và con số ấy chính xác. Cộng thêm
`0` không đổi được tổng, nên dù có in "Cộng 0" hay không thì dòng cuối vẫn là
Tổng 22.

Chỗ lệch nằm ở hai dòng "Cộng 0". Ở lượt có `gio` bằng `0`, máy chạy tới
`continue` rồi nhảy thẳng lên lượt sau — **cả** dòng `tong = tong + gio` **lẫn**
dòng `print` nằm dưới đều bị bỏ. Máy không chạy chúng rồi mới bỏ kết quả; nó
không chạy chúng lần nào.
::
:::

:::opt
Cộng 7 rồi máy đứng mãi ở số `0` thứ hai, không bao giờ dừng
::why
Gần đúng ở chỗ bạn đọc `continue` là *"quay lại đầu vòng"* — và đúng là nó quay
lại đầu vòng thật, đó là cách hiểu sát nghĩa nhất.

Chỗ lệch nằm ở chữ "đầu vòng" nghĩa là đầu lượt **nào**. Nó là đầu lượt **kế
tiếp**, không phải chạy lại lượt vừa rồi. Với `for`, mỗi lần lên đầu vòng là
`gio` nhận phần tử sau trong danh sách, nên `0` không bao giờ được xét hai lần.
::
:::
::::

::::explain{#continue-trong-while}
Trong `for`, danh sách hết là vòng hết, nên `continue` không gây rắc rối gì.

Trong `while` thì có một chỗ phải để ý, và nó nối thẳng vào bài "Vòng lặp không
chịu dừng": ở đó bạn học rằng mỗi `while` cần một **bước tiến** trong thân đẩy
điều kiện dần tới sai. `continue` bỏ phần còn lại của lượt — nên nếu bước tiến
ấy nằm **dưới** `continue`, nó cũng bị bỏ luôn:

```python
i = 0
while i < 5:
    if i == 2:
        continue
    print(i)
    i = i + 1
```

Tới lượt `i` bằng 2, máy nhảy lên đầu vòng mà chưa kịp chạy `i = i + 1`. Lượt
sau `i` vẫn bằng 2. Lượt sau nữa vẫn bằng 2. Đây đúng là vòng lặp vô hạn, và
bạn phải bấm Dừng.

Cách chữa là cách bạn vừa thấy ở ví dụ sổ chi tiêu: đẩy bước tiến lên **trên**
`continue`. Quy tắc chung chỉ có một câu — thứ gì cần chạy ở mọi lượt thì phải
đứng trước `continue`.
::::

::::code{#diem-lai-so-tuan}
Byte điểm lại sổ chi tiêu của một tuần. Ngày quán nghỉ thì trong danh sách là
số `0`.

Hàm phải làm ba việc cho mỗi ngày **có tiêu**: đếm nó vào tổng số ngày có tiêu,
in con số ra, và nếu vượt 200 nghìn thì in thêm một dòng ghi chú. Ngày nghỉ thì
không làm việc nào trong ba việc đó — nhưng số thứ tự ngày vẫn phải tăng.

Đoạn dưới thiếu hai chỗ. Chỗ thứ nhất là câu hỏi *"ngày này có phải ngày nghỉ
không?"*; chỗ thứ hai là **lệnh** bỏ lượt.

Byte gọi hàm với hai quyển sổ khác nhau, vì một vòng lặp chỉ được coi là viết
đúng khi nó xử đúng mọi quyển sổ, chứ không riêng tuần này.

```python title=starter
def diem_lai_so(so_chi):
    ngay = 0
    so_ngay_co_ghi = 0
    for tien in so_chi:
        ngay = ngay + 1
        if ___:
            ___
        so_ngay_co_ghi = so_ngay_co_ghi + 1
        print(f"Ngày {ngay}: {tien} đồng")
        if tien > 200000:
            print("   (ngày tiêu mạnh)")
    return so_ngay_co_ghi

tuan_nay = [120000, 0, 95000, 0, 260000, 0, 80000]
tuan_truoc = [0, 210000, 45000]

print(f"Tuần này có {diem_lai_so(tuan_nay)} ngày ghi sổ")
print(f"Tuần trước có {diem_lai_so(tuan_truoc)} ngày ghi sổ")
```

```python title=solution
def diem_lai_so(so_chi):
    ngay = 0
    so_ngay_co_ghi = 0
    for tien in so_chi:
        ngay = ngay + 1
        if tien == 0:
            continue
        so_ngay_co_ghi = so_ngay_co_ghi + 1
        print(f"Ngày {ngay}: {tien} đồng")
        if tien > 200000:
            print("   (ngày tiêu mạnh)")
    return so_ngay_co_ghi

tuan_nay = [120000, 0, 95000, 0, 260000, 0, 80000]
tuan_truoc = [0, 210000, 45000]

print(f"Tuần này có {diem_lai_so(tuan_nay)} ngày ghi sổ")
print(f"Tuần trước có {diem_lai_so(tuan_truoc)} ngày ghi sổ")
```

```python title=test
# Chấm trên BA quyển sổ, không phải một.
#
# Chấm bằng một quyển thì không phân biệt được đúng với gõ bừa: mọi câu điền
# hụt đều cho ra một con số nào đó, và một con số nào đó thì đôi khi trùng
# đáp án. Ba quyển đây được chọn để mỗi cách điền hụt đều lộ:
#   `if True:` ở chỗ một  → câu lệnh sau nó không bỏ được lượt nào, cả ba sổ
#                           đếm cả ngày nghỉ;
#   `if tien == 0: pass`  → y hệt trên, sổ tuần này ra 7 thay vì 4;
#   `if tien != 0:`       → bỏ đúng những ngày CÓ ghi, sổ toàn ngày nghỉ ra 3.
assert diem_lai_so([120000, 0, 95000, 0, 260000, 0, 80000]) == 4, "sổ tuần này bảy ngày mà quán nghỉ ba ngày, nên chỉ còn bốn ngày có con số để điểm"
assert diem_lai_so([0, 210000, 45000]) == 2, "sổ tuần trước nghỉ đúng ngày đầu, hai ngày sau vẫn có ghi — bỏ một ngày không được bỏ nốt phần còn lại"
assert diem_lai_so([0, 0, 0]) == 0, "quyển sổ mà ngày nào cũng nghỉ thì không có ngày nào đáng điểm cả"
```

:::hints
- kind: attention
  body: Hai chỗ trống nằm trên hai dòng liền nhau và không điền giống nhau. Chỗ thứ nhất là một câu hỏi đúng-sai, nằm giữa `if` và dấu hai chấm. Chỗ thứ hai đứng một mình cả dòng, thụt vào trong `if` — chỗ đó cần một lệnh, không phải một câu hỏi.
- kind: strategy
  body: Ngày nghỉ là ngày mà `tien` bằng `0`, nên câu hỏi ở chỗ thứ nhất so `tien` với `0`. Chỗ thứ hai cần lệnh nói "lượt này bỏ, đi tiếp" — không phải lệnh cắt cả vòng lặp, vì những ngày sau vẫn phải được điểm.
- kind: one-line
  body: "Viết `tien == 0` vào chỗ trống sau `if`, và viết `continue` vào dòng trống bên dưới."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Ngày 1: 120000 đồng\nNgày 3: 95000 đồng\nNgày 5: 260000 đồng\n   \(ngày tiêu mạnh\)\nNgày 7: 80000 đồng\nTuần này có 4 ngày ghi sổ\nNgày 2: 210000 đồng\n   \(ngày tiêu mạnh\)\nNgày 3: 45000 đồng\nTuần trước có 2 ngày ghi sổ\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ngày nghỉ bỏ qua, ngày có tiêu vẫn ghi. Sổ nào cũng đọc được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Sổ cả tháng có 30 ngày. Bà chủ chỉ hỏi tuần thứ hai — ngày 8 tới ngày 14. Với
`continue`, bạn viết được ngay: mỗi lượt, ngày nào nằm ngoài khoảng đó thì bỏ.

Nhưng đếm lại xem máy đã đi mấy lượt. **Ba mươi.** Nó vẫn ghé đủ 30 ngày, vào
tới dòng `if`, rồi mới quay ra tay không ở 23 ngày trong số đó. Bảy dòng cần in
phải trả giá bằng 23 lượt chạy suông.

Trong `for ngay in range(30):`, con số `30` là thứ duy nhất bạn nói cho máy về
chuyện đi tới đâu — và nó luôn khởi hành từ 0.

Bảo nó khởi hành thẳng từ ngày 8 có được không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
