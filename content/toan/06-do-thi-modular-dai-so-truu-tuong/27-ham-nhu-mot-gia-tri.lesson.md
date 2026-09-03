---
id: toan.do-thi-modular-dai-so-truu-tuong.ham-nhu-mot-gia-tri
title: Hàm như một giá trị
summary: "Hàm LÀ một GIÁ TRỊ — có thể GÁN cho biến, TRUYỀN làm đối số, TRẢ VỀ từ hàm KHÁC — bài NÀY đặt TÊN toán học: LAMBDA CALCULUS; hợp thành ánh xạ (T2.4 bài 27) LÀ MỘT phép toán HAI ngôi TRÊN tập các HÀM."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 27
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.function-as-value]
requires: [math.modular-group-example, math.associativity, math.relation-composition]
concepts: [math.ham-nhu-gia-tri]
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
Hợp thành ánh xạ (`f∘g`, T2.4 bài 27) — tập TẤT CẢ song ánh TỪ một
tập HỮU hạn VÀO chính nó, VỚI phép hợp thành — có phải MỘT nhóm
không?
::::

::::explain{#ham-nhu-mot-gia-tri}
CÓ. **Hàm LÀ một GIÁ TRỊ** — CÓ thể GÁN cho biến, TRUYỀN làm đối số,
TRẢ VỀ từ hàm KHÁC (đã dùng NHIỀU lần, `def`/`lambda` Python — bài
NÀY đặt TÊN toán học: **lambda calculus**, nền TOÁN của lập trình
HÀM). Hợp thành ánh xạ (T2.4 bài 27) LÀ MỘT phép toán HAI ngôi TRÊN
tập các HÀM — VÀ tập TẤT CẢ song ánh TỪ `{0,1,2}` vào CHÍNH nó (biểu
diễn MỖI song ánh bằng MỘT tuple, `(1,0,2)` nghĩa LÀ `f(0)=1, f(1)=0,
f(2)=2`), VỚI phép hợp thành, LÀ một nhóm:

```python title=readonly
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def la_nhom(tap, phep_toan):
    if not la_dong(tap, phep_toan):
        return False
    if not la_ket_hop(tap, phep_toan):
        return False
    e = tim_don_vi(tap, phep_toan)
    if e is None:
        return False
    return all(tim_nghich_dao(tap, phep_toan, e, a) is not None for a in tap)

def hop_thanh(f, g):
    return tuple(f[g[x]] for x in range(3))


S3 = {(0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0)}
print(la_nhom(S3, hop_thanh))
```

```text title=readonly
True
```

`S3` (SÁU song ánh CÓ thể có TRÊN `{0,1,2}`) VỚI phép hợp THÀNH —
ĐÓNG (hợp hai song ánh RA một song ánh), kết HỢP, CÓ đơn vị
(`(0,1,2)` — ánh xạ ĐỒNG NHẤT, KHÔNG đổi gì), MỌI song ánh CÓ nghịch
đảo — ĐỦ cả bốn, LÀ một nhóm.
::::

::::example{#khong-giao-hoan}
NHƯNG hợp thành KHÔNG giao HOÁN — thứ TỰ ghép quan TRỌNG:

```python title=readonly
def hop_thanh(f, g):
    return tuple(f[g[x]] for x in range(3))


f = (1, 0, 2)
g = (0, 2, 1)
print(hop_thanh(f, g))
print(hop_thanh(g, f))
```

```text title=readonly
(1, 2, 0)
(2, 0, 1)
```

`f∘g` VÀ `g∘f` RA HAI kết quả KHÁC nhau — kết HỢP (bài 22, thứ tự
NHÓM ngoặc không quan TRỌNG) HOÀN TOÀN KHÁC giao hoán (thứ tự ĐẶT
hai phần tử không quan TRỌNG) — `S3` kết hợp NHƯNG KHÔNG giao hoán.
::::

::::predict{#doan-nghich-dao-hoan-vi commitOnce}
Byte tìm nghịch đảo của song ánh `(1,2,0)` TRONG `S3`:

```python
def la_dong(tap, phep_toan):
    return all(phep_toan(a, b) in tap for a in tap for b in tap)

def la_ket_hop(tap, phep_toan):
    return all(phep_toan(phep_toan(a, b), c) == phep_toan(a, phep_toan(b, c))
               for a in tap for b in tap for c in tap)

def tim_don_vi(tap, phep_toan):
    for e in tap:
        if all(phep_toan(e, a) == a and phep_toan(a, e) == a for a in tap):
            return e
    return None

def tim_nghich_dao(tap, phep_toan, e, a):
    for x in tap:
        if phep_toan(a, x) == e and phep_toan(x, a) == e:
            return x
    return None

def hop_thanh(f, g):
    return tuple(f[g[x]] for x in range(3))

S3 = {(0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0)}
e = tim_don_vi(S3, hop_thanh)
print(tim_nghich_dao(S3, hop_thanh, e, (1, 2, 0)))
```

Dòng cuối in ra gì?

:::opt{correct}
`(2, 0, 1)`
:::

:::opt
`(0, 2, 1)` — vì đảo THỨ TỰ CÁC số trong tuple `(1,2,0)` (đọc NGƯỢC)
sẽ RA `(0,2,1)`, một CÁCH "lật ngược" TỰ nhiên để tìm nghịch đảo
::why
Gần đúng ở việc bạn nghĩ tới việc "ĐẢO ngược" `(1,2,0)` — một trực
giác HÌNH THỨC dễ hiểu NHẦM với nghịch đảo THẬT.

Chỗ lệch: nghịch đảo của một SONG ÁNH KHÔNG phải "đọc NGƯỢC dãy số"
— nó LÀ song ánh `x` sao `hop_thanh((1,2,0), x)` VÀ `hop_thanh(x,
(1,2,0))` ĐỀU RA đơn vị `(0,1,2)`. `(1,2,0)` GỬI `0→1, 1→2, 2→0`;
nghịch đảo PHẢI gửi NGƯỢC LẠI: `1→0, 2→1, 0→2`, TỨC LÀ `0→2, 1→0,
2→1` — viết thành tuple ĐÚNG thứ TỰ (`f(0),f(1),f(2)`) LÀ `(2,0,1)`,
KHÔNG phải `(0,2,1)`.
::
:::

:::opt
Máy báo lỗi khi chạy — `tim_nghich_dao` cần THAM số `e` (đơn vị),
NHƯNG `e` được TÍNH bằng `tim_don_vi` Ở dòng TRƯỚC, Python KHÔNG cho
phép dùng LẠI một biến ĐÃ tính TỪ hàm KHÁC làm ĐỐI số
::why
Gần đúng ở việc bạn để ý ĐÚNG `e` được TÍNH TRƯỚC RỒI mới dùng — một
quan sát VỀ trình tự MÃ.

Chỗ lệch: dùng KẾT quả của một LỆNH GỌI hàm làm ĐỐI số cho LỆNH gọi
KHÁC LÀ hoàn toàn BÌNH thường trong Python — KHÔNG có RÀNG buộc nào
cấm điều ĐÓ. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_hop_thanh}
Viết `hop_thanh(f, g)` — hợp THÀNH hai song ánh `f`, `g` trên
`{0,1,2}` (`(f∘g)(x) = f(g(x))`).

```python title=starter
def hop_thanh(f, g):
    return ___


f = (1, 0, 2)
g = (0, 2, 1)
print(hop_thanh(f, g))
```

```python title=solution
def hop_thanh(f, g):
    return tuple(f[g[x]] for x in range(3))


f = (1, 0, 2)
g = (0, 2, 1)
print(hop_thanh(f, g))
```

```python title=test
dong_nhat = (0, 1, 2)
assert hop_thanh(dong_nhat, (1, 2, 0)) == (1, 2, 0), "hop voi dong nhat -- khong doi"
f = (1, 0, 2)
g = (0, 2, 1)
assert hop_thanh(f, g) == (1, 2, 0), "f hop g"
assert hop_thanh(g, f) == (2, 0, 1), "g hop f -- khac f hop g"
assert hop_thanh(f, g) != hop_thanh(g, f), "hop thanh khong giao hoan"
```

:::hints
- kind: attention
  body: "(f hop g)(x) = f(g(x)) -- tinh g[x] truoc, roi dua vao f."
- kind: strategy
  body: "tuple(f[g[x]] for x in range(3))"
- kind: one-line
  body: "___ = tuple(f[g[x]] for x in range(3))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai tinh f[g[x]] cho moi x, dung tuple va range
  requireAst:
  - kind: uses-call, target: tuple, min: 1
  - kind: uses-call, target: range, min: 1
  - kind: uses-name, target: g, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^\(1, 2, 0\)\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đồ thị, modular, đại số trừu tượng — BA chủ đề, MỘT ý tưởng cốt lõi.
Sẵn sàng ghép TẤT CẢ lại trên MỘT sơ đồ tưới hoàn chỉnh chưa?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track NÀY đóng bằng việc NHÌN lại đồ thị VÀ modular QUA lăng kính
nhóm. Bậc, liên thông, đồng dư, nghịch đảo — TẤT CẢ gặp NHAU Ở đâu
trên MỘT sơ đồ tưới hoàn chỉnh?
::::

::::checkpoint{mastery=0.8}
::::
