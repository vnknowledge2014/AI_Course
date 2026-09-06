---
id: tri-tue-nhan-tao.bm25-va-tim-kiem-tu-khoa.tan-so-tu-tho-va-thien-vi-do-dai
title: "Tần số từ thô (TF): thiên vị đoạn dài"
summary: "dem_tan_so(tu, van_ban) dem so lan mot tu xuat hien trong van ban da tach tu (.lower().split()). DOAN_NGAN (11 tu) va DOAN_DAI (64 tu) deu la doan noi ve python; DOAN_NGAN dung tu 'python' dung 1 lan, DOAN_DAI dung 'python' dung 6 lan (do lap lai nhieu, khong phai do lien quan hon toi python). 6 > 1 -- do bang so THAT: TF tho thien vi doan DAI, giong het han che da do o q8.5b bai 1 cho dot product, gio ap dung lai cho tan so tu."
locale: vi
track: tri-tue-nhan-tao
module: bm25-va-tim-kiem-tu-khoa
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.tan-so-tu-tho-va-thien-vi-do-dai]
requires: [ai.boss-do-recall-va-toc-do-so-voi-vet-can]
concepts: [ai.tan-so-tu-tho-va-thien-vi-do-dai]
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
Quest `hnsw-tu-cai` (q8.5c) khép lại ở `8/8`: một chỉ mục đồ thị tự cài, tìm
đúng đoạn văn theo hướng VECTOR. Nhưng có một cách tìm khác, cũ hơn nhiều và
vẫn còn dùng khắp nơi: tìm theo TỪ KHOÁ. Quest này bắt đầu từ viên gạch đầu
tiên của nó.
::::

