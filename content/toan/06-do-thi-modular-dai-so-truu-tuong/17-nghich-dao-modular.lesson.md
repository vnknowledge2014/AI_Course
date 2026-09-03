---
id: toan.do-thi-modular-dai-so-truu-tuong.nghich-dao-modular
title: Nghịch đảo modular
summary: "a⁻¹ mod n — số x sao cho (a×x) mod n = 1; TỒN TẠI ⟺ gcd(a,n)=1 (a VÀ n NGUYÊN TỐ CÙNG NHAU) — thay THẾ được PHÉP CHIA modular (phép chia THƯỜNG không có Ý nghĩa TRONG modular)."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.modular-inverse]
requires: [math.gcd-euclid, math.modular-arithmetic]
concepts: [math.nghich-dao-modular]
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
`3` có nghịch đảo modulo `7` không? Thử NHÂN `3` VỚI từng số TỪ `1`
tới `6`, lấy dư CHO `7` — số NÀO cho kết quả `1`?
::::

::::explain{#nghich-dao-modular-la-gi}
CÓ — VÀ số ĐÓ LÀ `5`. **`a⁻¹ mod n`** — số `x` SAO cho `(a×x) mod n
= 1`; TỒN TẠI ⟺ `gcd(a,n)=1` (`a` VÀ `n` NGUYÊN TỐ CÙNG NHAU, bài
16) — thay THẾ được PHÉP CHIA modular (phép chia THƯỜNG KHÔNG có Ý
nghĩa TRONG modular):

```python title=readonly
def nghich_dao_modular(a, n):
    for x in range(1, n):
        if (a * x) % n == 1:
            return x
    return None


print(nghich_dao_modular(3, 7))
```

```text title=readonly
5
```

`3×5=15`, `15 mod 7 = 1` — `5` LÀ nghịch đảo modular của `3` (mod
`7`). Thử LẦN LƯỢT `x=1,2,3,4` TRƯỚC ĐÓ đều KHÔNG cho dư `1`
(`3,6,2,5`) — `x=5` LÀ số ĐẦU tiên khớp.
::::

::::example{#khong-nguyen-to-cung-nhau}
Khi `gcd(a,n)≠1` — KHÔNG nghịch đảo nào tồn tại:

```python title=readonly
def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a

def nghich_dao_modular(a, n):
    for x in range(1, n):
        if (a * x) % n == 1:
            return x
    return None


print(gcd(6, 9))
print(nghich_dao_modular(6, 9))
```

```text title=readonly
3
None
```

`gcd(6,9)=3≠1` — `6` VÀ `9` KHÔNG nguyên tố CÙNG nhau. Thử HẾT `x`
TỪ `1` tới `8`, KHÔNG lần nào `(6×x) mod 9` ra `1` — vòng lặp chạy
HẾT, trả VỀ `None`.
::::

::::predict{#doan-so-nguyen-to-cung-nhau commitOnce}
Byte kiểm `gcd(5, 12)` TRƯỚC khi tìm nghịch đảo modular của `5` mod
`12`:

```python
def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a

def nghich_dao_modular(a, n):
    for x in range(1, n):
        if (a * x) % n == 1:
            return x
    return None

print(gcd(5, 12))
print(nghich_dao_modular(5, 12))
```

Dòng cuối in ra gì?

:::opt{correct}
`5`
:::

:::opt
`None` — vì `12` LÀ hợp số (KHÔNG phải số nguyên TỐ), nên MỌI phép
tìm nghịch đảo modulo `12` ĐỀU thất bại, giống hệt trường hợp `mod 9`
Ở VÍ dụ TRƯỚC
::why
Gần đúng ở việc bạn nhớ ĐÚNG ví dụ TRƯỚC (`mod 9`) thất bại — một
liên hệ HỢP lý VỚI ví dụ VỪA thấy.

Chỗ lệch: `n` KHÔNG cần LÀ số nguyên TỐ để nghịch đảo TỒN tại — CHỈ
cần `gcd(a,n)=1` (nguyên tố CÙNG nhau, KHÔNG phải BẢN thân `n` là
nguyên tố). `gcd(5,12)=1` (`5` VÀ `12` KHÔNG chia hết CHO ước chung
nào NGOÀI `1`, dù `12=2²×3` LÀ hợp SỐ) — nghịch đảo VẪN tồn tại. Ca
`mod 9` thất bại VÌ `gcd(6,9)=3≠1`, KHÔNG PHẢI vì `9` LÀ hợp số.
::
:::

:::opt
`0` — vì `5` LÀ số LẺ, VÀ `12` LÀ số CHẴN, tích của một số LẺ VÀ một
số CHẴN LUÔN chia hết cho `12`, cho dư `0`
::why
Gần đúng ở việc bạn để ý ĐÚNG `5` lẻ VÀ `12` chẵn — một quan sát VỀ
TÍNH chẵn/lẻ.

Chỗ lệch: "lẻ nhân chẵn" KHÔNG hề đảm bảo chia HẾT cho `12` — vd
`5×1=5`, KHÔNG chia hết cho `12` (`5 mod 12 = 5`, không phải `0`).
Hàm tìm ĐÚNG số `x` sao `(5×x) mod 12 = 1` (KHÔNG phải `0`), VÀ số
`x=5` thoả: `5×5=25`, `25 mod 12 = 1`.
::
:::
::::

::::code{#viet_nghich_dao_modular}
Viết PHẦN CÒN LẠI của `nghich_dao_modular(a, n)` — kiểm `x` có LÀ
nghịch đảo modular của `a` (mod `n`) không.

```python title=starter
def nghich_dao_modular(a, n):
    for x in range(1, n):
        if ___:
            return x
    return None


print(nghich_dao_modular(3, 7))
```

```python title=solution
def nghich_dao_modular(a, n):
    for x in range(1, n):
        if (a * x) % n == 1:
            return x
    return None


print(nghich_dao_modular(3, 7))
```

```python title=test
assert nghich_dao_modular(3, 7) == 5, "3x5 mod 7 = 1"
assert nghich_dao_modular(1, 7) == 1, "1 la nghich dao cua chinh no"
assert nghich_dao_modular(6, 7) == 6, "6x6=36, 36 mod 7 = 1"
assert nghich_dao_modular(4, 8) is None, "gcd(4,8)=4 -- khong nguyen to cung nhau, khong co nghich dao"
assert nghich_dao_modular(5, 12) == 5, "5x5=25, 25 mod 12 = 1"
```

:::hints
- kind: attention
  body: "Kiem (a * x) mod n co bang 1 khong -- dung dung dinh nghia nghich dao modular."
- kind: strategy
  body: "(a * x) % n == 1"
- kind: one-line
  body: "if (a * x) % n == 1:"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai kiem (a * x) % n == 1
  requireAst:
  - kind: uses-operator, target: '*', min: 1
  - kind: uses-operator, target: '%', min: 1
  - kind: uses-operator, target: '==', min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^5\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nghịch đảo modular THAY thế phép chia. Nếu `p` LÀ số nguyên TỐ, `2^100
mod 101` — có cách nào TÍNH nhanh mà KHÔNG cần nhân LẶP một trăm lần
không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`2¹⁰⁰ mod 101` — nhân LẶP một trăm LẦN thì CHẬM. Có cách nào tính
NHANH mà KHÔNG cần nhân đủ MỘT trăm lần, KHI `101` LÀ số nguyên TỐ
không?
::::

::::checkpoint{mastery=0.8}
::::
