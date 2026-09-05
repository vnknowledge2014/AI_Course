---
id: tri-tue-nhan-tao.tong-quat-hoa-va-hop-nhat.bootstrap-resampling
title: "Bootstrap resampling"
summary: "12 số đo (trung bình gốc=124.5): lấy 1000 mẫu CÓ HOÀN LẠI (bootstrap, np.random.default_rng(42)) từ CHÍNH 12 số đó, tính trung bình của mỗi mẫu — độ lệch chuẩn GIỮA 1000 trung bình bootstrap đó là 5.2836, gần khớp sai số chuẩn tính bằng công thức giải tích std/√n = 5.2505 — ước lượng được độ KHÔNG CHẮC CHẮN của trung bình gốc mà không cần công thức, chỉ bằng lấy mẫu lại nhiều lần."
locale: vi
track: tri-tue-nhan-tao
module: tong-quat-hoa-va-hop-nhat
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.bootstrap-resampling]
requires: [ai.bias-variance-tradeoff]
concepts: [ai.bootstrap-resampling]
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
Bài trước đo variance bằng cách chạy lại với NHIỀU seed nhiễu khác nhau —
nhưng đó là một mánh của người viết bài, có quyền "sinh lại dữ liệu mới".
Ngoài đời chỉ có ĐÚNG một bộ dữ liệu đã thu thập. Vẫn đo được độ không chắc
chắn từ chính nó không?
::::