::::explain{#tan_so_tu_la_gi}
Mọi phép tìm kiếm VECTOR đã học (q8.5a đến q8.5c) đều đi qua một bước trung
gian: biến văn bản thành một dãy số, rồi so hai dãy số với nhau. Tìm kiếm
theo TỪ KHOÁ bỏ qua bước trung gian đó — nó hỏi thẳng một câu đơn giản hơn
nhiều: **một từ cụ thể xuất hiện trong một đoạn văn bao nhiêu lần?**

Con số đó gọi là **tần số từ** (term frequency, viết tắt **TF**). Cách tính:
tách đoạn văn thành danh sách từ (`.lower().split()`), rồi đếm số lần một từ
khoá cụ thể xuất hiện trong danh sách đó. Ý tưởng dùng TF để xếp hạng: đoạn
văn nào có TF CAO HƠN cho một từ khoá thì được coi là "liên quan hơn" tới từ
khoá đó.

Nhưng TF thô mắc đúng lỗ hổng mà `on-lai-dot-product-va-han-che-do-dai`
(q8.5b, bài `1`) đã đo cho tích vô hướng: **một đoạn văn CÀNG DÀI thì càng có
xu hướng lặp lại một từ NHIỀU LẦN hơn — không phải vì nó liên quan hơn, mà
đơn giản vì nó có nhiều CHỖ hơn để từ đó xuất hiện.** Một đoạn ngắn, khớp sát
với một chủ đề, dùng đúng một từ khoá đúng một lần vẫn có thể thua một đoạn
dài, lan man, nhưng tình cờ lặp lại từ khoá đó nhiều lần.
::::

::::example{#do_thien_vi_do_dai_bang_so}
Hai đoạn văn, cả hai đều nói về `python`: một đoạn NGẮN, đi thẳng vào việc
học `python`; một đoạn DÀI hơn nhiều, cũng nói về `python` nhưng lặp lại
tên đó nhiều lần trong lúc liệt kê nhiều ý khác nhau:

```python title=readonly
def dem_tan_so(tu, van_ban):
    cac_tu = van_ban.lower().split()
    return cac_tu.count(tu)


DOAN_NGAN = "toi hoc python de viet chuong trinh don gian moi ngay"
DOAN_DAI = (
    "python la mot ngon ngu lap trinh duoc nhieu nguoi lua chon hoc "
    "nhieu nguoi thich python vi python de hoc va de doc "
    "python co rat nhieu thu vien mo rong huu ich cho cong viec "
    "python duoc dung trong web va khoa hoc du lieu va tu dong hoa "
    "hoc python khong mat qua nhieu thoi gian neu kien tri moi ngay"
)

tf_ngan = dem_tan_so("python", DOAN_NGAN)
tf_dai = dem_tan_so("python", DOAN_DAI)

print(len(DOAN_NGAN.split()))
print(len(DOAN_DAI.split()))
print(tf_ngan)
print(tf_dai)
print(tf_dai > tf_ngan)
```

```text title=readonly
11
64
1
6
True
```

`DOAN_NGAN` chỉ có `11` từ, và nhắc tới `python` đúng `1` lần. `DOAN_DAI` dài
gấp gần `6` lần (`64` từ), và nhắc tới `python` những `6` lần — không phải vì
nó "về python" nhiều hơn `DOAN_NGAN` (cả hai đoạn đều chỉ nói về việc học và
dùng `python`), mà đơn giản vì nó DÀI hơn, có nhiều câu hơn, nên từ `python`
có nhiều cơ hội xuất hiện lại hơn. Kết quả: `tf_dai > tf_ngan` — nếu xếp hạng
hai đoạn này chỉ bằng TF thô cho từ khoá `"python"`, `DOAN_DAI` sẽ thắng, dù
không có lý do nội dung nào để nó "liên quan hơn" `DOAN_NGAN`.
::::

::::predict{#doan_tf_thien_vi_doan_dai commitOnce}
Xét đúng ví dụ trên: `DOAN_NGAN` (`11` từ) nhắc `"python"` đúng `1` lần.
`DOAN_DAI` (`64` từ, dài gần `6` lần) nhắc `"python"` đúng `6` lần.

**Trước khi chạy thử**, bạn đoán: xếp hạng chỉ bằng TF thô cho từ khoá
`"python"`, đoạn nào được coi là "liên quan hơn"?

:::opt{correct}
`DOAN_DAI` — TF thô chỉ đếm SỐ LẦN xuất hiện, không chia cho độ dài đoạn văn;
đoạn dài có nhiều câu hơn nên tự nhiên có nhiều cơ hội lặp lại một từ hơn,
bất kể nội dung có thực sự "về" từ đó nhiều hơn hay không
:::

:::opt
`DOAN_NGAN` — vì nó đi thẳng vào việc học `python`, không lan man sang các ý
khác như thư viện, web, khoa học dữ liệu; nội dung khớp sát chủ đề hơn hẳn
::why
Gần đúng ở trực giác NỘI DUNG: `DOAN_NGAN` thật sự khớp sát và không lan
man — quan sát đó đúng nếu đọc bằng mắt người.

Chỗ lệch: `dem_tan_so` không đọc hiểu nội dung, nó chỉ đếm chuỗi ký tự
`"python"` xuất hiện bao nhiêu lần trong danh sách từ đã tách. Việc `DOAN_DAI`
"lan man" sang nhiều ý khác không hề làm giảm số lần nó lặp lại từ
`"python"` — ngược lại, càng nhiều câu thì càng nhiều cơ hội lặp lại, nên TF
thô của nó vẫn cao hơn.
::
:::

:::opt
Bằng nhau — vì cả hai đoạn đều nói về đúng một chủ đề duy nhất (`python`),
nên tần số từ của từ khoá chính phải tỉ lệ với nhau theo cùng một mức
::why
Gần đúng ở việc CẢ HAI đoạn đúng là chỉ nói về một chủ đề (`python`) — quan
sát về chủ đề đó đúng.

Chỗ lệch: "cùng một chủ đề" không kéo theo "cùng tần số". Tần số phụ thuộc
vào SỐ LẦN từ đó được VIẾT RA, và không có quy tắc nào buộc một đoạn dài
phải lặp lại từ khoá theo đúng tỉ lệ với một đoạn ngắn — ở đây `DOAN_DAI`
dài gần `6` lần `DOAN_NGAN` nhưng lặp `"python"` nhiều gấp `6` lần, một sự
trùng hợp về tỉ lệ, không phải quy luật.
::
:::
::::

::::code{#viet_dem_tan_so_va_so_sanh}
Hoàn thiện `dem_tan_so` (đếm số lần một từ xuất hiện trong văn bản đã tách
từ) và `dai_hon_thi_tf_cao_hon` (so sánh xem đoạn dài có TF cao hơn đoạn
ngắn hay không).

```python title=starter
def dem_tan_so(tu, van_ban):
    cac_tu = van_ban.lower().split()
    return cac_tu.count(___)                                # tu


def dai_hon_thi_tf_cao_hon(tf_ngan, tf_dai):
    return ___                                              # tf_dai > tf_ngan


DOAN_NGAN = "toi hoc python de viet chuong trinh don gian moi ngay"
DOAN_DAI = (
    "python la mot ngon ngu lap trinh duoc nhieu nguoi lua chon hoc "
    "nhieu nguoi thich python vi python de hoc va de doc "
    "python co rat nhieu thu vien mo rong huu ich cho cong viec "
    "python duoc dung trong web va khoa hoc du lieu va tu dong hoa "
    "hoc python khong mat qua nhieu thoi gian neu kien tri moi ngay"
)

tf_ngan = dem_tan_so("python", DOAN_NGAN)
tf_dai = dem_tan_so("python", DOAN_DAI)

print(len(DOAN_NGAN.split()))
print(len(DOAN_DAI.split()))
print(tf_ngan)
print(tf_dai)
print(dai_hon_thi_tf_cao_hon(tf_ngan, tf_dai))
```

```python title=solution
def dem_tan_so(tu, van_ban):
    cac_tu = van_ban.lower().split()
    return cac_tu.count(tu)


def dai_hon_thi_tf_cao_hon(tf_ngan, tf_dai):
    return tf_dai > tf_ngan


DOAN_NGAN = "toi hoc python de viet chuong trinh don gian moi ngay"
DOAN_DAI = (
    "python la mot ngon ngu lap trinh duoc nhieu nguoi lua chon hoc "
    "nhieu nguoi thich python vi python de hoc va de doc "
    "python co rat nhieu thu vien mo rong huu ich cho cong viec "
    "python duoc dung trong web va khoa hoc du lieu va tu dong hoa "
    "hoc python khong mat qua nhieu thoi gian neu kien tri moi ngay"
)

tf_ngan = dem_tan_so("python", DOAN_NGAN)
tf_dai = dem_tan_so("python", DOAN_DAI)

print(len(DOAN_NGAN.split()))
print(len(DOAN_DAI.split()))
print(tf_ngan)
print(tf_dai)
print(dai_hon_thi_tf_cao_hon(tf_ngan, tf_dai))
```

```python title=test
assert tf_ngan == 1, f"tf_ngan phai la 1 -- dang ra {tf_ngan}"
assert tf_dai == 6, f"tf_dai phai la 6 -- dang ra {tf_dai}"
assert dai_hon_thi_tf_cao_hon(tf_ngan, tf_dai) == True, "doan DAI dang co TF cao hon doan NGAN -- day chinh la han che can do"

# kiem tra truc tiep ham dem_tan_so tren vi du nho, tu dem tay duoc
assert dem_tan_so("a", "a b a c a") == 3, f"'a' xuat hien 3 lan -- dang ra {dem_tan_so('a', 'a b a c a')}"
assert dem_tan_so("x", "a b c") == 0, "tu khong xuat hien phai cho tan so 0"
assert dem_tan_so("tu", "") == 0, "van ban rong phai cho tan so 0, khong loi"

# bien dai_hon_thi_tf_cao_hon: gia tri bang nhau hoac thap hon phai cho False
assert dai_hon_thi_tf_cao_hon(5, 5) == False, "hai gia tri BANG NHAU khong duoc coi la 'dai hon thi TF cao hon'"
assert dai_hon_thi_tf_cao_hon(5, 3) == False, "tf_dai THAP hon tf_ngan thi khong duoc coi la dung"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `dem_tan_so`) là đối số của `cac_tu.count(...)` — đếm số lần từ đang xét (tham số `tu`) xuất hiện trong danh sách `cac_tu`. Chỗ hai (trong `dai_hon_thi_tf_cao_hon`) là một phép SO SÁNH — trả về `True` khi `tf_dai` thật sự LỚN HƠN `tf_ngan` (không phải lớn hơn-hoặc-bằng).
- kind: strategy
  body: 'Chỗ đầu: `cac_tu.count(tu)` — đếm đúng tham số `tu`. Chỗ hai: `tf_dai > tf_ngan` — so sánh nghiêm ngặt bằng toán tử `>`.'
- kind: one-line
  body: 'Chỗ đầu là `cac_tu.count(tu)`, chỗ hai là `tf_dai > tf_ngan`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dem DUNG tham so 'tu' bang .count(tu) (khong duoc dem mot ten khac); VA cho trong hai phai SO SANH THAT bang toan tu '>' (khong duoc luon tra ve True/False chep san)
  requireAst:
  - kind: uses-call, target: count, min: 1
  - kind: uses-operator, target: ">", min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts vao mot file rieng,
  # chay qua python3 TREN CHINH van ban solution da trich tu file nay) -- ket
  # qua [1, 1] cho hai luat theo dung thu tu khai bao o tren.
  # count=1: CHI mot lan .count(...) trong toan bo solution, dung o cho trong
  # dau.
  # ">"=1: CHI mot lan toan tu '>' trong toan bo solution, dung o cho trong
  # hai.
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham _dem de
  # xac nhan, khong doan tay): dien "tf_dai > tf_ngan" vao cho trong dau
  # ("return cac_tu.count(tf_dai > tf_ngan)") VA dien "tu" vao cho trong hai
  # ("return tu" trong dai_hon_thi_tf_cao_hon) -- ket qua AST van la [1, 1],
  # Y HET ban dung (toan tu '>' va ham .count deu van xuat hien dung 1 lan,
  # chi doi VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong dem_tan_so, dong
  # "cac_tu.count(tf_dai > tf_ngan)" doc bien 'tf_dai'/'tf_ngan' -- nhung hai
  # ten nay CHUA duoc gan o cap module khi dem_tan_so("python", DOAN_NGAN)
  # duoc goi LAN DAU (chinh loi goi nay dang dinh gan cho tf_ngan) -- da tu
  # chay THAT mutant nay qua python3, xac nhan no nem NameError: name
  # 'tf_dai' is not defined ngay o lan goi dem_tan_so dau tien -- bi chan boi
  # tier 'run', doc lap voi static.
  # Da ra soat GOTCHA #6 (uses-call khong kiem doi so): dem_tan_so(tu,
  # van_ban) chi co dung mot tham so co the dua vao .count(...) mot cach hop
  # le ve kieu (tu la chuoi, van_ban la chuoi khac muc dich) -- khong co bien
  # nao khac cung scope de nham lan.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^11\\n64\\n1\\n6\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`6 > 1` — đoạn dài thắng đoạn ngắn chỉ vì nó dài hơn, đo được bằng số thật.
TF thô một mình chưa đủ để tìm kiếm công bằng. Bài sau đo một điều khác: một
từ khoá HIẾM có giá trị phân biệt cao hơn một từ khoá phổ biến — đó là mảnh
ghép tiếp theo.
::::

::::reflect{#nghi-lai}
Tần số từ thô không sai — nó làm đúng việc được định nghĩa: đếm. Nhưng dùng
một mình nó để xếp hạng "mức độ liên quan" thì lặp lại đúng lỗi mà tích vô
hướng thô đã mắc ở `q8.5b`: nó thiên vị đoạn văn DÀI, vì đoạn dài có nhiều
cơ hội lặp từ hơn, không liên quan gì tới việc đoạn đó có thực sự khớp với
một truy vấn hay không. Cosine similarity đã sửa lỗi này cho vector bằng
cách CHIA cho độ dài vector. Tìm kiếm theo từ khoá cần một cách sửa khác —
không chỉ đếm một từ xuất hiện bao nhiêu lần, mà còn phải biết từ đó CÓ GIÁ
TRỊ PHÂN BIỆT hay không. Bài sau đo chính điều đó: độ hiếm của một từ trên
toàn bộ kho ngữ liệu.
::::

::::checkpoint{mastery=0.85}
::::
