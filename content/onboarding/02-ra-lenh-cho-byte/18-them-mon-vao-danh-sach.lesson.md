---
id: onboarding.ra-lenh-cho-byte.them-mon-vao-danh-sach
title: Thêm món vào danh sách
summary: Danh sách sửa được - append gắn thêm một món vào cuối dãy ngay lúc chương trình đang chạy.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.list-append]
requires: [ctrl.for-each, core.list]
concepts: [core.danh-sach, core.sua-duoc]
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
Bảng phấn ngoài cửa viết thêm được. Dãy món trong chương trình cũng vậy.
::::

::::explain{#bang-phan-chu-khong-phai-bia-da}
Câu hỏi cuối bài trước: ba ô đã gõ sẵn ở dòng đầu, giữa buổi có thêm món thì làm
sao?

Câu trả lời ngắn: **thêm được**, ngay trong lúc chương trình đang chạy.

Nghĩ tới hai thứ trong quán. Tấm bia đá khắc tên quán trước cửa — khắc xong là
xong, muốn đổi một chữ thì phải đục cả tấm. Còn tấm bảng phấn kê cạnh nồi nước
dùng — bếp báo có thêm món, chủ quán cầm phấn viết thêm một dòng xuống dưới, hết
món thì lau đi.

Danh sách trong Python thuộc loại bảng phấn. Người ta gọi tính chất đó là
**sửa được** (tiếng Anh: `mutable`): sau khi đã tạo ra, cái dãy ấy vẫn nhận thêm
món, vẫn dài ra được, mà **vẫn là chính nó** — vẫn cái tên `thuc_don` ấy, chứ
không phải một dãy mới nào khác.
::::

::::example{#gan-them-vao-cuoi}
Câu lệnh viết thêm một dòng lên bảng phấn tên là `append` — tiếng Anh nghĩa là
*gắn thêm vào cuối*:

```python title=readonly
thuc_don = ["Phở tái", "Phở chín"]
thuc_don.append("Phở nạm")
print(thuc_don)
```

Màn hình hiện ra:

```text title=readonly
['Phở tái', 'Phở chín', 'Phở nạm']
```

Dòng đầu tạo dãy hai món. Dòng cuối in ra dãy **ba** món. Giữa hai dòng ấy không
có dấu `=` nào, không có cái tên mới nào — chính cái dãy cũ đã dài ra.

Đọc dòng giữa từ trái sang phải:

- `thuc_don` — tấm bảng cần viết thêm.
- Dấu chấm `.` — cách nói *"bảo chính cái này làm một việc"*. Gắn thêm món là
  việc của bản thân danh sách, nên tên việc đứng dính ngay sau dấu chấm.
- `append(...)` — tên việc, và món mới nằm trong ngoặc tròn, đủ hai dấu nháy vì
  nó là một câu chữ.
::::

::::explain{#mon-moi-dung-o-cuoi}
`append` luôn đặt món mới vào **cuối** dãy, không chen vào giữa. Gọi hai lần thì
hai món xếp sau nhau đúng theo thứ tự bạn gọi.

Dãy dài thêm một ô, nên chỗ đứng cũng theo đó mà dài ra:

> Trước khi thêm: Phở tái (0), Phở chín (1) — dãy hai ô, chỉ số cuối là 1.
> Sau khi thêm: Phở tái (0), Phở chín (1), Phở nạm (2) — ô mới nhận đúng con số
> bằng số món cũ.

Và đây là chỗ đáng giá nhất. Vòng lặp bạn viết ở bài trước **không phải sửa một
chữ nào**: nó nói "từng món trong `thuc_don`", nên thêm một món thì nó tự chạy
thêm một lượt. Bạn viết `for` một lần, còn số lượt để cho dãy tự quyết.
::::

::::predict{#doan-cho-dung commitOnce}
Đoạn dưới thêm hai món rồi mới in. **Trước khi bấm chạy**, bạn đoán màn hình
hiện ra gì?

```python title=readonly
thuc_don = ["Phở tái", "Phở chín"]
thuc_don.append("Phở gầu")
thuc_don.append("Phở nạm")
print(thuc_don[2])
```

:::opt{correct}
Phở gầu
:::

:::opt
Phở nạm
::why
Gần đúng ở hai chỗ liền: bạn nhớ đúng rằng `append` gắn món vào cuối dãy, và
"Phở nạm" đúng là món được gắn sau cùng nên nó đang đứng cuối.

Chỗ lệch là `[2]` không có nghĩa "món cuối". Nó vẫn là câu hỏi của bài trước:
*đi hai bước từ đầu dãy*. Lúc `print` chạy, dãy đang là Phở tái (0), Phở chín
(1), Phở gầu (2), Phở nạm (3).

Muốn lấy "Phở nạm" thì viết `thuc_don[3]`.
::
:::

:::opt
Phở chín
::why
Gần đúng ở chỗ bạn đọc `[2]` thành "món thứ hai" — cách đếm quen thuộc ngoài
đời, và ngoài đời thì "Phở chín" đúng là món thứ hai trên bảng.

Chỗ lệch: máy đếm từ 0, nên "Phở chín" mang chỉ số 1. Con số 2 rơi vào món đứng
kế sau nó, tức là món được `append` đầu tiên.
::
:::

:::opt
Máy báo lỗi, vì dòng đầu chỉ viết có hai món
::why
Gần đúng, và đây là một câu rất đáng hỏi: nếu chương trình dừng lại ngay sau
dòng đầu thì `[2]` thật sự xin một ô không tồn tại, và máy sẽ nói
`list index out of range` như bài trước.

Chỗ lệch nằm ở thứ tự thời gian. Máy chạy từ trên xuống, nên hai dòng `append`
chạy **trước** dòng `print`, và mỗi dòng ấy làm dãy dài thêm một ô. Tới lượt
`print`, dãy đã có bốn ô.

Đó chính là điều bài này nói: danh sách không nằm im như lúc bạn gõ nó ra.
::
:::
::::

::::code{#them-pho-gau}
Giữa buổi, bếp báo lên: hôm nay có thêm **Phở gầu**.

Hãy thêm món ấy vào thực đơn, đặt ở cuối bảng. Vòng lặp in bảng đã viết sẵn bên
dưới — làm đúng thì nó tự in ra bốn dòng.

```python title=starter
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
___
for mon in thuc_don:
    print(f"- {mon}")
```

```python title=solution
thuc_don = ["Phở tái", "Phở chín", "Phở nạm"]
thuc_don.append("Phở gầu")
for mon in thuc_don:
    print(f"- {mon}")
```

```python title=test
# Chạy xong chương trình, dãy phải có đúng bốn món và món mới phải nằm ở CUỐI.
# Thứ tự khác đi nghĩa là món mới đã chen vào giữa chứ không được gắn thêm.
assert thuc_don == ["Phở tái", "Phở chín", "Phở nạm", "Phở gầu"], "giữa buổi bếp báo thêm Phở gầu, nên thực đơn thành bốn món và món mới viết xuống dưới cùng, sau Phở nạm"
```

:::hints
- kind: attention
  body: Vòng lặp bên dưới đã biết đọc hết bảng, và nó đọc đúng những gì đang có trong `thuc_don`. Việc còn thiếu là làm cho dãy có thêm một ô trước khi vòng lặp chạy.
- kind: strategy
  body: Câu lệnh gắn thêm gồm ba phần dính nhau: tên danh sách, dấu chấm kèm tên việc, rồi món mới đặt trong ngoặc tròn. Món mới là một câu chữ nên nó cần đủ hai dấu nháy.
- kind: one-line
  body: Viết `thuc_don.append("Phở gầu")` vào chỗ trống, sát lề trái như dòng ngay trên nó.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng dài thêm một dòng, mà vòng lặp của bạn không phải sửa chữ nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp — lần này hãy nhìn vào chính chương trình của bạn,
đừng nhìn Byte.

Mấy bài vừa rồi, mỗi lần cần chào khách là bạn lại gõ đúng ba dòng giống hệt
nhau:

> `print("Quán Phở Thìn xin chào")` · `print("Mời anh chị ngồi bàn trống")` ·
> `print("Thực đơn hôm nay:")`

Một lần ở đoạn mở quán buổi sáng. Một lần nữa ở đoạn khách quen bước vào. Lần
thứ ba ở đoạn khách gọi mang về. Ba chỗ, cùng một nội dung.

Bây giờ chủ quán đổi câu chào. Bạn sửa chỗ thứ nhất, sửa chỗ thứ hai, rồi quên
mất chỗ thứ ba — thế là cùng một quán mà chương trình chào hai kiểu khác nhau
trong một buổi sáng.

Có cách nào viết đoạn ấy **đúng một lần**, đặt cho nó một cái tên, rồi chỗ nào
cần thì gọi tên ra dùng không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
