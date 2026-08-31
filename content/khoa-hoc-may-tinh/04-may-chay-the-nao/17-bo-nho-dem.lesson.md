---
id: khoa-hoc-may-tinh.may-chay-the-nao.bo-nho-dem
title: "Bộ nhớ đệm: trạm trung chuyển giữa CPU và RAM"
summary: "Thanh ghi quá nhỏ để chứa cả chương trình. Bộ nhớ đệm (cache) là trạm giữa — lớn hơn thanh ghi, nhỏ hơn RAM — giữ SẴN những gì CPU vừa dùng hoặc sắp dùng, đoán trước để đỡ phải đợi RAM."
locale: vi
track: khoa-hoc-may-tinh
module: may-chay-the-nao
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [may.cache]
requires: [may.registers]
concepts: [may.cache]
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
Có — một trạm trung chuyển, đứng đúng giữa hai thái cực vừa gặp.
::::

::::explain{#tram-trung-chuyen}
Thanh ghi (bài trước) cực nhanh nhưng cực nhỏ — vài chục chỗ chứa, không
đủ giữ nổi một danh sách vài trăm phần tử, chứ đừng nói cả chương trình.
RAM thì ngược lại — hàng tỉ ô, đủ chứa mọi thứ, nhưng mỗi lần đọc phải
gửi địa chỉ rồi ĐỢI câu trả lời quay về.

Ở giữa hai thái cực đó có một tầng thứ ba: **bộ nhớ đệm** (cache). Nó
lớn hơn thanh ghi rất nhiều — đủ giữ hàng nghìn tới hàng triệu byte, chứ
không phải vài chục con số — nhưng vẫn nhỏ hơn RAM RẤT nhiều. Đổi lại, nó
nằm GẦN CPU hơn hẳn RAM, nên đọc từ nó nhanh hơn đọc từ RAM đáng kể — dù
không nhanh bằng thanh ghi.

Bộ nhớ đệm không giữ NGẪU NHIÊN. Nó giữ đúng những gì CPU NHIỀU KHẢ NĂNG
sẽ cần lại, dựa trên hai kiểu đoán:

- **Vừa dùng thì có thể dùng lại ngay** — gọi là **tính cục bộ theo
  thời gian** (temporal locality). Một giá trị CPU vừa chạm tới thường
  được chạm lại rất sớm sau đó.
- **Ở gần một chỗ vừa dùng thì cũng có thể sắp cần tới** — gọi là
  **tính cục bộ theo không gian** (spatial locality). Bài sau sẽ đào sâu
  đúng ý này với mảng liền kề.

Bộ nhớ đệm thật ra không phải MỘT tầng đơn — CPU hiện đại thường có NHIỀU
mức bộ nhớ đệm xếp chồng: mức gần CPU nhất nhỏ và nhanh nhất, mức xa hơn
lớn hơn một chút và chậm hơn một chút — nhưng CẢ hai mức đó vẫn nhanh hơn
RAM rất nhiều. Track này gộp chúng lại dưới một cái tên chung, "bộ nhớ
đệm", và sẽ tách rõ khi xếp cả năm tầng cạnh nhau ở bài chốt cụm.
::::

::::example{#vong-lap-la-vi-du-tot-nhat}
Nhớ lại `dem_toi` từ bài "Vòng lặp là một lệnh NHẢY LÙI" (bài 4) — chạy
`dis.dis()` thật trên nó lần nữa, lần này để soi kỹ tính cục bộ theo
thời gian:

```python title=readonly
import dis

def dem_toi(n):
    tong = 0
    for i in range(n):
        tong += i
    return tong

dis.dis(dem_toi)
```

```text title=readonly
  4           RESUME                   0

  5           LOAD_CONST               1 (0)
              STORE_FAST               1 (tong)

  6           LOAD_GLOBAL              1 (range + NULL)
              LOAD_FAST                0 (n)
              CALL                     1
              GET_ITER
      L1:     FOR_ITER                 7 (to L2)
              STORE_FAST               2 (i)

  7           LOAD_FAST_LOAD_FAST     18 (tong, i)
              BINARY_OP               13 (+=)
              STORE_FAST               1 (tong)
              JUMP_BACKWARD            9 (to L1)

  6   L2:     END_FOR
              POP_TOP

  8           LOAD_FAST                1 (tong)
              RETURN_VALUE
```

Dòng `LOAD_GLOBAL 1 (range + NULL)` chỉ xuất hiện MỘT lần trong danh sách
— nó chạy đúng một lần, trước khi vòng lặp bắt đầu. Nhưng dòng
`LOAD_FAST_LOAD_FAST 18 (tong, i)` — dù cũng chỉ xuất hiện MỘT lần trong
văn bản `dis.dis()` — lại bị chạy LẠI rất nhiều lần lúc thực thi, đúng
mỗi lượt `JUMP_BACKWARD` quay về nhãn `L1:` (bài 4). Cùng hai cái tên
`tong` và `i`, bị CPU chạm tới liên tục — chính kiểu truy cập mà bộ nhớ
đệm dựng ra để phục vụ: giữ sẵn thứ vừa dùng, vì gần như chắc chắn sẽ
được dùng lại ngay lượt sau.
::::

::::predict{#kieu-truy-cap-nao-loi commitOnce}
Hai đoạn mã dưới đây đều tính ra một con số, cùng khối lượng công việc
số học tương đương. Đoạn nào có kiểu truy cập bộ nhớ đệm ƯA THÍCH hơn?

```python
# Đoạn A
tong = 0
for _ in range(1000):
    tong += 1

# Đoạn B
a0, a1, a2 = 1, 1, 1
b0, b1, b2 = 2, 2, 2
c0, c1, c2 = 3, 3, 3
ket_qua = a0 + a1 + a2 + b0 + b1 + b2 + c0 + c1 + c2
```

:::opt{correct}
Đoạn A — cùng biến `tong` bị chạm đi chạm lại 1000 lần, đúng tính cục
bộ theo thời gian mà bộ nhớ đệm ưa thích
:::

:::opt
Đoạn B — vì nó có nhiều biến hơn, nên bộ nhớ đệm có nhiều lựa chọn hơn
để giữ sẵn
::why
Gần đúng ở việc bạn để ý đúng chỗ khác nhau thật sự giữa hai đoạn: SỐ
LƯỢNG tên biến khác nhau.

Chỗ lệch là hướng suy luận. "Nhiều biến hơn" không giúp gì cho bộ nhớ
đệm — chín cái tên trong Đoạn B đều chỉ được ĐỌC ĐÚNG MỘT LẦN, không có
gì để "dùng lại". Bộ nhớ đệm không thưởng cho việc có nhiều lựa chọn, nó
thưởng cho việc CHẠM LẠI cùng một ô nhiều lần — đúng thứ Đoạn A làm với
`tong`.
::
:::

:::opt
Cả hai như nhau, vì tổng số phép cộng gần bằng nhau
::why
Gần đúng ở việc bạn đếm đúng: hai đoạn có LƯỢNG phép cộng gần tương
đương nhau về số học.

Chỗ lệch là bài này không hỏi về SỐ LƯỢNG phép tính — nó hỏi về KIỂU
TRUY CẬP bộ nhớ. Đoạn A chạm lại đúng MỘT ô nhớ (`tong`) một nghìn lần;
Đoạn B chạm chín ô khác nhau, mỗi ô đúng một lần. Cùng khối lượng tính
toán, nhưng khác hẳn nhau ở việc bộ nhớ đệm có cơ hội "đoán đúng" hay
không.
::
:::

:::opt
Đoạn B — vì các biến `a0, a1, a2...` được đặt tên liền số thứ tự, nên
nằm liền kề trong bộ nhớ
::why
Gần đúng ở việc bạn đang nghĩ tới tính cục bộ theo KHÔNG GIAN — một ý
đúng, sẽ đào sâu ở bài sau.

Chỗ lệch: đặt TÊN biến liền số thứ tự (`a0`, `a1`, `a2`) không hề bắt
buộc Python xếp CHÚNG liền nhau trong bộ nhớ thật — tên gọi chỉ là nhãn
người đọc thấy, không phải vị trí vật lý. Tính cục bộ theo không gian
thật sự tới từ cấu trúc như mảng (bài sau), không phải từ cách đặt tên.
::
:::
::::

::::code{#cham-lai-mot-o}
Byte cộng dồn giá từng món trong `gia_mon` — mỗi lượt lặp đọc lại VÀ ghi
lại đúng biến `tong`, đúng kiểu truy cập bộ nhớ đệm ưa thích vừa học.

```python title=starter
gia_mon = [15000, 22000, 9000, 31000, 18000]

tong = 0
for gia in gia_mon:
    tong = ___                 # cộng dồn: tong hiện có, cộng thêm gia

print(f"Tổng: {tong}")
```

```python title=solution
gia_mon = [15000, 22000, 9000, 31000, 18000]

tong = 0
for gia in gia_mon:
    tong = tong + gia

print(f"Tổng: {tong}")
```

```python title=test
assert tong == 95000, f"tong phải cộng dồn đủ cả năm món: 15000+22000+9000+31000+18000 = 95000 — đang ra {tong}"
```

:::hints
- kind: attention
  body: Chỗ trống là một phép cộng dồn — dùng đúng hai cái tên đã có sẵn ngay dòng đó, tong và gia. Đừng gõ thẳng một con số.
- kind: strategy
  body: 'Cộng dồn nghĩa là lấy giá trị tong ĐANG CÓ, cộng thêm gia của lượt này, rồi gán lại vào chính tong: tong = tong + gia. Mỗi lượt lặp đọc lại VÀ ghi lại đúng một ô nhớ — không phải tạo ô mới.'
- kind: one-line
  body: 'Chỗ trống là: tong + gia'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải THẬT SỰ đọc cả tong lẫn gia rồi cộng lại — không được gõ thẳng con số 95000 hay bất kỳ hằng số nào khác vào chỗ trống, vì bài này đang dạy đúng kiểu chạm-lại-một-ô mà bộ nhớ đệm ưa thích, không phải kết quả cuối cùng
  requireAst:
  # Đếm thật trên lời giải: tong đọc 2 lần (RHS của chỗ trống + f-string),
  # gia đọc 1 lần (RHS của chỗ trống). Một lời giải đúng khác dùng += thay
  # vì =: `tong += gia` — AugAssign đánh dấu target là Store, không phải
  # Load, nên tong chỉ còn đếm được 1 lần (f-string) — min:1 mới ôm được
  # cả hai cách viết đúng. gia luôn đếm đúng 1 lần ở cả hai cách, vì gia
  # chỉ xuất hiện trong RHS của chỗ trống, không nơi nào khác trong khung.
  # Hằng số hardcode (tong = 95000) không đọc gia lần nào — 0 < 1, chặn được.
  - kind: uses-name, target: gia, min: 1
  - kind: uses-name, target: tong, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: "Tổng: 95000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một ô, chạm lại năm lần liên tiếp — đúng kiểu truy cập bộ nhớ đệm
mê nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bộ nhớ đệm đoán khá giỏi — nó giữ sẵn đúng những gì CPU nhiều khả năng
cần lại. Nhưng KHÔNG BAO GIỜ nó đoán đúng 100%. Sớm muộn CPU cũng cần một
giá trị KHÔNG có sẵn trong bộ nhớ đệm.

Lúc đó CPU phải đi đâu để lấy giá trị ấy — và việc đó tốn kém hơn lúc có
sẵn trong bộ nhớ đệm bao nhiêu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
