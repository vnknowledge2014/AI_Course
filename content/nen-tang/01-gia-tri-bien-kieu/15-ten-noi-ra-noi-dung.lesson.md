---
id: nen-tang.gia-tri-bien-kieu.ten-noi-ra-noi-dung
title: Cái tên phải nói ra nội dung
summary: Sau 35 từ khoá thì máy hết ý kiến — người đọc là người còn lại. Tên tốt là tên nói ra nội dung, không nói kiểu, không nói thứ tự.
locale: vi
track: nen-tang
module: gia-tri-bien-kieu
order: 15
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.naming-convention]
requires: [core.variable, core.assignment, core.name-rules, core.builtin-shadowing, core.type-fn, core.fstring, core.arithmetic, core.floor-division]
concepts: [core.ten, core.doc-lai-ma]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Tên nào mình cũng đọc được như nhau. Người mở lại file này ba tháng nữa thì không.
::::

::::explain{#ai-con-lai}
Bài trước kết bằng câu hỏi: qua được ba cửa của máy rồi thì mọi cái tên đều
như nhau với nó — vậy ai là người còn lại quyết định tên nào tốt?

**Người đọc.** Và người đọc đầu tiên của code bạn viết hôm nay, chín trên mười
lần, chính là bạn của ba tháng sau.

Trong bếp quán phở có một dãy hộp inox giống hệt nhau. Chủ quán dán nhãn "hộp
1", "hộp 2", "hộp 3" — đúng luật, không ai cấm, và trong tuần đầu chủ quán nhớ
vanh vách hộp nào đựng gì. Sang tháng sau, đến giờ cao điểm, muốn lấy hành phi
thì phải mở ba hộp. Dãy hộp bên cạnh dán "hành phi", "tương ớt", "quế hồi" —
với tay là trúng.

Cái nhãn không đổi thứ nằm trong hộp. Nó đổi thời gian bạn cần để tìm ra hộp
đúng, và số lần bạn lấy nhầm hộp.

Cái tên trong code cũng vậy. Máy đã hết phần việc của nó ở bài trước; từ đây
trở đi, cái tên là thứ **viết cho người**.
::::

::::predict{#doan-may-co-phan-biet-khong commitOnce}
Hai đoạn dưới đây là cùng một hoá đơn: ba tô phở 45000đ một tô, cộng thêm
12000đ tiền gửi xe. Chỉ khác mấy cái tên. **Trước khi bấm chạy**, bạn đoán máy
in ra gì?

```python
d1 = 45000
d2 = 3
d3 = 12000
t = d1 * d2 + d3
print(t)
```

```python
gia_mot_to_pho = 45000
so_to = 3
tien_gui_xe = 12000
tong_hoa_don = gia_mot_to_pho * so_to + tien_gui_xe
print(tong_hoa_don)
```

:::opt{correct}
Cả hai in ra `147000` — không khác nhau chỗ nào
:::

:::opt
Đoạn trên in `147000`, đoạn dưới báo lỗi vì `so_to` trùng một tên có sẵn
::why
Gần đúng ở chỗ bạn đang dùng đúng phản xạ mới học: trước khi tin một cái tên,
kiểm xem nó có đè lên tên máy dọn sẵn không. Phản xạ ấy đáng giữ.

Chỗ lệch nằm ở chỗ bảng tên có sẵn toàn từ tiếng Anh — `print`, `input`, `int`,
`round`. Không có mục nào tên `so_to`. Dùng luôn mẹo của bài trước để tự trả
lời: gõ `print(so_to)` ở một file trắng, máy cho `NameError`, nghĩa là chỗ ấy
trống.
::
:::

:::opt
Cả hai in `147000`, nhưng đoạn dưới chạy chậm hơn vì tên dài, máy phải đọc nhiều chữ hơn
::why
Gần đúng ở chỗ bạn nhớ rằng máy có đọc từng chữ trong cái tên thật — nó phải
đọc thì mới biết `so_to` với `so_tien` là hai cái khác nhau.

Chỗ lệch nằm ở **lúc nào** nó đọc. Việc đọc chữ xảy ra một lần duy nhất, ở
bước soát, trước khi chạy — đúng bước đã tóm cái tên hỏng ở bài trước. Xong
bước đó, mỗi cái tên đã thành một chỗ trên bảng tên, và độ dài của nó không
còn có mặt trong lúc chạy nữa.
::
:::

:::opt
Máy nhắc một câu rằng `d1`, `d2`, `d3` là tên quá ngắn, rồi vẫn in `147000`
::why
Gần đúng ở chỗ bạn mong có ai đó nhắc mình chuyện này — và mong ấy hợp lý: có
những công cụ riêng chuyên đọc code rồi nhắc đúng những câu như vậy, dân lập
trình dùng chúng hằng ngày.

Chỗ lệch: Python lúc chạy không phải một công cụ như thế. Sau 35 từ khoá, nó
hết ý kiến — `d1` và `gia_mot_to_pho` với nó là hai cái tên ngang hàng.
::
:::
::::

::::example{#doc-lai-sau-ba-thang}
Máy trả lời xong rồi. Giờ tới lượt câu hỏi dành cho người — và đây mới là câu
hỏi thật của bài này.

Nhìn lại đoạn thứ nhất, đừng nhìn xuống đoạn thứ hai:

```python title=readonly
d1 = 45000
d2 = 3
d3 = 12000
t = d1 * d2 + d3
print(t)
```

Trả lời giúp Byte ba câu:

- `d3` là tiền gì?
- Dòng 4 nhân `d1` với `d2` — vì sao nhân hai cái đó với nhau mà không phải hai
  cái khác?
- Nếu hôm nay quán tăng giá, bạn sửa dòng nào?

Không đoán ra được, và không phải tại bạn: trong năm dòng đó không có chỗ nào
chứa câu trả lời. Muốn biết, phải có người kể lại — hoặc phải dò ngược ra hoá
đơn gốc.

Bây giờ đọc đoạn thứ hai:

```python title=readonly
gia_mot_to_pho = 45000
so_to = 3
tien_gui_xe = 12000
tong_hoa_don = gia_mot_to_pho * so_to + tien_gui_xe
print(tong_hoa_don)
```

Ba câu trả lời nằm sẵn trên màn hình. `tien_gui_xe` là tiền gửi xe. Dòng 4
nhân giá một tô với số tô, vì đó là tiền phở. Quán tăng giá thì sửa dòng 1.

Hai đoạn code cho máy là một. Cho người thì một đoạn cần người kể, một đoạn tự
kể.

Ba họ tên tệ hay gặp nhất, cả ba đều hợp lệ với máy:

| tên tệ | nó nói gì | vì sao hụt |
|---|---|---|
| `t`, `x1`, `data`, `dulieu` | không nói gì | phải đọc ngược lên mới biết trong đó là cái gì |
| `str_ten`, `int_tien` | nói **kiểu** | kiểu thì `type()` trả lời được bất cứ lúc nào, mà nội dung thì không |
| `so1`, `so2`, `mon1`, `mon2` | nói **thứ tự** | thêm `so3` là bạn phải nhớ số nào ứng với cái gì |

Họ thứ hai còn có một cái bẫy riêng: kiểu **đổi được**. Ô tiền trên phiếu lúc
mới gõ vào là chữ, đổi sang số xong thì cái tên `str_tien` bắt đầu nói dối —
và một cái tên nói dối còn tệ hơn một cái tên không nói gì.
::::

::::explain{#luat-cung-va-quy-uoc-mem}
Chốt lại một cách gọi. Ba luật của bài trước là **luật cứng**: máy giữ, phạm
là không chạy. Những gì bài này nói là **quy ước** — người giữ với nhau, máy
không kiểm.

Quy ước gồm hai vế.

**Vế thứ nhất — hình dạng chữ.** Dân Python viết tên biến bằng chữ thường hết,
các từ nối nhau bằng dấu `_`: `tien_ca_phe`, `so_nguoi_chia`, `tong_hoa_don`.
Cách viết này có tên riêng: **snake_case** — kiểu con rắn, vì cái tên nằm dài
một mạch sát đất, không có chữ nào nhô lên. Viết `tienCaPhe` thì máy vẫn nhận,
nhưng nó lạc lõng giữa một file Python, và mắt người đọc phải dừng lại đúng
một nhịp mỗi lần gặp — nhịp dừng ấy nhân với hai trăm dòng là một buổi chiều.

Trong khoá này, tên biến viết bằng tiếng Việt **không dấu**: `tien_ca_phe`,
không phải `tiền_cà_phê`. Máy nhận cả hai (bài trước đã thử rồi), nhưng bản
không dấu thì gõ nhanh hơn và tìm kiếm trong file dễ hơn.

**Vế thứ hai — cái tên nói ra nội dung.** Đây là vế đáng giá. Ba câu hỏi để tự
soát một cái tên:

- Đọc riêng cái tên, không nhìn dòng nào khác, có biết trong đó là gì không?
- Nó có đang nói kiểu (`str_`, `int_`) thay vì nói nội dung không?
- Nó có đang nói thứ tự (`1`, `2`) thay vì nói nội dung không?

Và một chỗ đừng quá đà: `tien_moi_nguoi_phai_tra_sau_khi_da_tru_phan_giam_gia`
thì nói đủ thật, nhưng dài tới mức dòng code gãy làm đôi. Chọn cái tên **ngắn
nhất mà vẫn nói đủ** — `tien_moi_nguoi` thường là đủ.

Có đúng một chỗ cái tên ngắn cũn lại là tên tốt: biến chạy vòng lặp mà đời của
nó gói gọn trong hai dòng, như `i` trong `for i in range(3)`. Ngoài chỗ đó ra,
đặt tên như dán nhãn hộp gia vị.
::::

::::code{#dat-lai-ten-cho-dong-so}
Byte ghi lại một lần trả tiền: khách đưa **200000đ**, hoá đơn hết **145000đ**,
và dòng cuối in số tiền thối lại.

Đoạn dưới chạy được, máy không kêu gì. Nhưng nó in ra `Thối lại -55000đ` —
thối lại một số âm.

Đặt lại tên cho ba giá trị: `tien_khach_dua`, `tien_hoa_don`, `tien_thoi_lai`.
Rồi đọc lại dòng 3 bằng tên mới và sửa nốt cái đang sai ở đó.

```python title=starter
a = 200000
b = 145000
c = b - a

print(f"Thối lại {c}đ")
```

```python title=solution
tien_khach_dua = 200000
tien_hoa_don = 145000
tien_thoi_lai = tien_khach_dua - tien_hoa_don

print(f"Thối lại {tien_thoi_lai}đ")
```

```python title=test
# Hai chuyện phải cùng đúng, nên phải hỏi bằng nhiều hơn một câu.
# Hai câu đầu: hai con số vẫn nằm đúng chỗ của chúng — ai đổi tên mà gán nhầm
# 145000 cho `tien_khach_dua` thì trượt ở đây.
# Câu thứ ba: phép trừ đã quay đúng chiều. Chỉ đổi tên mà giữ nguyên
# `tien_hoa_don - tien_khach_dua` thì vẫn ra -55000 và trượt đúng câu này.
assert tien_khach_dua == 200000, "khách đưa tờ 200 nghìn, nên con số ấy thuộc về ô tiền khách đưa"
assert tien_hoa_don == 145000, "hoá đơn hết 145 nghìn, nên con số ấy thuộc về ô hoá đơn chứ không phải ô tiền khách đưa"
assert tien_thoi_lai == 55000, "khách đưa 200 nghìn cho hoá đơn 145 nghìn thì phải thối lại 55 nghìn, và thối lại thì không bao giờ là số âm"
```

:::hints
- kind: attention
  body: Đổi tên ba dòng trước đã, đừng vội sửa phép tính. Xong rồi đọc to dòng 3 lên bằng tên mới — nó đang nói "tiền thối lại bằng tiền hoá đơn trừ tiền khách đưa". Câu đó có đúng với chuyện xảy ra ở quầy không?
- kind: strategy
  body: Khách đưa nhiều hơn hoá đơn, nên số thối lại phải là số dương. Muốn ra dương thì số lớn phải đứng trước dấu trừ. Đây là con bọ nằm sẵn trong đoạn code từ đầu — nó chỉ lộ ra khi cái tên nói đủ nội dung để bạn đọc dòng 3 thành một câu tiếng Việt.
- kind: one-line
  body: Ba dòng đầu thành `tien_khach_dua = 200000`, `tien_hoa_don = 145000`, `tien_thoi_lai = tien_khach_dua - tien_hoa_don`; dòng cuối in `tien_thoi_lai`.
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Thối lại 55000đ
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Con bọ nằm đó từ đầu. Đổi tên xong nó tự bò ra — mình chẳng phải chỉ chỗ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Mọi giá trị **có tên** trong chương trình của bạn giờ đã có một cái tên nói ra
nội dung. Nhưng thử nhìn dòng này:

```python
phai_tra_pho = tien_pho + tien_pho * 10 // 100
```

`tien_pho` có tên, và tên ấy nói ra nội dung. `phai_tra_pho` cũng vậy. Còn số
`10` thì không có tên nào cả — nó nằm trần giữa công thức, không nhãn, không
ai giới thiệu.

Ba tháng sau bạn mở lại file: `10` ấy là thuế, là chiết khấu khách quen, hay
là phần trăm phí ship? Và nếu nó nằm ở bốn dòng khác nhau trong file, hôm mức
thuế đổi, bạn có chắc mình sửa đủ bốn chỗ — và không sửa nhầm một chỗ vốn là
chiết khấu?

Luật của bài này vừa chỉ ra một lỗ hổng mà chính nó không vá được: nó chỉ nói
được về những giá trị **có tên**.

Vậy một con số trần như thế thì đặt tên thế nào, và đặt ở đâu?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
