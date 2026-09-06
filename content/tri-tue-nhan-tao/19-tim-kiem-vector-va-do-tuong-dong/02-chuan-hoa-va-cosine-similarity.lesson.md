---
id: tri-tue-nhan-tao.tim-kiem-vector-va-do-tuong-dong.chuan-hoa-va-cosine-similarity
title: "Chuẩn hoá và cosine similarity: sửa đúng chỗ lệch"
summary: "Do dai vector (do_dai_vector) = can bac hai tong binh phuong cac thanh phan. Cosine similarity = tich_vo_huong(v1,v2) / (do_dai(v1) * do_dai(v2)). Tren DUNG cap (CAU_HOI, DOAN_NGAN)/(CAU_HOI, DOAN_DAI) cua bai truoc: dot product tho cho DOAN_DAI thang (6 > 2), nhung cosine similarity DAO NGUOC ket qua -- cos(CAU_HOI, DOAN_NGAN) = 0,7071 CAO HON cos(CAU_HOI, DOAN_DAI) = 0,5. Vector toan so 0 (khong khop tu vung nao) duoc xu ly dung: cosine similarity tra ve 0,0 thay vi loi chia cho 0."
locale: vi
track: tri-tue-nhan-tao
module: tim-kiem-vector-va-do-tuong-dong
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.chuan-hoa-va-cosine-similarity]
requires: [ai.on-lai-dot-product-va-han-che-do-dai]
concepts: [ai.chuan-hoa-va-cosine-similarity]
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
Bài trước đo được lỗ hổng: đoạn dài lặp từ thắng đoạn ngắn khớp sát, dù đoạn
ngắn mới thực sự liên quan hơn. Bài này sửa đúng chỗ đó — bằng một phép
chuẩn hoá.
::::

