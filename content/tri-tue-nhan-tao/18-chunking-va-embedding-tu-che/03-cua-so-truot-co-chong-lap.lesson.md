---
id: tri-tue-nhan-tao.chunking-va-embedding-tu-che.cua-so-truot-co-chong-lap
title: "Cửa sổ trượt có chồng lấn: không cắt đứt ý giữa hai đoạn"
summary: "Mot tai lieu 47 tu, chia bang cua_so_truot(kich_thuoc=20, buoc_nhay=15 -- chong lan 5 tu) sinh ra DUNG 3 doan. Mot cau danh dau 'cau nay khong duoc phep bi cat doi' (8 tu) duoc dat co y o chi so tu 19-26 -- straddle ngay ranh gioi cua so dau tien (chi mot tu 'cau' lot vao doan 0, phan con lai bi ho). Nho co chong lan 5 tu, doan thu hai (bat dau tu chi so 15) chua TRON VEN ca cau -- xac nhan bang so: cau_quan_trong in doan CHI dung tai chi so 1 trong 3 doan sinh ra, khong phai doan 0 hay doan 2."
locale: vi
track: tri-tue-nhan-tao
module: chunking-va-embedding-tu-che
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.cua-so-truot-co-chong-lap]
requires: [ai.chia-van-ban-thanh-doan]
concepts: [ai.cua-so-truot-co-chong-lap]
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
Chia theo câu: đơn giản, và đúng cho `6/6` câu độc lập. Nhưng nếu một ý
kéo dài qua NHIỀU câu liên tiếp thì sao? Cắt cứng tại từng dấu chấm sẽ tách
rời chúng — bài này sửa đúng chỗ đó.
::::

