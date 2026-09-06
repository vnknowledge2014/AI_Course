---
id: tri-tue-nhan-tao.ha-tang-suy-luan.boss-hang-doi-suy-luan-gop-lo
title: "BOSS quý — hàng đợi suy luận có gộp lô: đo thông lượng, đóng q8.6a tại 5/5"
summary: "thong_luong(so_request, tong_thoi_gian) = so_request / tong_thoi_gian -- so request phuc vu duoc tren MOI don vi thoi gian. Rap lai tong_thoi_gian_tuan_tu (bai 2) va tong_thoi_gian_gop_lo (bai 3) tren MOT hang doi 10 request (moi request 10 don vi tinh toan, chi_phi_khoi_dong=200): chien luoc tuan tu cho tong_tuan_tu=2100, thong_luong=10/2100≈0,004762; chien luoc gop lo (kich_thuoc_lo=10, ca hang doi trong MOT lo) cho tong_gop_lo=210, thong_luong≈0,047619 -- CAO HON DUNG 10 lan (2100/210=10,0). gop_lo_thang = thong_luong_gop_lo > thong_luong_tuan_tu = True. Dong q8.6a tai 5/5."
locale: vi
track: tri-tue-nhan-tao
module: ha-tang-suy-luan
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ai.boss-hang-doi-suy-luan-gop-lo]
requires: [ai.paged-attention-cap-phat-theo-khoi]
concepts: [ai.boss-hang-doi-suy-luan-gop-lo]
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
Bốn bài: ba tầng kiến trúc AI Stack, chi phí khởi động bị nhân lên khi xử
lý tuần tự, gộp lô tăng tốc bằng `max` thay vì `sum`, cấp phát bộ nhớ theo
khối giảm lãng phí. BOSS quý này KHÔNG thêm khái niệm mới — nó RÁP LẠI bài
`2` VÀ bài `3` thành MỘT câu hỏi vận hành thật: hàng đợi `N` request tới
CÙNG lúc, chiến lược nào PHỤC VỤ được nhiều hơn trên mỗi đơn vị thời gian?
::::

