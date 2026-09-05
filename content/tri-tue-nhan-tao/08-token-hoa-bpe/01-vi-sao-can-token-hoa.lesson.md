---
id: tri-tue-nhan-tao.token-hoa-bpe.vi-sao-can-token-hoa
title: "Vì sao cần token hoá: máy tính không đọc được chữ"
summary: "Mạng nơ-ron (Value/Tensor, T8.2) chỉ nhận số — văn bản phải đổi thành số TRƯỚC. Hai cách cực đoan: theo TỪ (vocab 8 từ trên một corpus nhỏ, nhưng câu mới 'con cam an tan' có 2 từ ngoài từ vựng — OOV) ĐỐI CHIẾU theo KÝ TỰ ĐƠN (vocab chỉ 10 ký tự, cùng câu mới có 0 ký tự OOV — luôn phủ được, nhưng chuỗi token dài hơn nhiều, mất cấu trúc từ). BPE (bài sau) là điểm cân bằng ở giữa."
locale: vi
track: tri-tue-nhan-tao
module: token-hoa-bpe
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.vi-sao-can-token-hoa]
requires: [ai.boss-mang-no-ron-tu-so-0]
concepts: [ai.vi-sao-can-token-hoa]
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
T8.2 khép lại với một mạng nơ-ron chạy hoàn hảo trên số — `Value` chỉ biết
cộng, nhân, `tanh`. T8.3 "Transformer từ số 0" bắt đầu ở đây: trước khi một
mạng nơ-ron chạm được vào một CÂU VĂN, câu đó phải biến thành số đã.
::::

