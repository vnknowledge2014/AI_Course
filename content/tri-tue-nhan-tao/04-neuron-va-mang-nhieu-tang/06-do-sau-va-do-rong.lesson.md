---
id: tri-tue-nhan-tao.neuron-va-mang-nhieu-tang.do-sau-va-do-rong
title: "Độ sâu và độ rộng"
summary: "Bốn kiến trúc, cùng 4 đầu vào và 1 đầu ra: hẹp-nông [4,3,1]=19 tham số, rộng-nông [4,8,1]=49, rộng hơn-nông [4,16,1]=97 (rộng gấp đôi rong_nong), hẹp-sâu [4,8,8,1]=121 (thêm 1 tầng ẩn 8 neuron thay vì mở rộng tầng có sẵn) — kiến trúc SÂU hơn có NHIỀU tham số hơn kiến trúc RỘNG hơn dù cả hai cùng tăng từ rong_nong, vì kết nối GIỮA hai tầng ẩn (8×8=64) tốn nhiều hơn kết nối từ 4 đầu vào (4×16=64 nhưng cộng thêm phần tầng đầu 4×8=32 đã có sẵn)."
locale: vi
track: tri-tue-nhan-tao
module: neuron-va-mang-nhieu-tang
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.do-sau-va-do-rong]
requires: [ai.khoi-tao-trong-so]
concepts: [ai.do-sau-va-do-rong]
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
Mạng ở bài `mang-nhieu-tang-forward-pass` có đúng một tầng ẩn, hai neuron.
Sao không mười? Sao không thêm một tầng ẩn nữa? Câu trả lời không miễn phí.
::::