::::explain{#thong_luong_la_gi}
**Thông lượng** (throughput) — số request một hệ thống phục vụ được trên
MỖI đơn vị thời gian — LÀ phép đo QUAN TRỌNG hơn "tổng thời gian" khi so
sánh hai chiến lược xử lý một hàng đợi: chiến lược nào cho tổng thời gian
THẤP hơn, trên CÙNG một số request, LUÔN có thông lượng CAO hơn.

```
thong_luong(so_request, tong_thoi_gian) = so_request / tong_thoi_gian
```

Ráp lại đúng hai hàm đã viết:

- `tong_thoi_gian_tuan_tu(danh_sach, chi_phi_khoi_dong)` (bài `2`) — xử lý
  TỪNG request một, chi phí khởi động bị trả LẶP LẠI cho mỗi request.
- `tong_thoi_gian_gop_lo(danh_sach, chi_phi_khoi_dong, kich_thuoc_lo)` (bài
  `3`) — gộp NHIỀU request vào một lô, chi phí khởi động chỉ trả MỘT LẦN
  cho cả lô, thời gian tính LÀ `max` của lô (chạy song song).

Đưa CÙNG một hàng đợi qua CẢ HAI hàm, rồi tính thông lượng của MỖI chiến
lược — đó LÀ toàn bộ phép đo của BOSS quý này.
::::

::::example{#hang_doi_10_request_hai_chien_luoc}
```python title=readonly
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


def thong_luong(so_request, tong_thoi_gian):
    return so_request / tong_thoi_gian


HANG_DOI = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
CHI_PHI_KHOI_DONG = 200
KICH_THUOC_LO = 10

tong_tuan_tu = tong_thoi_gian_tuan_tu(HANG_DOI, CHI_PHI_KHOI_DONG)
tong_gop_lo = tong_thoi_gian_gop_lo(HANG_DOI, CHI_PHI_KHOI_DONG, KICH_THUOC_LO)

thong_luong_tuan_tu = thong_luong(len(HANG_DOI), tong_tuan_tu)
thong_luong_gop_lo = thong_luong(len(HANG_DOI), tong_gop_lo)

gop_lo_thang = thong_luong_gop_lo > thong_luong_tuan_tu

print(tong_tuan_tu)
print(tong_gop_lo)
print(round(thong_luong_tuan_tu, 6))
print(round(thong_luong_gop_lo, 6))
print(round(thong_luong_gop_lo / thong_luong_tuan_tu, 2))
print(gop_lo_thang)
```

```text title=readonly
2100
210
0.004762
0.047619
10.0
True
```

`HANG_DOI` gồm `10` request, mỗi request cần `10` đơn vị tính toán thật,
`CHI_PHI_KHOI_DONG = 200`. Xử lý TUẦN TỰ: `10 * (200+10) = 2100`, thông
lượng `10/2100 ≈ 0,004762` request MỖI đơn vị thời gian. Gộp CẢ `10`
request vào MỘT lô duy nhất (`KICH_THUOC_LO = 10`): chi phí khởi động
`200` chỉ trả MỘT lần, thời gian tính LÀ `max([10]*10) = 10` — tổng
`200+10 = 210`, thông lượng `10/210 ≈ 0,047619`. Vì cả hai thông lượng
CÙNG chia cho `10` (số request trong hàng đợi), tỉ lệ giữa chúng đúng bằng
tỉ lệ giữa hai TỔNG thời gian, tính NGƯỢC lại: `2100 / 210 = 10,0` — gộp lô
phục vụ được nhiều GẤP ĐÚNG `10` LẦN số request trên CÙNG một đơn vị thời
gian, chính bằng số request trong hàng đợi — vì chi phí khởi động (vốn
chiếm gần hết thời gian tuần tự) giờ chỉ còn xuất hiện đúng MỘT lần thay
vì `10` lần.
::::

::::predict{#doan_kich_thuoc_lo_bang_1 commitOnce}
Xét lại ĐÚNG `HANG_DOI` này, nhưng đổi `KICH_THUOC_LO` xuống `1` — nghĩa
là MỖI lô chỉ chứa đúng `1` request.

**Trước khi chạy thử**, bạn đoán: thông lượng gộp lô (`kich_thuoc_lo=1`)
so với thông lượng tuần tự sẽ THẾ NÀO?

:::opt{correct}
BẰNG NHAU TUYỆT ĐỐI — một lô kích thước `1` chỉ chứa ĐÚNG một request, nên
`max` của lô đó LÀ chính request đó, và chi phí khởi động vẫn bị trả cho
TỪNG lô một (tức TỪNG request một) — `tong_thoi_gian_gop_lo` với
`kich_thuoc_lo=1` tính ra ĐÚNG công thức của `tong_thoi_gian_tuan_tu`,
không hề có lợi ích gộp lô nào cả
:::

:::opt
Gộp lô vẫn nhanh hơn một chút, vì bản thân cơ chế "gộp lô" luôn mang lại
lợi ích, bất kể kích thước lô LÀ bao nhiêu
::why
Gần đúng ở việc gộp lô THƯỜNG mang lại lợi ích — với kích thước lô `> 1`,
điều đó đúng (chính LÀ nội dung bài `3`).

Chỗ lệch: lợi ích của gộp lô đến TỪ việc CHIA SẺ chi phí khởi động giữa
NHIỀU request trong CÙNG một lô. Với `kich_thuoc_lo=1`, mỗi lô chỉ có ĐÚNG
một request — không CÓ gì để chia sẻ cả. Công thức
`tong_thoi_gian_gop_lo` với `kich_thuoc_lo=1` thoái hoá về ĐÚNG hành vi của
`tong_thoi_gian_tuan_tu`: trả chi phí khởi động riêng cho từng request một.
::
:::

:::opt
Gộp lô sẽ CHẬM hơn tuần tự, vì việc chia hàng đợi thành các lô (dù mỗi lô
chỉ có `1` phần tử) vẫn tốn thêm bước xử lý so với vòng lặp tuần tự đơn
giản
::why
Gần đúng ở trực giác rằng có THÊM một bước (chia lô) có thể có chi phí
riêng — trong một hệ thống THẬT, việc quản lý lô đôi khi có chi phí phụ
nhỏ.

Chỗ lệch: trong đúng mô hình SỐ HỌC của bài này, `tong_thoi_gian_gop_lo`
không hề mô phỏng "chi phí quản lý lô" nào cả — nó CHỈ cộng
`chi_phi_khoi_dong + max(lo)` cho mỗi lô. Với `kich_thuoc_lo=1`, `max` của
một danh sách có `1` phần tử LUÔN LÀ chính phần tử đó, nên công thức thu
gọn về CHÍNH XÁC công thức tuần tự — không nhanh hơn, cũng không chậm hơn,
mà LÀ BẰNG NHAU TUYỆT ĐỐI.
::
:::
::::

::::code{#viet_thong_luong_va_ket_luan}
Hoàn thiện `thong_luong` (chia `so_request` cho `tong_thoi_gian`) và kết
luận cuối cùng: `gop_lo_thang` — gộp lô có đạt thông lượng cao hơn tuần tự
không?

```python title=starter
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


def thong_luong(so_request, tong_thoi_gian):
    return ___                                                # so_request / tong_thoi_gian


HANG_DOI = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
CHI_PHI_KHOI_DONG = 200
KICH_THUOC_LO = 10

tong_tuan_tu = tong_thoi_gian_tuan_tu(HANG_DOI, CHI_PHI_KHOI_DONG)
tong_gop_lo = tong_thoi_gian_gop_lo(HANG_DOI, CHI_PHI_KHOI_DONG, KICH_THUOC_LO)

thong_luong_tuan_tu = thong_luong(len(HANG_DOI), tong_tuan_tu)
thong_luong_gop_lo = thong_luong(len(HANG_DOI), tong_gop_lo)

gop_lo_thang = ___                                            # thong_luong_gop_lo > thong_luong_tuan_tu

print(tong_tuan_tu)
print(tong_gop_lo)
print(round(thong_luong_tuan_tu, 6))
print(round(thong_luong_gop_lo, 6))
print(round(thong_luong_gop_lo / thong_luong_tuan_tu, 2))
print(gop_lo_thang)
```

```python title=solution
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


def thong_luong(so_request, tong_thoi_gian):
    return so_request / tong_thoi_gian


HANG_DOI = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
CHI_PHI_KHOI_DONG = 200
KICH_THUOC_LO = 10

tong_tuan_tu = tong_thoi_gian_tuan_tu(HANG_DOI, CHI_PHI_KHOI_DONG)
tong_gop_lo = tong_thoi_gian_gop_lo(HANG_DOI, CHI_PHI_KHOI_DONG, KICH_THUOC_LO)

thong_luong_tuan_tu = thong_luong(len(HANG_DOI), tong_tuan_tu)
thong_luong_gop_lo = thong_luong(len(HANG_DOI), tong_gop_lo)

gop_lo_thang = thong_luong_gop_lo > thong_luong_tuan_tu

print(tong_tuan_tu)
print(tong_gop_lo)
print(round(thong_luong_tuan_tu, 6))
print(round(thong_luong_gop_lo, 6))
print(round(thong_luong_gop_lo / thong_luong_tuan_tu, 2))
print(gop_lo_thang)
```

```python title=test
assert thong_luong(10, 2100) == 10 / 2100, f"thong_luong(10, 2100) phai la 10/2100 -- dang ra {thong_luong(10, 2100)}"
assert thong_luong(1, 210) == 1 / 210, f"thong_luong(1, 210) phai la 1/210 -- dang ra {thong_luong(1, 210)}"
assert round(thong_luong_tuan_tu, 6) == round(10 / 2100, 6), f"thong_luong_tuan_tu phai xap xi 0,004762 -- dang ra {thong_luong_tuan_tu}"
assert round(thong_luong_gop_lo, 6) == round(10 / 210, 6), f"thong_luong_gop_lo phai xap xi 0,047619 -- dang ra {thong_luong_gop_lo}"
assert gop_lo_thang is True, f"gop lo PHAI dat thong luong cao hon tuan tu tren hang doi nay -- dang ra {gop_lo_thang}"
assert round(thong_luong_gop_lo / thong_luong_tuan_tu, 2) == 10.0, f"ti le thong luong gop_lo/tuan_tu phai la DUNG 10,0 -- dang ra {round(thong_luong_gop_lo / thong_luong_tuan_tu, 2)}"
assert tong_thoi_gian_gop_lo(HANG_DOI, CHI_PHI_KHOI_DONG, 1) == tong_thoi_gian_tuan_tu(HANG_DOI, CHI_PHI_KHOI_DONG), f"kich_thuoc_lo=1 phai cho KET QUA GIONG HET tuan tu (khong loi ich gop lo nao) -- gop_lo={tong_thoi_gian_gop_lo(HANG_DOI, CHI_PHI_KHOI_DONG, 1)}, tuan_tu={tong_thoi_gian_tuan_tu(HANG_DOI, CHI_PHI_KHOI_DONG)}"
assert tong_tuan_tu == 2100, f"tong_tuan_tu (bien demo) phai la 2100 -- dang ra {tong_tuan_tu}"
assert tong_gop_lo == 210, f"tong_gop_lo (bien demo) phai la 210 -- dang ra {tong_gop_lo}"
```

:::hints
- kind: attention
  body: "Hai cho trong o hai vi tri khac han nhau. Cho dau nam BEN TRONG ham thong_luong -- la GIA TRI TRA VE cua ham do (mot phep CHIA). Cho hai nam O CAP MODULE, SAU khi da tinh xong ca thong_luong_tuan_tu lan thong_luong_gop_lo -- la KET LUAN cuoi cung, mot phep SO SANH giua hai thong luong vua tinh."
- kind: strategy
  body: "Cho dau: so_request / tong_thoi_gian -- chia so request cho tong thoi gian, dung công thuc thong luong da neu o phan giai thich. Cho hai: thong_luong_gop_lo > thong_luong_tuan_tu -- so sanh truc tiep hai bien da tinh o hai dong ngay truoc do, KHONG tinh lai tu dau."
- kind: one-line
  body: "Cho dau la so_request / tong_thoi_gian, cho hai la thong_luong_gop_lo > thong_luong_tuan_tu."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau (ben trong ham thong_luong) phai la mot phep CHIA so_request / tong_thoi_gian; cho trong hai (ket luan gop_lo_thang) phai SO SANH bang toan tu > giua thong_luong_gop_lo va thong_luong_tuan_tu -- dung sai chieu (vd dung <) se bi static bat
  requireAst:
  - kind: uses-operator, target: "/", min: 2
  - kind: uses-operator, target: ">", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay) -- ket qua dung du kien:
  #   "/"=2: 1 lan o cho trong dau (so_request / tong_thoi_gian, BEN TRONG
  #   ham thong_luong), CONG 1 lan o dong in ket qua da cho san
  #   ("round(thong_luong_gop_lo / thong_luong_tuan_tu, 2)") -- tong THAT
  #   la 2.
  #   ">"=1: DUY NHAT o cho trong hai (thong_luong_gop_lo >
  #   thong_luong_tuan_tu). Khong co dau ">" nao khac trong ca file.
  # Dien bua "True" vao CA HAI cho trong cho "/"=1 (chi con phep chia da
  # cho san o dong in, mat phep chia trong ham thong_luong) VA ">"=0 --
  # CA HAI luat CHAN DUNG.
  # Mutant "sai chieu so sanh" (dung < thay vi >, hoac ke ca >=): da tu
  # dung "gop_lo_thang = thong_luong_gop_lo < thong_luong_tuan_tu" va CHAY
  # THAT qua kiemAst -- ">"=0 (target ">" chi khop ast.Gt, khong khop
  # ast.Lt) -- duoi nguong toi thieu 1, static CHAN DUNG NGAY, du output
  # tren du lieu demo van co the "tinh co" dung neu ai do doi ca huong so
  # sanh LAN gia tri boolean mong doi -- static bat theo HINH DANG, khong
  # phu thuoc may man so hoc.
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong
  # -- xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua
  # kiemAst va python3): dien "thong_luong_gop_lo > thong_luong_tuan_tu"
  # vao cho trong dau (ben trong ham thong_luong, dong "return
  # thong_luong_gop_lo > thong_luong_tuan_tu") VA dien "so_request /
  # tong_thoi_gian" vao cho trong hai (dong module-level "gop_lo_thang =
  # so_request / tong_thoi_gian") -- tong so lan "/" VA ">" tren TOAN BO
  # solution KHONG DOI (van la 2 va 1, chi doi VI TRI) -- static KHONG bat
  # duoc mutant nay, da xac nhan CHAY THAT qua kiemAst.
  # Mutant nay BI BAT boi tier 'run': ben trong ham thong_luong (tham so la
  # so_request, tong_thoi_gian), bieu thuc moi "thong_luong_gop_lo >
  # thong_luong_tuan_tu" dung hai TEN CHUA HE TON TAI trong scope cua ham
  # nay (chung la BIEN TOAN CUC duoc GAN SAU, boi CHINH loi goi ham nay) --
  # khi thong_luong(...) duoc goi LAN DAU (de tinh thong_luong_tuan_tu),
  # Python tim "thong_luong_gop_lo" o pham vi toan cuc va CHUA THAY (bien
  # do se duoc gan O DONG SAU) -- NameError NGAY LAP TUC. Da tu chay THAT
  # qua python3, xac nhan thong bao "name 'thong_luong_gop_lo' is not
  # defined" -- bi chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^2100\\n210\\n0\\.004762\\n0\\.047619\\n10\\.0\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`10` lần thông lượng cao hơn — trên CÙNG một hàng đợi, chỉ đổi CHIẾN LƯỢC
xử lý. `q8.6a` đóng tại `5/5`: ba tầng kiến trúc đã có tên, chi phí khởi
động lặp lại đã đo được, gộp lô đã tăng tốc VÀ tăng thông lượng, cấp phát
bộ nhớ theo khối đã giảm lãng phí. Track tiếp theo của `T8.6` — `ai-gateway`
— hỏi câu hỏi kế tiếp: cổng đứng TRƯỚC tầng `Inference` này giới hạn tốc
độ NHƯ THẾ NÀO cho công bằng?
::::

::::reflect{#nghi-lai}
BOSS quý này không đưa ra công thức MỚI nào — `thong_luong` chỉ LÀ một
phép chia. Cái nó làm được LÀ đặt CẢ HAI chiến lược (bài `2`, bài `3`) LÊN
cùng một hàng đợi, rồi hỏi đúng MỘT câu hỏi vận hành: chiến lược nào phục
vụ được NHIỀU request hơn trên MỖI đơn vị thời gian? Câu trả lời — gộp lô
nhanh gấp `10` lần trên hàng đợi `10` request NÀY — không phải một hằng số
CỐ ĐỊNH của mọi hệ thống; nó phụ thuộc TRỰC TIẾP vào tỉ lệ giữa chi phí
khởi động VÀ thời gian tính toán thật (câu hỏi bài `2` VÀ bài `3` đã đặt
ra). `q8.6a` đóng tại `5/5` — bốn khái niệm của `Chương 39.1`/`39.2` giờ
đều LÀ những hàm Python chạy được VÀ đo được bằng số cụ thể, không còn LÀ
văn xuôi mô tả một hệ thống production xa lạ.
::::

::::checkpoint{mastery=0.82}
::::