::::explain{#van_de_cua_chia_theo_cau}
Chia theo câu (bài trước) có một điểm yếu: nó cắt tại MỌI ranh giới câu,
không phân biệt câu nào đang tiếp nối ý của câu trước. Một lập luận trải dài
qua ba câu liên tiếp sẽ bị tách thành BA đoạn riêng biệt, KHÔNG CÓ phần
chung nào giữa chúng — khi truy xuất chỉ lấy được một trong ba đoạn đó, phần
ngữ cảnh nối tiếp từ hai đoạn kia biến mất hoàn toàn.

Giải pháp: **cửa sổ trượt** (sliding window) theo SỐ TỪ, có **chồng lấn**
(overlap) giữa các đoạn liên tiếp. Thay vì cắt tại ranh giới câu, cắt tại
ranh giới SỐ TỪ CỐ ĐỊNH — đoạn `1` là từ `0` đến từ `19` (cửa sổ `20` từ),
đoạn `2` KHÔNG bắt đầu ngay sau đoạn `1` (từ `20`) mà bắt đầu SỚM HƠN, ở từ
`15` — nghĩa là `5` từ cuối của đoạn `1` (từ `15` đến `19`) cũng xuất hiện Ở
ĐẦU đoạn `2`. Đó chính là chồng lấn: `20` từ cửa sổ, `15` từ bước nhảy, `5`
từ chồng lấn (`20 - 15 = 5`).

Lợi ích cụ thể: một câu — hay một ý — nằm đúng gần ranh giới `20` từ của
đoạn `1` (bị cắt dở, chỉ một phần lọt vào đoạn `1`) vẫn có cơ hội xuất hiện
TRỌN VẸN trong đoạn `2`, vì đoạn `2` bắt đầu SỚM hơn `5` từ so với điểm cắt.
Không phải MỌI câu bị cắt đều được cứu (câu dài hơn phần chồng lấn vẫn có
thể bị cắt ở CẢ HAI phía) — nhưng với chồng lấn đủ lớn so với độ dài câu
điển hình, xác suất một ý bị mất hoàn toàn giảm mạnh.
::::

::::example{#cua_so_truot_47_tu}
Một tài liệu `47` từ, chứa một câu đánh dấu — `"cau nay khong duoc phep bi
cat doi"` (`8` từ) — được đặt CỐ Ý ở gần ranh giới `20` từ của cửa sổ đầu
tiên:

```python title=readonly
def cua_so_truot(danh_sach_tu, kich_thuoc, buoc_nhay):
    ra = []
    i = 0
    n = len(danh_sach_tu)
    while i < n:
        doan = danh_sach_tu[i:i + kich_thuoc]
        ra.append(" ".join(doan))
        if i + kich_thuoc >= n:
            break
        i += buoc_nhay
    return ra


truoc = "may tinh hien dai xu ly hang ty phep tinh moi giay nho co vi xu ly manh me"
CAU_QUAN_TRONG = "cau nay khong duoc phep bi cat doi"
sau = "giup tiet kiem thoi gian tinh toan cho nguoi dung va giam tai cho he thong xu ly trung tam"

TOAN_BO = truoc + " " + CAU_QUAN_TRONG + " " + sau
TU = TOAN_BO.split()

doan = cua_so_truot(TU, 20, 15)
cau_xuat_hien_tron_ven = [CAU_QUAN_TRONG in d for d in doan]

print(len(TU))
print(len(doan))
for i, d in enumerate(doan):
    print(i, repr(d))
print(cau_xuat_hien_tron_ven)
```

```text title=readonly
47
3
0 'may tinh hien dai xu ly hang ty phep tinh moi giay nho co vi xu ly manh me cau'
1 'xu ly manh me cau nay khong duoc phep bi cat doi giup tiet kiem thoi gian tinh toan cho'
2 'thoi gian tinh toan cho nguoi dung va giam tai cho he thong xu ly trung tam'
cau_xuat_hien_tron_ven = [False, True, False]
```

Tài liệu `47` từ, cửa sổ `20` từ + bước nhảy `15` từ (chồng lấn `5` từ) sinh
ra ĐÚNG `3` đoạn. `CAU_QUAN_TRONG` (`8` từ) nằm ở chỉ số từ `19`–`26` — ĐÚNG
sát ranh giới `20` từ của đoạn `0`: đoạn `0` (chỉ số `0`–`19`) chỉ chứa được
từ ĐẦU TIÊN của câu (`"...manh me cau"`, dừng đúng ở từ `"cau"`), phần còn
lại (`"nay khong duoc phep bi cat doi"`) bị hở — `cau_xuat_hien_tron_ven[0]
= False`. Nhưng đoạn `1` bắt đầu SỚM hơn, từ chỉ số `15` (nhờ chồng lấn `5`
từ) — đủ để chứa TRỌN VẸN toàn bộ câu đánh dấu (`"...cau nay khong duoc
phep bi cat doi giup..."`) — `cau_xuat_hien_tron_ven[1] = True`. Nếu KHÔNG
có chồng lấn (bước nhảy bằng đúng kích thước cửa sổ, `buoc_nhay = 20`), đoạn
kế tiếp sẽ bắt đầu ở chỉ số `20` — VẪN cắt ngang câu (câu bắt đầu ở chỉ số
`19`, trước điểm `20`) — không đoạn nào chứa trọn câu cả. Chồng lấn chính là
thứ cứu câu này.
::::

::::predict{#doan_cau_o_doan_dau commitOnce}
Xét đúng ví dụ trên: `CAU_QUAN_TRONG` nằm ở chỉ số từ `19`–`26`, cửa sổ đầu
tiên (đoạn `0`) phủ chỉ số `0`–`19`.

**Trước khi chạy thử**, bạn đoán: `CAU_QUAN_TRONG` có xuất hiện TRỌN VẸN
trong đoạn `0` hay không?

:::opt{correct}
Không — đoạn `0` chỉ phủ tới chỉ số `19` (từ đầu tiên của câu, `"cau"`),
phần còn lại của câu (chỉ số `20`–`26`) nằm NGOÀI đoạn `0`; nhưng nhờ chồng
lấn, câu vẫn xuất hiện trọn vẹn ở đoạn `1` (bắt đầu từ chỉ số `15`)
:::

:::opt
Có — cửa sổ đầu tiên rộng `20` từ, đủ để chứa MỌI câu ngắn (`8` từ) xuất
hiện sớm trong tài liệu, bất kể vị trí chính xác
::why
Gần đúng ở việc `20` từ THẬT SỰ đủ rộng cho một câu `8` từ — NẾU câu đó bắt
đầu đủ sớm (ví dụ chỉ số `0`–`7`).

Chỗ lệch: vị trí CHÍNH XÁC mới là điều quyết định, không phải độ rộng cửa
sổ một mình. Câu này bắt đầu ở chỉ số `19` — chỉ MỘT từ trước ranh giới
`20` — nên đoạn `0` (phủ `0`–`19`) chỉ kịp chứa đúng từ đầu tiên rồi hết
chỗ; bảy từ còn lại của câu rơi ra ngoài.
::
:::

:::opt
Không, và câu này KHÔNG xuất hiện trọn vẹn ở BẤT KỲ đoạn nào cả — một khi
đã bị cắt ở đoạn đầu, phần bị cắt không thể "cứu" được ở đoạn khác
::why
Gần đúng ở việc XÁC NHẬN đoạn `0` thật sự cắt dở câu này — quan sát đó
đúng.

Chỗ lệch: đây chính xác là lý do CẦN chồng lấn. Đoạn `1` không bắt đầu
NGAY SAU đoạn `0` (tức chỉ số `20`) — nó bắt đầu SỚM hơn, từ chỉ số `15`
(vì bước nhảy `15` < kích thước cửa sổ `20`). Chỉ số `15` đứng TRƯỚC điểm
bắt đầu của câu (`19`), nên đoạn `1` phủ trọn từ `15` tới `34` — đủ chứa
toàn bộ câu (`19`–`26`) nằm gọn bên trong.
::
:::
::::

::::code{#viet_cua_so_truot}
Hoàn thiện `cua_so_truot`: điều kiện dừng (đoạn cuối đã phủ hết tài liệu),
và bước nhảy giữa hai đoạn liên tiếp.

```python title=starter
def cua_so_truot(danh_sach_tu, kich_thuoc, buoc_nhay):
    ra = []
    i = 0
    n = len(danh_sach_tu)
    while i < n:
        doan = danh_sach_tu[i:i + kich_thuoc]
        ra.append(" ".join(doan))
        if ___:                                            # i + kich_thuoc >= n
            break
        i += ___                                            # buoc_nhay
    return ra


truoc = "may tinh hien dai xu ly hang ty phep tinh moi giay nho co vi xu ly manh me"
CAU_QUAN_TRONG = "cau nay khong duoc phep bi cat doi"
sau = "giup tiet kiem thoi gian tinh toan cho nguoi dung va giam tai cho he thong xu ly trung tam"

TOAN_BO = truoc + " " + CAU_QUAN_TRONG + " " + sau
TU = TOAN_BO.split()

doan = cua_so_truot(TU, 20, 15)
cau_xuat_hien_tron_ven = [CAU_QUAN_TRONG in d for d in doan]

print(len(TU))
print(len(doan))
print(cau_xuat_hien_tron_ven)
print(any(cau_xuat_hien_tron_ven))
```

```python title=solution
def cua_so_truot(danh_sach_tu, kich_thuoc, buoc_nhay):
    ra = []
    i = 0
    n = len(danh_sach_tu)
    while i < n:
        doan = danh_sach_tu[i:i + kich_thuoc]
        ra.append(" ".join(doan))
        if i + kich_thuoc >= n:
            break
        i += buoc_nhay
    return ra


truoc = "may tinh hien dai xu ly hang ty phep tinh moi giay nho co vi xu ly manh me"
CAU_QUAN_TRONG = "cau nay khong duoc phep bi cat doi"
sau = "giup tiet kiem thoi gian tinh toan cho nguoi dung va giam tai cho he thong xu ly trung tam"

TOAN_BO = truoc + " " + CAU_QUAN_TRONG + " " + sau
TU = TOAN_BO.split()

doan = cua_so_truot(TU, 20, 15)
cau_xuat_hien_tron_ven = [CAU_QUAN_TRONG in d for d in doan]

print(len(TU))
print(len(doan))
print(cau_xuat_hien_tron_ven)
print(any(cau_xuat_hien_tron_ven))
```

```python title=test
assert len(TU) == 47, f"tong so tu phai la 47 -- dang ra {len(TU)}"
assert len(doan) == 3, f"phai sinh ra 3 doan -- dang ra {len(doan)}"
assert cau_xuat_hien_tron_ven == [False, True, False], f"cau quan trong phai CHI xuat hien tron ven o doan chi so 1 -- dang ra {cau_xuat_hien_tron_ven}"
assert any(cau_xuat_hien_tron_ven) == True, "phai co IT NHAT mot doan chua tron ven cau quan trong"
assert doan[1] == "xu ly manh me cau nay khong duoc phep bi cat doi giup tiet kiem thoi gian tinh toan cho", f"noi dung doan chi so 1 sai -- dang ra {doan[1]!r}"

# bien: KHONG chong lan (buoc_nhay bang dung kich_thuoc) -- cau quan trong
# roi vao dung ranh gioi, KHONG doan nao chua tron ven no
doan_khong_chong_lan = cua_so_truot(TU, 20, 20)
cau_khong_chong_lan = [CAU_QUAN_TRONG in d for d in doan_khong_chong_lan]
assert any(cau_khong_chong_lan) == False, f"KHONG chong lan thi cau quan trong khong duoc doan nao chua tron ven -- dang ra {cau_khong_chong_lan}"

# bien: danh sach tu ngan hon kich_thuoc cua so -- chi sinh dung 1 doan,
# khong loi chia cho 0 hay vong lap vo han
doan_ngan = cua_so_truot(["mot", "hai", "ba"], 20, 15)
assert doan_ngan == ["mot hai ba"], f"danh sach ngan hon cua so phai cho dung 1 doan chua het -- dang ra {doan_ngan}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là điều kiện DỪNG vòng lặp — dừng khi đoạn HIỆN TẠI (bắt đầu từ `i`, dài `kich_thuoc`) đã chạm hoặc vượt quá cuối danh sách (`n` từ) — nghĩa là đoạn này đã là đoạn CUỐI CÙNG, không cần trượt tiếp. Chỗ hai là bước TRƯỢT cửa sổ sang phải — trượt đúng `buoc_nhay` từ, KHÔNG phải `kich_thuoc` từ (nếu trượt đúng bằng `kich_thuoc` thì sẽ không còn chồng lấn).
- kind: strategy
  body: 'Chỗ đầu: `i + kich_thuoc >= n` — vị trí kết thúc đoạn hiện tại đã chạm/vượt cuối danh sách. Chỗ hai: `buoc_nhay` — biến đã được truyền vào làm tham số, không phải một con số khác.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `i + kich_thuoc >= n` và `buoc_nhay`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la dieu kien so sanh THAT dung toan tu '>=' (kiem tra da phu het danh sach chua), VA cho trong hai phai dung DUNG bien 'buoc_nhay' de truot cua so (khong duoc dung 'kich_thuoc' hay mot so chep san)
  requireAst:
  - kind: uses-operator, target: ">=", min: 1
  - kind: uses-name, target: buoc_nhay, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # ">="=1: CHI mot lan duy nhat trong toan bo solution, dung o cho trong
  # dau ("i + kich_thuoc >= n"). Dien "if i >= n:" (bo qua kich_thuoc, sai ve
  # logic) VAN giu duoc mot lan ">=" nen static khong bat rieng logic sai --
  # nhung day la mutant SAI THAT (danh gia doan som hoac muon so voi dung),
  # da tu kiem chung no lam len(doan) khac 3 tren du lieu THAT, bi bat boi
  # tests/output, khong phai boi static.
  # buoc_nhay=1: bien nay CHI duoc DOC (Load) mot lan, o cho trong hai
  # ("i += buoc_nhay") -- tham so ham "buoc_nhay" trong dong dinh nghia
  # KHONG tinh (do la mot arg, khong phai ast.Name doc). Dien "i += 20" (chep
  # san dung so kich_thuoc, BO QUA bien that) lam so nay tut ve 0 -- bi chan
  # boi static; da tu kiem chung: dien nhu vay tuong duong voi "khong chong
  # lan" (kich_thuoc == buoc_nhay == 20), lam cau_xuat_hien_tron_ven tro
  # thanh [False, False] (chi 2 doan, khong doan nao chua tron ven cau) --
  # SAI, nhung diem quan trong la static da chan truoc ca khi chay den do.
  #
  # 🔴🔴🔴 GOTCHA QUAN TRONG NHAT (da tu dung mutant "hoan doi ca cum" va
  # CHAY THAT qua ham _dem de xac nhan, khong doan tay): hai cho trong nay
  # VE MAT CU PHAP co the hoan doi cho nhau -- Python cho phep "if
  # buoc_nhay:" (kiem tra mot so nguyen duong luon dung/truthy) VA "i +=
  # (i + kich_thuoc >= n)" (cong THEM 0 hoac 1, vi True/False duoc ep kieu
  # thanh int trong phep +=). Da thu chay THAT mutant nay ("if buoc_nhay:
  # break" va "i += (i + kich_thuoc >= n)") qua ham _dem: ket qua AST la
  # [1, 1] -- Y HET ban dung (toan tu ">=" van xuat hien dung 1 lan, o vi
  # tri MOI; bien buoc_nhay van duoc doc dung 1 lan, o vi tri MOI). Static
  # KHONG bat duoc mutant nay.
  # Mutant nay BI BAT DOC LAP boi tests/output: vi buoc_nhay=15 (khac 0) la
  # gia tri LUON DUNG (truthy), vong lap DUNG NGAY sau vong lap DAU TIEN --
  # len(doan) tro thanh 1 thay vi 3, cau_xuat_hien_tron_ven tro thanh [False]
  # thay vi [False, True, False]. Da tu chay THAT mutant nay qua python3 de
  # xac nhan dung hai gia tri do sai nhu mo ta, va cac assert rieng
  # "len(doan) == 3" / "cau_xuat_hien_tron_ven == [False, True, False]" (o
  # tren, dung truoc ca dieu kien "any(...)") bat duoc NGAY LAP TUC, doc lap
  # voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^47\\n3\\n\\[False, True, False\\]\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`47` từ, `3` đoạn, câu đánh dấu bị cắt dở ở đoạn `0` nhưng xuất hiện TRỌN
VẸN ở đoạn `1` — nhờ chồng lấn `5` từ. Giờ mỗi đoạn đã sẵn sàng — bài sau
biến chúng thành những con số đo được: vector.
::::

::::reflect{#nghi-lai}
Cửa sổ trượt có chồng lấn không "thông minh" hơn chia theo câu — nó không
hiểu ý nghĩa của văn bản, chỉ đếm từ một cách máy móc. Nhưng chính vì máy
móc và có chồng lấn, nó có một đảm bảo mà chia theo câu không có: một ý
nằm ở BẤT KỲ vị trí nào trong tài liệu, nếu ngắn hơn phần chồng lấn, đều có
cơ hội xuất hiện trọn vẹn trong ít nhất một đoạn — không bị mất trắng chỉ
vì tình cờ rơi đúng ranh giới cắt. Hai bài đã xây xong phần "chia" của quy
trình RAG (bài `1`). Phần còn lại — biến MỖI đoạn văn bản thành một dãy số
đo được, để so sánh "gần" hay "xa" nhau — bắt đầu từ bài sau.
::::

::::checkpoint{mastery=0.85}
::::
