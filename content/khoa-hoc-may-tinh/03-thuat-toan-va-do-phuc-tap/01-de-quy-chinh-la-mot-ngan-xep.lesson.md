---
id: khoa-hoc-may-tinh.thuat-toan-va-do-phuc-tap.de-quy-chinh-la-mot-ngan-xep
title: "Đệ quy đã học — giờ nối lại với ngăn xếp bạn vừa tự dựng"
summary: "'Máy nhớ đường về' (đã học) và ngăn xếp .append/.pop (đã học) không phải hai chuyện khác nhau — một lời gọi hàm là ĐẨY một tờ phiếu, một lần trả về là LẤY nó ra, đúng luật vào-sau-ra-trước, chỉ khác là trình thông dịch giữ ngăn xếp giùm bạn."
locale: vi
track: khoa-hoc-may-tinh
module: thuat-toan-va-do-phuc-tap
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [alg.recursion-is-stack]
requires: [func.call-stack, func.recursion, ds.stack]
concepts: [alg.recursion-is-stack]
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
Tờ phiếu, chồng lời gọi, gỡ ra khi xong việc — bạn đã biết cả rồi. Hôm
nay chỉ gọi đúng tên của nó.
::::

::::explain{#hai-cai-ten-mot-thu}
Bạn đã học hai chuyện, ở hai chỗ khác nhau, và chưa ai nói chúng là một.

Chuyện thứ nhất: mỗi lần một hàm được gọi, máy đặt một **tờ phiếu** lên
một cái **chồng lời gọi** — ghi tên hàm, các giá trị nó đang cầm, và
dòng nào đang đợi kết quả. Hàm chạy xong thì tờ phiếu trên cùng bị gỡ,
và hàm bên dưới chạy tiếp từ đúng chỗ nó đang đợi. Đó là cách bạn hiểu
`RecursionError`: chồng phiếu cao quá mức máy cho phép.

Chuyện thứ hai, học ngay trước cụm bài này: một cấu trúc dựng bằng
`list`, chỉ cho phép hai việc — `.append(x)` thêm vào đuôi,
`.pop()` lấy đúng phần tử vừa thêm gần đây nhất ra. Luật của nó gói
trong bốn chữ **vào sau, ra trước**. Tên nó là **ngăn xếp**.

Đặt hai chuyện cạnh nhau:

| Chồng lời gọi (đã học) | Ngăn xếp (đã học) |
|---|---|
| gọi một hàm → đặt thêm một tờ phiếu | `ngan_xep.append(x)` |
| hàm chạy xong → gỡ tờ phiếu trên cùng | `ngan_xep.pop()` |
| tờ gỡ ra luôn là tờ đặt lên **gần đây nhất** | phần tử lấy ra luôn là phần tử thêm vào **gần đây nhất** |

Không phải một sự tương tự khiên cưỡng. Đó là **cùng một luật vào-sau-
ra-trước**, cùng một lý do một hàm gọi hàm khác không làm mất chỗ đứng
của hàm gọi. Khác biệt duy nhất: với ngăn xếp bạn tự tay viết
`.append`/`.pop`; với lời gọi hàm, trình thông dịch làm đúng hai việc
đó **giùm bạn**, không cần bạn viết dòng nào, kể cả khi hàm gọi lại
chính nó — tức là đệ quy.

Vậy câu trả lời cho tên bài: **đệ quy chính là một ngăn xếp**, chỉ là
ngăn xếp ấy do máy quản, không phải do bạn.
::::

::::example{#doi-chieu-tren-cung-mot-vi-du}
Hàm đếm ngược có nhánh dừng, đã chạy ở bài "Trường hợp dừng lại":

```python title=readonly
def dem_nguoc(n):
    if n == 0:
        print("Xong!")
    else:
        print(f"Byte đếm: {n}")
        dem_nguoc(n - 1)

dem_nguoc(2)
```

```text title=readonly
Byte đếm: 2
Byte đếm: 1
Xong!
```

Đúng khoảnh khắc `"Xong!"` in ra — lúc `dem_nguoc(0)` đang chạy — ba tờ
phiếu vẫn còn nguyên trên chồng, vì chưa tờ nào chạy xong để gỡ:
`dem_nguoc(2)` (đặt lên đầu tiên, dưới đáy), `dem_nguoc(1)` (đặt sau),
`dem_nguoc(0)` (đặt sau cùng, đang ở đỉnh).

Viết đúng ba lần đặt phiếu ấy bằng ngôn ngữ ngăn xếp bạn đã quen:

```python title=readonly
ngan_xep = []
ngan_xep.append("dem_nguoc(2)")
ngan_xep.append("dem_nguoc(1)")
ngan_xep.append("dem_nguoc(0)")
print(ngan_xep)
```

```text title=readonly
['dem_nguoc(2)', 'dem_nguoc(1)', 'dem_nguoc(0)']
```

Y hệt thứ tự cái chồng phiếu thật đang có trong bộ nhớ lúc đó. Từ đây
trở đi, mỗi lần một `dem_nguoc(...)` chạy xong, máy làm đúng việc
`ngan_xep.pop()` làm: gỡ đúng phần tử vừa đặt lên sau cùng —
`dem_nguoc(0)` gỡ trước, rồi `dem_nguoc(1)`, rồi `dem_nguoc(2)` gỡ sau
chót. Vào sau cùng, ra trước tiên, không lệch một bước.
::::

::::predict{#dinh-cua-chong commitOnce}
Cũng `dem_nguoc`, nhưng lần này gọi với `dem_nguoc(3)` — thêm đúng một
tầng so với ví dụ vừa xem:

```python
def dem_nguoc(n):
    if n == 0:
        print("Xong!")
    else:
        print(f"Byte đếm: {n}")
        dem_nguoc(n - 1)

dem_nguoc(3)
```

**Ngay khoảnh khắc** chữ `"Xong!"` vừa in ra — trước khi bất cứ lời gọi
nào kịp trả về — chồng lời gọi đang chứa đúng những lời gọi nào, xếp
theo thứ tự **từ đáy lên đỉnh** (từ lời gọi cũ nhất tới mới nhất)?

:::opt{correct}
`dem_nguoc(3)`, `dem_nguoc(2)`, `dem_nguoc(1)`, `dem_nguoc(0)` — cả
bốn, chưa tờ nào gỡ
:::

:::opt
Chỉ `dem_nguoc(0)` — vì ba lời gọi trước đã in xong câu của mình rồi
::why
Gần đúng ở chỗ `dem_nguoc(0)` đúng là tờ đang ở **đỉnh**, đang chạy —
phần đó bạn nhìn đúng.

Chỗ lệch: in xong một câu `print` không phải in xong toàn bộ thân hàm.
`dem_nguoc(3)` in câu của nó rồi mới GỌI `dem_nguoc(2)`, `dem_nguoc(2)`
in rồi gọi `dem_nguoc(1)` — mỗi dòng gọi ấy chưa xong nghĩa là chính
lượt gọi đó cũng chưa xong, nên không tờ phiếu nào trong ba tờ ấy bị
gỡ. Một tờ chỉ gỡ khi hàm ấy chạy **hết thân mình**, không phải khi nó
in xong một dòng.
::
:::

:::opt
`dem_nguoc(0)`, `dem_nguoc(1)`, `dem_nguoc(2)`, `dem_nguoc(3)` — đỉnh
xuống đáy
::why
Gần đúng ở việc bạn liệt kê đúng cả bốn lời gọi đang có mặt — không
thiếu, không thừa cái tên nào.

Chỗ lệch nằm ở câu hỏi: nó hỏi thứ tự **từ đáy lên đỉnh**, tức từ lời
gọi cũ nhất. `dem_nguoc(3)` được đặt lên đầu tiên nên nó nằm dưới đáy,
không phải trên đỉnh — thứ tự bạn liệt kê đang bị đảo ngược.
::
:::

:::opt
Không xác định được, vì `dem_nguoc(3)`, `dem_nguoc(2)` và `dem_nguoc(1)`
chạy song song với `dem_nguoc(0)`
::why
Gần đúng ở sự thận trọng khi không chắc — thái độ đó đúng đắn khi gặp
mã lạ.

Chỗ lệch: máy chạy Python làm đúng **một việc một lúc**, không có
"song song" ở đây. `dem_nguoc(3)` gọi `dem_nguoc(2)` rồi đứng **đợi**,
`dem_nguoc(2)` gọi `dem_nguoc(1)` rồi cũng đứng đợi — không lượt nào
làm gì khác trong lúc đợi — đó chính là lý do mỗi lượt cần một tờ
phiếu giữ chỗ, thay vì biến mất.
::
:::
::::

::::code{#nhat-ky-day-va-lay}
Byte muốn nhìn thấy chồng lời gọi rõ hơn nữa — không chỉ đoán, mà có
một **nhật ký** ghi lại đúng lúc nào một tờ được ĐẨY lên, lúc nào một
tờ được LẤY ra.

Hàm dưới đây tính tổng `1 + 2 + ... + n` bằng đệ quy. Cả nhánh dừng lẫn
lời gọi đệ quy đã viết sẵn — việc của bạn chỉ là thêm đúng hai dòng ghi
nhật ký, đặt đúng chỗ: một dòng ngay khi một lời gọi sắp KHÔNG gọi ai
nữa (tức sắp trả lời ngay), và một dòng ngay trước khi một lời gọi
TRẢ VỀ kết quả cho lời gọi đã sinh ra nó.

```python title=starter
nhat_ky = []

def tong_den(n):
    nhat_ky.append(f"ĐẨY tong_den({n})")
    if n == 0:
        ___                            # LẤY: nhánh này không gọi ai, trả lời ngay
        return 0
    ket_qua = n + tong_den(n - 1)
    ___                                # LẤY: vừa nhận kết quả từ lời gọi con, sắp trả về
    return ket_qua

ket_qua_cuoi = tong_den(3)
print(ket_qua_cuoi)
print(nhat_ky)
```

```python title=solution
nhat_ky = []

def tong_den(n):
    nhat_ky.append(f"ĐẨY tong_den({n})")
    if n == 0:
        nhat_ky.append(f"LẤY tong_den({n})")
        return 0
    ket_qua = n + tong_den(n - 1)
    nhat_ky.append(f"LẤY tong_den({n})")
    return ket_qua

ket_qua_cuoi = tong_den(3)
print(ket_qua_cuoi)
print(nhat_ky)
```

```python title=test
assert ket_qua_cuoi == 6, f"tong_den(3) phải là 1 + 2 + 3 = 6 — đang ra {ket_qua_cuoi}"
assert len(nhat_ky) == 8, f"bốn lời gọi (n=3,2,1,0), mỗi lời gọi ĐẨY một lần và LẤY một lần — phải đúng 8 dòng nhật ký, đang có {len(nhat_ky)}"
assert nhat_ky == [
    "ĐẨY tong_den(3)", "ĐẨY tong_den(2)", "ĐẨY tong_den(1)", "ĐẨY tong_den(0)",
    "LẤY tong_den(0)", "LẤY tong_den(1)", "LẤY tong_den(2)", "LẤY tong_den(3)",
], f"thứ tự ĐẨY phải là 3,2,1,0 (đi xuống), rồi LẤY phải là 0,1,2,3 (đi ngược lên) — đúng luật vào sau ra trước — đang ra {nhat_ky}"
```

:::hints
- kind: attention
  body: Hai chỗ trống đều chỉ làm MỘT việc — ghi một dòng vào nhat_ky bằng .append(...). Chưa cần lo cái gì, chỉ cần đúng chỗ và đúng công cụ .append đã quen từ ngăn xếp.
- kind: strategy
  body: 'Đúng lúc một tờ phiếu chuẩn bị bị gỡ (LẤY), hãy ghi lại đúng tên và giá trị của lời gọi ĐANG đứng ở dòng đó — dùng biến n, giống hệt câu ĐẨY ở đầu hàm đã viết sẵn cho bạn. Viết: nhat_ky.append(f"LẤY tong_den({n})").'
- kind: one-line
  body: 'Cả hai chỗ trống đều là: nhat_ky.append(f"LẤY tong_den({n})")'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ ghi vào nhat_ky bằng .append(...) — không được để trống bằng một câu không làm gì (như True, 1, 0), vì như vậy nhật ký sẽ thiếu đúng bốn dòng LẤY
  requireAst:
  - kind: uses-call, target: append, min: 3
  # min: 3 — đếm thật trên solution: .append() xuất hiện 3 lần trong MÃ NGUỒN
  # (không phải trong lúc chạy) — một lần đã có sẵn trong khung (dòng ĐẨY đầu
  # hàm), cộng đúng hai lần ở hai chỗ trống. Điền True/1/0 (một câu không làm
  # gì) vào một hoặc cả hai chỗ trống chỉ còn 1 hoặc 2 lần .append trong mã
  # nguồn — dưới 3, luật này chặn được, ĐÃ THỬ THẬT bằng cả ba cách True/1/0:
  # cả ba đều dừng an toàn (không lặp vô hạn, đệ quy đã có nhánh dừng và lời
  # gọi con nguyên vẹn trong khung), và cả ba đều cho ket_qua_cuoi == 6 đúng
  # (vì phép cộng không phụ thuộc nhật ký) NHƯNG nhat_ky chỉ còn 4 dòng ĐẨY,
  # thiếu sạch bốn dòng LẤY — nên assert độ dài và assert danh sách đều bắt
  # được độc lập với luật static này.
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^6\\n\\['ĐẨY tong_den\\(3\\)', 'ĐẨY tong_den\\(2\\)', 'ĐẨY tong_den\\(1\\)', 'ĐẨY tong_den\\(0\\)', 'LẤY tong_den\\(0\\)', 'LẤY tong_den\\(1\\)', 'LẤY tong_den\\(2\\)', 'LẤY tong_den\\(3\\)'\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
ĐẨY đi xuống 3, 2, 1, 0. LẤY đi ngược lên 0, 1, 2, 3. Đúng ngăn xếp,
chỉ là bạn không phải tự tay viết .append hay .pop nào cả — trừ trong
nhật ký này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài "Duyệt cây: ba thứ tự, ba câu chuyện khác nhau" đã duyệt hết một
cây bằng `while` cộng một ngăn xếp **tự quản** — một `list` bạn tự
`.append`/`.pop` bằng tay, nhìn thấy nó phình to rồi co lại từng bước.

Giờ vừa học xong: một lời gọi hàm đệ quy làm đúng việc ấy, chỉ là
trình thông dịch giữ ngăn xếp giùm. Vậy nếu viết lại CHÍNH VIỆC DUYỆT
CÂY đó — không phải một ví dụ mới, đúng cái cây bảy số báo danh cũ —
bằng đệ quy thay vì `while`, hai bản có cho ra cùng một kết quả không?

Bài sau đặt hai bản cạnh nhau để bạn tự thấy.
::::

::::checkpoint{mastery=0.8}
::::
