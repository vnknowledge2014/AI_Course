---
id: nen-tang.chuong-trinh-that.in-de-nhin-vao-ben-trong
title: In ra để nhìn vào bên trong
summary: "Khi máy không báo lỗi mà kết quả vẫn sai, chèn một dòng `print` tạm vào giữa chương trình để thấy giá trị THẬT ở điểm đó, thay vì đoán."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 24
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.print-debug]
requires: [core.main-guard, core.with-open, core.file-write, core.file-readlines, io.readlines, core.newline-char, core.strip-newline, core.string-split, core.int-cast, core.list, core.len, core.list-index, core.variable, core.assignment, core.fstring, core.output, core.print-variable, core.accumulator, ctrl.for-each, err.traceback]
concepts: [core.gia-tri-that, core.diem-giua-chung, core.con-mat-tam]
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
Máy không kêu ca gì cả. Vậy thì mình tự mở nắp ra nhìn.
::::

::::explain{#khong-co-ai-de-hoi-thi-hoi-chinh-chuong-trinh}
`python main.py xem so.txt` chạy êm, không một dòng báo lỗi, và in ra
`Tổng chi: 0 đồng` cho một cuốn sổ 12 khoản. Câu hỏi cuối bài trước là: hỏi ai?

Câu trả lời hơi phũ: **không có ai để hỏi**. Traceback mà bài 17 của Realm 0
dạy đọc chỉ xuất hiện khi máy phải dừng lại giữa chừng — nó là lời khai của
một chương trình vừa gục xuống. Chương trình này không gục. Với nó, mọi dòng
đều chạy trót lọt, mọi phép tính đều hợp lệ. Nó không biết là có gì sai, nên
nó không có gì để khai.

Cái sai nằm ở chỗ khác: **giá trị thật bên trong chương trình khác với giá trị
bạn tưởng nó đang giữ**. Bạn tưởng biến ấy đang giữ 12 khoản; có thể nó đang
giữ một thứ khác. Bạn tưởng con số kia đang cộng dồn; có thể nó không. Cả hai
điều "bạn tưởng" ấy đều không nằm trong máy — chúng nằm trong đầu bạn, và máy
không có cách nào đối chiếu giúp.

Nên việc phải làm là mở nắp ra nhìn. Công cụ đầu tiên bạn có sẵn từ bài 1 của
Realm 0: `print`. Từ trước tới giờ bạn dùng nó để **nói với người dùng**. Hôm
nay nó có thêm một việc thứ hai: **hỏi chính chương trình**.

Cách làm gọn tới mức gần như không có gì để dạy — và đó là điểm mạnh của nó:

- chọn một **điểm giữa chừng**, tức là một chỗ nằm sau đoạn bạn nghi và trước
  đoạn cho ra kết quả sai;
- chèn thêm một dòng `print` ngay tại đó, in ra cái tên bạn muốn biết;
- chạy lại, đọc con số hiện ra, so với con số bạn tưởng.

Ba mẹo nhỏ làm nó dễ đọc hơn nhiều:

- **In kèm nhãn.** `print(tong)` cho một con số trơ trọi giữa màn hình đầy
  chữ; `print("MẮT 1 — tong =", tong)` thì bạn biết con số ấy từ đâu ra.
- **In cả cái độ dài, đừng chỉ in nội dung.** Một chuỗi có ký tự thừa ở đuôi
  nhìn y hệt một chuỗi sạch, nhưng `len` của chúng thì khác nhau.
- **In ngay trước và ngay sau chỗ nghi.** Giá trị đúng ở trước mà sai ở sau
  nghĩa là thủ phạm nằm giữa hai con mắt ấy.

Những dòng `print` kiểu này là **tạm**: chúng vào để bạn nhìn, và ra khi bạn
nhìn xong. Chúng không phải một phần của chương trình bạn giao cho người dùng.
::::

::::example{#dat-mot-con-mat-vao-giua-vong-lap}
Một đoạn cộng tiền ngắn, với đúng một con mắt tạm đặt vào giữa vòng lặp.

```python title=readonly
cac_dong = ["cà phê,25000", "bún bò,40000"]

tong = 0
for dong in cac_dong:
    manh = dong.split(",")
    print("MẮT 1 — manh đang là:", manh)
    tong = tong + int(manh[1])

print("Tổng:", tong)
```

Máy in ra:

```text title=readonly
MẮT 1 — manh đang là: ['cà phê', '25000']
MẮT 1 — manh đang là: ['bún bò', '40000']
Tổng: 65000
```

Ba chỗ đáng dừng lại nhìn:

- **Con mắt nằm trong thân vòng lặp nên nó nói một lần cho mỗi vòng.** Hai
  khoản, hai dòng. Đó vừa là chỗ mạnh — bạn thấy giá trị đổi qua từng vòng —
  vừa là chỗ phiền: một cuốn sổ 12 khoản sẽ cho 12 dòng như thế.
- **Nó in ra thứ đang thật sự nằm trong `manh`**, chứ không in ra thứ đoạn code
  *lẽ ra* phải tạo ra. Đây là toàn bộ giá trị của việc chèn `print`: nó không
  đọc ý định của bạn, nó chỉ thuật lại hiện trạng.
- **Nhãn `MẮT 1` là do bạn tự đặt.** Máy không cần nó. Bạn cần, vì lát nữa
  màn hình sẽ có `MẮT 2` và `MẮT 3` nữa.
::::

::::predict{#doan-do-dai-dong-doc-len commitOnce}
Byte ghi một dòng xuống file rồi đọc ngay lên bằng `.readlines()`, và đặt một
con mắt vào đúng chỗ ấy để đo độ dài của dòng vừa đọc.

Chuỗi `cà phê,25000` có 12 ký tự — bạn đếm được bằng tay.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra con số nào?

```python title=readonly
with open("ghi_thu.txt", "w") as f:
    f.write("cà phê,25000\n")

with open("ghi_thu.txt", "r") as f:
    cac_dong = f.readlines()

print("MẮT — dòng đầu dài bao nhiêu ký tự:", len(cac_dong[0]))
```

:::opt{correct}
`13`
:::

:::opt
`12`
::why
Gần đúng ở chỗ bạn đếm đúng cái mình nhìn thấy: `cà phê,25000` đúng là 12 ký
tự, không hơn không kém. Phép đếm của bạn không sai một chữ nào.

Chỗ lệch là ký tự bạn **không nhìn thấy**. Dòng ghi xuống file có thêm `\n` ở
đuôi — chính bạn viết nó vào ở bài 5, để ba khoản khỏi dính liền một dòng. Và
`.readlines()` cắt file thành từng dòng nhưng **giữ nguyên** ký tự ấy ở đuôi
mỗi phần tử, đúng như bài 7 và bài 8 đã nói. Nên thứ máy đang cầm dài hơn thứ
bạn gõ đúng một ký tự.
::
:::

:::opt
`14`
::why
Gần đúng ở chỗ bạn nhớ ra ký tự xuống dòng và cộng nó vào — bước ấy đúng, và
nó là bước mà đáp án 12 bỏ sót.

Chỗ lệch là cộng bao nhiêu. Trên màn hình soạn thảo, `\n` chiếm hai chỗ nhìn
thấy được: một dấu gạch chéo ngược và một chữ n. Nhưng bài 6 đã nói rõ đó là
cách **viết ra** một ký tự duy nhất, giống như cách viết một chữ cái đặc biệt.
Máy đếm nó là một, nên tổng chỉ hơn 12 đúng một đơn vị.
::
:::

:::opt
`1`
::why
Gần đúng ở chỗ bạn tính đúng một con số có thật trong đoạn này: file chỉ có
một dòng, nên `cac_dong` quả thật đang giữ đúng một phần tử.

Chỗ lệch là `len` đang được hỏi về cái gì. Trong ngoặc không phải `cac_dong`
mà là `cac_dong[0]` — tức là đã lấy phần tử đầu ra rồi mới đo. Đo một danh
sách thì được số phần tử; đo một chuỗi thì được số ký tự. Chỗ này đang đo
chuỗi.
::
:::
::::

::::code{#dat-hai-con-mat-vao-cho-doc-so}
Đến lượt bạn lần theo con số 0 đồng.

Bước lần theo đầu tiên bao giờ cũng là bước rẻ nhất: **máy có thật sự đọc được
cuốn sổ không**. Nếu ngay chỗ này đã hỏng thì không cần soi tiếp phần cộng
tiền; còn nếu chỗ này lành thì bạn vừa loại được một nửa chương trình khỏi
diện nghi ngờ.

Cuốn sổ trong khung tập là cuốn sổ thật của bạn lúc này: mười khoản đã có từ
cuối track trước, cộng hai khoản vừa ghi thêm hôm nay — **12 khoản** tất cả,
đúng con số bạn đọc thấy khi mở file ra nhìn.

Hai chỗ trống là hai con mắt tạm:

- **MẮT 1** hỏi máy đang cầm bao nhiêu dòng;
- **MẮT 2** hỏi dòng đầu tiên nó đang cầm dài bao nhiêu ký tự.

Cả hai đều phải **đo trên chính `cac_dong`**. Chép sẵn một con số vào là bạn
đang in lại điều mình tưởng, mà điều mình tưởng chính là thứ đang cần kiểm.

```python title=starter
# ── Phần Byte làm sẵn: ghi cuốn sổ 12 khoản xuống file ───────────────
CAC_KHOAN = [
    "cà phê,25000",
    "sửa xe,500000",
    "bún bò,40000",
    "đổ xăng,100000",
    "vở ghi,15000",
    "sách,120000",
    "cơm trưa,35000",
    "gửi xe,10000",
    "khoá tiếng Anh,300000",
    "trà sữa,45000",
    "trà đá,5000",
    "in tài liệu,20000",
]

with open("so.txt", "w") as f:
    for khoan in CAC_KHOAN:
        f.write(khoan + "\n")

# ── Chỗ chương trình đọc sổ lên ──────────────────────────────────────
with open("so.txt", "r") as f:
    cac_dong = f.readlines()

# ── Hai con mắt tạm. Đừng đoán — hỏi máy. ────────────────────────────
so_dong_may_dang_cam = ___
do_dai_dong_dau = ___

print(f"MẮT 1 — máy đang cầm {so_dong_may_dang_cam} dòng")
print(f"MẮT 2 — dòng đầu dài {do_dai_dong_dau} ký tự")
```

```python title=solution
# ── Phần Byte làm sẵn: ghi cuốn sổ 12 khoản xuống file ───────────────
CAC_KHOAN = [
    "cà phê,25000",
    "sửa xe,500000",
    "bún bò,40000",
    "đổ xăng,100000",
    "vở ghi,15000",
    "sách,120000",
    "cơm trưa,35000",
    "gửi xe,10000",
    "khoá tiếng Anh,300000",
    "trà sữa,45000",
    "trà đá,5000",
    "in tài liệu,20000",
]

with open("so.txt", "w") as f:
    for khoan in CAC_KHOAN:
        f.write(khoan + "\n")

# ── Chỗ chương trình đọc sổ lên ──────────────────────────────────────
with open("so.txt", "r") as f:
    cac_dong = f.readlines()

# ── Hai con mắt tạm. Đừng đoán — hỏi máy. ────────────────────────────
so_dong_may_dang_cam = len(cac_dong)
do_dai_dong_dau = len(cac_dong[0])

print(f"MẮT 1 — máy đang cầm {so_dong_may_dang_cam} dòng")
print(f"MẮT 2 — dòng đầu dài {do_dai_dong_dau} ký tự")
```

```python title=test
# Chỗ trống 1: đo cả danh sách. Điền nhầm phép đo của MẮT 2 vào đây thì
# ra 13 và câu này vỡ.
assert so_dong_may_dang_cam == 12, "cuốn sổ ghi xuống 12 dòng, nên cac_dong phải đang giữ đúng 12 phần tử"
# Chỗ trống 2: đo phần tử ĐẦU của danh sách ấy. Điền nhầm phép đo của
# MẮT 1 vào đây thì ra 12 và câu này vỡ; đo nhầm sang phần tử thứ hai
# thì ra 14, cũng vỡ.
assert do_dai_dong_dau == 13, "dòng đầu là cà phê,25000 — 12 ký tự bạn gõ, cộng thêm ký tự xuống dòng nằm ở đuôi, thành 13"
# Hai con mắt là để NHÌN, không phải để sửa: cuốn sổ máy đang cầm phải
# còn nguyên như lúc đọc lên.
assert cac_dong[0] == "cà phê,25000\n", "dòng đầu mà máy đang cầm vẫn phải là cà phê,25000 kèm ký tự xuống dòng ở đuôi"
assert cac_dong[11] == "in tài liệu,20000\n", "dòng cuối trong 12 dòng ấy vẫn phải là in tài liệu,20000 kèm ký tự xuống dòng ở đuôi"
```

:::hints
- kind: attention
  body: "Cả hai con mắt đều đo bằng cùng một công cụ đo — cái bạn dùng từ T1.1 để hỏi một thứ dài bao nhiêu. Khác nhau nằm ở chỗ bạn đưa cái gì vào cho nó đo: một bên là cả cuốn danh sách, một bên là một dòng lấy ra từ cuốn ấy."
- kind: strategy
  body: "MẮT 1 đo thẳng `cac_dong`, và kết quả là số phần tử. MẮT 2 phải lấy phần tử đầu ra trước đã — đếm từ 0, đúng như T1.4 — rồi mới đo, và kết quả lần này là số ký tự của một chuỗi."
- kind: one-line
  body: "MẮT 1 viết `len(cac_dong)`, MẮT 2 viết `len(cac_dong[0])`."
:::

:::validate
- tier: static
  onFail: hai con mắt phải ĐO trên chính `cac_dong`, không chép sẵn con số nào vào
  requireAst:
  # `min: 2` cho cả hai luật, vì có hai chỗ trống và khung chưa đo gì lần nào:
  # `cac_dong` mới chỉ được GÁN, chưa được đọc. Nên hai luật này cùng lúc chặn
  # đáp án gõ bừa lẫn đáp án chép cứng con số 12 và 13.
  - kind: uses-call, target: len, min: 2
  - kind: uses-name, target: cac_dong, min: 2
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: "MẮT 1 — máy đang cầm 12 dòng"
- tier: output
  expect: "MẮT 2 — dòng đầu dài 13 ký tự"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai con mắt, hai câu trả lời thật. Chỗ đọc sổ lành lặn — vậy là còn nửa kia
để soi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hai con mắt vừa rồi trả lời gọn: máy cầm đủ 12 dòng, và dòng đầu dài 13 ký tự
— dài hơn đúng một, vì cái `\n` bạn tự tay ghi vào ở bài 5. Con số 13 ấy không
phải phát hiện mới; bài 8 đã nói `\n` dính ở đuôi mỗi dòng đọc lên. Cái mới là
bạn **nhìn thấy** nó ở đúng điểm giữa chừng, thay vì nhớ ra.

Chỗ đọc sổ lành. Nên con số 0 đồng sinh ra ở đâu đó phía sau, và bạn rắc tiếp
`print` đi tìm: một dòng trước vòng lặp, một dòng trong vòng lặp, một dòng sau
vòng lặp, rồi mấy dòng nữa bên `so_sach.py`. Chín dòng `print` nằm rải khắp hai
file, màn hình ngập chữ tới mức phải kéo lên mới thấy dòng đầu.

Rồi ba chuyện phiền chồng lên nhau. Sửa xong còn phải đi xoá từng dòng một —
lần trước bạn sót một dòng và nó theo vào bản giao cho người dùng. Mỗi lần
muốn xem thêm **một** biến nữa lại phải mở code ra sửa, rồi chạy lại từ đầu.
Và tệ nhất: bạn phải **đoán trước** mình sẽ cần nhìn cái gì, ngay lúc còn chưa
biết chuyện gì đang xảy ra.

Có cách nào bắt chương trình dừng lại ngay tại một điểm, rồi hỏi nó bất kỳ cái
tên nào bạn nghĩ ra lúc đó — mà không phải sửa code và chạy lại?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
