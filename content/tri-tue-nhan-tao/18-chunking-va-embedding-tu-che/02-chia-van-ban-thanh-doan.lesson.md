---
id: tri-tue-nhan-tao.chunking-va-embedding-tu-che.chia-van-ban-thanh-doan
title: "Chia văn bản thành đoạn: chunking theo câu"
summary: "Mot tai lieu 6 cau (ve chu de may tinh) duoc chia bang chia_theo_cau (tach tai dau cham, loc chuoi rong) thanh DUNG 6 doan -- khong phai 7 (neu khong loc chuoi rong o cuoi do dau cham cuoi tai lieu). So tu moi doan: 12, 9, 10, 10, 8, 9 -- tong 58 tu tren 6 doan, khong doan nao qua dai de nhet vao mot ngu canh gioi han. Day la chien luoc chia (chunking) DON GIAN NHAT: cat tai moi dau cau."
locale: vi
track: tri-tue-nhan-tao
module: chunking-va-embedding-tu-che
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.chia-van-ban-thanh-doan]
requires: [ai.vi-sao-can-rag]
concepts: [ai.chia-van-ban-thanh-doan]
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
Bài trước: một đoạn văn bản liên quan, đưa vào ngữ cảnh, cứu một câu trả lời
khỏi ảo giác. Nhưng đoạn đó tới từ đâu? Một tài liệu THẬT dài hơn nhiều một
câu — phải chia nó ra trước đã.
::::

