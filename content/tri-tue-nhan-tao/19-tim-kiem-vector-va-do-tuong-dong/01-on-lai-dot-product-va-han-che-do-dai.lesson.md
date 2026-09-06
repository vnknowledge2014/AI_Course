---
id: tri-tue-nhan-tao.tim-kiem-vector-va-do-tuong-dong.on-lai-dot-product-va-han-che-do-dai
title: "Ôn lại dot product: đoạn càng dài, tích vô hướng càng ăn gian"
summary: "Doan NGAN (19 tu, dung DUNG 4 tu vung cong nghe moi tu 1 lan: may_tinh, phan_mem, lap_trinh, du_lieu) so voi mot CAU_HOI ngan cung chu de cho tich_vo_huong bang 2. Doan DAI (261 tu, LAP LAI ca 8 tu vung cong nghe moi tu DUNG 3 lan) so voi CUNG cau hoi do cho tich_vo_huong bang 6. 6 > 2 -- doan DAI thang, DU doan NGAN moi la doan khop sat cau hoi hon (dung dung 4 trong so 4 tu ma cau hoi de cap toi y). Day la han che ma bai do-tuong-dong-hai-doan-van (q8.5a) da neu trong reflect nhung chua do: tich vo huong tho khong chuan hoa theo do dai vector."
locale: vi
track: tri-tue-nhan-tao
module: tim-kiem-vector-va-do-tuong-dong
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 10
teaches: [ai.on-lai-dot-product-va-han-che-do-dai]
requires: [ai.boss-chia-va-tinh-vector-tai-lieu]
concepts: [ai.on-lai-dot-product-va-han-che-do-dai]
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
Quest `chunking-va-embedding-tu-che` khép lại ở `6/6`: chia đoạn, tính
vector, đo tích vô hướng. T8.5 tiếp tục — và bắt đầu bằng một lỗ hổng chưa
được đo.
::::

