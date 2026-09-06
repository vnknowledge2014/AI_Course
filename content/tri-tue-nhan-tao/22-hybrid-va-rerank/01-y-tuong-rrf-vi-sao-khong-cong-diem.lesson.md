---
id: tri-tue-nhan-tao.hybrid-va-rerank.y-tuong-rrf-vi-sao-khong-cong-diem
title: "Ý tưởng RRF: vì sao KHÔNG cộng điểm số trực tiếp"
summary: "Diem BM25 khong co tran co dinh (vi du 8,7117), con cosine similarity tren vector dem tu khong am luon nam trong [0,1] -- hai THANG DO khac nhau. Cong truc tiep diem(8,7117)+diem(0,1)=8,8117 cho doan A va diem(2,1)+diem(0,9)=3,0 cho doan B: doan A THANG AP DAO chi vi thang do BM25 lon hon han thang do cosine, khong phai vi no lien quan hon. RRF (Reciprocal Rank Fusion) bo qua diem so goc, chi dung THU HANG: dong gop cua mot hang r la 1/(k+r). Voi k=1: doan A hang 1/hang 2 (dong gop 0,5 + 0,3333 = 0,8333), doan B hang 2/hang 1 (dong gop 0,3333 + 0,5 = 0,8333) -- HOA, dao nguoc hoan toan ket qua cong truc tiep."
locale: vi
track: tri-tue-nhan-tao
module: hybrid-va-rerank
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.y-tuong-rrf-vi-sao-khong-cong-diem]
requires: [ai.boss-bm25-doi-dau-cosine-vector]
concepts: [ai.y-tuong-rrf-vi-sao-khong-cong-diem]
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
Quest `bm25-va-tim-kiem-tu-khoa` (q8.5d) đóng tại `6/6`: BM25 và cosine mỗi
cách có một điểm mù RIÊNG. `hybrid-va-rerank` (q8.5e, `4` bài) ráp cả hai
thành một xếp hạng hợp nhất — bắt đầu bằng câu hỏi: ráp bằng cách nào?
::::

