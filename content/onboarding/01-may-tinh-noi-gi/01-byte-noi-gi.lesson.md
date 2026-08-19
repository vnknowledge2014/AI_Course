---
id: onboarding.may-tinh-noi-gi.byte-noi-gi
title: Byte nói gì
summary: Bài đầu tiên. Bạn sẽ khiến máy tính nói ra một câu.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 8
teaches: [core.output, core.string-literal]
requires: []
concepts: [core.chuong-trinh, core.chuoi]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: human
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Chào! Mình là Byte. Mình làm được đúng một việc: làm theo lời bạn.
::::

::::explain{#may-cho-lenh}
Máy tính không tự nghĩ ra việc gì để làm. Nó **chờ**.

Bạn nói một câu, nó làm đúng câu đó. Bạn không nói, nó ngồi im mãi mãi.

Nghe có vẻ ngốc. Nhưng đó chính là điều làm nó hữu ích: nó làm **chính xác**
điều bạn bảo, một triệu lần, không mệt, không nhớ nhầm.

Vấn đề duy nhất — cũng là vấn đề bạn sẽ dành cả khoá học này để giải — là:
*bảo cho đúng thì khó hơn ta tưởng.*
::::

::::example{#cau-lenh-dau-tien}
Đây là một câu lệnh. Nó bảo máy nói ra một câu.

```python title=readonly
print("Chào Byte")
```

Đọc từ trái sang phải:

- `print` là **tên việc cần làm** — "nói ra".
- Hai dấu ngoặc `(` `)` bọc lấy **thứ cần nói**.
- `"Chào Byte"` là câu chữ, nằm giữa hai dấu nháy kép.

Dấu nháy kép không phải trang trí. Nó là cách bạn nói với máy:
*"phần này là chữ, đừng cố hiểu nó, cứ đọc nguyên văn."*
::::

::::predict{#doan-ket-qua commitOnce}
Byte sắp chạy đoạn dưới. **Trước khi bấm chạy**, bạn đoán nó in ra gì?

```python title=readonly
print("2 + 3")
```

:::opt{correct}
2 + 3
:::

:::opt
5
::why
Gần đúng ở chỗ bạn nhận ra `2 + 3` là một phép cộng. Nhưng ở đây nó nằm **trong
dấu nháy kép** — nên với máy, đó chỉ là ba ký tự `2`, `+`, `3` xếp cạnh nhau,
y như `"mèo"` là ba chữ cái. Máy đọc nguyên văn, không tính.

Thử bỏ dấu nháy đi ở bài sau, bạn sẽ thấy nó đổi khác ngay.
::
:::

:::opt
Máy sẽ báo lỗi
::why
Không có gì sai ở đây cả. Máy hiểu rất rõ việc cần làm: đọc nguyên văn phần
nằm giữa hai dấu nháy.
::
:::

::::

::::explain{#vi-sao-doan-truoc}
Bạn vừa làm một việc mà lập trình viên giỏi làm suốt ngày: **đoán trước rồi mới
chạy**.

Nghe thì mất công. Nhưng nó là khác biệt giữa hai kiểu người:

- Người sửa code bằng cách đổi lung tung rồi bấm chạy xem sao.
- Người biết trước máy sẽ làm gì, nên khi nó làm khác, họ *học được điều gì đó*.

Trong khoá này, Byte sẽ hỏi bạn "đoán xem" rất nhiều lần. Đoán sai không sao —
đoán sai rồi thấy kết quả thật mới là lúc bạn nhớ lâu nhất.
::::

::::code{#lam-byte-noi}
Đến lượt bạn. Hãy khiến Byte nói ra đúng câu **Xin chào**.

```python title=starter
print(___)
```

```python title=solution
print("Xin chào")
```

```python title=test
# Bài đầu tiên chấm bằng OUTPUT, không bằng assert trên hàm — người học chưa
# biết hàm là gì. Khối test này chỉ khẳng định chương trình chạy được tới cuối.
pass
```

:::hints
- kind: attention
  body: Byte cần biết **chữ nào** để nói. Chỗ trống nằm giữa hai dấu ngoặc.
- kind: strategy
  body: Nhớ lại ví dụ ở trên. Câu chữ luôn nằm giữa hai dấu nháy kép.
- kind: one-line
  body: "Viết `\"Xin chào\"` vào chỗ trống — đủ cả hai dấu nháy."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: Xin chào
:::
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp:

Nếu bạn viết `print(Xin chào)` — **không có** dấu nháy — bạn nghĩ máy sẽ làm gì?

Đừng trả lời vội. Bài sau bạn sẽ thử, và câu trả lời sẽ giải thích luôn vì sao
dấu nháy tồn tại.
::::

::::checkpoint{mastery=0.8}
::::
