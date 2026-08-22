---
id: nen-tang.re-nhanh-va-lap.viet-gon-phep-cong-don
title: Viết gọn phép cộng dồn
summary: Cộng thêm vào một cái tên có lối viết ngắn — vẫn là đọc–sửa–ghi, chỉ gõ tên một lần.
locale: vi
track: nen-tang
module: re-nhanh-va-lap
order: 13
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.augmented-assign]
requires: [core.reassign, ctrl.for-each, core.variable]
concepts: [core.bien, core.gan-lai, ctrl.lap]
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
Một dòng mà cái tên phải gõ hai lần. Mình có lối viết ngắn cho đúng việc đó.
::::

::::explain{#mot-cai-ten-go-hai-lan}
Cộng dồn cả sổ chi tiêu thì bạn viết thế này: một cái tên `tong = 0` sinh ra
trước vòng, rồi mỗi lượt trong thân vòng chạy đúng dòng này:

```python
tong = tong + tien
```

Dòng ấy làm ba việc, theo đúng thứ tự này: **đọc** giá trị cũ của `tong`,
**cộng** thêm `tien`, rồi **dán lại** cái tên `tong` lên kết quả mới. Đọc – sửa
– ghi.

Ba việc đó đều cần. Chỗ phiền nằm ở chữ `tong`: nó phải gõ hai lần trên cùng một
dòng, và hai lần ấy buộc phải giống hệt nhau. Gõ nhầm thành `tong = tng + tien`
thì máy không kêu lúc bạn viết — nó đợi tới lúc chạy mới nói
`NameError: name 'tng' is not defined`.

Người bán hàng ngoài chợ không chép lại con số cũ mỗi lần cộng. Họ có một cột
"thêm", và chỉ ghi vào đó phần thêm vào: +45.000, rồi +120.000. Con số cũ nằm
sẵn trên giấy rồi.

Python có đúng lối viết ấy:

```python
tong += tien
```

Đọc thành lời: *cộng thêm `tien` vào `tong`*. Hai ký tự `+` và `=` viết dính
nhau, không có khoảng trắng chen vào giữa.

Đây không phải một phép tính mới. Máy vẫn làm đủ ba việc cũ — đọc giá trị cũ,
cộng, dán lại tên. Chỉ khác ở chỗ bạn nhắc tên `tong` một lần thay vì hai. Tên
riêng của lối viết này là **toán tử gán gộp** (tiếng Anh: *augmented
assignment*), cần từ đó khi bạn đi tra cứu.

Cùng một khuôn còn có `-=` và `*=`:

```python
vi_tien = 500000
vi_tien -= 45000
print(vi_tien)
```

In ra `455000` — trả tiền một tô phở, ví vơi đi đúng chừng ấy. `so_to *= 2` thì
nhân đôi số tô đang có.

Một điều kiện kèm theo, và nó đến thẳng từ chữ "đọc" trong đọc–sửa–ghi: cái tên
phải **đã tồn tại** trước khi bạn cộng thêm vào nó. Không có giá trị cũ thì
không có gì để đọc.
::::

::::example{#ca-so-chi-tieu-trong-bon-dong}
Sổ chi tiêu sáu ngày của Byte, cộng dồn bằng lối viết mới:

```python title=readonly
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
tong = 0
for tien in chi_tieu:
    tong += tien
print(f"Cả tuần tiêu {tong} đồng")
```

Máy in ra:

```text
Cả tuần tiêu 750000 đồng
```

Ba chỗ đáng nhìn kỹ:

- `tong = 0` nằm **ngoài** vòng, sát lề trái. Khai nó trong thân vòng thì mỗi
  lượt lại dán `tong` về 0, xoá sạch những gì lượt trước cộng được.
- `tong += tien` nằm **trong** thân vòng, thụt vào bốn dấu cách. Nó chạy sáu
  lần, mỗi lần với một con số khác — vì `tien` đổi giá trị mỗi vòng.
- `print` lại nằm ngoài vòng. Bạn muốn một dòng tổng kết, không phải sáu dòng
  báo cáo giữa chừng.

Con số trong `tong` lớn dần qua từng lượt: 45000, rồi 165000, rồi 195000,
455000, 540000, và cuối cùng 750000.
::::

::::predict{#thieu-mot-dong commitOnce}
Byte gõ vội và quên mất một dòng. **Trước khi bấm chạy**, bạn đoán màn hình hiện
ra gì?

```python
chi_tieu = [45000, 120000, 30000]
for tien in chi_tieu:
    tong += tien
print(tong)
```

:::opt{correct}
Máy dừng ngay ở lượt đầu tiên với `NameError: name 'tong' is not defined`
:::

:::opt
195000 — máy cộng đủ ba con số
::why
Gần đúng ở chỗ phép cộng bạn nhẩm hoàn toàn chính xác: 45000 + 120000 + 30000
đúng là 195000, và đó cũng đúng là con số người viết đoạn này muốn thấy.

Chỗ lệch nằm ở lượt đầu tiên. `tong += tien` bắt đầu bằng việc **đọc giá trị cũ
của `tong`** — mà ở lượt đầu, cái tên `tong` chưa từng được dán lên giá trị nào.
Máy đi tìm và không thấy, nên nó dừng trước cả khi cộng.
::
:::

:::opt
0 — máy hiểu một cái tên đang cộng dồn thì bắt đầu từ 0
::why
Gần đúng ở chỗ 0 chính là con số hợp lý để bắt đầu — đoạn ví dụ phía trên cũng
mở sổ bằng đúng `tong = 0`.

Chỗ lệch là ở chỗ ai chọn. Máy không tự bổ sung thứ bạn quên nói — nguyên tắc
này theo bạn từ những bài đầu tiên của Realm 0. Nếu nó tự cho `tong` bằng 0, thì
lần khác bạn gõ nhầm tên biến, nó cũng sẽ lặng lẽ tạo ra một cái tên mới bằng 0
và cộng dồn vào đó cả buổi mà bạn không hay.
::
:::

:::opt
Máy báo `SyntaxError`, và báo trước khi chạy dòng nào
::why
Gần đúng ở chỗ bạn nhận ra đoạn này có thiếu sót thật, và bạn nhớ rằng có một
loại lỗi máy bắt được từ trước khi chạy — điều đó chính xác.

Chỗ lệch: dòng `tong += tien` viết đúng cú pháp từng chữ một, máy đọc hiểu hoàn
toàn. Nó chỉ hỏng lúc **thực thi**, khi máy đi tìm giá trị cũ của `tong`. Lỗi
tìm không thấy một cái tên có tên riêng là `NameError`, và nó chỉ lộ ra khi
chương trình đã chạy tới dòng đó.
::
:::
::::

::::code{#cong-don-ca-tuan}
Sổ chi tiêu sáu ngày đã nằm sẵn trong danh sách, và `tong` đã sinh ra trước
vòng. Chỉ còn chỗ trống trong thân vòng — hãy viết nó bằng lối viết gọn của bài
này.

```python title=starter
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
tong = 0
for tien in chi_tieu:
    tong ___ tien
print(f"Cả tuần tiêu {tong} đồng")
```

```python title=solution
chi_tieu = [45000, 120000, 30000, 260000, 85000, 210000]
tong = 0
for tien in chi_tieu:
    tong += tien
print(f"Cả tuần tiêu {tong} đồng")
```

```python title=test
# Byte cộng tay sáu con số trong sổ và được 750000. Sau khi vòng chạy xong,
# cái tên `tong` phải đang giữ đúng con số đó.
assert tong == 750000, "sáu ngày trong sổ cộng lại là 750 nghìn — lối viết gọn phải cộng dồn được y hệt lối viết dài, không sót ngày nào"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa cái tên `tong` và cái tên `tien`. Nó phải làm đúng việc mà `tong = tong + tien` làm, nhưng chỉ được nhắc tên `tong` một lần.
- kind: strategy
  body: Lối viết gọn ghép đúng hai ký tự — dấu của phép tính, rồi tới dấu bằng. Hai ký tự đó dính liền nhau, không có khoảng trắng chen vào giữa.
- kind: one-line
  body: "`tong += tien`"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Cả tuần tiêu 750000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu ngày gộp lại trong một dòng, và cái tên tong chỉ phải gõ một lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tong += tien` cộng ở **mọi** lượt. Sáu ngày trong sổ thì cả sáu con số đều vào
tổng, không ngày nào bị bỏ sót — đó đúng là thứ bạn cần khi hỏi "cả tuần tiêu
hết bao nhiêu".

Nhưng câu hỏi tiếp theo trong sổ của Byte là: **bao nhiêu ngày tiêu quá 200
nghìn?**

Vẫn phải đi qua đủ sáu ngày, vì không xem thì không biết. Nhưng lần này không
phải ngày nào cũng được tính. Ngày 45 nghìn thì không thêm gì cả; ngày 260 nghìn
thì đáng một vạch.

Cộng ở một số lượt và bỏ qua các lượt còn lại — bạn đã có sẵn công cụ để hỏi
"ngày này có vượt 200 nghìn không". Chuyện còn lại là đặt nó ở đâu, và đặt cái
vạch ở đâu. Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
