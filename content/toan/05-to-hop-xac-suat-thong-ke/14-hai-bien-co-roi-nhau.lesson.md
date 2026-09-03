---
id: toan.to-hop-xac-suat-thong-ke.hai-bien-co-roi-nhau
title: Hai biến cố rời nhau
summary: "P(A∪B) = P(A) + P(B) khi A ∩ B = ∅ (T2.4 bài 10) — ứng trực tiếp quy tắc cộng (bài 1) lên xác suất."
locale: vi
track: toan
module: to-hop-xac-suat-thong-ke
order: 14
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [math.probability-disjoint-union]
requires: [math.complement-probability]
concepts: [math.xac-suat-roi-nhau]
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
Cà chua và xà lách — rời nhau (một hạt không thể là cả hai). Cộng
thẳng hai xác suất có ra đúng "rút được cà chua HOẶC xà lách" không?
::::

::::explain{#xac-suat-roi-nhau}
Có. **`P(A∪B) = P(A) + P(B)`** khi `A ∩ B = ∅` (T2.4 bài 10) — ứng
TRỰC TIẾP quy tắc cộng (bài 1) lên xác suất:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop_roi_nhau(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(xac_suat_hop_roi_nhau(ca_chua, xa_lach, omega))
```

```text title=readonly
0.8
```

`ca_chua` VÀ `xa_lach` KHÔNG chung hạt nào (`ca_chua ∩ xa_lach = ∅`)
— cộng thẳng `0.5+0.3=0.8` là xác suất "rút được cà chua HOẶC xà
lách".
::::

::::example{#kiem-lai-bang-hop-that}
Kiểm lại BẰNG cách tính xác suất trên HỢP thật của hai tập — CÙNG
kết quả:

```python title=readonly
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop_roi_nhau(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(xac_suat_hop_roi_nhau(ca_chua, xa_lach, omega))
print(la_xac_suat(ca_chua | xa_lach, omega))
```

```text title=readonly
0.8
0.8
```

Cộng thẳng HAI xác suất (bài này, KHÔNG cần dựng hợp) VÀ tính xác
suất TRÊN hợp thật (`ca_chua | xa_lach`, cần dựng hợp trước) — RA
CÙNG một số. Rời nhau thì KHÔNG cần dựng hợp mới tính được.
::::

::::predict{#doan-roi-nhau-chong-lan commitOnce}
Byte thử HAI nhãn CHỒNG lấn (bài 1: "giống mới" `h1,h2,h3`, "chịu
hạn" `h2,h3,h4` — `h2`, `h3` mang CẢ HAI) thay vì hai loại rau:

```python
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop_roi_nhau(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega)

omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
giong_moi = {"h1", "h2", "h3"}
chiu_han = {"h2", "h3", "h4"}

print(xac_suat_hop_roi_nhau(giong_moi, chiu_han, omega))
print(la_xac_suat(giong_moi | chiu_han, omega))
```

Hai dòng cuối in ra gì?

:::opt{correct}
`0.6`, rồi `0.4`
:::

:::opt
`0.6`, rồi `0.6` — công thức bài này LUÔN khớp đúng xác suất hợp
THẬT, bất kể hai biến cố có rời nhau hay không
::why
Gần đúng ở việc bạn tính ĐÚNG dòng đầu `0.3+0.3=0.6` — phép cộng
thẳng luôn ra `0.6`, không sai.

Chỗ lệch: `giong_moi` VÀ `chiu_han` KHÔNG rời nhau (`h2`, `h3` chung
CẢ HAI) — công thức bài này (`P(A)+P(B)`) CHỈ đúng khi rời nhau,
đúng như bài 1 (T2.4) đã cảnh báo Ở phần đếm. Hợp THẬT của hai nhãn
chỉ có bốn hạt (`h1,h2,h3,h4`), nên `P` của nó LÀ `4/10=0.4`, không
phải `0.6` — cộng thẳng đã đếm `h2`, `h3` HAI LẦN.
::
:::

:::opt
Máy báo lỗi khi chạy — `xac_suat_hop_roi_nhau` được ĐẶT TÊN "rời
nhau", Python kiểm tên hàm khớp với dữ liệu và từ chối chạy trên hai
tập chồng lấn
::why
Gần đúng ở việc bạn để ý CÁI TÊN hàm ("roi_nhau") gợi Ý điều kiện nó
GIẢ ĐỊNH — một quan sát tinh Ý về Ý NGHĨA đặt tên.

Chỗ lệch: Python KHÔNG hề đọc TÊN hàm để suy ra điều kiện gì — tên
hàm chỉ LÀ chữ, không mang RÀNG BUỘC thực thi nào. Hàm chạy TRÊN bất
kỳ hai tập nào được đưa vào, đúng hay sai VỀ MẶT toán học tuỳ người
gọi tự chịu trách nhiệm kiểm trước.
::
:::
::::

::::code{#viet_xac_suat_hop_roi_nhau}
Viết `xac_suat_hop_roi_nhau(a, b, omega)` — cộng thẳng `P(a)` VÀ
`P(b)`, áp dụng khi `a`, `b` rời nhau.

```python title=starter
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop_roi_nhau(a, b, omega):
    return ___


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(xac_suat_hop_roi_nhau(ca_chua, xa_lach, omega))
```

```python title=solution
def la_xac_suat(a, omega):
    return len(a) / len(omega)

def xac_suat_hop_roi_nhau(a, b, omega):
    return la_xac_suat(a, omega) + la_xac_suat(b, omega)


omega = {"h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10"}
ca_chua = {"h1", "h2", "h3", "h4", "h5"}
xa_lach = {"h6", "h7", "h8"}

print(xac_suat_hop_roi_nhau(ca_chua, xa_lach, omega))
```

```python title=test
assert xac_suat_hop_roi_nhau(set(), set(), omega) == 0.0, "hai tap rong -- tong 0"
ca_rot = {"h9", "h10"}
assert xac_suat_hop_roi_nhau(xa_lach, ca_rot, omega) == 0.5, "0.3 + 0.2 = 0.5"
assert xac_suat_hop_roi_nhau(ca_chua, xa_lach, omega) == la_xac_suat(ca_chua | xa_lach, omega), "khi roi nhau, cong thang phai khop hop that"
```

:::hints
- kind: attention
  body: "Cong THANG la_xac_suat(a, omega) voi la_xac_suat(b, omega), khong dung hop truoc."
- kind: strategy
  body: "la_xac_suat(a, omega) + la_xac_suat(b, omega)"
- kind: one-line
  body: "___ = la_xac_suat(a, omega) + la_xac_suat(b, omega)"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai cong THANG la_xac_suat(a, omega) va la_xac_suat(b, omega)
  requireAst:
  - kind: uses-call, target: la_xac_suat, min: 2
  - kind: uses-operator, target: '+', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^0\.8\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rời nhau thì cộng thẳng, chồng lấn thì sai âm thầm — y hệt bài 1.
Sửa được cho xác suất giống như đã sửa cho đếm không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte kiểm: nếu `A` và `B` rời nhau, `P(A∩B) = P(∅) = 0`. Có công
thức nào cho `P(A∪B)` ĐÚNG trong CẢ hai trường hợp — rời nhau LẪN
chồng lấn — mà khi rời nhau thì TỰ RÚT GỌN về công thức bài này
không?
::::

::::checkpoint{mastery=0.8}
::::
