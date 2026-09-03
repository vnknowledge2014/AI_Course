---
id: toan.do-thi-modular-dai-so-truu-tuong.bai-toan-bay-cay-cau
title: Bài toán bảy cây cầu
summary: "Bài toán Königsberg — đi qua ĐỦ bảy cây cầu, MỖI cầu ĐÚNG một lần, rồi VỀ điểm xuất phát — Euler (1736) chứng minh KHÔNG THỂ bằng cách NHÌN vào bậc các vùng đất, không thử hết đường đi (T2.3 bài 21)."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 9
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [math.konigsberg-problem]
requires: [math.vertex-degree, logic.finite-check-not-proof]
concepts: [math.bai-toan-konigsberg]
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
Bảy cây cầu, bốn vùng đất — người dân Königsberg MUỐN đi dạo qua ĐỦ
cả bảy cầu, MỖI cầu ĐÚNG một lần, RỒI về nhà. Có ai LÀM được chưa?
::::

::::explain{#bai-toan-konigsberg}
CHƯA ai làm được — VÀ năm 1736, Euler CHỨNG MINH điều ĐÓ LÀ KHÔNG
THỂ. Ông KHÔNG thử hết mọi đường đi CÓ thể (T2.3 bài 21: kiểm hữu
hạn KHÔNG phải chứng minh, dù thử BAO nhiêu lần) — ông NHÌN vào
**bậc** (bài 2) của bốn vùng đất: `A=5, B=3, C=3, D=3` (mỗi cây cầu
NỐI hai vùng, TÍNH gộp cả cầu ĐÔI giữa cùng một cặp vùng). Ông ĐẾM
xem CÓ bao nhiêu vùng đất có bậc LẺ:

```python title=readonly
def dem_dinh_bac_le(bac):
    return sum(1 for v in bac if bac[v] % 2 == 1)


bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}

print(dem_dinh_bac_le(bac_konigsberg))
```

```text title=readonly
4
```

CẢ BỐN vùng đất ĐỀU bậc LẺ — KHÔNG một vùng nào bậc chẵn. Đây chính
LÀ manh mối Euler dùng để CHỨNG minh sự BẤT LỰC (bài 11 sẽ nói RÕ VÌ
SAO số NÀY quyết định TẤT CẢ).
::::

::::example{#tong-bac-van-chan}
Dù TỪNG vùng đất bậc LẺ, TỔNG bậc vẫn CHẴN — ĐÚNG bổ đề bắt tay (bài
3):

```python title=readonly
def dem_dinh_bac_le(bac):
    return sum(1 for v in bac if bac[v] % 2 == 1)


bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}

print(sum(bac_konigsberg.values()))
```

```text title=readonly
14
```

`5+3+3+3=14`, một số CHẴN — bổ đề bắt tay KHÔNG hề bị VI PHẠM. Bốn
số LẺ cộng lại VẪN ra số chẵn (LẺ+LẺ=CHẴN, gộp HAI cặp), ĐÚNG vì SỐ
LƯỢNG vùng bậc LẺ (bốn) LÀ một số CHẴN — KHÔNG PHẢI trùng hợp: số
đỉnh bậc lẻ trong BẤT KỲ đồ thị nào LUÔN LÀ số chẵn.
::::

::::predict{#doan-bot-mot-cau commitOnce}
Byte tưởng tượng BỚT một cây cầu GIỮA vùng `C` VÀ `D` — bậc của CẢ
HAI vùng đó GIẢM đúng `1`:

```python
def dem_dinh_bac_le(bac):
    return sum(1 for v in bac if bac[v] % 2 == 1)

bac_bot_cau = {"A": 5, "B": 3, "C": 2, "D": 2}
print(dem_dinh_bac_le(bac_bot_cau))
```

Dòng cuối in ra gì?

:::opt{correct}
`2`
:::

:::opt
`4` — vì BỚT một cây cầu KHÔNG đổi TÍNH lẻ/chẵn của bậc "một cách hệ
thống", nên VẪN còn đủ bốn vùng bậc lẻ NHƯ cũ
::why
Gần đúng ở việc bạn nghĩ tính LẺ/chẵn LÀ một thứ "ổn định", ít khi
đổi — một trực giác dễ hiểu NHƯNG bỏ SÓT phép TÍNH cụ thể.

Chỗ lệch: bớt MỘT cây cầu LÀM giảm bậc của CẢ HAI đầu mút đúng `1`
— `C` từ `3` (LẺ) XUỐNG `2` (CHẴN), `D` từ `3` (LẺ) XUỐNG `2` (CHẴN).
HAI vùng ĐÓ đổi TỪ lẻ sang chẵn, CHỈ CÒN `A` VÀ `B` LÀ lẻ — đếm LẠI
ra `2`, KHÔNG còn `4`.
::
:::

:::opt
Máy báo lỗi khi chạy — bậc của `C` VÀ `D` giờ BẰNG nhau (`2` VÀ `2`),
mà một `dict` Python KHÔNG cho phép HAI khoá khác nhau có CÙNG giá
trị
::why
Gần đúng ở việc bạn để ý ĐÚNG `C` VÀ `D` giờ cùng bậc `2` — một quan
sát VỀ dữ liệu.

Chỗ lệch: `dict` KHÔNG hề cấm hai khoá khác nhau CÓ chung một giá
trị — CHỈ khoá (`"C"`, `"D"`) mới cần DUY NHẤT, giá trị (bậc) hoàn
toàn CÓ thể trùng nhau. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_dem_dinh_bac_le}
Viết `dem_dinh_bac_le(bac)` — đếm số vùng đất (khoá của `bac`) có
bậc LẺ.

```python title=starter
def dem_dinh_bac_le(bac):
    return ___


bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}
print(dem_dinh_bac_le(bac_konigsberg))
```

```python title=solution
def dem_dinh_bac_le(bac):
    return sum(1 for v in bac if bac[v] % 2 == 1)


bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}
print(dem_dinh_bac_le(bac_konigsberg))
```

```python title=test
assert dem_dinh_bac_le({}) == 0, "khong vung dat nao -- dem ra 0"
assert dem_dinh_bac_le({"x": 2}) == 0, "bac chan -- khong dem"
assert dem_dinh_bac_le({"x": 3}) == 1, "bac le -- dem dung mot"
bac_konigsberg = {"A": 5, "B": 3, "C": 3, "D": 3}
assert dem_dinh_bac_le(bac_konigsberg) == 4, "ca bon vung deu bac le"
bac_bot_cau = {"A": 5, "B": 3, "C": 2, "D": 2}
assert dem_dinh_bac_le(bac_bot_cau) == 2, "bot mot cau -- chi con hai vung bac le"
```

:::hints
- kind: attention
  body: "Dem so khoa v trong bac ma gia tri bac[v] chia 2 du 1 (le)."
- kind: strategy
  body: "sum(1 for v in bac if bac[v] % 2 == 1)"
- kind: one-line
  body: "___ = sum(1 for v in bac if bac[v] % 2 == 1)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai dem so khoa co gia tri le, dung % va ==
  requireAst:
  - kind: uses-operator, target: '%', min: 1
  - kind: uses-operator, target: '==', min: 1
  - kind: uses-name, target: bac, min: 2
  - kind: comprehension, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^4\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn vùng đất TOÀN bậc lẻ. Euler KHÔNG thử hết mọi đường đi — con số
"bốn" ĐÓ liên quan GÌ tới việc bài toán VÔ VỌNG?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Euler KHÔNG thử hết mọi đường đi (T2.3 bài 21: kiểm hữu hạn KHÔNG
phải chứng minh) — ông tìm RA một tính chất CHUNG khiến bài toán VÔ
VỌNG. Tính chất ĐÓ liên quan GÌ tới bậc của các đỉnh (bài 2) — và
liệu CÓ áp dụng được cho MỌI sơ đồ, không riêng gì Königsberg?
::::

::::checkpoint{mastery=0.8}
::::
