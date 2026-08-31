---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.dem-buoc-cho-mot-vong-lap
title: "Đếm bước cho một vòng lặp, không chỉ đệ quy"
summary: "Bộ đếm bài trước dùng cho một hàm gọi chính nó không có gì đặc biệt dành riêng cho đệ quy — cùng một biến, cùng một chỗ tăng lên 1, đem đặt vào một vòng for hay while bình thường vẫn đếm đúng. Đếm số lần chạm việc chính, không đoán bằng cảm giác."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [alg.count-steps-loop]
requires: [alg.count-recursive-calls]
concepts: [alg.count-steps-loop]
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
Dùng được. Bộ đếm không quan tâm chỗ nó đứng là một lượt gọi hàm hay một
lượt lặp.
::::

::::explain{#bien-tang-khong-quan-tam-no-dung-o-dau}
Bài trước bộ đếm nằm bên trong một hàm gọi lại chính nó — mỗi lần
`fib` được gọi, `so_lan_goi += 1`. Nhưng nhìn kỹ lại thì kỹ thuật đó
chưa từng hỏi "đây có phải đệ quy không". Nó chỉ hỏi đúng một câu: mỗi
lần chạm vào **việc chính**, có tăng biến đếm lên 1 hay không.

"Việc chính" của một vòng `for` là mỗi lượt lặp — mỗi lần lấy ra một
phần tử để xử lý. Đặt bộ đếm ngay đầu thân vòng lặp là đếm đúng:

```python
so_buoc = 0
for x in [4, 7, 2, 9, 1]:
    so_buoc += 1
    # ... xử lý x ...
print(so_buoc)     # 5 — đúng bằng số phần tử, không thiếu không thừa
```

"Việc chính" của một vòng `while` thường là mỗi lần **so sánh**. Ví dụ
so từng cặp số liền kề trong một danh sách — so `mang[0]` với
`mang[1]`, rồi `mang[1]` với `mang[2]`, cứ thế tới hết:

```python
mang = [4, 7, 2, 9, 1]
so_buoc = 0
i = 0
while i < len(mang) - 1:
    so_buoc += 1                  # đây là việc chính: MỘT lần so sánh
    if mang[i] > mang[i + 1]:
        pass
    i += 1
print(so_buoc)     # 4 — đúng bằng số CẶP liền kề, ít hơn số phần tử một
```

Điều kiện `i < len(mang) - 1` dừng vòng lặp sớm hơn một bước so với
`i < len(mang)` — vì cặp cuối cùng cần xét là `(mang[3], mang[4])`, và
`mang[i + 1]` sẽ vượt ra ngoài danh sách nếu `i` chạy tới tận chỉ số
cuối. Năm phần tử cho đúng bốn cặp liền kề, không phải năm.

Cùng một kỹ thuật, cùng một chỗ đặt bộ đếm — chỉ khác `for`/`while` là
cú pháp của việc LẶP, không phải cú pháp của việc ĐẾM.
::::

::::example{#dem-cap-sai-thu-tu}
Byte đếm xem trong một danh sách có bao nhiêu **cặp liền kề sai thứ
tự** — cặp mà số đứng trước lại LỚN HƠN số đứng sau, dấu hiệu danh sách
chưa được sắp xếp tăng dần:

```python title=readonly
mang = [5, 3, 8, 1, 9, 2]
so_buoc = 0
con_dem_sai = 0
i = 0
while i < len(mang) - 1:
    so_buoc += 1
    if mang[i] > mang[i + 1]:
        con_dem_sai += 1
    i += 1

print(f"so bước: {so_buoc}, số cặp sai thứ tự: {con_dem_sai}")
```

```text title=readonly
so bước: 5, số cặp sai thứ tự: 3
```

Sáu phần tử cho đúng năm cặp liền kề (`so_buoc == 5`) — đúng công thức
vừa nêu ở trên, số cặp luôn ít hơn số phần tử đúng một. Trong năm cặp
đó, ba cặp sai thứ tự: `(5, 3)`, `(8, 1)`, `(9, 2)` — mỗi cặp này có số
đứng trước lớn hơn số đứng sau.

Hai bộ đếm này đo hai thứ khác nhau: `so_buoc` đếm TỔNG số lần vòng lặp
chạy — bất kể cặp đó đúng hay sai thứ tự; `con_dem_sai` chỉ tăng khi
CHẠM đúng điều kiện `if`. Đặt bộ đếm sai chỗ — ví dụ đặt cả hai bên
trong khối `if` — sẽ đếm thiếu, như phần dự đoán dưới đây sẽ cho thấy.
::::

::::predict{#doan-dat-sai-cho commitOnce}
Byte định đếm SỐ LẦN đã so sánh, nhưng lỡ tay đặt dòng tăng biến vào
NHẦM CHỖ — bên trong khối `if` thay vì ngay đầu thân vòng lặp:

```python
def dem_buoc_loi(mang):
    so_buoc = 0
    i = 0
    while i < len(mang) - 1:
        if mang[i] > mang[i + 1]:
            so_buoc += 1
        i += 1
    return so_buoc

print(dem_buoc_loi([5, 3, 8, 1, 9, 2]))
```

**Trước khi chạy**, bạn đoán dòng in ra là con số nào?

:::opt{correct}
3 — vì `so_buoc += 1` chỉ chạy khi `mang[i] > mang[i + 1]` đúng, nên nó
đếm số cặp SAI thứ tự, không đếm tổng số lần so sánh
:::

:::opt
5 — vì vòng lặp so sánh đúng năm cặp liền kề trong một danh sách sáu
phần tử, và biến `so_buoc` phải tăng ở mỗi lần so sánh
::why
Gần đúng ở chỗ đúng là vòng lặp chạy qua đúng năm cặp liền kề — số đó
không sai, và đúng bằng số lần điều kiện `while` được kiểm tra thành
công.

Chỗ lệch nằm ở CHỖ dòng `so_buoc += 1` được đặt. Nó nằm bên TRONG khối
`if`, không nằm ngay đầu thân vòng lặp như ví dụ trước. Nên biến chỉ
tăng vào ba trong năm lần so sánh đó — đúng ba lần cặp bị lệch thứ tự
— không tăng ở cả năm lần.
::
:::

:::opt
6 — vì danh sách có sáu phần tử, và mỗi phần tử làm vòng lặp chạy thêm
một lượt
::why
Gần đúng ở việc bạn nhớ đúng danh sách có sáu phần tử — con số đó
không sai.

Chỗ lệch: điều kiện dừng là `i < len(mang) - 1`, không phải
`i < len(mang)`, đúng lý do đã nêu ở phần giải thích — cặp cuối cần so
là `(mang[3], mang[4])`, lùi lại một bước để `mang[i + 1]` không vượt
ra ngoài danh sách. Vòng lặp chạy đúng NĂM lượt, không phải sáu, và
trong năm lượt đó biến chỉ tăng ở ba lượt vì lý do đã nêu ở trên.
::
:::

:::opt
2 — vì chỉ có hai cặp liên tiếp cùng lúc sai, còn lại là những cặp lẻ
tẻ không tính
::why
Gần đúng ở việc bạn nghi ngờ không phải MỌI cặp sai đều "đáng tính" —
sự thận trọng đó có lý trong một số bài toán khác.

Chỗ lệch: đoạn mã này không phân biệt cặp sai đứng "liên tiếp" hay
"lẻ tẻ" — mọi lần `mang[i] > mang[i + 1]` đúng đều làm biến tăng, bất
kể trước hay sau nó có một cặp sai khác hay không. Ba cặp `(5, 3)`,
`(8, 1)`, `(9, 2)` đều được đếm, không phải hai.
::
:::
::::

::::code{#dem-hai-thu-cung-luc}
Viết một hàm đếm ĐÚNG cả hai con số cùng lúc trong một lượt duyệt: tổng
số lần so sánh (`so_buoc`), và số cặp liền kề sai thứ tự
(`con_dem_sai`). Đặt bộ đếm đúng chỗ — bài học vừa rồi cho thấy đặt sai
chỗ là đếm thiếu.

```python title=starter
def dem_cap_khong_dung_thu_tu(mang):
    so_buoc = 0
    con_dem_sai = 0
    i = 0
    while i < len(mang) - 1:
        ___                        # tăng so_buoc — MỖI LẦN so sánh, không chỉ khi sai
        if mang[i] > mang[i + 1]:
            ___                    # tăng con_dem_sai — CHỈ khi cặp này sai thứ tự
        i += 1
    return so_buoc, con_dem_sai

mang_kiem_tra = [3, 1, 4, 1, 5, 9, 2, 6]
so_buoc, con_dem_sai = dem_cap_khong_dung_thu_tu(mang_kiem_tra)
print(f"số bước: {so_buoc}, số cặp sai thứ tự: {con_dem_sai}")
```

```python title=solution
def dem_cap_khong_dung_thu_tu(mang):
    so_buoc = 0
    con_dem_sai = 0
    i = 0
    while i < len(mang) - 1:
        so_buoc += 1
        if mang[i] > mang[i + 1]:
            con_dem_sai += 1
        i += 1
    return so_buoc, con_dem_sai

mang_kiem_tra = [3, 1, 4, 1, 5, 9, 2, 6]
so_buoc, con_dem_sai = dem_cap_khong_dung_thu_tu(mang_kiem_tra)
print(f"số bước: {so_buoc}, số cặp sai thứ tự: {con_dem_sai}")
```

```python title=test
assert so_buoc == 7, f"tám phần tử phải cho đúng bảy cặp liền kề — đang ra {so_buoc}"
assert con_dem_sai == 3, f"phải đếm đúng ba cặp sai thứ tự trong [3, 1, 4, 1, 5, 9, 2, 6] — đang ra {con_dem_sai}"
assert con_dem_sai < so_buoc, "số cặp SAI thứ tự phải ít hơn tổng số cặp đã so — không thể mọi cặp đều sai trong danh sách này"
```

:::hints
- kind: attention
  body: Hai chỗ trống tăng hai biến KHÁC nhau, ở hai vị trí khác nhau. Chỗ trống đầu nằm NGAY ĐẦU thân vòng lặp — phải chạy ở MỌI lượt lặp, không phân biệt cặp đó đúng hay sai thứ tự. Chỗ trống sau nằm BÊN TRONG khối if — chỉ chạy khi điều kiện if đúng.
- kind: strategy
  body: 'Chỗ trống đầu: so_buoc += 1 — đo TỔNG số lần vòng lặp chạm vào một cặp. Chỗ trống sau: con_dem_sai += 1 — chỉ đo những cặp mà mang[i] > mang[i + 1] là True. Đặt so_buoc += 1 vào trong if (như ví dụ dự đoán vừa rồi) sẽ đếm thiếu — nó sẽ ra cùng con số với con_dem_sai, mất luôn ý nghĩa "tổng số bước".'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `so_buoc += 1` (ngay đầu thân vòng lặp) và `con_dem_sai += 1` (bên trong khối if).'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống đầu phải THẬT SỰ tăng so_buoc ở MỌI lượt lặp (không chỉ khi if đúng); chỗ trống sau phải THẬT SỰ tăng con_dem_sai — không phải một câu không làm gì như True, 1, 0
  requireAst:
  - kind: gan-ten, target: so_buoc, min: 3
  - kind: gan-ten, target: con_dem_sai, min: 3
  # min: 3 cho mỗi biến — ĐO LẠI THẬT bằng kiemAst(), không đoán. `ast.walk`
  # đi qua CẢ MODULE, không chỉ thân hàm, nên dòng unpack ở cuối bài
  # (`so_buoc, con_dem_sai = dem_cap_khong_dung_thu_tu(...)`) TỰ NÓ đã là
  # một lần gán cho cả hai tên — min: 2 (khởi tạo "= 0" trong khung + dòng
  # unpack này) LUỐN LUÔN đạt, KỂ CẢ KHI CẢ HAI CHỖ TRỐNG ĐỀU ĐIỀN True/1/0,
  # vì hai lần gán ấy không nằm trong chỗ trống chút nào. Đây từng là một
  # lỗ chấm thật (bản trước ghi min: 2 và tin lời giải mẫu, đúng lỗi Luật 2
  # cảnh báo) — phát hiện khi nghiệm thu, đo lại bằng kiemAst() trên chính
  # ba cách điền hụt: solution đếm được 3 (khởi tạo + tăng ở chỗ trống +
  # dòng unpack), hollow True/1/0 chỉ đếm được 2 (thiếu đúng lần tăng ở chỗ
  # trống) — min: 3 phân biệt đúng, đã thử lại cả ba cách hollow lẫn hollow
  # riêng từng chỗ trống, cả năm ca đều `dat: false`. Vòng lặp không treo ở
  # bất kỳ ca nào — điều khiển `i < len(mang) - 1` và `i += 1` nằm ngoài
  # hai chỗ trống. Đã thử thêm một lời giải ĐÚNG khác dùng
  # `so_buoc = so_buoc + 1` / `con_dem_sai = con_dem_sai + 1` thay vì
  # `+=` — vẫn đếm đủ 3 lần gán mỗi biến, luật không đánh trượt nhầm.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^số bước: 7, số cặp sai thứ tự: 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
7 bước, 3 cặp sai — cùng một lượt duyệt, hai con số, đặt đúng chỗ là ra
đủ cả hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Con số 7, con số 3 — hay con số 177 và 1973 ở bài trước — đều là những
con số THẬT, đo được, không đoán. Nhưng đứng riêng lẻ, một con số như
"7 bước" tự nó chưa nói được gì về việc đoạn mã đó NHANH hay CHẬM. Bảy
bước là nhanh hay chậm còn tuỳ: nhanh so với cái gì?

Nếu cho đúng đoạn mã ấy chạy trên một danh sách LỚN HƠN — không phải
tám phần tử, mà 16, hay 80 — số bước đo được sẽ đổi ra sao? Đổi theo
đúng tỉ lệ với cỡ dữ liệu, hay đổi theo một cách khác hẳn?

Bài sau đo thật, trên nhiều cỡ dữ liệu cùng lúc.
::::

::::checkpoint{mastery=0.8}
::::