::::explain{#do-sau-do-rong-va-nang-luc}
Hai trục để "làm mạng lớn hơn":

> **Độ rộng** (width) — số neuron trong MỘT tầng. Tăng độ rộng nghĩa là
> thêm neuron song song trong cùng tầng ẩn.
>
> **Độ sâu** (depth) — số tầng nối tiếp nhau. Tăng độ sâu nghĩa là thêm
> HẲN một tầng ẩn mới, xử lý đầu ra của tầng trước đó.

Cả hai đều làm tăng **năng lực mô hình** (capacity — khả năng biểu diễn
những hàm số phức tạp), đo thô bằng SỐ THAM SỐ (mỗi trọng số và mỗi độ
lệch là một tham số học được). Một tầng có `n_vào` đầu vào và `n_ra`
neuron cần `n_vào × n_ra` trọng số (ma trận `W`) cộng `n_ra` độ lệch (một
`b` mỗi neuron) — tổng `n_vào × n_ra + n_ra` tham số cho riêng tầng đó.

`do-sau-va-do-rong` không huấn luyện gì — chỉ ĐẾM tham số để thấy hai trục
này tốn kém khác nhau ra sao, rồi nối lại với khái niệm đã học ở
`bias-variance-tradeoff` (T8.1c): mạng CÀNG NHIỀU tham số càng có capacity
cao — dễ khớp sát dữ liệu train hơn (bias thấp hơn), nhưng cũng dễ nhớ cả
nhiễu của train hơn (variance cao hơn, cần nhiều dữ liệu hơn để tránh
overfit). Mạng ÍT tham số hơn thì ngược lại: bias cao hơn (có thể không đủ
sức khớp một quan hệ phức tạp), variance thấp hơn (ổn định hơn khi đổi tập
train). Đúng đánh đổi đã đo bằng số ở T8.1c — chỉ đổi trục hoành từ "bậc đa
thức" sang "độ sâu/độ rộng mạng".
::::

::::example{#dem_tham_so_bon_kien_truc}
Bốn kiến trúc, cùng `4` đầu vào và `1` đầu ra, khác nhau ở PHẦN GIỮA:

```python title=readonly
def dem_tham_so(kich_thuoc):
    tong = 0
    for n_vao, n_ra in zip(kich_thuoc[:-1], kich_thuoc[1:]):
        tong += n_vao * n_ra + n_ra
    return tong

kien_truc = {
    "hep_nong": [4, 3, 1],       # 1 tang an, 3 neuron
    "rong_nong": [4, 8, 1],      # 1 tang an, 8 neuron
    "rong_hon_nong": [4, 16, 1], # 1 tang an, 16 neuron (RONG gap doi rong_nong)
    "hep_sau": [4, 8, 8, 1],     # 2 tang an, 8 neuron moi tang (SAU hon rong_nong)
}

for ten, kt in kien_truc.items():
    print(f"{ten}: {kt} -> {dem_tham_so(kt)} tham so")
```

```text title=readonly
hep_nong: [4, 3, 1] -> 19 tham so
rong_nong: [4, 8, 1] -> 49 tham so
rong_hon_nong: [4, 16, 1] -> 97 tham so
hep_sau: [4, 8, 8, 1] -> 121 tham so
```

`rong_nong` (`49`) tăng độ RỘNG gấp đôi thành `rong_hon_nong` (`97`) —
tăng gần gấp đôi số tham số, hợp lý vì tầng `[4→16]` cần `4×16+16=80` tham
số so với `4×8+8=40` của tầng `[4→8]`. Nhưng `hep_sau` (`121`, thêm HẲN một
tầng `[8→8]` thay vì mở rộng tầng có sẵn) có SỐ THAM SỐ CÒN CAO HƠN
`rong_hon_nong` — dù cả hai đều "gấp đôi" một cách nào đó so với
`rong_nong`. Lý do: tầng MỚI nối hai tầng ẩn (`8→8`) tốn `8×8+8=72` tham
số riêng cho một tầng — kết nối giữa hai tầng RỘNG tốn theo TÍCH của hai độ
rộng, trong khi mở rộng tầng đầu chỉ tốn theo tích của độ rộng MỚI với số
đầu vào cố định (`4`).
::::

::::predict{#doan_kien_truc_nhieu_tham_so_hon commitOnce}
Bốn kiến trúc trên, cùng `4` đầu vào/`1` đầu ra: `rong_nong` (`49`),
`rong_hon_nong` (`97`, rộng gấp đôi), `hep_sau` (`121`, thêm một tầng ẩn
`8` neuron nữa).

**Trước khi đọc lại**, bạn đoán: nếu tiếp tục xu hướng này, một kiến trúc
`[4, 8, 8, 8, 1]` (BA tầng ẩn, mỗi tầng `8` neuron — thêm MỘT tầng `8→8`
nữa so với `hep_sau`) sẽ có số tham số nhiều hơn hay ít hơn `rong_hon_nong`
(`97`, chỉ một tầng ẩn `16` neuron)?

:::opt{correct}
Nhiều hơn hẳn — mỗi tầng `[8→8]` thêm vào tốn `72` tham số, và `hep_sau`
(một tầng `[8→8]`, đã có `121`) cộng thêm một tầng `[8→8]` NỮA sẽ vượt xa
`97` của `rong_hon_nong`
:::

:::opt
Ít hơn — `rong_hon_nong` có `16` neuron trong MỘT tầng, nhiều neuron hơn
bất kỳ tầng nào của `[4,8,8,8,1]` (mỗi tầng chỉ `8`), nên tổng phải ít hơn
::why
Gần đúng ở việc so sánh ĐỘ RỘNG của TỪNG tầng riêng lẻ — đúng là không tầng
nào trong `[4,8,8,8,1]` rộng bằng tầng `16` neuron của `rong_hon_nong`.

Chỗ lệch: tổng số tham số không cộng dồn theo ĐỘ RỘNG của từng tầng, mà
theo TÍCH của độ rộng hai tầng LIÊN TIẾP cộng lại qua nhiều tầng. Thêm một
tầng ẩn `8→8` KHÔNG thay thế một tầng rộng — nó CỘNG THÊM `72` tham số vào
tổng đã có, và vài lần cộng thêm `72` như vậy vượt xa mức tăng một lần từ
mở rộng một tầng duy nhất từ `8` lên `16`.
::
:::

:::opt
Bằng nhau — cả hai kiến trúc đều "gấp đôi" quy mô của `rong_nong` theo một
cách nào đó, nên phải tốn số tham số ngang nhau
::why
Gần đúng ở việc nhận ra CẢ HAI kiến trúc đều là một dạng "mở rộng" so với
`rong_nong` — quan sát về việc cả hai đều lớn hơn bản gốc không sai.

Chỗ lệch: "gấp đôi quy mô" không phải một phép đo CHÍNH XÁC và duy nhất —
tăng ĐỘ RỘNG (nhân đôi số neuron một tầng) và tăng ĐỘ SÂU (thêm hẳn tầng
mới) tốn tham số theo hai công thức khác hẳn nhau (cộng thêm `n_vào×Δ` so
với thêm hẳn `n×n` cho một tầng mới), nên không có lý do gì để hai con số
đó trùng nhau.
::
:::
::::

::::code{#dem_tham_so_va_tim_lon_nhat}
Hoàn thiện `dem_tham_so` (cộng dồn số tham số của MỖI tầng: `n_vào×n_ra +
n_ra`), rồi tìm kiến trúc có TỔNG số tham số LỚN NHẤT trong bốn kiến trúc.

```python title=starter
def dem_tham_so(kich_thuoc):
    tong = 0
    for n_vao, n_ra in zip(kich_thuoc[:-1], kich_thuoc[1:]):
        tong += ___                    # n_vao * n_ra + n_ra
    return tong

kien_truc = {
    "hep_nong": [4, 3, 1],
    "rong_nong": [4, 8, 1],
    "rong_hon_nong": [4, 16, 1],
    "hep_sau": [4, 8, 8, 1],
}

so_tham_so_theo_ten = {}
for ten, kt in kien_truc.items():
    so_tham_so_theo_ten[ten] = dem_tham_so(kt)

ten_nhieu_tham_so_nhat = ___          # max(so_tham_so_theo_ten, key=so_tham_so_theo_ten.get)

print(so_tham_so_theo_ten["hep_nong"], so_tham_so_theo_ten["rong_nong"], so_tham_so_theo_ten["rong_hon_nong"], so_tham_so_theo_ten["hep_sau"])
print(ten_nhieu_tham_so_nhat)
```

```python title=solution
def dem_tham_so(kich_thuoc):
    tong = 0
    for n_vao, n_ra in zip(kich_thuoc[:-1], kich_thuoc[1:]):
        tong += n_vao * n_ra + n_ra
    return tong

kien_truc = {
    "hep_nong": [4, 3, 1],
    "rong_nong": [4, 8, 1],
    "rong_hon_nong": [4, 16, 1],
    "hep_sau": [4, 8, 8, 1],
}

so_tham_so_theo_ten = {}
for ten, kt in kien_truc.items():
    so_tham_so_theo_ten[ten] = dem_tham_so(kt)

ten_nhieu_tham_so_nhat = max(so_tham_so_theo_ten, key=so_tham_so_theo_ten.get)

print(so_tham_so_theo_ten["hep_nong"], so_tham_so_theo_ten["rong_nong"], so_tham_so_theo_ten["rong_hon_nong"], so_tham_so_theo_ten["hep_sau"])
print(ten_nhieu_tham_so_nhat)
```

```python title=test
assert so_tham_so_theo_ten["hep_nong"] == 19, f"hep_nong phai co 19 tham so -- dang ra {so_tham_so_theo_ten['hep_nong']}"
assert so_tham_so_theo_ten["rong_nong"] == 49, f"rong_nong phai co 49 tham so -- dang ra {so_tham_so_theo_ten['rong_nong']}"
assert so_tham_so_theo_ten["rong_hon_nong"] == 97, f"rong_hon_nong phai co 97 tham so -- dang ra {so_tham_so_theo_ten['rong_hon_nong']}"
assert so_tham_so_theo_ten["hep_sau"] == 121, f"hep_sau phai co 121 tham so -- dang ra {so_tham_so_theo_ten['hep_sau']}"
assert ten_nhieu_tham_so_nhat == "hep_sau", f"kien truc nhieu tham so nhat phai la hep_sau -- dang ra {ten_nhieu_tham_so_nhat}"
assert so_tham_so_theo_ten["hep_sau"] > so_tham_so_theo_ten["rong_hon_nong"], "kien truc SAU hon (hep_sau) phai co nhieu tham so hon kien truc RONG hon (rong_hon_nong)"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là công thức số tham số của MỘT tầng — `n_vào × n_ra + n_ra` (ma trận trọng số cộng vector độ lệch), cộng dồn vào `tong` qua mỗi cặp `(n_vào, n_ra)` liên tiếp trong `kích_thước`. Chỗ sau tìm khoá có GIÁ TRỊ lớn nhất trong dict `so_tham_so_theo_ten` — dùng `max(...)` với tham số `key`, cùng khuôn với `min(...)` đã dùng ở các bài trước khi chọn giá trị nhỏ nhất.
- kind: strategy
  body: 'tong += n_vao * n_ra + n_ra. ten_nhieu_tham_so_nhat: `max(so_tham_so_theo_ten, key=so_tham_so_theo_ten.get)`.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `n_vao * n_ra + n_ra` và `max(so_tham_so_theo_ten, key=so_tham_so_theo_ten.get)`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dem_tham_so phai tinh THAT so tham so tu n_vao/n_ra cua tung tang (khong duoc tra ve bang tra cuu chep san); ten_nhieu_tham_so_nhat phai dung THAT max(...) tren so_tham_so_theo_ten
  requireAst:
  - kind: uses-operator, target: "*", min: 1
  - kind: uses-name, target: n_vao, min: 1
  - kind: uses-name, target: n_ra, min: 2
  - kind: uses-call, target: max, min: 1
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai
  # dung dat=true, ca bon luat qua sach. Cheat "dem_tham_so tra ve tu mot
  # bang tra cuu {kich_thuoc: so_tham_so}, khong tinh tu n_vao/n_ra" lam ca
  # ba luat dau (*, n_vao, n_ra) ve 0 -- bi chan boi ca ba. Cheat "chep san
  # ten_nhieu_tham_so_nhat = 'hep_sau'" lam "max" ve 0 -- bi chan.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^19 49 97 121\\nhep_sau\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`hep_sau` (`121` tham số) vượt `rong_hon_nong` (`97`) dù cả hai đều "gấp
đôi" một cách nào đó — thêm tầng tốn kém khác hẳn mở rộng tầng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này (q8.2a) chỉ dùng mạng cho phân loại nhị phân — tầng ra `1`
neuron, `sigmoid`. Nhưng `neuron-don` (bài 1) đã nói: hàm kích hoạt của
tầng CUỐI không nhất thiết phải là `sigmoid`. Nếu bài toán là dự đoán một
con số liên tục (hồi quy, như `hoi-quy-tuyen-tinh-tu-so-0` ở T8.1a) thay vì
phân loại — tầng ra của mạng cần đổi những gì?
::::

::::checkpoint{mastery=0.8}
::::
