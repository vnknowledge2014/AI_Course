---
id: toan.do-thi-modular-dai-so-truu-tuong.duong-di-euler
title: Đường đi Euler
summary: "Đường đi Euler — đi qua MỌI cạnh của đồ thị ĐÚNG một lần (khác đường đi bài 4, vốn không lặp ĐỈNH; đường Euler không lặp CẠNH, đỉnh có thể ghé lại); chu trình Euler — đường Euler mà điểm đầu trùng điểm cuối."
locale: vi
track: toan
module: do-thi-modular-dai-so-truu-tuong
order: 10
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [math.euler-path]
requires: [math.path-and-cycle, math.konigsberg-problem]
concepts: [math.duong-di-euler, math.chu-trinh-euler]
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
Byte muốn đi bộ dọc THEO MỌI ống tưới, MỖI ống ĐÚNG một lần, để kiểm
tra rò rỉ. Luôn LÀM được không, hay TUỲ sơ đồ?
::::

::::explain{#duong-di-euler-la-gi}
TUỲ sơ đồ. **Đường đi Euler** — đi qua MỌI cạnh của đồ thị ĐÚNG một
lần (KHÁC đường đi Ở bài 4, vốn KHÔNG lặp ĐỈNH; đường Euler KHÔNG
lặp CẠNH, đỉnh CÓ thể ghé lại NHIỀU lần). **Chu trình Euler** — đường
Euler mà điểm ĐẦU trùng điểm CUỐI (khép kín):

```python title=readonly
def canh_that(E):
    return {tuple(sorted(e)) for e in E}

def la_duong_di_euler(day, E):
    for i in range(len(day) - 1):
        if (day[i], day[i + 1]) not in E:
            return False
    buoc = [tuple(sorted((day[i], day[i + 1]))) for i in range(len(day) - 1)]
    return len(buoc) == len(set(buoc)) == len(canh_that(E))


E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}

print(la_duong_di_euler(["luong_1", "luong_2", "luong_3", "luong_1"], E_vong))
```

```text title=readonly
True
```

Dãy `luong_1→luong_2→luong_3→luong_1` đi qua ĐÚNG ba cạnh của tam
giác, MỖI cạnh MỘT lần, RỒI quay VỀ điểm xuất phát — một CHU TRÌNH
Euler.
::::

::::example{#duong-thang-khong-khep-kin}
Trên đồ thị ĐƯỜNG THẲNG (bài 7, KHÔNG có cạnh khép vòng) — đường
Euler VẪN tồn tại, NHƯNG KHÔNG khép kín:

```python title=readonly
def canh_that(E):
    return {tuple(sorted(e)) for e in E}

def la_duong_di_euler(day, E):
    for i in range(len(day) - 1):
        if (day[i], day[i + 1]) not in E:
            return False
    buoc = [tuple(sorted((day[i], day[i + 1]))) for i in range(len(day) - 1)]
    return len(buoc) == len(set(buoc)) == len(canh_that(E))


E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}

print(la_duong_di_euler(["luong_1", "luong_2", "luong_3"], E_duong))
```

```text title=readonly
True
```

`luong_1→luong_2→luong_3` đi qua ĐỦ HAI cạnh, MỖI cạnh MỘT lần —
VẪN LÀ đường đi Euler, dù điểm ĐẦU (`luong_1`) KHÁC điểm CUỐI
(`luong_3`) — CHỈ khi khép kín MỚI gọi LÀ chu trình.
::::

::::predict{#doan-bo-sot-canh commitOnce}
Byte thử đi `luong_1→luong_2→luong_3` TRÊN đồ thị TAM GIÁC (`E_vong`,
BA cạnh) — CHỨ không phải đường thẳng:

```python
def canh_that(E):
    return {tuple(sorted(e)) for e in E}

def la_duong_di_euler(day, E):
    for i in range(len(day) - 1):
        if (day[i], day[i + 1]) not in E:
            return False
    buoc = [tuple(sorted((day[i], day[i + 1]))) for i in range(len(day) - 1)]
    return len(buoc) == len(set(buoc)) == len(canh_that(E))

E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}
print(la_duong_di_euler(["luong_1", "luong_2", "luong_3"], E_vong))
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì MỖI bước (`luong_1→luong_2`, `luong_2→luong_3`) ĐỀU LÀ
cạnh CÓ thật trong `E_vong`, VÀ không cạnh nào bị đi LẶP lại
::why
Gần đúng ở việc bạn kiểm ĐÚNG từng bước riêng lẻ CÓ hợp lệ hay không
— cả hai bước ĐÓ đúng LÀ cạnh THẬT, không sai.

Chỗ lệch: đường Euler đòi đi qua MỌI cạnh, KHÔNG chỉ những cạnh
"hợp lệ" — tam giác CÓ BA cạnh (`canh_that(E_vong)` dài `3`), NHƯNG
dãy NÀY chỉ đi qua HAI bước (thiếu cạnh `luong_3-luong_1`, cạnh khép
vòng). `len(buoc)=2 ≠ len(canh_that(E_vong))=3` — BỎ SÓT một cạnh,
KHÔNG PHẢI đường Euler.
::
:::

:::opt
Máy báo lỗi khi chạy — `canh_that(E_vong)` dùng `tuple(sorted(e))`
Ở TRONG một set-comprehension, mà `sorted()` trả VỀ MỘT `list`,
`tuple()` KHÔNG thể BỌC một `list` bên TRONG một set-comprehension
::why
Gần đúng ở việc bạn để ý ĐÚNG `sorted()` trả VỀ `list`, VÀ `tuple()`
bọc NGOÀI nó — một quan sát VỀ kiểu dữ liệu chính XÁC.

Chỗ lệch: `tuple(mot_list)` LÀ một cách CHUYỂN kiểu HOÀN TOÀN hợp lệ
trong Python (biến MỘT danh sách thành MỘT tuple bất biến, để CÓ thể
đưa VÀO `set` — tuple băm được, list THÌ không) — KHÔNG có gì mâu
thuẫn hay lỗi Ở đây cả. Biên dịch sạch, chạy sạch.
::
:::
::::

::::code{#viet_la_duong_di_euler}
Viết PHẦN CÒN LẠI của `la_duong_di_euler(day, E)` — so sánh số bước
đi qua VỚI số cạnh THẬT của đồ thị (không thiếu, không lặp).

```python title=starter
def canh_that(E):
    return {tuple(sorted(e)) for e in E}

def la_duong_di_euler(day, E):
    for i in range(len(day) - 1):
        if (day[i], day[i + 1]) not in E:
            return False
    buoc = [tuple(sorted((day[i], day[i + 1]))) for i in range(len(day) - 1)]
    return ___


E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}

print(la_duong_di_euler(["luong_1", "luong_2", "luong_3", "luong_1"], E_vong))
```

```python title=solution
def canh_that(E):
    return {tuple(sorted(e)) for e in E}

def la_duong_di_euler(day, E):
    for i in range(len(day) - 1):
        if (day[i], day[i + 1]) not in E:
            return False
    buoc = [tuple(sorted((day[i], day[i + 1]))) for i in range(len(day) - 1)]
    return len(buoc) == len(set(buoc)) == len(canh_that(E))


E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}

print(la_duong_di_euler(["luong_1", "luong_2", "luong_3", "luong_1"], E_vong))
```

```python title=test
E_vong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2"), ("luong_3", "luong_1"), ("luong_1", "luong_3")}
E_duong = {("luong_1", "luong_2"), ("luong_2", "luong_1"), ("luong_2", "luong_3"), ("luong_3", "luong_2")}
assert la_duong_di_euler(["luong_1", "luong_2", "luong_3", "luong_1"], E_vong) is True, "chu trinh euler tren tam giac"
assert la_duong_di_euler(["luong_1", "luong_2", "luong_3"], E_vong) is False, "bo sot canh khep vong -- khong phai euler"
assert la_duong_di_euler(["luong_1", "luong_2", "luong_1", "luong_3", "luong_2", "luong_3"], E_vong) is False, "lap lai canh -- khong phai euler"
assert la_duong_di_euler(["luong_1", "luong_2", "luong_3"], E_duong) is True, "duong di euler khong khep kin van hop le"
```

:::hints
- kind: attention
  body: "So sanh len(buoc) (so buoc di), len(set(buoc)) (so canh KHONG trung), va len(canh_that(E)) (so canh that) -- ba con so phai BANG nhau."
- kind: strategy
  body: "len(buoc) == len(set(buoc)) == len(canh_that(E))"
- kind: one-line
  body: "___ = len(buoc) == len(set(buoc)) == len(canh_that(E))"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai so sanh len(buoc), len(set(buoc)) va len(canh_that(E))
  requireAst:
  - kind: uses-name, target: buoc, min: 2
  - kind: uses-call, target: canh_that, min: 1
  - kind: uses-call, target: set, min: 1
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^True\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tam giác CÓ chu trình Euler, đường thẳng CÓ đường đi Euler. Bảy cầu
Königsberg (bốn vùng, TOÀN bậc lẻ) — có đường Euler nào không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bảy cầu Königsberg có BỐN vùng đất, TẤT CẢ đều bậc LẺ (bài 9). Tam
giác (bậc MỌI đỉnh CHẴN — 2) CÓ chu trình Euler; đường thẳng (HAI
đỉnh bậc LẺ, MỘT đỉnh bậc chẵn) CÓ đường đi Euler. VẬY, với BỐN đỉnh
bậc lẻ — Königsberg CÓ đường đi Euler nào không?
::::

::::checkpoint{mastery=0.8}
::::
