---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.gap-doi-du-lieu-buoc-tang-the-nao
title: "Gấp đôi dữ liệu, bước tăng thế nào"
summary: "Chạy cùng một đoạn mã trên danh sách 10, 20, 40, 80 phần tử, ghi lại số bước đếm được ở mỗi cỡ bằng đúng bộ đếm bài trước. Có đoạn bước tăng GẤP ĐÔI đúng như dữ liệu, có đoạn tăng GẤP BỐN — dữ liệu thật, đo thật, chưa có tên gọi."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.growth-observed]
requires: [alg.count-steps-loop]
concepts: [alg.growth-observed]
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
Đổi theo đúng tỉ lệ, hay đổi theo một cách khác hẳn — câu trả lời không
đoán được bằng mắt. Đo mới biết.
::::

::::explain{#thi-nghiem-gap-doi}
Bài trước dựng xong công cụ: một biến đếm, tăng đúng lúc chạm việc
chính. Giờ dùng công cụ đó làm một thí nghiệm nhỏ nhưng nói được nhiều
điều — chạy CÙNG một đoạn mã trên bốn cỡ dữ liệu khác nhau: 10, 20, 40,
80 phần tử. Mỗi cỡ sau gấp đúng đôi cỡ trước.

Xét đoạn mã đơn giản nhất có thể — một vòng `for` duy nhất, không lồng
gì cả, đi qua từng phần tử đúng một lần:

```python
def dem_buoc_mot_vong(n):
    mang = list(range(n))
    so_buoc = 0
    for x in mang:
        so_buoc += 1
    return so_buoc

for co in [10, 20, 40, 80]:
    print(co, "->", dem_buoc_mot_vong(co))
```

```text
10 -> 10
20 -> 20
40 -> 40
80 -> 80
```

Không có gì bất ngờ: một vòng lặp không lồng thì số bước LUÔN đúng
bằng cỡ dữ liệu. Cỡ dữ liệu gấp đôi (10 → 20), số bước cũng gấp đúng
đôi (10 → 20). Cỡ gấp đôi tiếp (20 → 40), số bước gấp đôi tiếp
(20 → 40). Đúng tỉ lệ, mọi lần.
::::

::::example{#khi-vong-lap-long-nhau}
Giờ xét một đoạn mã khác — cũng đếm bước, nhưng có MỘT vòng lặp LỒNG
bên trong một vòng lặp khác. Với mỗi phần tử, nó lại đi qua TOÀN BỘ
danh sách một lần nữa — ví dụ đếm xem có bao nhiêu cặp chỉ số `(i, j)`
mà `mang[i] + mang[j]` bằng một số mục tiêu cho trước:

```python title=readonly
def dem_buoc_hai_vong(n, muc_tieu):
    mang = list(range(n))
    so_buoc = 0
    for i in range(n):
        for j in range(n):
            so_buoc += 1
            if mang[i] + mang[j] == muc_tieu:
                pass
    return so_buoc

for co in [10, 20, 40, 80]:
    print(co, "->", dem_buoc_hai_vong(co, 5))
```

```text title=readonly
10 -> 100
20 -> 400
40 -> 1600
80 -> 6400
```

Lần này khác hẳn. Cỡ dữ liệu gấp đôi (10 → 20), số bước KHÔNG gấp đôi
— nó nhảy từ 100 lên 400, gấp **bốn** lần. Gấp đôi tiếp (20 → 40), số
bước lại nhảy từ 400 lên 1600 — gấp bốn lần nữa. Kiểu tăng này lặp lại
đều đặn ở mọi cặp cỡ liền nhau: mỗi lần dữ liệu gấp đôi, số bước gấp
BỐN, không phải gấp đôi.

Lý do nằm ngay trong hình dạng đoạn mã: có HAI vòng lặp lồng nhau, mỗi
vòng đều chạy hết `n` lượt. Gấp đôi `n` thì cả vòng ngoài LẪN vòng
trong đều gấp đôi số lượt — mà hai cái gấp đôi NHÂN với nhau thì thành
gấp bốn.
::::

::::predict{#doan-mot-hinh-thu-ba commitOnce}
Một đoạn mã thứ ba — không lồng vòng lặp nào, chỉ nhìn đúng MỘT phần
tử đầu tiên của danh sách, bất kể danh sách dài bao nhiêu:

```python
def dem_buoc_chi_nhin_dau(n):
    mang = list(range(n))
    so_buoc = 0
    so_buoc += 1          # chỉ chạm mang[0], đúng một lần
    return so_buoc
```

Đã đo được: `dem_buoc_chi_nhin_dau(10)` cho `so_buoc == 1`.

**Trước khi đọc tiếp**, bạn đoán `dem_buoc_chi_nhin_dau(80)` — cỡ dữ
liệu gấp tám lần 10 — cho `so_buoc` bằng bao nhiêu?

:::opt{correct}
1 — vẫn đúng 1, không đổi, vì đoạn mã không hề lặp qua danh sách; nó
chỉ chạm đúng một phần tử bất kể danh sách dài bao nhiêu
:::

:::opt
80 — vì cỡ dữ liệu là 80, và số bước phải khớp với cỡ dữ liệu như hai
đoạn mã trước đã cho thấy
::why
Gần đúng ở việc bạn áp dụng đúng bài học vừa thấy: hai đoạn mã trước
đó số bước đúng là ĐỔI theo cỡ dữ liệu, có lý do để đoán mọi đoạn mã
đều vậy.

Chỗ lệch: cả hai đoạn mã trước đều có một vòng lặp CHẠY QUA danh sách
— đó là lý do số bước phụ thuộc `n`. Đoạn mã này không hề có vòng lặp
nào chạm vào danh sách; dòng `so_buoc += 1` chỉ chạy đúng một lần, viết
sẵn trong mã, không phụ thuộc `n` chút nào — `n` chỉ dùng để tạo ra
`mang`, còn `mang` sau đó không hề được duyệt qua.
::
:::

:::opt
8 — vì 80 gấp 8 lần 10, nên số bước cũng phải gấp 8 lần con số ban đầu
::why
Gần đúng ở phép tính 80 ÷ 10 = 8 — phép chia đó không sai, và đúng kiểu
suy luận "tỉ lệ" đã dùng ở hai đoạn mã trước.

Chỗ lệch: phép "gấp bao nhiêu lần" chỉ có ý nghĩa khi số bước THẬT SỰ
phụ thuộc vào `n`. Ở đây số bước luôn là 1, không phụ thuộc `n` — nhân
1 với 8 lần vẫn ra 1, không ra 8.
::
:::

:::opt
Không đoán được nếu chưa biết nội dung cụ thể của danh sách 80 phần tử
::why
Gần đúng ở sự thận trọng — với nhiều đoạn mã khác (ví dụ dò tìm một
giá trị cụ thể), nội dung danh sách đúng là ảnh hưởng tới số bước.

Chỗ lệch: đoạn mã này không hề NHÌN vào nội dung của `mang` — dòng
`so_buoc += 1` không có điều kiện nào, không so sánh gì với phần tử
nào. Nó chạy đúng một lần bất kể danh sách chứa số gì, dài bao nhiêu.
::
:::
::::

::::code{#do-bon-co}
Đo đoạn mã hai vòng lồng nhau (`dem_buoc_hai_vong`, đã cho sẵn — readonly)
trên đúng bốn cỡ dữ liệu `[10, 20, 40, 80]`, và gom kết quả vào một
danh sách `ket_qua`.

```python title=starter
def dem_buoc_hai_vong(n, muc_tieu):
    mang = list(range(n))
    so_buoc = 0
    for i in range(n):
        for j in range(n):
            so_buoc += 1
            if mang[i] + mang[j] == muc_tieu:
                pass
    return so_buoc

cac_co = [10, 20, 40, 80]
ket_qua = []
for n in cac_co:
    so_buoc_do = ___                       # đo dem_buoc_hai_vong tại đúng cỡ n này, muc_tieu=5
    ket_qua.append(___)                    # thêm số bước vừa đo vào ket_qua

print(ket_qua)
```

```python title=solution
def dem_buoc_hai_vong(n, muc_tieu):
    mang = list(range(n))
    so_buoc = 0
    for i in range(n):
        for j in range(n):
            so_buoc += 1
            if mang[i] + mang[j] == muc_tieu:
                pass
    return so_buoc

cac_co = [10, 20, 40, 80]
ket_qua = []
for n in cac_co:
    so_buoc_do = dem_buoc_hai_vong(n, 5)
    ket_qua.append(so_buoc_do)

print(ket_qua)
```

```python title=test
assert ket_qua == [100, 400, 1600, 6400], f"phải đo đúng [100, 400, 1600, 6400] trên bốn cỡ 10/20/40/80 — đang ra {ket_qua}"
assert ket_qua[1] / ket_qua[0] == 4, "cỡ dữ liệu gấp đôi (10 -> 20) phải cho số bước gấp BỐN, đúng đặc trưng của hai vòng lặp lồng nhau"
assert ket_qua[3] / ket_qua[2] == 4, "cỡ dữ liệu gấp đôi (40 -> 80) cũng phải cho số bước gấp BỐN — tỉ lệ này phải giữ nguyên ở mọi cặp cỡ liền nhau"
```

:::hints
- kind: attention
  body: Chỗ trống đầu phải THẬT SỰ gọi dem_buoc_hai_vong với đúng biến n của lượt lặp hiện tại (không phải một con số cố định) — vì bốn lượt lặp phải đo bốn cỡ KHÁC nhau. Chỗ trống sau chỉ đơn giản đưa kết quả vừa đo vào danh sách ket_qua.
- kind: strategy
  body: 'Chỗ trống đầu: dem_buoc_hai_vong(n, 5) — gọi hàm với n của vòng lặp hiện tại, muc_tieu=5 (giá trị nào cũng được, không ảnh hưởng số bước vì so_buoc đếm MỌI lượt của hai vòng lồng nhau, không phụ thuộc điều kiện if). Chỗ trống sau: so_buoc_do — biến vừa gán ở chỗ trống đầu.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `dem_buoc_hai_vong(n, 5)` và `so_buoc_do`.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: static
  onFail: chỗ trống đầu phải THẬT SỰ gọi dem_buoc_hai_vong (không phải một câu không làm gì như True, 1, 0) — nếu không gọi hàm này, không có số bước nào được đo cả
  requireAst:
  - kind: uses-call, target: dem_buoc_hai_vong, min: 1
  # min: 1 — đếm thật trên solution: dem_buoc_hai_vong được GỌI đúng
  # một lần TRONG MÃ NGUỒN (ở chỗ trống đầu; vòng lặp bên ngoài chạy nó
  # bốn lần lúc THỰC THI, nhưng AST chỉ đếm số lần lời gọi xuất hiện
  # trong VĂN BẢN mã, đúng một lần). Điền True/1/0 vào CẢ HAI chỗ trống
  # xoá hẳn lời gọi này khỏi mã nguồn — count tụt xuống 0, luật chặn
  # được. ĐÃ THỬ THẬT cả ba cách True/1/0 cho cả hai chỗ trống cùng lúc:
  # cả ba dừng NGAY (đây là vòng for qua danh sách 4 phần tử cố định,
  # không phải while/đệ quy, không có rủi ro lặp vô hạn) và cả ba cho
  # ket_qua == [True, True, True, True] hoặc [1,1,1,1] hoặc [0,0,0,0],
  # sai hẳn so với [100, 400, 1600, 6400], bị tests bắt độc lập với
  # luật static này. Đã thử thêm một lời giải ĐÚNG khác — viết
  # so_buoc_do rồi append riêng một dòng thay vì gộp — vẫn gọi hàm đúng
  # một lần trong mã nguồn, luật không đánh trượt nhầm.
- tier: tests
  timeoutMs: 5000
- tier: output
  match: regex
  expect: "^\\[100, 400, 1600, 6400\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một vòng lặp: gấp đôi dữ liệu, gấp đôi bước. Hai vòng lặp lồng nhau:
gấp đôi dữ liệu, gấp BỐN bước. Con số thật, đo được, không cần đoán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa đo được BA kiểu tăng khác nhau bằng chính con số, không phải
bằng cảm giác: một đoạn mã mà số bước KHÔNG đổi dù dữ liệu gấp đôi
(nhìn đúng phần tử đầu); một đoạn mã mà số bước tăng ĐÚNG GẤP ĐÔI khi
dữ liệu gấp đôi (một vòng lặp không lồng); và một đoạn mã mà số bước
tăng GẤP BỐN khi dữ liệu gấp đôi (hai vòng lặp lồng nhau).

Ba kiểu tăng này lặp lại y hệt ở rất nhiều đoạn mã khác nhau — đủ phổ
biến để đáng có một cái TÊN riêng, thay vì mỗi lần gặp lại phải tả dài
dòng như vừa làm ở trên. Ba cái tên đó là gì?
::::

::::checkpoint{mastery=0.8}
::::
