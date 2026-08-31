---
id: khoa-hoc-may-tinh.may-chay-the-nao.ngan-xep-goi-ham-khac-ngan-xep-cau-truc
title: "'Ngăn xếp gọi hàm' khác 'ngăn xếp cấu trúc dữ liệu' ở chỗ nào"
summary: "T3.2 dạy ngăn xếp là một list bạn tự .append/.pop. Vùng bộ nhớ giữ các khung gọi hàm CŨNG gọi là ngăn xếp — cùng luật vào-sau-ra-trước, nhưng interpreter tự quản, không phải bạn gõ lệnh đẩy/lấy. Trùng tên, không trùng vai trò."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.call-stack-vs-ds-stack]
requires: [may.process-memory-layout, may.call-frame, ds.stack]
concepts: [may.call-stack-vs-ds-stack]
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
Cùng bốn chữ "vào sau, ra trước" — chỉ khác ai là người cầm lệnh đẩy,
lệnh lấy. Hôm nay nói thẳng ra, không lờ đi nữa.
::::

::::explain{#trung-ten-khac-nguoi-cam-quyen}
Đặt hai chuyện cạnh nhau, THẲNG THẮN:

| | Ngăn xếp T3.2 (cấu trúc dữ liệu) | Ngăn xếp gọi hàm (vùng bộ nhớ, bài 22) |
|---|---|---|
| Ai đẩy vào, ai lấy ra | BẠN, gõ tay `.append(x)` / `.pop()` | Trình thông dịch — tự làm việc đó khi lệnh `CALL` (bài 11) chạy, và khi `RETURN_VALUE` (bài 13) chạy |
| Mỗi lần đẩy là gì | Một giá trị bất kỳ bạn chọn | Một **khung** (frame, bài 12) — chứa biến cục bộ, con trỏ lệnh riêng, và địa chỉ quay về |
| Luật | Vào sau, ra trước | Vào sau, ra trước — Y HỆT |

Dòng cuối là chỗ quan trọng nhất: LUẬT giống nhau tuyệt đối. Khi hàm `A`
gọi hàm `B`, khung của `B` đặt LÊN TRÊN khung của `A`. Khi `B` chạy xong
(`RETURN_VALUE`), khung của nó bị gỡ, và khung của `A` — khung nằm NGAY
DƯỚI — lại là khung trên cùng, chạy tiếp đúng chỗ nó đang đợi. Đây chính
là chuyện "tờ phiếu" mà bạn học ở R1.T1.3, và là chuyện "đệ quy chính là
một ngăn xếp" T3.3 bài 1 đã nối lại một lần.

Vậy tên gọi "ngăn xếp" không phải trùng chữ tình cờ — cả hai đều đúng
luật vào-sau-ra-trước. Nhưng chúng KHÔNG phải một vật. Ngăn xếp T3.2 là
một `list` bạn TỰ TAY quản — bạn gõ `.append`, bạn gõ `.pop`, và list đó
có một cái tên bạn tự đặt. Ngăn xếp gọi hàm là một VÙNG BỘ NHỚ riêng
(bài 22) mà TRÌNH THÔNG DỊCH tự quản — không dòng code Python nào của
bạn gọi `.append`/`.pop` lên nó cả, kể cả khi hàm gọi lại chính nó
nhiều lần.
::::

::::example{#hoan-tat-dung-thu-tu}
Hai hàm, hàm ngoài gọi hàm trong ngay giữa thân mình. Mỗi hàm ghi tên
mình vào một nhật ký NGAY TRƯỚC KHI trả về — tức đúng lúc khung của nó
sắp bị gỡ:

```python title=readonly
thu_tu_hoan_tat = []

def dong_goi(don):
    thu_tu_hoan_tat.append("dong_goi")
    return don

def chuan_bi_don(don):
    ket = dong_goi(don)
    thu_tu_hoan_tat.append("chuan_bi_don")
    return ket

chuan_bi_don("bún chả")
print(thu_tu_hoan_tat)
```

```text title=readonly
['dong_goi', 'chuan_bi_don']
```

`chuan_bi_don` được GỌI trước — khung của nó đặt xuống đầu tiên, nằm
DƯỚI. Nhưng nó chưa được GỠ trước, vì nó còn đang đợi `dong_goi` chạy
xong ở dòng `ket = dong_goi(don)`. `dong_goi` không gọi ai nữa, chạy hết
thân, trả về — khung của nó bị gỡ ĐẦU TIÊN. Chỉ sau đó `chuan_bi_don`
mới chạy nốt dòng còn lại và bị gỡ SAU CÙNG.

Không một dòng nào trong hai hàm này viết `.append`/`.pop` lên một ngăn
xếp gọi hàm nào cả — bạn chỉ viết `.append` lên `thu_tu_hoan_tat`, một
list THƯỜNG bạn tự dựng để QUAN SÁT. Thứ tự `['dong_goi',
'chuan_bi_don']` là bằng chứng: khung nào ĐẶT SAU (`dong_goi`, sâu hơn)
lại GỠ TRƯỚC — đúng luật vào-sau-ra-trước, do trình thông dịch tự làm.
::::

::::predict{#doan-thu-tu-hoan-tat-ba-tang commitOnce}
Ba hàm, mỗi hàm gọi hàm kế tiếp trong thân mình, và mỗi hàm ghi tên
mình vào nhật ký NGAY TRƯỚC KHI trả về:

```python
nhat_ky = []

def buoc_c(x):
    nhat_ky.append("buoc_c")
    return x

def buoc_b(x):
    ket = buoc_c(x)
    nhat_ky.append("buoc_b")
    return ket

def buoc_a(x):
    ket = buoc_b(x)
    nhat_ky.append("buoc_a")
    return ket

buoc_a(1)
print(nhat_ky)
```

**Trước khi chạy**, bạn đoán `nhat_ky` in ra thứ tự nào?

:::opt{correct}
`['buoc_c', 'buoc_b', 'buoc_a']` — khung đặt SAU CÙNG (`buoc_c`, sâu
nhất) gỡ TRƯỚC TIÊN
:::

:::opt
`['buoc_a', 'buoc_b', 'buoc_c']` — theo đúng thứ tự chúng được GỌI, từ
ngoài vào trong
::why
Gần đúng ở việc bạn nhớ đúng THỨ TỰ GỌI: `buoc_a` gọi trước, rồi tới
`buoc_b`, rồi `buoc_c` — ba lần gọi lồng nhau đúng như bạn liệt kê.

Chỗ lệch: nhật ký ghi lúc mỗi hàm SẮP TRẢ VỀ, không phải lúc nó BẮT ĐẦU
chạy. Khung của `buoc_a` được đặt lên ngăn xếp gọi hàm ĐẦU TIÊN, nhưng
nó vẫn còn ĐANG ĐỢI ở dòng `ket = buoc_b(x)` cho tới khi `buoc_b` (và
bên trong nó là `buoc_c`) chạy xong hẳn. Khung đặt lên SAU CÙNG
(`buoc_c`) mới là khung không còn đợi ai — nó gỡ trước, và tên nó vào
nhật ký trước.
::
:::

:::opt
`['buoc_c', 'buoc_a', 'buoc_b']`
::why
Gần đúng ở việc bạn nhận ra `buoc_c` phải đứng ĐẦU — đúng, vì nó là
khung sâu nhất, không đợi ai, gỡ trước tiên.

Chỗ lệch nằm ở hai vị trí sau. `buoc_b` là hàm TRỰC TIẾP gọi `buoc_c` —
khung của nó nằm ngay dưới khung `buoc_c`, nên nó phải là khung GỠ THỨ
HAI, ngay sau `buoc_c`. `buoc_a` nằm dưới đáy, đợi cả hai tầng kia xong
mới tới lượt nó — nó phải gỡ SAU CÙNG, không phải thứ hai.
::
:::

:::opt
Không đoán trước được thứ tự, vì Python không đảm bảo hàm nào chạy
trước hàm nào khi có lồng gọi nhau
::why
Gần đúng ở sự thận trọng — cẩn thận trước một đoạn mã có nhiều lời gọi
lồng nhau là thái độ đúng.

Chỗ lệch: thứ tự ở đây HOÀN TOÀN xác định, không có gì ngẫu nhiên. Máy
chạy Python từng lệnh một, và ngăn xếp gọi hàm tuân đúng luật vào-sau-ra-
trước bài này vừa nêu — không có "may rủi" nào trong việc khung nào gỡ
trước, khung nào gỡ sau.
::
:::
::::

::::code{#doi-chieu-hai-ngan-xep}
Ba hàm chuẩn bị món, mỗi hàm gọi hàm kế tiếp rồi tự ghi tên mình vào
`da_tra_ve` NGAY TRƯỚC KHI trả về — đúng lúc khung của nó sắp bị gỡ.

Việc của bạn: tự dựng một ngăn xếp `list` (đúng kiểu T3.2 dạy), rồi lấy
ra đúng thứ tự, và xác nhận nó khớp với thứ tự THẬT mà máy vừa gỡ khung.

```python title=starter
da_tra_ve = []

def mon_1(don):
    ket = mon_2(don)
    da_tra_ve.append("mon_1")
    return ket

def mon_2(don):
    ket = mon_3(don)
    da_tra_ve.append("mon_2")
    return ket

def mon_3(don):
    da_tra_ve.append("mon_3")
    return don

mon_1("phở")

# Ngăn xếp BẠN tự dựng — ba lời gọi đang dở lúc mon_3 vừa được gọi,
# đặt theo đúng thứ tự GỌI (mon_1 dưới đáy, mon_3 trên đỉnh)
ngan_xep_tu_dung = ["mon_1", "mon_2", "mon_3"]

lay_ra_dau_tien = ngan_xep_tu_dung.pop()   # đã cho sẵn — lấy đúng luật ngăn xếp
lay_ra_thu_hai = ___                        # lấy tiếp, cũng đúng luật ấy
lay_ra_cuoi = ngan_xep_tu_dung.pop()

thu_tu_tu_dung = [lay_ra_dau_tien, lay_ra_thu_hai, lay_ra_cuoi]
khop_voi_may_that = thu_tu_tu_dung == da_tra_ve

print(da_tra_ve)
print(thu_tu_tu_dung)
print(khop_voi_may_that)
```

```python title=solution
da_tra_ve = []

def mon_1(don):
    ket = mon_2(don)
    da_tra_ve.append("mon_1")
    return ket

def mon_2(don):
    ket = mon_3(don)
    da_tra_ve.append("mon_2")
    return ket

def mon_3(don):
    da_tra_ve.append("mon_3")
    return don

mon_1("phở")

ngan_xep_tu_dung = ["mon_1", "mon_2", "mon_3"]

lay_ra_dau_tien = ngan_xep_tu_dung.pop()
lay_ra_thu_hai = ngan_xep_tu_dung.pop()
lay_ra_cuoi = ngan_xep_tu_dung.pop()

thu_tu_tu_dung = [lay_ra_dau_tien, lay_ra_thu_hai, lay_ra_cuoi]
khop_voi_may_that = thu_tu_tu_dung == da_tra_ve

print(da_tra_ve)
print(thu_tu_tu_dung)
print(khop_voi_may_that)
```

```python title=test
assert da_tra_ve == ["mon_3", "mon_2", "mon_1"], f"máy phải gỡ khung mon_3 trước, rồi mon_2, rồi mon_1 — đang ra {da_tra_ve}"
assert thu_tu_tu_dung == ["mon_3", "mon_2", "mon_1"], f"ngăn xếp bạn tự dựng phải lấy ra đúng thứ tự mon_3, mon_2, mon_1 — đang ra {thu_tu_tu_dung}"
assert khop_voi_may_that is True, "thứ tự bạn tự dựng bằng .pop() phải KHỚP với thứ tự khung THẬT bị gỡ — cùng một luật vào-sau-ra-trước"
```

:::hints
- kind: attention
  body: Chỉ một chỗ trống. Hai lượt .pop() còn lại đã có sẵn trong khung — chỗ trống đứng giữa hai lượt đó, làm đúng việc y hệt.
- kind: strategy
  body: 'Lấy tiếp phần tử TRÊN CÙNG của ngan_xep_tu_dung, dùng đúng công cụ hai dòng xung quanh đã dùng — .pop() không tham số.'
- kind: one-line
  body: 'Chỗ trống là: ngan_xep_tu_dung.pop()'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ gọi ngan_xep_tu_dung.pop() — đúng ba lượt .pop() trong toàn bài (hai đã cho sẵn, một ở chỗ trống), không phải một câu không làm gì như True, 1, 0
  requireAst:
  # min: 3 — đếm thật trên solution: .pop() xuất hiện đúng 3 lần trong mã
  # nguồn (hai lượt đã có sẵn trong khung + chỗ trống). Điền True/1/0 chỉ còn
  # 2 lần — dưới 3, luật này chặn được. ĐÃ THỬ THẬT bằng cả ba cách True/1/0:
  # không vòng lặp nào trong bài (ba lượt .pop() viết THẲNG HÀNG, không phải
  # trong while), nên không cách nào chạy vô hạn — cả ba dừng ngay, và cả ba
  # cho khop_voi_may_that == False (thu_tu_tu_dung lẫn True/1/0 vào giữa,
  # không khớp da_tra_ve) — assert bắt được độc lập với luật static này.
  - kind: uses-call, target: pop, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\['mon_3', 'mon_2', 'mon_1'\\]\\n\\['mon_3', 'mon_2', 'mon_1'\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khung `mon_3` vào sau cùng, ra trước tiên — không viết một dòng
`.append`/`.pop` nào lên ngăn xếp gọi hàm, mà nó vẫn đúng luật, từng
bước.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Ngăn xếp" vừa được nói thẳng: trùng tên với T3.2, nhưng khác vai trò —
một cái là `list` bạn tự quản, một cái là vùng bộ nhớ interpreter tự
quản.

Bài 22 còn nhắc tới một vùng khác cũng trùng tên với một cấu trúc dữ
liệu T3.2 đã dạy: **đống**. Nó có giống "ngăn xếp" vừa rồi — cùng luật,
khác người cầm quyền — hay khác hẳn, theo một kiểu khác?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
