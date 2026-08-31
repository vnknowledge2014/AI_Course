---
id: khoa-hoc-may-tinh.may-chay-the-nao.con-tro-lenh
title: "Con trỏ lệnh: máy luôn biết đang đứng ở đâu"
summary: "Vòng lấy-hiểu-làm (bài 6) cần nhớ ĐANG Ở LỆNH NÀO — một con số gọi là con trỏ lệnh (instruction pointer), tăng lên sau mỗi lệnh thường, nhảy thẳng tới chỗ khác khi gặp lệnh NHẢY — đúng cơ chế JUMP_BACKWARD/POP_JUMP_IF_FALSE bài 4-5 đã thấy, giờ dựng bằng while thay vì for."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [may.instruction-pointer]
requires: [may.fetch-decode-execute]
concepts: [may.instruction-pointer]
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
Một con số, không hơn — nhưng chính con số ấy quyết định máy đang đứng
ở đâu, và có thể nhảy tới bất cứ chỗ nào nó muốn.
::::

::::explain{#con-so-giu-cho}
Vòng `for lenh in chuong_trinh:` ở bài trước có một giới hạn: nó luôn
đi từng phần tử THEO ĐÚNG THỨ TỰ danh sách, không thể nào NHẢY LÙI về
một phần tử đã đi qua, cũng không thể NHẢY VƯỢT qua vài phần tử. Python
tự lo việc "đi lệnh kế tiếp" giùm bạn, và bạn không có quyền can thiệp
vào đó.

Bài 4 đã cho thấy `JUMP_BACKWARD` làm đúng việc mà `for` KHÔNG làm
được: đưa vị trí đang chạy quay ngược về một lệnh đã đi qua. Muốn làm
được việc đó, vòng lặp lấy-hiểu-làm THẬT không thể dùng `for` — nó cần
một CON SỐ riêng, tự tay quản lý, ghi lại chính xác "đang đứng ở lệnh
thứ mấy". Con số ấy có tên: **con trỏ lệnh** (tiếng Anh: instruction
pointer).

Luật của con trỏ lệnh chỉ có hai:

- Sau khi làm xong một lệnh THƯỜNG (không phải lệnh nhảy), con trỏ TỰ
  TĂNG lên 1 — trỏ sang lệnh ngay kế tiếp.
- Khi lệnh vừa làm là một lệnh NHẢY (như `JUMP_BACKWARD` bài 4, hay
  `POP_JUMP_IF_FALSE` bài 5), con trỏ KHÔNG tăng lên 1 nữa — nó được
  GÁN THẲNG một giá trị khác, đúng bằng vị trí lệnh muốn nhảy tới.

Vòng lặp lấy-hiểu-làm khi đó không còn là `for`, mà là
`while con_tro < so_luong_lenh:` — chạy tới khi con trỏ vượt quá lệnh
cuối cùng.
::::