::::explain{#vi_sao_khong_cong_diem}
Cách đầu tiên người ta thường nghĩ tới: tính điểm BM25, tính điểm cosine
similarity, rồi CỘNG hai điểm lại, xếp hạng theo tổng. Cách này có một lỗi
nền tảng — hai điểm số này sống trên hai **thang đo** hoàn toàn khác nhau.

Điểm BM25 (bài `4`/`5` của q8.5d, hàm `diem_bm25_doan`) không có trần cố
định — nó phụ thuộc `idf` (phụ thuộc quy mô kho) và tần số từ, nên có thể là
`0,84`, có thể là `8,7117`, có thể lớn hơn nhiều trên một kho khác. Điểm
cosine similarity (`tuong_dong_cosine`, q8.5a/b) trên vector đếm từ KHÔNG ÂM
luôn nằm gọn trong `[0, 1]`.

Cộng trực tiếp `diem_bm25 + diem_cosine` không có ý nghĩa toán học nào cả —
nó chỉ để thang đo NÀO LỚN HƠN lấn át thang đo kia một cách TUỲ TIỆN, không
liên quan gì tới việc đoạn nào thực sự phù hợp hơn với câu hỏi.

**Reciprocal Rank Fusion (RRF)** né hẳn vấn đề này bằng một quyết định đơn
giản: KHÔNG dùng điểm số gốc, chỉ dùng **thứ hạng** (rank — một số nguyên
`1, 2, 3, ...`, vị trí của một tài liệu trong MỘT hệ thống xếp hạng). Đóng
góp của một tài liệu đứng ở hạng `r` trong một hệ thống là:

```
dong_gop_rrf(hang, k) = 1 / (k + hang)
```

`k` là một hằng số làm mượt (tài liệu gốc về RRF thường chọn `k=60` để hạng
`1` không lấn át quá mức khi hợp nhất NHIỀU hệ thống trên danh sách dài).
Bài này — và cả quest — dùng `k=1`: hạng càng thấp (càng tốt), đóng góp
càng lớn (`1/(1+1)=0,5`), hạng càng cao, đóng góp càng nhỏ (`1/(1+2)≈0,3333`)
— vẫn đúng ý tưởng gốc, chỉ khác ở mức độ chênh lệch giữa các hạng, và cho
số liệu tính tay gọn hơn hẳn `k=60`.
::::

::::example{#doi_chieu_cong_truc_tiep_va_rrf}
Hai đoạn giả định, mỗi đoạn có điểm ở HAI hệ thống khác thang đo — hệ thống
`1` giống BM25 (không trần cố định), hệ thống `2` giống cosine (`[0, 1]`):

```python title=readonly
def cong_truc_tiep(diem_1, diem_2):
    return diem_1 + diem_2


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


# doan A: he thong 1 (BM25-like) = 8,7117 (hang 1) -- he thong 2 (cosine-like) = 0,1 (hang 2)
# doan B: he thong 1 (BM25-like) = 2,1    (hang 2) -- he thong 2 (cosine-like) = 0,9 (hang 1)
diem_truc_tiep_A = cong_truc_tiep(8.7117, 0.1)
diem_truc_tiep_B = cong_truc_tiep(2.1, 0.9)

k = 1
diem_rrf_A = dong_gop_rrf(1, k) + dong_gop_rrf(2, k)   # hang 1 o he thong 1, hang 2 o he thong 2
diem_rrf_B = dong_gop_rrf(2, k) + dong_gop_rrf(1, k)   # hang 2 o he thong 1, hang 1 o he thong 2

print(round(diem_truc_tiep_A, 4))
print(round(diem_truc_tiep_B, 4))
print(diem_truc_tiep_A > diem_truc_tiep_B)
print(round(diem_rrf_A, 4))
print(round(diem_rrf_B, 4))
print(diem_rrf_A == diem_rrf_B)
```

```text title=readonly
8.8117
3.0
True
0.8333
0.8333
True
```

Cộng trực tiếp: đoạn A thắng ÁP ĐẢO (`8,8117` so với `3,0`) — không phải vì
đoạn A "phù hợp hơn" theo cả hai hệ thống, mà đơn thuần vì hệ thống `1` (kiểu
BM25) có thang đo lớn hơn hẳn hệ thống `2` (kiểu cosine), nên điểm hệ thống
`1` của đoạn A (`8,7117`) áp đảo mọi thứ khác trong phép cộng.

RRF nhìn khác hẳn: đoạn A đứng hạng `1` ở hệ thống `1` nhưng hạng `2` ở hệ
thống `2`; đoạn B thì NGƯỢC LẠI — hạng `2` ở hệ thống `1`, hạng `1` ở hệ
thống `2`. Đây là hai tình huống ĐỐI XỨNG hoàn hảo (mỗi đoạn có đúng một
hạng `1` và một hạng `2`, chỉ khác nhau ở hệ thống nào cho hạng nào) — nên
tổng RRF của hai đoạn bằng nhau TUYỆT ĐỐI: `0,8333 = 0,8333`. RRF, dựa thuần
trên thứ hạng, không hề bị đánh lừa bởi con số `8,7117` to lớn của hệ thống
`1` — nó thấy đúng điều có thật: hai đoạn này ngang tài ngang sức, mỗi đoạn
giỏi ở một hệ thống và kém ở hệ thống còn lại.
::::

::::predict{#doan_rrf_hoa_hay_thang commitOnce}
Xét đúng ví dụ trên: cộng trực tiếp cho đoạn A thắng áp đảo (`8,8117` so với
`3,0`). Đoạn A đứng hạng `1` ở hệ thống `1`, hạng `2` ở hệ thống `2`. Đoạn B
đứng NGƯỢC LẠI: hạng `2` ở hệ thống `1`, hạng `1` ở hệ thống `2`.

**Trước khi chạy thử**, bạn đoán: nếu CHỈ dùng thứ hạng qua công thức RRF
(`k=1`), đoạn nào "thắng"?

:::opt{correct}
Hòa — cả hai đoạn có tổng RRF NHƯ NHAU (`0,8333`), vì mỗi đoạn nhận đúng một
hạng `1` và một hạng `2` giữa hai hệ thống, chỉ khác nhau ở hệ thống nào cho
hạng nào; RRF (dựa thuần trên thứ hạng) không phân biệt được hai tình huống
đối xứng như vậy
:::

:::opt
Đoạn A vẫn thắng, giống hệt cách cộng điểm trực tiếp — vì đoạn A có điểm hệ
thống `1` cao hơn hẳn, và thứ hạng cũng phải phản ánh đúng lợi thế đó
::why
Gần đúng ở việc đoạn A THẬT SỰ có điểm hệ thống `1` cao hơn hẳn (`8,7117` so
với `2,1`) — quan sát về điểm số gốc đó đúng.

Chỗ lệch: RRF loại bỏ hoàn toàn ĐỘ LỚN của điểm số gốc, chỉ giữ lại THỨ HẠNG
(`1` hay `2`). Việc đoạn A "thắng áp đảo" khi cộng trực tiếp là một ẢO GIÁC
do thang đo — nó biến mất hoàn toàn khi bạn bỏ đi con số gốc và chỉ nhìn vào
vị trí xếp hạng, vì ở góc nhìn thứ hạng, đoạn A và đoạn B đối xứng tuyệt đối.
::
:::

:::opt
Đoạn B thắng, vì hệ thống `2` (kiểu cosine) là phép đo đã chuẩn hoá, đáng
tin cậy hơn hệ thống `1`, nên hạng `1` của đoạn B ở đó phải được ưu tiên hơn
::why
Gần đúng ở việc hệ thống `2` (cosine) THẬT SỰ đã được chuẩn hoá về `[0, 1]`
(bài `2` của q8.5a) — quan sát đó đúng về mặt kỹ thuật.

Chỗ lệch: công thức RRF cơ bản không hề gán một hệ thống nào "đáng tin hơn"
— nó cộng dồn đóng góp từ CẢ HAI hệ thống một cách BÌNH ĐẲNG. Đoạn B giỏi ở
hệ thống `2` nhưng kém ở hệ thống `1`, đối xứng CHÍNH XÁC với đoạn A (giỏi ở
hệ thống `1`, kém ở hệ thống `2`) — không có lý do nào trong công thức để ưu
tiên một phía.
::
:::
::::

::::code{#viet_cong_truc_tiep_va_dong_gop_rrf}
Hoàn thiện `cong_truc_tiep` (cộng thẳng hai điểm số, cách trực giác đầu tiên nhưng
có lỗi thang đo) và `dong_gop_rrf` (đóng góp RRF của một hạng, công thức
`1/(k+hang)`).

```python title=starter
def cong_truc_tiep(diem_1, diem_2):
    return ___                                              # diem_1 + diem_2


def dong_gop_rrf(hang, k):
    return ___                                              # 1 / (k + hang)


diem_truc_tiep_A = cong_truc_tiep(8.7117, 0.1)
diem_truc_tiep_B = cong_truc_tiep(2.1, 0.9)

k = 1
diem_rrf_A = dong_gop_rrf(1, k) + dong_gop_rrf(2, k)
diem_rrf_B = dong_gop_rrf(2, k) + dong_gop_rrf(1, k)

print(round(diem_truc_tiep_A, 4))
print(round(diem_truc_tiep_B, 4))
print(diem_truc_tiep_A > diem_truc_tiep_B)
print(round(diem_rrf_A, 4))
print(round(diem_rrf_B, 4))
print(diem_rrf_A == diem_rrf_B)
```

```python title=solution
def cong_truc_tiep(diem_1, diem_2):
    return diem_1 + diem_2


def dong_gop_rrf(hang, k):
    return 1 / (k + hang)


diem_truc_tiep_A = cong_truc_tiep(8.7117, 0.1)
diem_truc_tiep_B = cong_truc_tiep(2.1, 0.9)

k = 1
diem_rrf_A = dong_gop_rrf(1, k) + dong_gop_rrf(2, k)
diem_rrf_B = dong_gop_rrf(2, k) + dong_gop_rrf(1, k)

print(round(diem_truc_tiep_A, 4))
print(round(diem_truc_tiep_B, 4))
print(diem_truc_tiep_A > diem_truc_tiep_B)
print(round(diem_rrf_A, 4))
print(round(diem_rrf_B, 4))
print(diem_rrf_A == diem_rrf_B)
```

```python title=test
assert round(diem_truc_tiep_A, 4) == 8.8117, f"diem_truc_tiep_A phai la 8,8117 -- dang ra {diem_truc_tiep_A}"
assert diem_truc_tiep_B == 3.0, f"diem_truc_tiep_B phai la 3,0 -- dang ra {diem_truc_tiep_B}"
assert diem_truc_tiep_A > diem_truc_tiep_B, "cong truc tiep phai cho doan A thang ap dao (thang do BM25-like lon hon)"
assert round(diem_rrf_A, 4) == 0.8333, f"diem_rrf_A phai xap xi 0,8333 -- dang ra {diem_rrf_A}"
assert diem_rrf_A == diem_rrf_B, "RRF phai cho HAI DOAN DOI XUNG mot tong BANG NHAU TUYET DOI, dao nguoc ket qua cong truc tiep"

# kiem tra truc tiep tung ham tren gia tri nho, tu tinh tay duoc
assert cong_truc_tiep(3, 4) == 7, f"cong_truc_tiep(3, 4) phai la 7 -- dang ra {cong_truc_tiep(3, 4)}"
assert dong_gop_rrf(1, 1) == 0.5, f"dong_gop_rrf(1, 1) phai la 0,5 -- dang ra {dong_gop_rrf(1, 1)}"
assert round(dong_gop_rrf(2, 1), 4) == 0.3333, f"dong_gop_rrf(2, 1) phai xap xi 0,3333 -- dang ra {dong_gop_rrf(2, 1)}"
assert round(dong_gop_rrf(1, 60), 4) == round(1 / 61, 4), "dong_gop_rrf phai dung DUNG cong thuc 1/(k+hang), thu voi k=60 (gia tri kinh dien)"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `cong_truc_tiep`) là phép CỘNG hai đối số `diem_1` và `diem_2` — cách trực giác đầu tiên để ráp hai điểm số. Chỗ hai (trong `dong_gop_rrf`) là công thức RRF cho một hạng — chia `1` cho tổng của hằng số làm mượt `k` và hạng `hang`.
- kind: strategy
  body: 'Chỗ đầu: `diem_1 + diem_2` — cộng thẳng, không biến đổi gì thêm. Chỗ hai: `1 / (k + hang)` — đúng công thức `1/(k+r)` đã nêu ở phần giải thích, với `r` chính là tham số `hang`.'
- kind: one-line
  body: 'Chỗ đầu là `diem_1 + diem_2`, chỗ hai là `1 / (k + hang)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai CONG (dung toan tu '+') hai doi so diem_1, diem_2 -- khong duoc nhan hay chep san mot so; VA cho trong hai phai dung DUNG cong thuc RRF, CHIA 1 cho tong cua k va hang (dung ca toan tu '+' lan '/')
  requireAst:
  - kind: uses-operator, target: "+", min: 4
  - kind: uses-operator, target: "/", min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [4, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # "+"=4 (TONG THAT, da xac nhan bang cong cu, khong doan tay): 1 lan o cho
  # trong dau ("diem_1 + diem_2"), 1 lan BEN TRONG cho trong hai ("1 / (k +
  # hang)" -- phep cong k+hang la mot phan CUA CHINH cong thuc, khong the
  # tach roi), va 2 lan o code da cho san ben ngoai hai ham ("dong_gop_rrf(1,
  # k) + dong_gop_rrf(2, k)" va "dong_gop_rrf(2, k) + dong_gop_rrf(1, k)").
  # Neu chi dat min=1 (ngay tho, chi tinh cho trong dau), mot mutant dien
  # "diem_1 - diem_2" hay "diem_1 * diem_2" vao cho trong dau van khong bi
  # bat NEU luat chi kiem cho trong dau rieng le -- nhung o day dem TOAN BO
  # solution nen mutant do lam tong '+' TUT xuong 3, duoi nguong 4, bi chan.
  # "/"=1: CHI mot lan duy nhat trong toan bo solution, dung o cho trong hai.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "1 / (k + hang)" vao cho trong dau
  # ("return 1 / (k + hang)" trong cong_truc_tiep) VA dien "diem_1 + diem_2"
  # vao cho trong hai ("return diem_1 + diem_2" trong dong_gop_rrf) -- tong so
  # lan '+' VA '/' trong toan bo solution KHONG DOI (van la 4 va 1, chi doi
  # VI TRI hai bieu thuc cho nhau). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong cong_truc_tiep (tham so
  # diem_1, diem_2), bieu thuc moi "1 / (k + hang)" dung ten 'k' va 'hang' --
  # ca hai CHUA HE TON TAI trong scope cua cong_truc_tiep; ben trong
  # dong_gop_rrf (tham so hang, k), bieu thuc moi "diem_1 + diem_2" dung ten
  # 'diem_1'/'diem_2' -- ca hai CHUA HE TON TAI trong scope cua dong_gop_rrf.
  # Da tu chay THAT mutant nay qua python3, xac nhan no nem NameError ngay
  # khi cong_truc_tiep(8.7117, 0.1) duoc goi lan dau ("name 'k' is not
  # defined") -- bi chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^8\\.8117\\n3\\.0\\nTrue\\n0\\.8333\\n0\\.8333\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`8,8117 > 3,0` khi cộng thẳng — nhưng `0,8333 = 0,8333` khi chỉ dùng thứ
hạng. Cùng hai đoạn, cùng dữ liệu, kết quả đảo ngược hoàn toàn chỉ vì đổi
cách kết hợp. Bài sau viết công thức RRF ĐẦY ĐỦ, hợp nhất NHIỀU hệ thống xếp
hạng thật — `xep_hang_bm25` và `xep_hang_cosine`.
::::

::::reflect{#nghi-lai}
"Cộng điểm trực tiếp" nghe hợp lý — hai con số, mỗi con số đo một khía cạnh
liên quan, cộng lại tưởng như tổng hợp được cả hai. Vấn đề không nằm ở Ý
TƯỞNG cộng dồn, mà ở việc hai con số ấy được đo bằng hai THƯỚC khác nhau: một
thước không có vạch trần, một thước có trần cố định ở `1`. Cộng trực tiếp
biến việc "thước nào có vạch lớn hơn" thành yếu tố quyết định, dù điều đó
chẳng liên quan gì tới câu hỏi thật sự đang được hỏi: đoạn nào phù hợp hơn?
RRF sửa đúng chỗ đó bằng cách VỨT BỎ hoàn toàn con số gốc, chỉ giữ lại thứ tự
tương đối (thứ hạng) — một thông tin không phụ thuộc thước đo nào cả. Bài
này mới dùng RRF trên đúng HAI hạng cho trước sẵn; bài sau viết công thức
tổng quát, áp lên hai hệ thống xếp hạng THẬT: `xep_hang_bm25` và
`xep_hang_cosine`.
::::

::::checkpoint{mastery=0.85}
::::