::::explain{#lay-mau-lai-co-hoan-lai}
Byte có đúng `12` số đo (giả sử: thời gian phản hồi của một API, tính bằng
mili-giây). Trung bình của `12` số đó là MỘT con số cụ thể — nhưng nếu đo
lại `12` lần KHÁC (một ngày khác, một đợt traffic khác), trung bình sẽ ra
một con số hơi khác. Câu hỏi: con số trung bình này "chắc" tới mức nào?
Không thể đo lại thật — chỉ có đúng một bộ dữ liệu trong tay.

**Bootstrap** giải quyết đúng tình huống đó: giả vờ chính bộ dữ liệu ĐÃ CÓ
là "toàn bộ dân số", rồi lấy mẫu LẠI từ nó, **có hoàn lại** (with
replacement) — mỗi lần lấy một mẫu CÙNG kích thước `n` với dữ liệu gốc,
nhưng CHO PHÉP một điểm bị chọn nhiều lần (và một điểm khác không được chọn
lần nào). Với mỗi mẫu bootstrap, tính lại thống kê quan tâm (ở đây là
trung bình). Lặp lại việc đó hàng nghìn lần, thu được hàng nghìn con số
trung bình — một PHÂN PHỐI của thống kê đó, ước lượng được TỪ CHÍNH một bộ
dữ liệu duy nhất, không cần đo lại gì thêm và không cần biết trước công
thức giải tích của sai số chuẩn.

Độ lệch chuẩn của phân phối bootstrap đó — độ "trải" của hàng nghìn giá trị
trung bình bootstrap quanh nhau — chính là ước lượng cho **sai số chuẩn**
(standard error) của trung bình gốc: nó nói lên, nếu đo lại dữ liệu, trung
bình có khả năng dao động khoảng bao nhiêu. Với trung bình, có sẵn một công
thức giải tích để đối chiếu (`độ_lệch_chuẩn / √n`) — bài này verify bootstrap
bằng cách so nó với công thức đó. Nhưng sức mạnh thật của bootstrap nằm ở
chỗ nó áp dụng được cho MỌI thống kê, kể cả những thống kê không có công
thức giải tích gọn (trung vị, hệ số tương quan, ...).

`np.random.default_rng(seed).integers(0, n, size=n)` sinh ra `n` chỉ số
NGẪU NHIÊN, mỗi chỉ số nằm trong khoảng `[0, n)`, và CHO PHÉP lặp lại — đúng
định nghĩa "lấy mẫu có hoàn lại".
::::

::::example{#uoc-luong-sai-so-chuan-bang-bootstrap}
Mười hai số đo thời gian phản hồi (ms). Trung bình gốc, sai số chuẩn tính
bằng công thức giải tích, rồi `1000` mẫu bootstrap:

```python title=readonly
import numpy as np

do_tre = np.array([120, 135, 98, 142, 110, 128, 155, 101, 133, 119, 145, 108], dtype=float)
n = len(do_tre)

tb_goc = do_tre.mean()
se_giai_tich = do_tre.std(ddof=1) / np.sqrt(n)
print("trung binh goc:", tb_goc)
print("sai so chuan (cong thuc giai tich):", round(se_giai_tich, 4))

rng = np.random.default_rng(42)
SO_MAU = 1000
tb_bootstrap = np.zeros(SO_MAU)
for i in range(SO_MAU):
    idx = rng.integers(0, n, size=n)
    mau = do_tre[idx]
    tb_bootstrap[i] = mau.mean()

print("trung binh cua 1000 tb_bootstrap:", round(tb_bootstrap.mean(), 4))
print("do lech chuan cua 1000 tb_bootstrap (SE uoc luong):", round(tb_bootstrap.std(), 4))
```

```text title=readonly
trung binh goc: 124.5
sai so chuan (cong thuc giai tich): 5.2505
trung binh cua 1000 tb_bootstrap: 124.6757
do lech chuan cua 1000 tb_bootstrap (SE uoc luong): 5.2836
```

`5.2836` (bootstrap) so với `5.2505` (công thức giải tích) — hai con số RẤT
gần nhau, dù bootstrap không hề dùng công thức `std/√n` nào cả — nó chỉ lấy
mẫu lại nhiều lần rồi đo độ trải của kết quả. Trung bình của `1000` giá trị
bootstrap (`124.6757`) cũng rất gần trung bình gốc (`124.5`) — bootstrap
không làm lệch ước lượng trung tâm, nó chỉ cho thêm thông tin về ĐỘ KHÔNG
CHẮC CHẮN quanh ước lượng đó.
::::

::::predict{#doan-neu-doi-thong-ke commitOnce}
Vẫn mười hai số đo trên. Giả sử, thay vì tính TRUNG BÌNH của mỗi mẫu
bootstrap, Byte tính TRUNG VỊ (median) của mỗi mẫu — một thống kê KHÔNG có
công thức giải tích gọn cho sai số chuẩn của nó.

**Trước khi đọc tiếp**, bạn đoán: quy trình bootstrap (lấy mẫu có hoàn lại,
lặp `1000` lần, đo độ lệch chuẩn giữa các kết quả) có còn áp dụng được để
ước lượng độ không chắc chắn của trung vị không?

:::opt{correct}
Có — bootstrap không hề dựa vào công thức riêng của trung bình; nó chỉ cần
TÍNH LẠI bất kỳ thống kê nào trên mỗi mẫu rồi đo độ trải, nên đổi sang trung
vị chỉ cần đổi HÀM tính thống kê, quy trình lấy mẫu giữ nguyên
:::

:::opt
Không — bootstrap chỉ được định nghĩa cho trung bình, vì công thức
`std/√n` chỉ áp dụng cho trung bình
::why
Gần đúng ở việc bạn nhớ đúng: công thức `std/√n` đúng là CHỈ áp dụng cho
trung bình — không có công thức tương tự gọn cho trung vị.

Chỗ lệch: bootstrap KHÔNG PHẢI là công thức `std/√n` — công thức đó chỉ
được dùng ở bài này để ĐỐI CHIẾU, xác nhận bootstrap ước lượng đúng. Bản
thân quy trình bootstrap (lấy mẫu có hoàn lại, tính lại thống kê, đo độ
trải) không hề nhắc tới công thức nào của trung bình — nó áp dụng y hệt cho
trung vị, chỉ cần đổi `mau.mean()` thành `np.median(mau)`.
::
:::

:::opt
Có, nhưng phải tăng số mẫu bootstrap lên rất nhiều (ví dụ một triệu mẫu),
vì trung vị "khó tính" hơn trung bình
::why
Gần đúng ở trực giác "một số thống kê phức tạp hơn có thể cần thêm mẫu để
ổn định" — một cân nhắc thực tế đôi khi đúng.

Chỗ lệch: đây không phải một yêu cầu BẮT BUỘC của riêng trung vị. Số mẫu
bootstrap cần thiết phụ thuộc vào việc ước lượng có ổn định hay không nói
chung (cả trung bình lẫn trung vị đều có thể cần nhiều hơn hoặc ít hơn tuỳ
dữ liệu), không phải một luật riêng "trung vị luôn cần nhiều mẫu hơn hẳn".
`1000` mẫu, cùng mức đã dùng cho trung bình, vẫn là một điểm khởi đầu hợp lý
cho trung vị.
::
:::
::::

::::code{#uoc_luong_bootstrap}
Hoàn thiện vòng lặp bootstrap: lấy `n` chỉ số ngẫu nhiên CÓ HOÀN LẠI, lấy ra
mẫu tương ứng, rồi tính trung bình của mẫu đó.

```python title=starter
import numpy as np

do_tre = np.array([120, 135, 98, 142, 110, 128, 155, 101, 133, 119, 145, 108], dtype=float)
n = len(do_tre)

tb_goc = do_tre.mean()
se_giai_tich = do_tre.std(ddof=1) / np.sqrt(n)

rng = np.random.default_rng(42)
SO_MAU = 1000
tb_bootstrap = np.zeros(SO_MAU)
for i in range(SO_MAU):
    idx = ___                          # rng.integers(0, n, size=n) -- CO HOAN LAI
    mau = ___                          # do_tre[idx]
    tb_bootstrap[i] = ___              # mau.mean()

print(round(tb_goc, 4))
print(round(se_giai_tich, 4))
print(round(tb_bootstrap.mean(), 4))
print(round(tb_bootstrap.std(), 4))
```

```python title=solution
import numpy as np

do_tre = np.array([120, 135, 98, 142, 110, 128, 155, 101, 133, 119, 145, 108], dtype=float)
n = len(do_tre)

tb_goc = do_tre.mean()
se_giai_tich = do_tre.std(ddof=1) / np.sqrt(n)

rng = np.random.default_rng(42)
SO_MAU = 1000
tb_bootstrap = np.zeros(SO_MAU)
for i in range(SO_MAU):
    idx = rng.integers(0, n, size=n)
    mau = do_tre[idx]
    tb_bootstrap[i] = mau.mean()

print(round(tb_goc, 4))
print(round(se_giai_tich, 4))
print(round(tb_bootstrap.mean(), 4))
print(round(tb_bootstrap.std(), 4))
```

```python title=test
assert tb_goc == 124.5, f"trung binh goc phai la 124.5 -- dang ra {tb_goc}"
assert round(se_giai_tich, 4) == 5.2505, f"sai so chuan giai tich phai la 5.2505 -- dang ra {round(se_giai_tich, 4)}"
assert round(tb_bootstrap.mean(), 4) == 124.6757, f"trung binh cua tb_bootstrap phai la 124.6757 -- dang ra {round(tb_bootstrap.mean(), 4)}"
assert round(tb_bootstrap.std(), 4) == 5.2836, f"do lech chuan cua tb_bootstrap phai la 5.2836 -- dang ra {round(tb_bootstrap.std(), 4)}"
assert np.isclose(tb_bootstrap.std(), se_giai_tich, atol=0.1), "do lech chuan bootstrap phai GAN voi sai so chuan giai tich (sai lech duoi 0.1)"
assert len(tb_bootstrap) == 1000, "phai co dung 1000 gia tri tb_bootstrap"
```

:::hints
- kind: attention
  body: Ba chỗ trống theo đúng thứ tự của một lần lấy mẫu bootstrap. `idx` là `n` chỉ số ngẫu nhiên, CÓ HOÀN LẠI, trong khoảng `[0, n)` — dùng `rng.integers(0, n, size=n)`. `mau` là các phần tử của `do_tre` tại những chỉ số đó — `do_tre[idx]`. Dòng cuối lưu trung bình của MẪU đó (không phải của `do_tre` gốc) vào `tb_bootstrap[i]`.
- kind: strategy
  body: 'idx: `rng.integers(0, n, size=n)`. mau: `do_tre[idx]`. tb_bootstrap[i]: `mau.mean()`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `rng.integers(0, n, size=n)`, `do_tre[idx]`, và `mau.mean()`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: idx phai goi THAT rng.integers(0, n, size=n) (lay mau CO HOAN LAI, khong duoc dung np.arange hay mot day co dinh); mau phai lay tu do_tre bang idx; tb_bootstrap[i] phai la mau.mean(), khong phai do_tre.mean()
  requireAst:
  - kind: uses-call, target: integers, min: 1
  - kind: uses-name, target: idx, min: 1
  - kind: uses-name, target: mau, min: 1
  - kind: uses-name, target: do_tre, min: 4
  # Da thu that (goi kiemAst that tren code day du): loi giai dung dat=true,
  # ca bon luat qua sach. Cheat thay idx = np.arange(n) (khong con goi
  # integers, mat het tinh ngau nhien) lam luat "integers" tut ve 0 -- bi
  # chan; chay THAT cheat nay cho tb_bootstrap.std() = 0.0 (khac han 5.2836
  # ky vong), cung bi chan boi tier tests doc lap.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^124\\.5\\n5\\.2505\\n124\\.6757\\n5\\.2836\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`5.2836` so với `5.2505` — không hề gọi công thức giải tích nào, chỉ lấy
mẫu lại một nghìn lần rồi đo độ trải. Cùng một bộ dữ liệu, đo được cả trung
tâm LẪN độ không chắc chắn quanh nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bootstrap vừa dùng để ước lượng ĐỘ KHÔNG CHẮC CHẮN của MỘT con số thống kê
(trung bình). Nhưng ý tưởng "lấy mẫu có hoàn lại từ cùng dữ liệu, nhiều
lần" có dùng được cho việc khác không — chẳng hạn, thay vì chỉ đo, dùng
MỖI mẫu bootstrap để HUẤN LUYỆN một mô hình riêng, rồi kết hợp nhiều mô
hình đó lại?

Bài sau trả lời — và nối thẳng lại với câu hỏi mà bài `bias-variance-
tradeoff` để lại: có cách nào làm variance của một mô hình thấp xuống,
không đổi bản thân mô hình?
::::

::::checkpoint{mastery=0.8}
::::