::::example{#dem-toi-ba-bang-con-tro}
Chương trình nhỏ dưới đây đếm `i` từ 0 tới 2 — làm đúng việc một
`for i in range(3)` làm, nhưng viết bằng đúng năm "lệnh máy" tự đặt
tên, có cả một lệnh nhảy CÓ ĐIỀU KIỆN (`NHAY_NEU_HET`, cùng tinh thần
`POP_JUMP_IF_FALSE` bài 5) và một lệnh nhảy LÙI (`NHAY_LUI`, cùng tinh
thần `JUMP_BACKWARD` bài 4):

```python title=readonly
chuong_trinh = [
    ("DAT", "i", 0),              # lệnh 0: i = 0
    ("NHAY_NEU_HET", "i", 3, 5),  # lệnh 1: nếu i >= 3, nhảy tới lệnh 5
    ("GHI", "i"),                  # lệnh 2: ghi i vào nhật ký
    ("TANG", "i"),                  # lệnh 3: i += 1
    ("NHAY_LUI", 1),                # lệnh 4: nhảy lùi về lệnh 1
]

bien = {}
nhat_ky = []
con_tro = 0
while con_tro < len(chuong_trinh):
    lenh = chuong_trinh[con_tro]
    ten = lenh[0]
    if ten == "DAT":
        bien[lenh[1]] = lenh[2]
        con_tro += 1
    elif ten == "NHAY_NEU_HET":
        if bien[lenh[1]] >= lenh[2]:
            con_tro = lenh[3]        # NHẢY: gán thẳng, không +1
        else:
            con_tro += 1
    elif ten == "GHI":
        nhat_ky.append(bien[lenh[1]])
        con_tro += 1
    elif ten == "TANG":
        bien[lenh[1]] += 1
        con_tro += 1
    elif ten == "NHAY_LUI":
        con_tro = lenh[1]            # NHẢY LÙI: gán thẳng, không +1

print(nhat_ky)
print(con_tro)
```

```text title=readonly
[0, 1, 2]
5
```

Con trỏ đi: `0 → 1 → 2 → 3 → 4 → 1` (nhảy lùi!) `→ 2 → 3 → 4 → 1`
(nhảy lùi lần hai) `→ 2 → 3 → 4 → 1` (nhảy lùi lần ba) `→ 5` — tới đây
`bien["i"]` đã bằng 3, `NHAY_NEU_HET` KHÔNG nhảy lùi nữa mà gán thẳng
con trỏ = 5. Vòng `while con_tro < len(chuong_trinh)` (5 lệnh, độ dài
5) kiểm `5 < 5` là sai, vòng dừng. `nhat_ky` chỉ ghi được ba lần — đúng
ba lần `GHI` chạy trước khi `i` chạm mốc dừng.

So với `for lenh in chuong_trinh:` bài trước: `while` này không đi
"lệnh kế tiếp trong danh sách" một cách mù quáng — nó đi ĐÚNG LỆNH mà
con trỏ đang trỏ tới, và con trỏ có quyền nhảy tới bất kỳ đâu.
::::

::::predict{#du-doan-con-tro commitOnce}
Đổi đúng một con số trong chương trình vừa xem: ngưỡng dừng từ `3`
thành `2` (lệnh 1 đổi thành `("NHAY_NEU_HET", "i", 2, 5)`), giữ nguyên
mọi lệnh khác.

Chạy lại chương trình đã đổi, `nhat_ky` in ra là gì?

:::opt{correct}
`[0, 1]` — chỉ hai lần GHI, vì `i` chạm ngưỡng `2` sớm hơn
:::

:::opt
`[0, 1, 2]` — vẫn ba lần GHI như cũ, vì ngưỡng dừng không ảnh hưởng số
lần GHI
::why
Gần đúng ở việc bạn nhớ đúng kết quả CŨ (ngưỡng 3) — ba lần GHI đúng là
kết quả trước khi đổi.

Chỗ lệch: `NHAY_NEU_HET` kiểm tra `bien["i"] >= nguong` MỖI LẦN con trỏ
đi qua lệnh 1. Đổi ngưỡng từ 3 xuống 2 nghĩa là điều kiện dừng bị chạm
SỚM HƠN một vòng — ngay khi `i` vừa tăng lên 2, con trỏ nhảy thẳng ra
lệnh 5, không còn dịp chạy `GHI` lần thứ ba nữa.
::
:::

:::opt
`[0, 1, 2, 3]` — bốn lần GHI, vì ngưỡng lớn hơn nghĩa là chạy lâu hơn
::why
Gần đúng ở trực giác "ngưỡng liên quan tới số lần lặp" — đúng hướng suy
luận.

Chỗ lệch: bạn đang suy luận NGƯỢC. Ngưỡng NHỎ HƠN (`2` thay vì `3`)
khiến điều kiện `i >= nguong` chạm SỚM HƠN, tức con trỏ nhảy ra khỏi
vòng SỚM HƠN — ít lần `GHI` hơn, không phải nhiều hơn.
::
:::

:::opt
Chương trình chạy mãi không dừng, vì `NHAY_LUI` luôn đưa con trỏ về
lệnh 1
::why
Gần đúng ở việc bạn để ý đúng: `NHAY_LUI` (lệnh 4) THẬT SỰ luôn đưa con
trỏ về lệnh 1, không hỏi han gì — giống `JUMP_BACKWARD` bài 4, không có
điều kiện.

Chỗ lệch: lệnh 4 không phải lệnh DUY NHẤT chạm vào con trỏ. Lệnh 1
(`NHAY_NEU_HET`) đứng NGAY SAU điểm `NHAY_LUI` quay về, và nó kiểm tra
điều kiện MỖI LẦN — khi `i` đạt ngưỡng, nó gán con trỏ = 5, THOÁT khỏi
đường quay-lại-lệnh-1-rồi-nhảy-lùi-tiếp. Vòng có điểm dừng thật, chỉ là
điểm dừng ấy nằm ở lệnh 1, không nằm ở lệnh 4.
::
:::
::::

::::code{#doc-con-tro-tren-chuong-trinh-cu}
Dưới đây là ĐÚNG chương trình con trỏ lệnh của phần ví dụ — không đổi
gì cả, và nó THẬT SỰ chạy khi bạn bấm nút (không phải chỉ đọc bằng mắt
lần này). Việc của bạn: dự đoán trước hai con số, rồi so với thứ máy
tự tính ra.

```python title=starter
chuong_trinh = [
    ("DAT", "i", 0),
    ("NHAY_NEU_HET", "i", 3, 5),
    ("GHI", "i"),
    ("TANG", "i"),
    ("NHAY_LUI", 1),
]

bien = {}
nhat_ky = []
con_tro = 0
so_lan_nhay_lui = 0
while con_tro < len(chuong_trinh):
    lenh = chuong_trinh[con_tro]
    ten = lenh[0]
    if ten == "DAT":
        bien[lenh[1]] = lenh[2]
        con_tro += 1
    elif ten == "NHAY_NEU_HET":
        if bien[lenh[1]] >= lenh[2]:
            con_tro = lenh[3]
        else:
            con_tro += 1
    elif ten == "GHI":
        nhat_ky.append(bien[lenh[1]])
        con_tro += 1
    elif ten == "TANG":
        bien[lenh[1]] += 1
        con_tro += 1
    elif ten == "NHAY_LUI":
        so_lan_nhay_lui += 1
        con_tro = lenh[1]

du_doan_so_lan_nhay_lui = ___    # bao nhiêu lần NHAY_LUI chạy trước khi while dừng?
du_doan_con_tro_luc_dung = ___   # con trỏ mang giá trị bao nhiêu ngay khi while dừng?

print(nhat_ky)
print(so_lan_nhay_lui == du_doan_so_lan_nhay_lui)
print(con_tro == du_doan_con_tro_luc_dung)
```

```python title=solution
chuong_trinh = [
    ("DAT", "i", 0),
    ("NHAY_NEU_HET", "i", 3, 5),
    ("GHI", "i"),
    ("TANG", "i"),
    ("NHAY_LUI", 1),
]

bien = {}
nhat_ky = []
con_tro = 0
so_lan_nhay_lui = 0
while con_tro < len(chuong_trinh):
    lenh = chuong_trinh[con_tro]
    ten = lenh[0]
    if ten == "DAT":
        bien[lenh[1]] = lenh[2]
        con_tro += 1
    elif ten == "NHAY_NEU_HET":
        if bien[lenh[1]] >= lenh[2]:
            con_tro = lenh[3]
        else:
            con_tro += 1
    elif ten == "GHI":
        nhat_ky.append(bien[lenh[1]])
        con_tro += 1
    elif ten == "TANG":
        bien[lenh[1]] += 1
        con_tro += 1
    elif ten == "NHAY_LUI":
        so_lan_nhay_lui += 1
        con_tro = lenh[1]

du_doan_so_lan_nhay_lui = 3
du_doan_con_tro_luc_dung = 5

print(nhat_ky)
print(so_lan_nhay_lui == du_doan_so_lan_nhay_lui)
print(con_tro == du_doan_con_tro_luc_dung)
```

```python title=test
assert nhat_ky == [0, 1, 2], f"nhat_ky phải là [0, 1, 2] — ba lần GHI trước khi i chạm ngưỡng 3, đang ra {nhat_ky}"
assert du_doan_so_lan_nhay_lui == 3, f"NHAY_LUI phải chạy đúng 3 lần trước khi con trỏ bị NHAY_NEU_HET đưa thẳng ra lệnh 5 — đang đoán {du_doan_so_lan_nhay_lui}"
assert du_doan_con_tro_luc_dung == 5, f"con trỏ dừng đúng ở giá trị 5 — nơi NHAY_NEU_HET gán thẳng khi i đã đạt ngưỡng — đang đoán {du_doan_con_tro_luc_dung}"
```

:::hints
- kind: attention
  body: Vòng while không cần sửa gì cả — nó đã tự đếm giùm bạn bằng so_lan_nhay_lui, và tự dừng đúng lúc. Việc của bạn chỉ là ĐOÁN hai con số TRƯỚC khi đọc kết quả thật.
- kind: strategy
  body: Đi tay theo đúng bảng con trỏ trong phần ví dụ - 0→1→2→3→4→1 (nhảy lần 1)→2→3→4→1 (nhảy lần 2)→2→3→4→1 (nhảy lần 3)→5. Đếm đúng bao nhiêu lần con trỏ nhảy lùi, và giá trị nó dừng lại.
- kind: one-line
  body: 'Hai chỗ trống lần lượt là 3 và 5.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^\\[0, 1, 2\\]\\nTrue\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba lần nhảy lùi, dừng đúng ở lệnh 5 — con trỏ không đoán mò, nó đi đúng
luật: tăng 1 mỗi lệnh thường, gán thẳng mỗi lệnh nhảy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Con trỏ lệnh chỉ là một con số, và vòng `while` bạn vừa đọc chỉ là một
đoạn mã Python BÌNH THƯỜNG — không có gì đặc biệt về mặt cú pháp. Nhưng
CHÍNH cái vòng lặp lấy-hiểu-làm đó — thứ đọc con trỏ, tra ra đúng lệnh,
rồi thi hành — bản thân NÓ là cái gì? Nó có tự nhiên có sẵn bên trong
máy tính, chờ đó từ trước, hay chính nó cũng phải là một chương trình
CÓ AI ĐÓ đã viết ra?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
