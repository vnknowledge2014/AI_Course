---
id: nen-tang.chuong-trinh-that.import-chay-ca-file
title: import chạy cả file được mượn
summary: "`import` không chép mỗi cái tên sang — nó đọc và chạy toàn bộ file được mượn, từ dòng đầu tới dòng cuối, đúng một lần."
locale: vi
track: nen-tang
module: chuong-trinh-that
order: 22
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.import-runs-file]
requires: [core.import-from, core.own-module, core.import-module, mod.import, mod.dotted-access, core.with-open, core.file-write, core.newline-char, core.list, core.len, core.function-def, core.function-call, core.function-return, core.variable, core.assignment, core.string-literal, core.fstring, core.output, core.print-variable]
concepts: [core.hop-do-nghe, core.le-trai, core.dan-ten]
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
Bạn thêm đúng một dòng vào sổ sách. Máy đọc luôn cả cuốn.
::::

::::explain{#cau-do-hien-len-truoc-khi-goi-ham}
Bài trước bạn lấy riêng một cái tên ra khỏi `so_sach.py`, rồi gõ thêm một dòng
`print("đang mở sổ...")` ở lề trái file ấy để thử xem nó có được đọc tới
không. Chạy `main.py` — chưa gọi hàm nào cả — mà câu đó đã hiện lên màn hình.

Đó không phải trục trặc. Đó là `import` đang làm đúng việc của nó, và việc ấy
lớn hơn cái tên "mượn một cái tên" nghe có vẻ gợi ra.

Khi Python gặp dòng `import so_sach`, nó làm ba chuyện, theo thứ tự:

1. đi tìm file `so_sach.py`;
2. **đọc và chạy toàn bộ file ấy**, từ dòng đầu xuống dòng cuối, y như bạn gõ
   `python so_sach.py`;
3. gom mọi cái tên vừa sinh ra trong lúc chạy, gói lại thành một hộp, rồi đặt
   hộp đó vào chương trình của bạn dưới cái tên `so_sach`.

Bước 2 là bước bị bỏ quên. Cái tên `doc_so` mà bạn muốn mượn không có sẵn ở đâu
cả — nó chỉ tồn tại **sau khi** dòng `def doc_so():` được chạy qua. Muốn có cái
tên, phải chạy cái file. Không có đường tắt nào khác.

Vậy còn dòng `def` thì sao — nó cũng chạy à? Có, và đây là chỗ đáng phân biệt
cho rõ:

- Một dòng ở lề trái **chạy thật**, ngay lúc đó. `print(...)` thì in ra màn
  hình. `SO_KHOAN = len(CAC_DONG)` thì đếm và cất kết quả.
- Dòng `def doc_so():` cũng là một dòng ở lề trái, nên nó cũng chạy thật. Chỉ
  có điều việc nó làm là **dán cái tên `doc_so` lên thân hàm** rồi thôi. Thân
  hàm nằm thụt vào trong, và phần thụt vào ấy chỉ chạy khi có người gọi —
  đúng như bài đầu của T1.3 đã nói.

Nên nhìn từ ngoài vào, `import` cho cảm giác lặng lẽ khi file được mượn chỉ
toàn `def`: mỗi dòng `def` chạy qua, dán một cái tên, không kêu tiếng nào. Thêm
một dòng `print` ở lề trái là bạn vừa cho cái file ấy một cái miệng.

Còn một chuyện nữa, và nó là nửa sau của luật: **đúng một lần**. Lần đầu gặp
`import so_sach`, Python chạy file rồi cất cái hộp lại. Những lần gặp sau — dù
ở file khác, dù cách đó hai chục dòng — nó thấy hộp đã có sẵn nên đưa lại hộp
cũ, không đọc lại file lần nào nữa.
::::

::::example{#hai-file-mot-lan-doc}
Hai file nằm cạnh nhau trong một thư mục.

```python title=readonly
# ── so_sach.py ──────────────────────────────────────────────────────
print("so_sach.py: đang mở sổ...")

CAC_DONG = ["cà phê,25000", "bún bò,40000", "gửi xe,10000"]
SO_KHOAN = len(CAC_DONG)


def doc_so():
    print("so_sach.py: doc_so vừa được gọi")
    return CAC_DONG
```

```python title=readonly
# ── main.py ─────────────────────────────────────────────────────────
import so_sach

print("main.py: bắt đầu")
print("main.py: sổ có", so_sach.SO_KHOAN, "khoản")
```

Gõ `python main.py`, máy in ra:

```text title=readonly
so_sach.py: đang mở sổ...
main.py: bắt đầu
main.py: sổ có 3 khoản
```

Ba chỗ đáng dừng lại nhìn:

- **Dòng của `so_sach.py` đứng TRƯỚC dòng đầu tiên của `main.py`.** Câu
  `print("main.py: bắt đầu")` là câu lệnh thứ hai của `main.py`, nhưng nó vẫn
  hiện sau. Vì câu lệnh thứ nhất — `import so_sach` — chưa xong việc: nó còn
  đang chạy cả một file khác.
- **`SO_KHOAN` đã bằng 3 mà không ai gọi gì.** Không có lời gọi hàm nào trong
  `main.py` cả. Con số ấy được tính lúc dòng `SO_KHOAN = len(CAC_DONG)` chạy
  qua, tức là lúc import.
- **Câu "doc_so vừa được gọi" không hiện.** Cái tên `doc_so` đã có mặt trong
  hộp rồi, nhưng thân hàm thì chưa chạy lần nào. Dán tên là một chuyện, gọi là
  chuyện khác.

Viết `from so_sach import doc_so` như bài trước cũng vậy thôi: dạng viết ấy chỉ
đổi chỗ cái tên đi đâu trong chương trình của bạn, còn bước "chạy cả file được
mượn" thì vẫn xảy ra nguyên vẹn.
::::

::::predict{#doan-hai-lan-import commitOnce}
Byte sửa `main.py`: viết hẳn hai dòng `import so_sach` cho chắc, vì hai chỗ
trong file đều cần tới sổ.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
# ── so_sach.py ──────────────────────────────────────────────────────
print("so_sach.py: đang mở sổ...")

CAC_DONG = ["cà phê,25000", "bún bò,40000", "gửi xe,10000"]
```

```python title=readonly
# ── main.py ─────────────────────────────────────────────────────────
import so_sach
import so_sach

print("main.py: bắt đầu")
```

:::opt{correct}
`so_sach.py: đang mở sổ...` một lần, rồi `main.py: bắt đầu`
:::

:::opt
`so_sach.py: đang mở sổ...` hai lần, rồi `main.py: bắt đầu`
::why
Gần đúng ở chỗ bạn theo dõi rất sát cái luật vừa học: `import` chạy cả file
được mượn, mà đây có hai dòng `import`, nên hai lần chạy. Suy luận ấy đi đúng
đường; nó chỉ thiếu nửa sau của luật.

Chỗ lệch: Python cất cái hộp lại sau lần chạy đầu tiên. Tới dòng `import` thứ
hai, nó nhìn vào chỗ cất, thấy hộp `so_sach` đã nằm sẵn ở đó, nên đưa lại đúng
hộp ấy và không mở file lần nào nữa. Dòng `print` ở lề trái chỉ có một cơ hội
để chạy, và cơ hội đó đã dùng xong.
::
:::

:::opt
Chỉ có `main.py: bắt đầu`, vì `import` chưa dùng tới cái tên nào nên chưa chạy gì
::why
Gần đúng ở chỗ bạn đang nghĩ theo một lối rất hợp lý: chưa cần thì chưa làm.
Có những công cụ trong lập trình đúng là làm việc theo lối ấy.

Chỗ lệch là `import` không thuộc số đó. Cái tên `so_sach.SO_KHOAN` hay
`so_sach.doc_so` chỉ tồn tại **sau khi** file kia đã chạy xong — trước đó
không có gì để mà lấy. Nên Python phải chạy trước, chạy ngay tại dòng `import`,
rồi mới có hộp để đưa cho bạn.
::
:::

:::opt
`main.py: bắt đầu` trước, rồi `so_sach.py: đang mở sổ...`
::why
Gần đúng ở chỗ bạn đọc đúng thứ tự các dòng trong `main.py` theo một nghĩa nào
đó: dòng `print` của `main.py` là dòng có chữ "bắt đầu", nghe như nó phải hiện
đầu tiên.

Chỗ lệch là ở chỗ máy đọc từ trên xuống, không đọc theo nghĩa của chữ. `import`
nằm ở dòng trên, nên nó chạy trước, và nó chỉ trả quyền lại cho `main.py` sau
khi đã chạy hết file `so_sach.py`. Chữ "bắt đầu" là bạn đặt cho mình đọc, máy
không nhìn tới.
::
:::
::::

::::code{#muon-so-sach-nho}
Đến lượt bạn nhìn tận mắt.

Khung tập này chỉ có một ô nhập, mà `import` thì cần hai file nằm cạnh nhau.
Nên đoạn đầu cho chương trình **tự ghi ra** file thứ hai — tên nó là
`so_sach_nho.py`, một bản sổ sách rút gọn ba dòng — bằng đúng động tác
`with open(...)` bạn học ở bài 3. Đoạn ấy không có chỗ trống nào; bạn chỉ cần
đọc để biết file kia chứa gì.

Hai chỗ trống nằm ở hai câu hỏi khác nhau:

- một câu hỏi thứ **đã có sẵn ngay sau `import`**, do một dòng ở lề trái của
  file kia chạy qua;
- một câu hỏi thứ **chỉ có khi bạn gọi hàm**.

```python title=starter
# ── Phần Byte làm sẵn: ghi ra file so_sach_nho.py ────────────────────
with open("so_sach_nho.py", "w") as f:
    f.write('print("so_sach_nho.py: đang mở sổ...")\n')
    f.write('\n')
    f.write('CAC_DONG = ["cà phê,25000", "bún bò,40000", "gửi xe,10000"]\n')
    f.write('SO_KHOAN = len(CAC_DONG)\n')
    f.write('\n')
    f.write('def doc_so():\n')
    f.write('    print("so_sach_nho.py: doc_so vừa được gọi")\n')
    f.write('    return CAC_DONG\n')

# ── Từ đây là chương trình của bạn ───────────────────────────────────
import so_sach_nho

# Chưa gọi hàm nào. Nhưng dòng `SO_KHOAN = len(CAC_DONG)` nằm ở lề trái
# của file kia, nên nó đã chạy rồi. Lấy con số ấy ra khỏi hộp.
so_khoan_ngay_sau_import = ___

# Bây giờ mới tới lượt gọi hàm.
cac_dong = ___

print(f"Ngay sau import, SO_KHOAN đã bằng {so_khoan_ngay_sau_import}")
print(f"Gọi hàm xong mới cầm được {len(cac_dong)} dòng")
```

```python title=solution
# ── Phần Byte làm sẵn: ghi ra file so_sach_nho.py ────────────────────
with open("so_sach_nho.py", "w") as f:
    f.write('print("so_sach_nho.py: đang mở sổ...")\n')
    f.write('\n')
    f.write('CAC_DONG = ["cà phê,25000", "bún bò,40000", "gửi xe,10000"]\n')
    f.write('SO_KHOAN = len(CAC_DONG)\n')
    f.write('\n')
    f.write('def doc_so():\n')
    f.write('    print("so_sach_nho.py: doc_so vừa được gọi")\n')
    f.write('    return CAC_DONG\n')

# ── Từ đây là chương trình của bạn ───────────────────────────────────
import so_sach_nho

# Chưa gọi hàm nào. Nhưng dòng `SO_KHOAN = len(CAC_DONG)` nằm ở lề trái
# của file kia, nên nó đã chạy rồi. Lấy con số ấy ra khỏi hộp.
so_khoan_ngay_sau_import = so_sach_nho.SO_KHOAN

# Bây giờ mới tới lượt gọi hàm.
cac_dong = so_sach_nho.doc_so()

print(f"Ngay sau import, SO_KHOAN đã bằng {so_khoan_ngay_sau_import}")
print(f"Gọi hàm xong mới cầm được {len(cac_dong)} dòng")
```

```python title=test
# Chỗ trống 1 bị soi bởi câu ngay dưới đây. Chương trình của bạn không
# dựng danh sách nào và không đếm gì cả, nên con số 3 chỉ có thể tới từ
# một dòng đã chạy sẵn bên trong file được mượn.
assert so_khoan_ngay_sau_import == 3, "so_sach_nho.py giữ ba dòng sổ, và dòng SO_KHOAN = len(CAC_DONG) ở lề trái file ấy đã chạy ngay lúc import, nên ngay sau import con số này phải là 3"
# Chỗ trống 2 bị soi bởi hai chỗ: câu dưới đây đòi đúng ba dòng của cuốn
# sổ, còn luật output đòi câu "doc_so vừa được gọi" phải hiện ra — mà câu
# ấy chỉ hiện khi thân hàm thật sự chạy. Lấy thẳng so_sach_nho.CAC_DONG
# thì qua được câu này nhưng trượt luật kia.
assert cac_dong == ["cà phê,25000", "bún bò,40000", "gửi xe,10000"], "doc_so đưa lại đúng ba dòng mà so_sach_nho.py đang giữ, theo thứ tự cà phê,25000 rồi bún bò,40000 rồi gửi xe,10000"
# Đọc một cái hộp không làm hộp ấy đổi.
assert so_sach_nho.SO_KHOAN == 3, "sau khi bạn gọi hàm, hộp so_sach_nho vẫn phải giữ nguyên SO_KHOAN bằng 3"
```

:::hints
- kind: attention
  body: "Sau dòng `import so_sach_nho`, mọi thứ mà file kia sinh ra đều nằm trong một cái hộp mang đúng cái tên ấy. Chỗ trống thứ nhất hỏi một con số đã nằm sẵn trong hộp; chỗ trống thứ hai hỏi một việc phải nhờ hộp làm giúp."
- kind: strategy
  body: "Cách lấy đồ trong hộp là viết tên hộp, một dấu chấm, rồi tên món đồ — đúng như bài 20. Với chỗ trống thứ hai, món đồ là một cái hàm, mà một cái hàm chỉ chạy khi có cặp ngoặc đi kèm phía sau; thiếu cặp ngoặc thì bạn mới chỉ cầm cái hàm chứ chưa gọi nó."
- kind: one-line
  body: "Chỗ trống thứ nhất viết `so_sach_nho.SO_KHOAN`, chỗ thứ hai viết `so_sach_nho.doc_so()` — khác nhau đúng cặp ngoặc ở cuối."
:::

:::validate
- tier: static
  onFail: cả hai chỗ trống đều phải đi qua hộp `so_sach_nho`, và chỗ thứ hai phải GỌI hàm chứ không lấy thẳng danh sách
  requireAst:
  # `min: 2` vì có hai chỗ trống, và cái tên `so_sach_nho` chưa được ĐỌC lần
  # nào trong khung — dòng `import` không tính là đọc. Nên luật này chặn cả
  # con số 3 chép tay lẫn danh sách gõ lại bằng tay.
  - kind: uses-name, target: so_sach_nho, min: 2
  # Và chỗ trống thứ hai phải là một lời GỌI, không phải một phép lấy đồ.
  - kind: uses-call, target: doc_so, min: 1
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: "Ngay sau import, SO_KHOAN đã bằng 3"
- tier: output
  expect: "Gọi hàm xong mới cầm được 3 dòng"
- tier: output
  expect: "so_sach_nho.py: doc_so vừa được gọi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mượn một file là chạy cả file đó. Giờ thì câu chào của nó không còn bí ẩn nữa.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vậy là rõ: mọi dòng ở lề trái của `so_sach.py` đều chạy, mỗi khi có ai đó
import nó. Câu chào, phép đếm, và cả những dòng bạn viết ra chỉ để thử cho
nhanh.

Mà bạn thì vẫn muốn giữ mấy dòng thử ấy. Gõ `python so_sach.py` một cái là
thấy ngay sổ đang có mấy khoản — tiện hơn nhiều so với việc mở một file khác
ra chỉ để kiểm tra. Bỏ chúng đi thì mất chỗ thử; để nguyên thì `main.py` phải
gánh chúng mỗi lần chạy.

Làm sao để phần thử **chỉ** chạy khi bạn gọi thẳng file ấy, và nằm im khi nó
bị mượn?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