::::explain{#vi_sao_can_token_hoa}
Mọi phép toán mà `Value` (và sau này `Tensor`) biết làm — cộng, nhân, đạo
hàm — đều là phép toán trên SỐ. Một câu văn như `"con mèo ăn cá"` không
phải là một con số, và không có phép cộng/nhân nào áp dụng được trực tiếp
lên một chuỗi ký tự. **Token hoá** là bước bắt buộc đứng trước mọi mô hình
ngôn ngữ: cắt văn bản thành các đơn vị rời rạc (gọi là **token**), rồi ánh
xạ mỗi token sang một số nguyên duy nhất (**token ID**) qua một bảng tra
cứu gọi là **từ vựng** (vocabulary). Chỉ sau bước này, câu văn mới trở
thành một danh sách số nguyên — thứ mà `Value`/`Tensor` biết xử lý.

Câu hỏi kế tiếp: token hoá theo ĐƠN VỊ gì? Hai lựa chọn cực đoan, mỗi cái
có một điểm yếu rõ rệt:

> **Theo TỪ NGUYÊN VẸN (word-level)** — mỗi từ hoàn chỉnh là một token
> (`"con"`, `"mèo"`, `"ăn"`, `"cá"` — bốn token cho bốn từ). Từ vựng xây từ
> TOÀN BỘ các từ DUY NHẤT xuất hiện trong corpus (kho văn bản dùng để huấn
> luyện). Vấn đề: corpus không thể chứa MỌI từ có thể xuất hiện trong
> tương lai — một từ MỚI, chưa từng thấy lúc xây từ vựng (gọi là từ
> **ngoài từ vựng**, out-of-vocabulary — **OOV**), không có ID nào để ánh
> xạ tới. Mô hình HOÀN TOÀN bó tay trước một từ nó chưa từng thấy, dù từ đó
> chỉ khác một chữ cái so với một từ đã biết.

> **Theo KÝ TỰ ĐƠN (character-level)** — mỗi ký tự riêng lẻ là một token
> (`"c"`, `"o"`, `"n"`, `" "`, `"m"`, ... — mười mấy token cho cùng bốn
> từ). Từ vựng chỉ cần liệt kê các ký tự DUY NHẤT xuất hiện — với hầu hết
> ngôn ngữ, đó là một tập RẤT NHỎ và gần như CỐ ĐỊNH (chữ cái + dấu câu +
> khoảng trắng). Từ MỚI nào cũng ghép được từ những ký tự đã biết — không
> còn vấn đề OOV. Nhưng cái giá phải trả: chuỗi token DÀI HẲN ra (một từ `4`
> ký tự tốn `4` token thay vì `1`), và mỗi ký tự đơn lẻ gần như không mang
> nghĩa gì — mô hình phải tự học lại cấu trúc "từ" từ đầu, một việc mà
> token theo từ đã cho MIỄN PHÍ.

BPE (**Byte Pair Encoding** — bài học kế tiếp sẽ cài đặt nó từng bước) là
điểm cân bằng ở GIỮA: bắt đầu từ ký tự đơn, rồi GHÉP DẦN những cặp ký tự
xuất hiện thường xuyên nhất thành các đơn vị lớn hơn — token có thể là một
ký tự, một phần của từ, hoặc cả một từ ngắn phổ biến, tuỳ vào tần suất xuất
hiện thực tế trong corpus. Không còn OOV tuyệt đối (mọi từ vẫn ghép được từ
ký tự đơn nếu cần), mà chuỗi token cũng không dài như character-level
thuần tuý.
::::

::::example{#minh_hoa_oov}
Một corpus nhỏ (`12` từ, có lặp) để xây hai loại từ vựng, rồi thử một câu
MỚI chứa hai từ chưa từng xuất hiện trong corpus:

```python title=readonly
corpus = "con meo an ca con cho an com con ga an thoc"

vocab_tu = set(corpus.split())
vocab_ky_tu = set(corpus)

print("so tu duy nhat trong vocab:", len(vocab_tu))
print("so ky tu duy nhat trong vocab:", len(vocab_ky_tu))

cau_moi = "con cam an tan"
tu_oov = [t for t in cau_moi.split() if t not in vocab_tu]
ky_tu_oov = [k for k in cau_moi if k not in vocab_ky_tu]

print("tu OOV:", tu_oov)
print("ky tu OOV:", ky_tu_oov)
```

```text title=readonly
so tu duy nhat trong vocab: 8
so ky tu duy nhat trong vocab: 10
tu OOV: ['cam', 'tan']
ky tu OOV: []
```

Từ vựng theo từ chỉ có `8` mục (`an`, `ca`, `cho`, `com`, `con`, `ga`,
`meo`, `thoc`); từ vựng theo ký tự có `10` mục (mọi chữ cái + khoảng
trắng xuất hiện trong corpus). Câu mới `"con cam an tan"` chứa hai từ
CHƯA TỪNG xuất hiện — `"cam"` và `"tan"` — cả hai đều là từ OOV theo
từ vựng-theo-từ. Nhưng xét theo KÝ TỰ, mọi ký tự tạo nên `"cam"` và
`"tan"` (`c`, `a`, `m`, `t`, `n`) đều ĐÃ có sẵn trong từ vựng-theo-ký-tự
(dùng lại chính những chữ cái đã thấy ở `"con"`, `"ca"`, `"an"`, ...) —
không một ký tự OOV nào, dù cả hai TỪ đều mới hoàn toàn.
::::

::::predict{#doan_oov commitOnce}
Xét một từ vựng-theo-từ xây từ một corpus tiếng Việt CỠ TRUNG BÌNH (vài
nghìn câu) — không phải corpus nhỏ `12` từ ở trên.

**Trước khi đọc tiếp**, bạn đoán: có tồn tại một corpus LỚN đến mức nào đó
mà từ vựng-theo-từ xây từ nó KHÔNG BAO GIỜ gặp một từ OOV nào nữa, dù đưa
vào bất kỳ câu tiếng Việt nào trong tương lai?

:::opt{correct}
Không — ngôn ngữ tự nhiên liên tục sinh từ MỚI (tên riêng, từ vay mượn,
từ ghép mới, lỗi chính tả cố ý...); corpus dù lớn đến đâu vẫn chỉ là một
MẪU hữu hạn, không bao giờ phủ hết mọi từ có thể xuất hiện — vấn đề OOV
của token hoá theo từ là một giới hạn CẤU TRÚC, không phải một vấn đề chỉ
xảy ra vì corpus "chưa đủ lớn"
:::

:::opt
Có — chỉ cần corpus đủ lớn (ví dụ toàn bộ Wikipedia tiếng Việt), từ vựng sẽ
phủ được mọi từ tiếng Việt tồn tại
::why
Gần đúng ở việc corpus càng lớn thì từ vựng càng phủ được NHIỀU từ hơn —
quan sát đó đúng, và là lý do các mô hình lớn dùng corpus khổng lồ.

Chỗ lệch: "nhiều hơn" không phải "mọi". Ngôn ngữ không phải một tập từ CỐ
ĐỊNH đóng băng tại thời điểm thu thập corpus — người dùng luôn tạo ra từ
mới (tên người/địa danh mới, từ lóng, từ ghép tuỳ ngữ cảnh) SAU thời điểm
đó. Một corpus dù lấy toàn bộ Wikipedia vẫn không chứa những từ sẽ được tạo
ra NGÀY MAI — vấn đề OOV không biến mất, chỉ giảm tần suất gặp phải.
::
:::

:::opt
Có, nếu từ vựng thêm một mục đặc biệt gọi là `<UNK>` (unknown) để đại diện
cho MỌI từ chưa biết — vậy là đã "phủ" được mọi từ, kể cả từ tương lai
::why
Gần đúng ở việc mục `<UNK>` là một kỹ thuật CÓ THẬT, dùng phổ biến để tránh
mô hình bị crash khi gặp từ lạ.

Chỗ lệch: `<UNK>` không giải quyết vấn đề, nó chỉ NÉ nó — mọi từ OOV, dù
khác nhau hoàn toàn về nghĩa (`"cam"` và `"tan"` ở ví dụ trên), đều bị gộp
chung thành CÙNG một token `<UNK>`, xoá sạch mọi thông tin phân biệt giữa
chúng. Mô hình vẫn hoàn toàn "mù" trước sự khác biệt giữa hai từ lạ — đây
là lý do BPE (bài sau) ra đời: giữ được khả năng biểu diễn từ MỚI mà không
cần đánh đồng chúng thành một token vô nghĩa.
::
:::
::::

::::code{#viet_dem_tu_oov}
Hoàn thiện `dem_tu_oov`: trả về danh sách các từ trong `cau` KHÔNG có mặt
trong `vocab_tu`.

```python title=starter
def dem_tu_oov(cau, vocab_tu):
    tu_cau = cau.split()
    return [t for t in tu_cau if t ___ vocab_tu]     # not in

corpus = "con meo an ca con cho an com con ga an thoc"
vocab_tu = set(corpus.split())
vocab_ky_tu = set(corpus)

cau_moi = "con cam an tan"
tu_oov = dem_tu_oov(cau_moi, vocab_tu)
ky_tu_oov = [k for k in cau_moi if k not in vocab_ky_tu]

print(tu_oov)
print(len(tu_oov))
print(ky_tu_oov)
```

```python title=solution
def dem_tu_oov(cau, vocab_tu):
    tu_cau = cau.split()
    return [t for t in tu_cau if t not in vocab_tu]

corpus = "con meo an ca con cho an com con ga an thoc"
vocab_tu = set(corpus.split())
vocab_ky_tu = set(corpus)

cau_moi = "con cam an tan"
tu_oov = dem_tu_oov(cau_moi, vocab_tu)
ky_tu_oov = [k for k in cau_moi if k not in vocab_ky_tu]

print(tu_oov)
print(len(tu_oov))
print(ky_tu_oov)
```

```python title=test
assert dem_tu_oov("con cam an tan", vocab_tu) == ['cam', 'tan'], f"dem_tu_oov cau moi sai -- dang ra {dem_tu_oov('con cam an tan', vocab_tu)}"
assert dem_tu_oov("con an", vocab_tu) == [], f"cau khong co tu OOV nao phai tra ve danh sach rong -- dang ra {dem_tu_oov('con an', vocab_tu)}"
assert dem_tu_oov("vit boi", vocab_tu) == ['vit', 'boi'], f"cau toan tu OOV phai tra ve CA HAI tu -- dang ra {dem_tu_oov('vit boi', vocab_tu)}"
assert tu_oov == ['cam', 'tan'], f"tu_oov sai -- dang ra {tu_oov}"
assert len(tu_oov) == 2, f"phai co dung 2 tu OOV -- dang ra {len(tu_oov)}"
assert ky_tu_oov == [], f"ky_tu_oov phai la danh sach RONG (khong ky tu nao OOV) -- dang ra {ky_tu_oov}"
assert len(vocab_tu) == 8, f"vocab_tu phai co 8 tu duy nhat -- dang ra {len(vocab_tu)}"
assert len(vocab_ky_tu) == 10, f"vocab_ky_tu phai co 10 ky tu duy nhat -- dang ra {len(vocab_ky_tu)}"
```

:::hints
- kind: attention
  body: Một chỗ trống, trong điều kiện của list comprehension. Cần giữ lại những từ `t` mà KHÔNG có mặt trong `vocab_tu` — Python có đúng một toán tử diễn tả "không có mặt trong": `not in`.
- kind: strategy
  body: 'Điều kiện lọc: `t not in vocab_tu` — giữ `t` lại nếu nó không phải một phần tử của tập `vocab_tu`.'
- kind: one-line
  body: 'Chỗ trống là `not in`, cho dòng `return [t for t in tu_cau if t not in vocab_tu]`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dem_tu_oov phai dung toan tu "not in" (khong duoc dao thanh "in" -- se tra ve dung NGUOC LAI, tuc cac tu DA CO trong vocab thay vi cac tu OOV)
  requireAst:
  - kind: uses-operator, target: "not in", min: 2
  - kind: comprehension, min: 2
  # Da thu that (goi kiemAst that tren code trich tu solution): loi giai dung
  # dat=true, ca hai luat qua sach. "not in"=2: mot lan trong dem_tu_oov (cho
  # tro), mot lan trong ky_tu_oov = [... if k not in vocab_ky_tu] o script
  # muc top-level (KHONG bi cho trong, luon co san) -- neu nguoi hoc dao
  # nguoc thanh "in" trong ham, dem "not in" tut xuong con 1 (chi con dong
  # ky_tu_oov) -- bi chan. comprehension=2: hai list comprehension (trong
  # dem_tu_oov va dong ky_tu_oov).
  #
  # Cheat "t in vocab_tu" (dao nguoc, quen "not") lam dem_tu_oov tra ve
  # NGUOC: voi cau "con cam an tan" se tra ve ['con', 'an'] thay vi
  # ['cam', 'tan'] -- bi chan boi ca output/tests (khac han ket qua mong
  # doi) LAN static rieng (da tu kiem chung ca hai).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\['cam', 'tan'\\]\\n2\\n\\[\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai từ vựng, hai đánh đổi hoàn toàn khác nhau. Bài sau cài đặt cách đơn
giản nhất trong hai cách — theo ký tự đơn — làm điểm khởi đầu cho BPE.
::::

::::reflect{#nghi-lai}
Từ vựng-theo-từ mất từ mới; từ vựng-theo-ký-tự không mất gì nhưng chuỗi
token dài và vô nghĩa từng-ký-tự-một. Bài sau cài đặt CHÍNH XÁC cách
token hoá theo ký tự đơn — mã hoá, giải mã, đo số ký tự duy nhất trên một
corpus thật — làm nền cho thuật toán BPE sẽ xây suốt phần còn lại của
quest này.
::::

::::checkpoint{mastery=0.8}
::::