::::explain{#vi_sao_phai_chia_doan}
Một tài liệu dài (một trang tài liệu kỹ thuật, một bài báo, một chương sách)
không thể nhét NGUYÊN VẸN vào một lượt truy xuất — vừa vì cửa sổ ngữ cảnh có
trần (bài `dem-token-that`, q8.4d), vừa vì phần lớn tài liệu dài không liên
quan tới MỌI câu hỏi — chỉ một vài đoạn nhỏ mới thật sự liên quan. Quá trình
cắt một tài liệu dài thành nhiều mảnh nhỏ gọi là **chunking**, và mỗi mảnh
gọi là một **đoạn** (chunk).

*(Ghi chú thiết kế: quest này đếm độ dài đoạn bằng SỐ TỪ — `len(van_ban.split())`
— thay vì token BPE thật của `token-hoa-bpe`/q8.3a. Đếm từ đơn giản hơn và
đủ để dạy CƠ CHẾ chia đoạn; số liệu chính xác vẫn đo được thật, chỉ đơn vị đo
khác — không phải giản lược làm mất bản chất bài học.)*

Chiến lược chia đơn giản nhất: **chia theo câu**. Một câu (kết thúc bằng dấu
chấm hoặc dấu chấm hỏi) thường mang MỘT ý tương đối trọn vẹn, nên cắt tại
ranh giới câu ít có khả năng cắt đứt một ý giữa chừng hơn là cắt tại một vị
trí ký tự tuỳ ý. Cách làm: tách văn bản tại mỗi dấu chấm, loại bỏ khoảng
trắng thừa ở đầu/cuối mỗi phần, và LOẠI BỎ phần rỗng (một tài liệu kết thúc
bằng dấu chấm sẽ luôn sinh ra một phần tử rỗng ở cuối khi tách — phần đó
không phải một đoạn, phải lọc đi).

Một tài liệu `N` câu, chia theo chiến lược này, cho ra ĐÚNG `N` đoạn — mỗi
đoạn là một câu.
::::

::::example{#chia_tai_lieu_6_cau}
Một tài liệu ngắn về máy tính, gồm đúng `6` câu:

```python title=readonly
def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TAI_LIEU = (
    "may tinh hien dai xu ly hang ty phep tinh moi giay. "
    "phan mem duoc viet bang ngon ngu lap trinh. "
    "vi xu ly la bo nao dieu khien toc do. "
    "bo nho luu tam thoi du lieu dang xu ly. "
    "o cung luu tru du lieu lau dai. "
    "mang internet ket noi may tinh khap the gioi."
)

doan = chia_theo_cau(TAI_LIEU)
so_tu_moi_doan = [len(d.split()) for d in doan]

print(len(doan))
print(doan)
print(so_tu_moi_doan)
print(sum(so_tu_moi_doan))
```

```text title=readonly
6
['may tinh hien dai xu ly hang ty phep tinh moi giay', 'phan mem duoc viet bang ngon ngu lap trinh', 'vi xu ly la bo nao dieu khien toc do', 'bo nho luu tam thoi du lieu dang xu ly', 'o cung luu tru du lieu lau dai', 'mang internet ket noi may tinh khap the gioi']
[12, 9, 10, 10, 8, 9]
58
```

Tài liệu có đúng `6` câu, và `chia_theo_cau` cho ra đúng `6` đoạn — một-một.
Độ dài mỗi đoạn khác nhau (`12, 9, 10, 10, 8, 9` từ), tổng cộng `58` từ trên
toàn tài liệu, không đoạn nào áp đảo — mỗi đoạn đủ ngắn để nhét thoải mái vào
một ngữ cảnh có trần thấp. Quan trọng hơn: nếu KHÔNG lọc chuỗi rỗng,
`TAI_LIEU.split(".")` (tách thô, không lọc) cho ra `7` phần tử — phần tử cuối
là chuỗi rỗng `""`, sinh ra bởi dấu chấm kết thúc tài liệu — và đó KHÔNG phải
một đoạn thật.
::::

::::predict{#doan_so_doan_sinh_ra commitOnce}
Xét đúng `TAI_LIEU` (`6` câu, kết thúc bằng dấu chấm) và hàm `chia_theo_cau`
ở ví dụ trên.

**Trước khi chạy thử**, bạn đoán: `len(chia_theo_cau(TAI_LIEU))` là bao
nhiêu?

:::opt{correct}
`6` — đúng bằng số câu trong tài liệu; `chia_theo_cau` LỌC bỏ chuỗi rỗng
sinh ra bởi dấu chấm kết thúc, nên phần tử rỗng đó không được tính vào kết
quả
:::

:::opt
`7` — vì `van_ban.split(".")` tách tại MỌI dấu chấm, kể cả dấu chấm cuối
cùng, nên phải sinh ra `7` phần tử (`6` câu cộng một chuỗi rỗng ở cuối)
::why
Gần đúng ở việc `TAI_LIEU.split(".")` (tách THÔ, không qua `chia_theo_cau`)
thật sự cho ra `7` phần tử — quan sát đó đúng nếu dừng lại ở bước `.split(".")`.

Chỗ lệch: `chia_theo_cau` không dừng ở đó — nó còn có một bước LỌC
(`if cau_sach != "":`) loại bỏ mọi phần tử rỗng SAU KHI `.strip()`. Phần tử
thứ `7` (chuỗi rỗng do dấu chấm cuối) bị loại đúng ở bước này, nên kết quả
CUỐI CÙNG của `chia_theo_cau` là `6`, không phải `7`.
::
:::

:::opt
`5` — vì câu cuối cùng (`"mang internet ket noi may tinh khap the gioi"`)
chỉ là một câu kết luận, không mang thông tin kỹ thuật mới nên bị coi là
thừa và bỏ qua
::why
Gần đúng ở trực giác "câu kết luận ít quan trọng hơn" — một nhận xét hợp lý
khi ĐỌC HIỂU văn bản.

Chỗ lệch: `chia_theo_cau` không hề phân biệt câu nào "quan trọng" hay "kết
luận" — nó chỉ kiểm tra một điều duy nhất: sau khi `.strip()`, phần còn lại
có khác chuỗi rỗng hay không. Câu cuối cùng, dù mang ý gì, sau `.strip()`
vẫn là một chuỗi khác rỗng (`"mang internet ket noi may tinh khap the
gioi"`), nên nó vẫn được giữ lại như mọi câu khác.
::
:::
::::

::::code{#viet_dem_doan_va_tong_tu}
Hoàn thiện phần đếm: số đoạn sinh ra, và tổng số từ trên toàn bộ các đoạn.

```python title=starter
def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TAI_LIEU = (
    "may tinh hien dai xu ly hang ty phep tinh moi giay. "
    "phan mem duoc viet bang ngon ngu lap trinh. "
    "vi xu ly la bo nao dieu khien toc do. "
    "bo nho luu tam thoi du lieu dang xu ly. "
    "o cung luu tru du lieu lau dai. "
    "mang internet ket noi may tinh khap the gioi."
)

doan = chia_theo_cau(TAI_LIEU)
so_tu_moi_doan = [len(d.split()) for d in doan]

so_doan = ___                                              # len(doan)
tong_so_tu = ___                                            # sum(so_tu_moi_doan)

print(so_doan)
print(so_tu_moi_doan)
print(tong_so_tu)
```

```python title=solution
def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


TAI_LIEU = (
    "may tinh hien dai xu ly hang ty phep tinh moi giay. "
    "phan mem duoc viet bang ngon ngu lap trinh. "
    "vi xu ly la bo nao dieu khien toc do. "
    "bo nho luu tam thoi du lieu dang xu ly. "
    "o cung luu tru du lieu lau dai. "
    "mang internet ket noi may tinh khap the gioi."
)

doan = chia_theo_cau(TAI_LIEU)
so_tu_moi_doan = [len(d.split()) for d in doan]

so_doan = len(doan)
tong_so_tu = sum(so_tu_moi_doan)

print(so_doan)
print(so_tu_moi_doan)
print(tong_so_tu)
```

```python title=test
assert so_doan == 6, f"so_doan phai la 6 (mot cho moi cau) -- dang ra {so_doan}"
assert tong_so_tu == 58, f"tong_so_tu phai la 58 -- dang ra {tong_so_tu}"
assert so_tu_moi_doan == [12, 9, 10, 10, 8, 9], f"so tu moi doan sai -- dang ra {so_tu_moi_doan}"

# bien: tai lieu KHONG ket thuc bang dau cham -- van phai chia dung, khong
# mat cau cuoi cung
tai_lieu_khong_cham_cuoi = "cau mot. cau hai. cau ba"
doan_khong_cham_cuoi = chia_theo_cau(tai_lieu_khong_cham_cuoi)
assert len(doan_khong_cham_cuoi) == 3, f"tai lieu khong co dau cham cuoi van phai cho 3 doan -- dang ra {len(doan_khong_cham_cuoi)}"
assert doan_khong_cham_cuoi[-1] == "cau ba", f"cau cuoi (khong co dau cham) phai duoc giu nguyen -- dang ra {doan_khong_cham_cuoi[-1]!r}"

# bien: chuoi rong -- khong loi, tra ve danh sach rong
assert chia_theo_cau("") == [], "chuoi rong phai cho danh sach rong, khong loi"
```

:::hints
- kind: attention
  body: Hai chỗ trống, cả hai đều là một phép ĐẾM trên dữ liệu đã có sẵn — không tính toán gì phức tạp. Chỗ đầu đếm SỐ ĐOẠN — `doan` đã là một danh sách các đoạn, đếm ĐỘ DÀI danh sách đó. Chỗ hai đếm TỔNG SỐ TỪ trên toàn tài liệu — `so_tu_moi_doan` đã là một danh sách số từ của TỪNG đoạn, cộng dồn tất cả chúng lại.
- kind: strategy
  body: 'Chỗ đầu: `len(doan)` — đếm số phần tử trong danh sách đoạn. Chỗ hai: `sum(so_tu_moi_doan)` — cộng dồn danh sách số từ từng đoạn thành một tổng.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `len(doan)` và `sum(so_tu_moi_doan)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dem THAT do dai danh sach 'doan' (khong phai mot danh sach nao khac) bang len(...) (khong duoc chep san so 6), VA cho trong hai phai cong don THAT danh sach 'so_tu_moi_doan' bang sum(...) (khong duoc chep san so 58)
  requireAst:
  - kind: uses-call, target: len, min: 2
  - kind: uses-call, target: sum, min: 1
  - kind: uses-name, target: doan, min: 2
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [2, 1, 2] cho ba
  # luat theo dung thu tu khai bao o tren.
  # len=2: MOT lan da CO SAN trong bieu thuc list-comprehension
  # "[len(d.split()) for d in doan]" (tinh so tu MOI doan, khong phai cho
  # trong), MOT lan CHINH la cho trong dau ("so_doan = len(doan)"). Neu chi
  # dat min=1 (ngay tho), mot mutant dien "so_doan = 6" (chep san, dung tren
  # du lieu THAT nhung bo qua goi ham that) van qua duoc vi con lai 1 lan
  # goi len(...) trong comprehension -- GOTCHA "boilerplate-threshold-
  # masking"; da tu kiem chung bang python that: mutant nay cho ket qua IN RA
  # giong het ban dung (vi 6 dung la gia tri that), khong bi bat boi tests/
  # output -- CHI static voi min=2 (tong THAT, da xac nhan bang cong cu,
  # khong doan tay) moi chan duoc.
  # sum=1: chi mot lan GOI THAT, dung o cho trong hai -- khong co lan nao
  # khac trong boilerplate goi sum(...). Dien "tong_so_tu = 58" (chep san)
  # lam so nay tut ve 0 -- bi chan.
  # doan=2 (LUAT MOI, them SAU KHI bo sinh dot bien TU DONG cua tools/
  # kiem_dot_bien.mjs bat duoc mot lo THAT tren ban dau chua co luat nay):
  # bien 'doan' duoc DOC (Load) dung 2 lan trong solution -- mot lan CO SAN
  # trong list-comprehension ("for d in doan"), mot lan CHINH la cho trong
  # dau ("len(doan)"). Dien "so_doan = len(so_tu_moi_doan)" (dung SAI danh
  # sach nhung CUNG do dai voi 'doan' -- ca hai deu co 6 phan tu, MOI cau
  # sinh dung MOT gia tri do dai tu tuong ung) lam so lan doc 'doan' tut ve
  # 1 (duoi nguong 2) -- bi chan boi luat nay. Da tu xac nhan bang kiemAst()
  # THAT: mutant nay VAN qua duoc CA static (chi voi hai luat len/sum) LAN
  # tests/output (vi len(so_tu_moi_doan)==len(doan)==6 TRUNG KHOP tinh co
  # trong du lieu nay) -- day la mot lo dot bien THAT do bo sinh dot bien
  # TU DONG cua du an (khong phai mutant tu tay toi/agent dung) phat hien,
  # KHAC voi lop GOTCHA "hoan doi ca cum" da ghi o duoi (mutant nay chi doi
  # MOT ten bien o MOT cho trong, khong hoan doi hai cho trong cho nhau) --
  # mot lop lo hong THU BA: dung SAI bien nhung CUNG DO DAI mot cach tinh
  # co tren du lieu cu the. Da them luat 'uses-name doan min=2' rieng de
  # buoc phai doc dung bien 'doan' (khong phai mot danh sach nao khac cung
  # do dai) — da xac nhan lai bang kiemAst() THAT: loi giai dung van dat=true,
  # mutant nay gio dat=false.
  #
  # 🔴🔴🔴 GOTCHA QUAN TRONG NHAT (da tu dung mutant "hoan doi ca cum" va
  # CHAY THAT qua ham _dem de xac nhan, khong doan tay): hai cho trong nay
  # CUNG la so nguyen (int) va co the hoan doi VE MAT CU PHAP --
  # "so_doan = sum(so_tu_moi_doan)" roi "tong_so_tu = len(doan)" (dat dung
  # SAU khi so_tu_moi_doan da duoc gan, tuc phai doi cho ca dong khai bao
  # so_tu_moi_doan len truoc). Da thu chay THAT mutant nay qua ham _dem: ket
  # qua AST la [2, 1] -- Y HET ban dung, vi tong so lan goi len(...) va
  # sum(...) trong toan bo doan ma KHONG doi (chi DI CHUYEN vi tri, khong
  # doi so luong). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT DOC LAP boi tests/output: no lam so_doan=58 (sai, dung
  # ra phai la 6) va tong_so_tu=6 (sai, dung ra phai la 58) -- nguoc hoan
  # toan so voi mong doi. Da tu chay THAT mutant nay qua python3 de xac nhan
  # dong in dau tien tro thanh "58" thay vi "6", va assert rieng
  # "so_doan == 6" / "tong_so_tu == 58" (o tren) bat duoc NGAY LAP TUC, doc
  # lap voi static va voi thu tu cac dong print.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^6\\n\\[12, 9, 10, 10, 8, 9\\]\\n58\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`6` câu, `6` đoạn, `58` từ tổng cộng — chia theo câu đơn giản, và đúng.
Nhưng nếu một Ý trải dài qua NHIỀU câu liên tiếp, cắt cứng tại từng dấu
chấm sẽ tách rời chúng ra — bài sau xử lý đúng vấn đề đó.
::::

::::reflect{#nghi-lai}
Chia theo câu là chiến lược chunking ĐƠN GIẢN NHẤT: cắt tại mỗi dấu chấm,
lọc phần rỗng, xong. Nó hoạt động tốt khi mỗi câu độc lập mang một ý trọn
vẹn — như tài liệu `6` câu ở bài này. Nhưng ngôn ngữ tự nhiên không phải
lúc nào cũng gọn gàng như vậy: một lập luận, một chuỗi nhân-quả, một ví dụ
minh hoạ thường trải dài qua HAI, BA câu liên tiếp — và cắt cứng tại từng
dấu chấm phá vỡ sự liên kết đó, biến một ý thống nhất thành nhiều mảnh rời
rạc không còn ngữ cảnh nối tiếp nhau. Bài sau giải quyết đúng vấn đề này
bằng một chiến lược chia khác: cửa sổ trượt theo số từ, có chồng lấn.
::::

::::checkpoint{mastery=0.85}
::::