::::explain{#han_che_cua_tich_vo_huong_tho}
Bài `do-tuong-dong-hai-doan-van` (q8.5a) đã dạy tích vô hướng
(`tich_vo_huong`): nhân từng cặp thành phần tương ứng của hai vector, cộng
dồn lại thành một con số. Chính bài đó, trong phần ghi chú, đã CẢNH BÁO một
điều nhưng CHƯA đo:

> *"đây là tích vô hướng THÔ, CHƯA chuẩn hoá theo độ dài (độ lớn) của
> vector — một đoạn văn DÀI hơn, dùng từ vựng nhiều lần hơn, sẽ tự nhiên có
> tích vô hướng LỚN hơn một đoạn ngắn, dù mức độ 'cùng chủ đề' tương
> đương."*

Bài này đo đúng điều đó, bằng số thật. Trực giác: `tich_vo_huong(v1, v2) =
sum(a * b for a, b in zip(v1, v2))` — nếu một đoạn văn LẶP LẠI một từ vựng
nhiều lần, thành phần tương ứng trong vector của nó LỚN hơn (thay vì `1`,
có thể là `3`, `5`, ...). Tích của một số lớn với một số dương khác vẫn LỚN
hơn tích của `1` với cùng số đó. Kết quả: đoạn càng dài, càng lặp từ vựng
nhiều lần, tích vô hướng với BẤT KỲ câu hỏi nào chạm tới từ vựng đó càng có
xu hướng cao hơn — bất kể đoạn đó có thực sự "khớp nội dung" với câu hỏi hay
không. Đây không phải một trường hợp hiếm — nó là hệ quả TẤT YẾU của phép
nhân và cộng, xảy ra với MỌI cặp đoạn văn mà một đoạn lặp từ nhiều hơn đoạn
kia.
::::

::::example{#do_han_che_do_dai_bang_so}
Một câu hỏi ngắn (`CAU_HOI`), một đoạn NGẮN cùng chủ đề công nghệ (dùng đúng
`4` mục từ vựng, mỗi mục đúng `1` lần), và một đoạn DÀI cùng chủ đề công
nghệ nhưng LẶP LẠI cả `8` mục từ vựng công nghệ, mỗi mục đúng `3` lần:

```python title=readonly
TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]

CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


CAU_HOI = "may tinh chay phan mem nao on dinh nhat"
DOAN_NGAN = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh de xu ly du lieu"
DOAN_DAI = (
    "may tinh hien dai giup con nguoi lam viec nhanh hon truoc "
    "moi nguoi deu muon mua mot chiec may tinh that tot "
    "gia ban mot chiec may tinh ngay nay re hon nam truoc "
    "phan mem tot giup nguoi dung cam thay hai long "
    "ai cung thich mot phan mem de dung moi ngay "
    "cap nhat phan mem thuong xuyen giup tranh loi vat "
    "hoc lap trinh can kien nhan va luyen tap moi ngay "
    "mot du an lap trinh lon can nhieu nguoi cung lam "
    "nguoi moi bat dau lap trinh nen chon mot ngon ngu de "
    "du lieu duoc luu tru o nhieu noi khac nhau "
    "phan tich du lieu giup doanh nghiep ra quyet dinh tot hon "
    "bao ve du lieu ca nhan la viec quan trong "
    "vi xu ly manh giup thiet bi chay nhanh hon "
    "moi vi xu ly deu co so nhan khac nhau "
    "vi xu ly the he moi tieu thu it dien nang hon "
    "ket noi mang on dinh giup lam viec tu xa de dang "
    "mot duong ket noi mang tot khong nen bi gian doan "
    "nguoi dung luon can ket noi mang nhanh de xem video "
    "mot thuat toan sap xep giup danh sach gon gang hon "
    "thuat toan tim kiem cung rat huu ich khi can thiet "
    "hieu ro thuat toan giup giai quyet van de nhanh hon "
    "mot ung dung di dong duoc hang trieu nguoi su dung "
    "ung dung tot phai de dung va de hieu "
    "nha phat trien luon cap nhat ung dung theo thoi gian"
)

vCH = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vNgan = tinh_vector_dem_tu(DOAN_NGAN, TU_VUNG, CUM_TU_GOC)
vDai = tinh_vector_dem_tu(DOAN_DAI, TU_VUNG, CUM_TU_GOC)

dot_ngan = tich_vo_huong(vCH, vNgan)
dot_dai = tich_vo_huong(vCH, vDai)

print(len(DOAN_NGAN.split()))
print(len(DOAN_DAI.split()))
print(vNgan)
print(vDai)
print(dot_ngan)
print(dot_dai)
print(dot_dai > dot_ngan)
```

```text title=readonly
19
261
[1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
[3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0]
2
6
True
```

`DOAN_NGAN` (`19` từ) dùng đúng `4` mục từ vựng công nghệ (`may_tinh`,
`phan_mem`, `lap_trinh`, `du_lieu`), mỗi mục đúng `1` lần — vector của nó
chỉ có `4` chỉ số khác `0`, mỗi chỉ số bằng `1`. `DOAN_DAI` (`261` từ, gấp
hơn `13` lần độ dài) lặp lại CẢ `8` mục từ vựng công nghệ, mỗi mục đúng
`3` lần — mọi chỉ số công nghệ trong vector của nó đều bằng `3`.
`CAU_HOI` chỉ chạm tới `2` mục từ vựng (`may_tinh`, `phan_mem`, mỗi mục
`1` lần). Với `DOAN_NGAN`: `tich_vo_huong = 1*1 (may_tinh) + 1*1 (phan_mem)
= 2`. Với `DOAN_DAI`: `tich_vo_huong = 1*3 (may_tinh) + 1*3 (phan_mem) =
6`. Kết quả: `6 > 2` — đoạn DÀI thắng, DÙ đoạn NGẮN mới là đoạn dùng ĐÚNG
những từ mà câu hỏi nhắc tới, không lặp thừa, không pha thêm nội dung khác.
Đo bằng số thật, không suy đoán: dot product thô thiên vị độ dài.
::::

::::predict{#doan_dot_uu_ai_doan_dai commitOnce}
Xét đúng ví dụ trên: `CAU_HOI` chỉ chạm `2` mục từ vựng (`may_tinh`,
`phan_mem`). `DOAN_NGAN` (`19` từ) dùng đúng `4` mục từ vựng công nghệ mỗi
mục `1` lần. `DOAN_DAI` (`261` từ) lặp lại `8` mục từ vựng công nghệ mỗi
mục `3` lần.

**Trước khi chạy thử**, bạn đoán: `tich_vo_huong(CAU_HOI, DOAN_DAI)` so với
`tich_vo_huong(CAU_HOI, DOAN_NGAN)` — cái nào CAO HƠN?

:::opt{correct}
`DOAN_DAI` cao hơn (`6 > 2`) — mỗi chỉ số công nghệ mà `CAU_HOI` chạm tới
đều bằng `3` ở `DOAN_DAI` (do lặp lại) nhưng chỉ bằng `1` ở `DOAN_NGAN`;
tích vô hướng cộng dồn theo SỐ LẦN LẶP LẠI, không quan tâm đoạn nào thực sự
khớp sát nội dung câu hỏi hơn
:::

:::opt
`DOAN_NGAN` cao hơn, vì nó ngắn gọn và dùng ĐÚNG những từ vựng liên quan
tới câu hỏi, không lặp thừa hay pha thêm nội dung khác không liên quan
::why
Gần đúng ở trực giác NỘI DUNG: `DOAN_NGAN` thật sự là đoạn khớp "sạch" hơn
với câu hỏi — không có từ thừa, không lặp lại vô ích.

Chỗ lệch: `tich_vo_huong` không đo được "khớp nội dung" theo nghĩa đó — nó
chỉ nhân từng cặp thành phần rồi cộng dồn. Một chỉ số bằng `3` (do lặp lại
nhiều lần) luôn góp phần LỚN HƠN một chỉ số bằng `1`, bất kể việc lặp lại
đó có mang thêm ý nghĩa gì hay không. Phép đo thô này không phân biệt được
"khớp sát, không lặp" với "khớp lỏng, lặp nhiều".
::
:::

:::opt
Bằng nhau, vì cả hai đoạn đều thuộc đúng chủ đề công nghệ và đều dùng từ
vựng nằm trong cùng nửa `8` mục công nghệ của `TU_VUNG`
::why
Gần đúng ở việc CẢ HAI đoạn đúng là cùng chủ đề công nghệ — quan sát đó
đúng.

Chỗ lệch: "cùng chủ đề" không có nghĩa là "cùng tích vô hướng". Tích vô
hướng phụ thuộc vào SỐ LẦN mỗi từ vựng xuất hiện ở TỪNG đoạn — `DOAN_NGAN`
có mỗi từ liên quan xuất hiện `1` lần, `DOAN_DAI` có mỗi từ liên quan xuất
hiện `3` lần; hai con số khác nhau này nhân với vector câu hỏi cho ra hai
kết quả khác nhau (`2` và `6`), không phải một giá trị chung.
::
:::
::::

::::code{#viet_tich_vo_huong_va_dai_thang_ngan}
Hoàn thiện `tich_vo_huong` (ôn lại từ q8.5a: nhân từng cặp thành phần, cộng
dồn) và `dai_thang_ngan` (so sánh xem đoạn dài có tích vô hướng cao hơn
đoạn ngắn hay không).

```python title=starter
TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]

CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(___ for a, b in zip(v1, v2))                # a * b


def dai_thang_ngan(dot_ngan, dot_dai):
    return ___                                              # dot_dai > dot_ngan


CAU_HOI = "may tinh chay phan mem nao on dinh nhat"
DOAN_NGAN = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh de xu ly du lieu"
DOAN_DAI = (
    "may tinh hien dai giup con nguoi lam viec nhanh hon truoc "
    "moi nguoi deu muon mua mot chiec may tinh that tot "
    "gia ban mot chiec may tinh ngay nay re hon nam truoc "
    "phan mem tot giup nguoi dung cam thay hai long "
    "ai cung thich mot phan mem de dung moi ngay "
    "cap nhat phan mem thuong xuyen giup tranh loi vat "
    "hoc lap trinh can kien nhan va luyen tap moi ngay "
    "mot du an lap trinh lon can nhieu nguoi cung lam "
    "nguoi moi bat dau lap trinh nen chon mot ngon ngu de "
    "du lieu duoc luu tru o nhieu noi khac nhau "
    "phan tich du lieu giup doanh nghiep ra quyet dinh tot hon "
    "bao ve du lieu ca nhan la viec quan trong "
    "vi xu ly manh giup thiet bi chay nhanh hon "
    "moi vi xu ly deu co so nhan khac nhau "
    "vi xu ly the he moi tieu thu it dien nang hon "
    "ket noi mang on dinh giup lam viec tu xa de dang "
    "mot duong ket noi mang tot khong nen bi gian doan "
    "nguoi dung luon can ket noi mang nhanh de xem video "
    "mot thuat toan sap xep giup danh sach gon gang hon "
    "thuat toan tim kiem cung rat huu ich khi can thiet "
    "hieu ro thuat toan giup giai quyet van de nhanh hon "
    "mot ung dung di dong duoc hang trieu nguoi su dung "
    "ung dung tot phai de dung va de hieu "
    "nha phat trien luon cap nhat ung dung theo thoi gian"
)

vCH = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vNgan = tinh_vector_dem_tu(DOAN_NGAN, TU_VUNG, CUM_TU_GOC)
vDai = tinh_vector_dem_tu(DOAN_DAI, TU_VUNG, CUM_TU_GOC)

dot_ngan = tich_vo_huong(vCH, vNgan)
dot_dai = tich_vo_huong(vCH, vDai)

print(dot_ngan)
print(dot_dai)
print(dai_thang_ngan(dot_ngan, dot_dai))
```

```python title=solution
TU_VUNG = [
    "may_tinh", "phan_mem", "lap_trinh", "du_lieu", "vi_xu_ly", "ket_noi_mang", "thuat_toan", "ung_dung",
    "mon_an", "cong_thuc", "gia_vi", "nha_bep", "dau_bep", "thuc_pham", "nau_an", "mon_trang_mieng",
]

CUM_TU_GOC = {
    "may_tinh": "may tinh", "phan_mem": "phan mem", "lap_trinh": "lap trinh", "du_lieu": "du lieu",
    "vi_xu_ly": "vi xu ly", "ket_noi_mang": "ket noi mang", "thuat_toan": "thuat toan", "ung_dung": "ung dung",
    "mon_an": "mon an", "cong_thuc": "cong thuc", "gia_vi": "gia vi", "nha_bep": "nha bep",
    "dau_bep": "dau bep", "thuc_pham": "thuc pham", "nau_an": "nau an", "mon_trang_mieng": "mon trang mieng",
}


def chuan_hoa_cum_tu(van_ban, cum_tu_goc):
    kq = van_ban
    for token, cum in cum_tu_goc.items():
        kq = kq.replace(cum, token)
    return kq


def tinh_vector_dem_tu(van_ban, tu_vung, cum_tu_goc):
    vb = chuan_hoa_cum_tu(van_ban.lower(), cum_tu_goc)
    cac_tu = vb.split()
    return [cac_tu.count(tu) for tu in tu_vung]


def tich_vo_huong(v1, v2):
    return sum(a * b for a, b in zip(v1, v2))


def dai_thang_ngan(dot_ngan, dot_dai):
    return dot_dai > dot_ngan


CAU_HOI = "may tinh chay phan mem nao on dinh nhat"
DOAN_NGAN = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh de xu ly du lieu"
DOAN_DAI = (
    "may tinh hien dai giup con nguoi lam viec nhanh hon truoc "
    "moi nguoi deu muon mua mot chiec may tinh that tot "
    "gia ban mot chiec may tinh ngay nay re hon nam truoc "
    "phan mem tot giup nguoi dung cam thay hai long "
    "ai cung thich mot phan mem de dung moi ngay "
    "cap nhat phan mem thuong xuyen giup tranh loi vat "
    "hoc lap trinh can kien nhan va luyen tap moi ngay "
    "mot du an lap trinh lon can nhieu nguoi cung lam "
    "nguoi moi bat dau lap trinh nen chon mot ngon ngu de "
    "du lieu duoc luu tru o nhieu noi khac nhau "
    "phan tich du lieu giup doanh nghiep ra quyet dinh tot hon "
    "bao ve du lieu ca nhan la viec quan trong "
    "vi xu ly manh giup thiet bi chay nhanh hon "
    "moi vi xu ly deu co so nhan khac nhau "
    "vi xu ly the he moi tieu thu it dien nang hon "
    "ket noi mang on dinh giup lam viec tu xa de dang "
    "mot duong ket noi mang tot khong nen bi gian doan "
    "nguoi dung luon can ket noi mang nhanh de xem video "
    "mot thuat toan sap xep giup danh sach gon gang hon "
    "thuat toan tim kiem cung rat huu ich khi can thiet "
    "hieu ro thuat toan giup giai quyet van de nhanh hon "
    "mot ung dung di dong duoc hang trieu nguoi su dung "
    "ung dung tot phai de dung va de hieu "
    "nha phat trien luon cap nhat ung dung theo thoi gian"
)

vCH = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vNgan = tinh_vector_dem_tu(DOAN_NGAN, TU_VUNG, CUM_TU_GOC)
vDai = tinh_vector_dem_tu(DOAN_DAI, TU_VUNG, CUM_TU_GOC)

dot_ngan = tich_vo_huong(vCH, vNgan)
dot_dai = tich_vo_huong(vCH, vDai)

print(dot_ngan)
print(dot_dai)
print(dai_thang_ngan(dot_ngan, dot_dai))
```

```python title=test
assert dot_ngan == 2, f"dot_ngan phai la 2 -- dang ra {dot_ngan}"
assert dot_dai == 6, f"dot_dai phai la 6 -- dang ra {dot_dai}"
assert dai_thang_ngan(dot_ngan, dot_dai) == True, "doan DAI (lap tu nhieu) dang co dot product CAO HON doan NGAN cung chu de -- day chinh la han che can do"

# kiem tra truc tiep ham tich_vo_huong tren vector nho, tu tinh tay duoc
assert tich_vo_huong([1, 2, 3], [4, 5, 6]) == 32, f"1*4+2*5+3*6=32 -- dang ra {tich_vo_huong([1, 2, 3], [4, 5, 6])}"
assert tich_vo_huong([0, 0, 0], [1, 2, 3]) == 0, "tich vo huong voi vector toan so 0 phai la 0"

# bien dai_thang_ngan: gia tri bang nhau hoac thap hon phai cho False
assert dai_thang_ngan(5, 5) == False, "hai gia tri BANG NHAU khong duoc coi la 'dai thang ngan'"
assert dai_thang_ngan(5, 3) == False, "dot_dai THAP hon dot_ngan thi khong duoc coi la 'dai thang ngan'"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `tich_vo_huong`) là biểu thức bên trong `sum(... for a, b in zip(v1, v2))` — với mỗi cặp thành phần tương ứng, tính TÍCH của chúng (ôn lại nguyên vẹn từ q8.5a). Chỗ hai (trong `dai_thang_ngan`) là một phép SO SÁNH — trả về `True` khi đối số `dot_dai` thật sự LỚN HƠN đối số `dot_ngan` (không phải lớn hơn-hoặc-bằng).
- kind: strategy
  body: 'Chỗ đầu: `a * b` — nhân hai thành phần tương ứng. Chỗ hai: `dot_dai > dot_ngan` — so sánh nghiêm ngặt bằng toán tử `>`.'
- kind: one-line
  body: 'Chỗ đầu là `a * b`, chỗ hai là `dot_dai > dot_ngan`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai NHAN (dung toan tu '*') hai thanh phan tuong ung a, b -- khong duoc cong hay chep san mot so; VA cho trong hai phai SO SANH THAT bang toan tu '>' (khong duoc luon tra ve True/False chep san)
  requireAst:
  - kind: uses-operator, target: "*", min: 1
  - kind: uses-operator, target: ">", min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # "*"=1: CHI mot lan duy nhat trong toan bo solution, dung o cho trong dau
  # ("a * b" ben trong generator expression cua sum(...)).
  # ">"=1: CHI mot lan duy nhat, dung o cho trong hai ("dot_dai > dot_ngan").
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "dot_dai > dot_ngan" vao cho
  # trong dau ("return sum(dot_dai > dot_ngan for a, b in zip(v1, v2))") VA
  # dien "a * b" vao cho trong hai ("return a * b" trong dai_thang_ngan) --
  # ket qua AST van la [1, 1], Y HET ban dung (toan tu '*' va '>' van moi
  # thu xuat hien dung 1 lan, chi doi VI TRI). Static KHONG bat duoc mutant
  # nay.
  # Mutant nay BI BAT boi tier 'run': goi tich_vo_huong(...) se dung bien
  # 'dot_dai'/'dot_ngan' -- nhung hai bien nay CHUA HE TON TAI ben trong ham
  # tich_vo_huong (chi la tham so cua ham dai_thang_ngan, o mot scope khac
  # hoan toan) -- da tu chay THAT mutant nay qua python3, xac nhan no nem
  # NameError: name 'dot_dai' is not defined ngay khi goi tich_vo_huong(vCH,
  # vNgan) lan dau tien -- bi chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^2\\n6\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`6 > 2` — đoạn dài lặp từ thắng đoạn ngắn khớp sát, đo được bằng số thật.
Đây chính là lỗ hổng bài `do-tuong-dong-hai-doan-van` đã cảnh báo. Bài sau
sửa nó bằng một phép chuẩn hoá: cosine similarity.
::::

::::reflect{#nghi-lai}
Tích vô hướng thô không "sai" — nó làm đúng việc nó được định nghĩa để làm:
nhân và cộng. Vấn đề nằm ở việc DÙNG nó để so sánh "mức độ liên quan" giữa
các đoạn văn có ĐỘ DÀI khác nhau: một đoạn dài, lặp từ vựng nhiều lần, luôn
có lợi thế cộng dồn so với một đoạn ngắn, dù đoạn ngắn mới thực sự khớp sát
nội dung câu hỏi hơn. Bài này đã đo lỗ hổng đó bằng một cặp cụ thể (`19` từ
so với `261` từ, `2` so với `6`) — không suy đoán, một hiện tượng có thật.
Cách sửa không phải là bỏ tích vô hướng đi, mà là CHUẨN HOÁ nó theo độ dài
của từng vector trước khi so sánh — đó là nội dung của bài sau: độ tương
đồng cô-sin (cosine similarity).
::::

::::checkpoint{mastery=0.85}
::::