::::explain{#chuan_hoa_va_cosine}
Vấn đề của tích vô hướng thô: nó cộng dồn theo SỐ LẦN LẶP LẠI, không quan
tâm tới ĐỘ LỚN tổng thể của từng vector. Cách sửa: chia mỗi vector cho ĐỘ
DÀI (magnitude, norm) của chính nó, TRƯỚC khi so sánh.

**Độ dài của một vector** (`do_dai_vector`) là căn bậc hai của tổng bình
phương các thành phần: `sqrt(sum(x**2 for x in v))`. Với vector `[3, 4]`,
độ dài là `sqrt(9 + 16) = sqrt(25) = 5`.

**Chuẩn hoá** (normalize) một vector nghĩa là chia MỖI thành phần của nó
cho độ dài của chính nó — kết quả là một **vector đơn vị** (unit vector),
có độ dài đúng bằng `1`. Chuẩn hoá không đổi HƯỚNG của vector, chỉ đổi ĐỘ
LỚN — một đoạn văn dài lặp từ nhiều và một đoạn văn ngắn cùng tỉ lệ từ vựng
sẽ chuẩn hoá về CÙNG một vector đơn vị.

**Cosine similarity** giữa hai vector là tích vô hướng của HAI VECTOR ĐÃ
CHUẨN HOÁ đó — tương đương với công thức:

```
tuong_dong_cosine(v1, v2) = tich_vo_huong(v1, v2) / (do_dai_vector(v1) * do_dai_vector(v2))
```

Với vector đếm từ (luôn KHÔNG ÂM, như ở đây), cosine similarity luôn nằm
trong khoảng `[0, 1]`: `0` nghĩa là không chung từ vựng nào (hai đoạn hoàn
toàn khác chủ đề), `1` nghĩa là hai vector CÙNG HƯỚNG tuyệt đối (cùng tỉ lệ
từ vựng, dù độ dài đoạn văn khác nhau bao nhiêu).

Có một trường hợp biên PHẢI xử lý đúng: nếu một vector toàn số `0` (đoạn
văn rỗng, hoặc không khớp mục từ vựng nào), độ dài của nó bằng `0` — chia
cho `0` sẽ ném lỗi `ZeroDivisionError`. Quy ước đúng: khi độ dài của BẤT KỲ
vector nào bằng `0`, trả về cosine similarity `= 0.0` (không "liên quan"
theo hướng nào cả), không được để chương trình sập.
::::

::::example{#sua_han_che_bang_cosine}
Dùng lại ĐÚNG cặp `(CAU_HOI, DOAN_NGAN)` và `(CAU_HOI, DOAN_DAI)` của bài
trước — nơi tích vô hướng thô cho `DOAN_DAI` thắng (`6 > 2`):

```python title=readonly
import math

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


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


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

cos_ngan = tuong_dong_cosine(vCH, vNgan)
cos_dai = tuong_dong_cosine(vCH, vDai)

print(round(cos_ngan, 4))
print(round(cos_dai, 4))
print(cos_ngan > cos_dai)
print(tuong_dong_cosine([0, 0, 0], [1, 2, 3]))
```

```text title=readonly
0.7071
0.5
True
0.0
```

`vNgan` có độ dài `sqrt(1+1+1+1) = 2`, `vDai` có độ dài `sqrt(8 * 3**2) =
sqrt(72) ≈ 8,4853` — gấp hơn `4` lần. Cosine similarity CHIA đi chính sự
chênh lệch độ dài đó: `cos(CAU_HOI, DOAN_NGAN) = 2 / (sqrt(2) * 2) ≈
0,7071`, còn `cos(CAU_HOI, DOAN_DAI) = 6 / (sqrt(2) * sqrt(72)) = 6/12 =
0,5`. Kết quả ĐẢO NGƯỢC hoàn toàn so với bài trước: `0,7071 > 0,5` —
`DOAN_NGAN` (khớp sát, không lặp thừa) giờ đứng CAO HƠN `DOAN_DAI` (lặp từ
nhiều), đúng với trực giác về nội dung. Dòng cuối xác nhận trường hợp biên:
`tuong_dong_cosine([0,0,0], [1,2,3])` trả về `0.0` — không ném lỗi chia cho
`0`, dù vector đầu tiên toàn số `0`.
::::

::::predict{#doan_cosine_dao_nguoc commitOnce}
Xét đúng ví dụ trên: dot product thô cho `dot_dai = 6 > dot_ngan = 2` (bài
trước). Độ dài `vDai` gấp hơn `4` lần độ dài `vNgan`.

**Trước khi chạy thử**, bạn đoán: giữa `cos(CAU_HOI, DOAN_NGAN)` và
`cos(CAU_HOI, DOAN_DAI)`, cái nào CAO HƠN?

:::opt{correct}
`cos(CAU_HOI, DOAN_NGAN)` cao hơn — cosine similarity chia tích vô hướng
cho TÍCH độ dài hai vector; `vDai` có độ dài lớn hơn `vNgan` rất nhiều
(gấp hơn `4` lần), nên dù tử số (`6`) của nó lớn hơn, mẫu số cũng lớn hơn
NHIỀU HƠN tương ứng, kéo kết quả cuối cùng xuống thấp hơn `vNgan`
:::

:::opt
`cos(CAU_HOI, DOAN_DAI)` vẫn cao hơn, vì `DOAN_DAI` vẫn dùng nhiều từ vựng
liên quan tới câu hỏi hơn hẳn `DOAN_NGAN`, và chuẩn hoá không thể xoá bỏ
hoàn toàn lợi thế đó
::why
Gần đúng ở việc `DOAN_DAI` THẬT SỰ có tích vô hướng thô cao hơn (`6 > 2`,
bài trước) — quan sát về dot product thô đó đúng.

Chỗ lệch: cosine similarity không chỉ nhìn tử số — nó chia cho tích ĐỘ DÀI
của cả hai vector. Vì `DOAN_DAI` dài hơn `DOAN_NGAN` rất nhiều lần, độ dài
vector của nó cũng lớn hơn rất nhiều, và phép chia này áp đảo lợi thế của
tử số, kéo kết quả cuối cùng xuống DƯỚI `DOAN_NGAN`.
::
:::

:::opt
Bằng nhau, vì chuẩn hoá luôn đưa mọi vector về cùng một mức so sánh công
bằng, xoá sạch mọi khác biệt về độ dài đoạn văn
::why
Gần đúng ở Ý TƯỞNG chuẩn hoá "công bằng hoá" theo độ dài — đúng ở mức khái
niệm chung.

Chỗ lệch: chuẩn hoá xoá bỏ ảnh hưởng của ĐỘ LỚN, nhưng KHÔNG xoá bỏ ảnh
hưởng của HƯỚNG — `vNgan` và `vDai` không cùng hướng (tỉ lệ giữa các chỉ số
khác nhau vì `vDai` còn có thêm nhiều chỉ số công nghệ khác `0` mà `vCH`
không chạm tới, kéo hướng của nó lệch đi so với `vCH`). Hai vector khác
hướng thì cosine similarity khác nhau, dù đã chuẩn hoá.
::
:::
::::

::::code{#viet_do_dai_va_cosine}
Hoàn thiện `do_dai_vector` (tổng bình phương từng thành phần) và
`tuong_dong_cosine` (chia tích vô hướng cho tích hai độ dài).

```python title=starter
import math

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


def do_dai_vector(v):
    return math.sqrt(sum(___ for x in v))                  # x ** 2


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / ___                       # (d1 * d2)


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

cos_ngan = tuong_dong_cosine(vCH, vNgan)
cos_dai = tuong_dong_cosine(vCH, vDai)

print(round(cos_ngan, 4))
print(round(cos_dai, 4))
print(cos_ngan > cos_dai)
```

```python title=solution
import math

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


def do_dai_vector(v):
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


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

cos_ngan = tuong_dong_cosine(vCH, vNgan)
cos_dai = tuong_dong_cosine(vCH, vDai)

print(round(cos_ngan, 4))
print(round(cos_dai, 4))
print(cos_ngan > cos_dai)
```

```python title=test
assert round(cos_ngan, 4) == 0.7071, f"cos_ngan phai xap xi 0.7071 -- dang ra {cos_ngan}"
assert cos_dai == 0.5, f"cos_dai phai la 0.5 -- dang ra {cos_dai}"
assert cos_ngan > cos_dai, "cos_ngan (doan NGAN cung chu de) phai CAO HON cos_dai (doan DAI lap tu) -- nguoc voi dot product tho o bai truoc"

# kiem tra truc tiep do_dai_vector tren vector 3-4-5 kinh dien
assert do_dai_vector([3, 4]) == 5.0, f"do dai cua [3,4] phai la 5.0 -- dang ra {do_dai_vector([3, 4])}"

# kiem tra truc tiep tuong_dong_cosine tren vector nho
assert tuong_dong_cosine([3, 4], [3, 4]) == 1.0, "hai vector CUNG huong (giong het nhau) phai cho cosine = 1.0"
assert tuong_dong_cosine([1, 0], [0, 1]) == 0.0, "hai vector VUONG GOC (khong chung chi so khac 0) phai cho cosine = 0.0"

# bien QUAN TRONG: vector toan so 0 -- KHONG duoc loi chia cho 0
assert tuong_dong_cosine([0, 0, 0], [1, 2, 3]) == 0.0, "vector toan so 0 phai cho cosine = 0.0, khong duoc nem loi ZeroDivisionError"
assert tuong_dong_cosine([1, 2, 3], [0, 0, 0]) == 0.0, "vector toan so 0 o vi tri thu hai cung phai cho 0.0"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `do_dai_vector`) là biểu thức bên trong `sum(... for x in v)` — với mỗi thành phần, tính BÌNH PHƯƠNG của nó (dùng toán tử `**`). Chỗ hai (trong `tuong_dong_cosine`) là MẪU SỐ của phép chia — tích của hai độ dài đã tính (`d1`, `d2`), đặt trong ngoặc.
- kind: strategy
  body: 'Chỗ đầu: `x ** 2` — bình phương từng thành phần trước khi cộng dồn và lấy căn bậc hai. Chỗ hai: `(d1 * d2)` — tích của hai độ dài, dùng làm mẫu số chia cho tích vô hướng.'
- kind: one-line
  body: 'Chỗ đầu là `x ** 2`, chỗ hai là `(d1 * d2)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai BINH PHUONG (dung toan tu '**') tung thanh phan x -- khong duoc cong hay lay gia tri tuyet doi; VA cho trong hai phai NHAN (dung toan tu '*') hai do dai d1, d2 lam mau so
  requireAst:
  - kind: uses-operator, target: "**", min: 1
  - kind: uses-operator, target: "*", min: 2
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 2] cho hai
  # luat theo dung thu tu khai bao o tren.
  # "**"=1: CHI mot lan duy nhat trong toan bo solution, dung o cho trong dau
  # ("x ** 2" ben trong generator expression cua sum(...) trong do_dai_vector).
  # "*"=2 (TONG THAT, da xac nhan bang cong cu, khong doan tay): MOT lan CO
  # SAN trong boilerplate cua tich_vo_huong ("a * b", da day o bai truoc),
  # MOT lan CHINH la cho trong hai ("(d1 * d2)"). Neu chi dat min=1 (ngay
  # tho), mot mutant dien "return tich_vo_huong(v1, v2) / d1" (bo qua d2
  # hoan toan, SAI logic) van qua duoc vi con lai 1 lan '*' trong boilerplate
  # tich_vo_huong -- GOTCHA "boilerplate-threshold-masking"; da tu kiem
  # chung: mutant nay cho cos_dai tro thanh mot gia tri KHAC 0.5 (vi chia
  # cho d1 thay vi d1*d2), bi bat CA boi static (voi min=2, dung) LAN boi
  # assertion "cos_dai == 0.5" (neu chi dung static min=1 thi assertion van
  # bat duoc, nhung static se sai lam qua som).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "(d1 * d2)" vao cho trong dau
  # ("math.sqrt(sum((d1 * d2) for x in v))" trong do_dai_vector) VA dien
  # "x ** 2" vao cho trong hai ("return tich_vo_huong(v1, v2) / (x ** 2)"
  # trong tuong_dong_cosine) -- ket qua AST van la [1, 2], Y HET ban dung
  # (toan tu '**' va '*' van xuat hien dung so lan cu, chi doi VI TRI: '**'
  # gio nam trong tuong_dong_cosine thay vi do_dai_vector, mot trong hai '*'
  # gio nam trong do_dai_vector thay vi tuong_dong_cosine). Static KHONG bat
  # duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong do_dai_vector, bien 'd1' va
  # 'd2' CHUA HE TON TAI (do_dai_vector chi nhan tham so 'v', khong nhan d1/
  # d2) -- da tu chay THAT mutant nay qua python3, xac nhan no nem NameError:
  # name 'd1' is not defined ngay khi do_dai_vector(v1) duoc goi lan dau
  # (ben trong tuong_dong_cosine) -- bi chan boi tier 'run', doc lap voi
  # static.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0\\.7071\\n0\\.5\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0,7071 > 0,5` — chuẩn hoá đảo ngược đúng thứ tự, và xử lý đúng vector toàn
số `0`. Giờ ta có một phép đo "gần nhau" công bằng — bài sau dùng nó để tìm
`k` lân cận gần nhất trong một kho tài liệu.
::::

::::reflect{#nghi-lai}
Cosine similarity không phải một công thức bí ẩn — nó là tích vô hướng của
bài trước, CHIA cho đúng thứ đã gây ra lỗ hổng: độ lớn của từng vector.
Phép chia đó không đổi HƯỚNG mà một vector đang chỉ tới — nó chỉ xoá đi ảnh
hưởng của ĐỘ DÀI đoạn văn. Kết quả: hai đoạn cùng tỉ lệ từ vựng, dù một
đoạn dài gấp `10` lần đoạn kia, vẫn cho cosine similarity giống hệt nhau.
Bài này cũng đã xử lý đúng một trường hợp biên dễ bị bỏ sót: vector toàn số
`0` (đoạn rỗng, hoặc không khớp từ vựng nào) có độ dài `0` — thay vì để
chương trình sập vì chia cho `0`, quy ước đúng là trả về `0.0`. Từ bài sau,
`tuong_dong_cosine` sẽ là phép đo "gần nhau" DUY NHẤT được dùng để tìm kiếm
trong một kho tài liệu nhiều đoạn — không còn tích vô hướng thô nào nữa,
trừ khi đối chiếu có chủ đích với cosine để thấy sự khác biệt.
::::

::::checkpoint{mastery=0.85}
::::
