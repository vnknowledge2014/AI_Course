---
id: tri-tue-nhan-tao.tim-kiem-vector-va-do-tuong-dong.boss-tim-dung-k-doan-lien-quan
title: "BOSS — Tìm đúng k đoạn liên quan: ráp trọn quest, đóng q8.5b tại 5/5"
summary: "Mot tai lieu 9 cau tu nghi (5 cong nghe, 4 am thuc) duoc chia bang chia_theo_cau (q8.5a) thanh dung 9 doan. Voi mot cau hoi cong nghe cu the: tim k=3 lan can bang COSINE (vet can) cho DUNG chi so [0,1,2] -- ca 3/3 deu dung chu de cong nghe (100%). Doi chieu: dung DOT PRODUCT THO thay vi cosine cho chi so [4,0,1] -- CHI 2/3 dung chu de, vi doan chi so 4 (am thuc, dai 67 tu, lap 'ung dung'/'may tinh' de noi ve mot ung dung NAU AN) co dot product = 6, cao hon ca 3 doan cong nghe that (dot=2 moi doan) -- mot minh chung so hoc cu the cho ly do cosine can thiet. Dong quest q8.5b tai 5/5 bai."
locale: vi
track: tri-tue-nhan-tao
module: tim-kiem-vector-va-do-tuong-dong
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-tim-dung-k-doan-lien-quan]
requires: [ai.do-toc-do-va-lam-chuan-doi-chieu]
concepts: [ai.boss-tim-dung-k-doan-lien-quan]
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

::::byte{trigger=enter mood=happy pose=jump}
Bốn bài: ôn dot product và hạn chế độ dài, chuẩn hoá và cosine similarity,
tìm `k` lân cận vét cạn, đo tốc độ. Bài này ráp CẢ BỐN vào một quy trình
duy nhất, trên một tài liệu hoàn toàn MỚI — và đóng quest `q8.5b` tại
`5/5`.
::::

::::explain{#rap_toan_bo_quy_trinh}
Một quy trình "tìm đúng `k` đoạn liên quan" ĐÚNG CÁCH gồm ĐÚNG bốn bước đã
xây, chạy theo thứ tự:

> **(a) Chia thành đoạn** (`chia-van-ban-thanh-doan`, q8.5a) — tách tài
> liệu dài thành nhiều đoạn nhỏ bằng `chia_theo_cau`.
>
> **(b) Tính vector mỗi đoạn** (`vector-tu-che-dem-tu`, q8.5a) — biến câu
> hỏi VÀ mỗi đoạn thành một vector đếm từ, trên CÙNG một bộ từ vựng.
>
> **(c) Đo cosine similarity** (`chuan-hoa-va-cosine-similarity`, bài `2`)
> — chuẩn hoá theo độ dài, để một đoạn dài không "ăn gian" so với một đoạn
> ngắn khớp sát hơn.
>
> **(d) Tìm `k` lân cận gần nhất, vét cạn** (`tim-k-lan-can-gan-nhat-vet-
> can`, bài `3`) — so cosine similarity với TỪNG đoạn, không bỏ sót, rồi
> LẤY LẠI đúng nội dung văn bản của `k` đoạn đó qua chỉ số.

Bài này dùng một tài liệu `9` câu, TỰ NGHĨ, chưa từng xuất hiện ở bài nào
trước — `5` câu về chủ đề công nghệ, `4` câu về chủ đề ẩm thực. Một câu hỏi
cụ thể thuộc chủ đề công nghệ, tìm `k=3` đoạn liên quan nhất.

Đo bằng số THẬT: cả `3` đoạn trả về bằng cosine similarity đều ĐÚNG chủ đề
với câu hỏi — `100%` chính xác, vì đã dùng đúng chuẩn (không xấp xỉ, bài
`3`/`4`). Đối chiếu THÊM: nếu dùng dot product THÔ (bài `1`, chưa chuẩn
hoá) thay vì cosine, kết quả SAI — một đoạn ẩm thực bị một đoạn công nghệ
thật đẩy khỏi `top-3`, vì có một đoạn ẩm thực DÀI, lặp từ vựng công nghệ
nhiều lần (nói về một ứng dụng NẤU ĂN chạy trên máy tính), "đánh lừa" dot
product thô — bằng chứng số học cụ thể cho việc TẠI SAO cosine similarity
cần thiết.
::::

::::example{#boss_tim_dung_3_doan}
Tài liệu `9` câu tự nghĩ, chưa từng dùng ở bài nào trước — chạy trọn quy
trình bốn bước trên nó, rồi đối chiếu với dot product thô:

```python title=readonly
import math

def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def tim_k_lan_can_theo_dot_tho(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tich_vo_huong(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def dem_dung_chu_de(chi_so_ket_qua, chu_de_kho, chu_de_muc_tieu):
    return sum(1 for i in chi_so_ket_qua if chu_de_kho[i] == chu_de_muc_tieu)


TAI_LIEU_BOSS = (
    "may tinh hien dai co the chay duoc nhieu ung dung cung mot luc. "
    "phan mem tot giup moi ung dung chay muot ma hon truoc. "
    "moi nguoi deu can mot chiec may tinh va mot phan mem thuc su tot. "
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan. "
    "ung dung nau an tren may tinh dang ngay cang pho bien voi nhieu nguoi "
    "moi ung dung nau an thuong huong dan cong thuc va goi y gia vi phu hop "
    "mot chiec may tinh cu cung du de chay tot cac ung dung nau an don gian "
    "gia dinh nao cung nen co it nhat mot may tinh de tim cong thuc nau an hay. "
    "vi xu ly manh giup thiet bi chay nhanh hon rat nhieu. "
    "dau bep gioi luon lam viec can than trong nha bep sach se. "
    "lap trinh vien dung thuat toan toi uu de xu ly du lieu nhanh hon. "
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh."
)
CHU_DE = [
    "cong_nghe", "cong_nghe", "cong_nghe", "am_thuc", "am_thuc",
    "cong_nghe", "am_thuc", "cong_nghe", "am_thuc",
]
CAU_HOI = "may tinh chay ung dung va phan mem nao tot nhat"

doan = chia_theo_cau(TAI_LIEU_BOSS)
vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in doan]

chi_so_cosine = tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)
chi_so_dot_tho = tim_k_lan_can_theo_dot_tho(vector_cau_hoi, vector_kho, 3)

doan_lien_quan = [doan[i] for i in chi_so_cosine]

so_dung_cosine = dem_dung_chu_de(chi_so_cosine, CHU_DE, "cong_nghe")
so_dung_dot_tho = dem_dung_chu_de(chi_so_dot_tho, CHU_DE, "cong_nghe")

print(len(doan))
print(chi_so_cosine)
print(so_dung_cosine, "/", 3)
print(chi_so_dot_tho)
print(so_dung_dot_tho, "/", 3)
```

```text title=readonly
9
[0, 1, 2]
3 / 3
[4, 0, 1]
2 / 3
```

Tài liệu `9` câu chia đúng `9` đoạn. Ba đoạn đầu (chỉ số `0`, `1`, `2`) đều
là công nghệ, mỗi đoạn cho cosine similarity `≈ 0,8165` với `CAU_HOI`
(cùng dùng `2` trong `3` mục từ vựng mà câu hỏi chạm tới: `may_tinh`,
`phan_mem`, `ung_dung`). Đoạn chỉ số `4` — dài `67` từ, chủ đề ẩm thực (nói
về một ứng dụng nấu ăn) — LẶP LẠI `"may tinh"` `3` lần và `"ung dung"` `3`
lần, cho dot product thô `= 6`, CAO HƠN cả `3` đoạn công nghệ thật (dot
product `= 2` mỗi đoạn) — nhưng cosine similarity của nó chỉ `≈ 0,5547`,
THẤP HƠN cả `3` đoạn công nghệ (`0,8165`), vì độ dài vector của nó lớn hơn
nhiều (còn chứa các mục từ vựng ẩm thực khác như `mon_an`, `cong_thuc`,
`nau_an`).

Kết quả: dùng COSINE, `top-3 = [0, 1, 2]` — cả `3/3` đúng chủ đề công
nghệ, `100%` chính xác. Dùng DOT PRODUCT THÔ, `top-3 = [4, 0, 1]` — đoạn
`4` (ẩm thực, bị đánh lừa bởi lặp từ) lọt vào, đẩy đoạn `2` (công nghệ
thật) ra ngoài — chỉ `2/3` đúng chủ đề. Một minh chứng số học cụ thể: cùng
một kho, cùng một câu hỏi, khác nhau đúng MỘT thứ — có chuẩn hoá theo độ
dài hay không.
::::

::::predict{#doan_dot_tho_sai_chu_de commitOnce}
Xét đúng ví dụ trên: đoạn chỉ số `4` (ẩm thực, `67` từ) có dot product thô
`= 6` với `CAU_HOI`, cao hơn cả `3` đoạn công nghệ thật (dot product `= 2`
mỗi đoạn). Cosine similarity của đoạn `4` chỉ `≈ 0,5547`, thấp hơn cả `3`
đoạn công nghệ (`0,8165`).

**Trước khi chạy thử**, bạn đoán: dùng DOT PRODUCT THÔ để tìm `k=3` lân
cận, kết quả có ĐÚNG `100%` chủ đề công nghệ như cosine không?

:::opt{correct}
Không — dot product thô xếp đoạn `4` (ẩm thực) ở hạng nhất (dot `= 6`),
đẩy MỘT đoạn công nghệ thật ra khỏi `top-3`; chỉ `2/3` kết quả đúng chủ đề,
KHÁC với cosine (`3/3`)
:::

:::opt
Có — vì đoạn `4` tuy dài nhưng vẫn nhắc tới `"may tinh"` và `"ung dung"`
nhiều lần, nên về bản chất nó vẫn liên quan tới chủ đề công nghệ của câu
hỏi, dù được xếp vào nhãn ẩm thực
::why
Gần đúng ở việc đoạn `4` THẬT SỰ có nhắc tới các từ vựng công nghệ đó — số
lần lặp lại (`3` lần mỗi từ) là có thật, không bịa.

Chỗ lệch: việc một đoạn NHẮC TỚI từ vựng của một chủ đề không có nghĩa nó
THUỘC chủ đề đó — đoạn `4` là một đoạn về nấu ăn, chỉ tình cờ dùng đúng
những từ (`"may tinh"`, `"ung dung"`) mà từ vựng cố định của quest này gán
cho chủ đề công nghệ. Đây chính xác là điểm yếu của một "embedding" đếm từ
đơn giản: nó không phân biệt được ngữ cảnh, chỉ đếm sự xuất hiện của chuỗi
ký tự.
::
:::

:::opt
Có, nhưng chỉ vì đoạn `4` được đặt ở vị trí đầu tài liệu — nếu đổi thứ tự
các câu trong `TAI_LIEU_BOSS`, kết quả dot product thô sẽ khác đi
::why
Gần đúng ở việc THỨ TỰ đoạn trong danh sách CÓ ảnh hưởng tới việc phá vỡ
"hoà" khi hai đoạn có similarity bằng nhau tuyệt đối (`sorted` giữ ổn định
thứ tự gốc cho các giá trị bằng nhau).

Chỗ lệch: ở đây không có tình huống "hoà" nào — dot product của đoạn `4`
(`= 6`) THỰC SỰ khác biệt, cao hơn hẳn `3` đoạn công nghệ thật (`= 2` mỗi
đoạn), không phải bằng nhau rồi phân định bằng thứ tự xuất hiện. Đổi thứ
tự các câu trong tài liệu sẽ đổi CHỈ SỐ của từng đoạn, nhưng không đổi GIÁ
TRỊ dot product của chính đoạn `4` — nó vẫn sẽ đứng hạng nhất.
::
:::
::::

::::code{#viet_tim_theo_dot_tho_va_dem_dung_chu_de}
Hoàn thiện `tim_k_lan_can_theo_dot_tho` (giống hệt `tim_k_lan_can_vet_can`
của bài `3`, nhưng dùng `tich_vo_huong` thô thay vì `tuong_dong_cosine`) và
`dem_dung_chu_de` (đếm bao nhiêu kết quả trả về đúng chủ đề mục tiêu).

```python title=starter
import math

def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def tim_k_lan_can_theo_dot_tho(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [___ for v in danh_sach_vector_kho]        # tich_vo_huong(vector_cau_hoi, v)
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def dem_dung_chu_de(chi_so_ket_qua, chu_de_kho, chu_de_muc_tieu):
    return sum(1 for i in chi_so_ket_qua if ___)                # chu_de_kho[i] == chu_de_muc_tieu


TAI_LIEU_BOSS = (
    "may tinh hien dai co the chay duoc nhieu ung dung cung mot luc. "
    "phan mem tot giup moi ung dung chay muot ma hon truoc. "
    "moi nguoi deu can mot chiec may tinh va mot phan mem thuc su tot. "
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan. "
    "ung dung nau an tren may tinh dang ngay cang pho bien voi nhieu nguoi "
    "moi ung dung nau an thuong huong dan cong thuc va goi y gia vi phu hop "
    "mot chiec may tinh cu cung du de chay tot cac ung dung nau an don gian "
    "gia dinh nao cung nen co it nhat mot may tinh de tim cong thuc nau an hay. "
    "vi xu ly manh giup thiet bi chay nhanh hon rat nhieu. "
    "dau bep gioi luon lam viec can than trong nha bep sach se. "
    "lap trinh vien dung thuat toan toi uu de xu ly du lieu nhanh hon. "
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh."
)
CHU_DE = [
    "cong_nghe", "cong_nghe", "cong_nghe", "am_thuc", "am_thuc",
    "cong_nghe", "am_thuc", "cong_nghe", "am_thuc",
]
CAU_HOI = "may tinh chay ung dung va phan mem nao tot nhat"

doan = chia_theo_cau(TAI_LIEU_BOSS)
vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in doan]

chi_so_cosine = tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)
chi_so_dot_tho = tim_k_lan_can_theo_dot_tho(vector_cau_hoi, vector_kho, 3)

doan_lien_quan = [doan[i] for i in chi_so_cosine]

so_dung_cosine = dem_dung_chu_de(chi_so_cosine, CHU_DE, "cong_nghe")
so_dung_dot_tho = dem_dung_chu_de(chi_so_dot_tho, CHU_DE, "cong_nghe")

print(len(doan))
print(chi_so_cosine)
print(so_dung_cosine, "/", 3)
print(chi_so_dot_tho)
print(so_dung_dot_tho, "/", 3)
```

```python title=solution
import math

def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


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


def tim_k_lan_can_vet_can(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def tim_k_lan_can_theo_dot_tho(vector_cau_hoi, danh_sach_vector_kho, k):
    do_tuong_dong = [tich_vo_huong(vector_cau_hoi, v) for v in danh_sach_vector_kho]
    chi_so_sap_xep = sorted(range(len(danh_sach_vector_kho)), key=lambda i: do_tuong_dong[i], reverse=True)
    return chi_so_sap_xep[:k]


def dem_dung_chu_de(chi_so_ket_qua, chu_de_kho, chu_de_muc_tieu):
    return sum(1 for i in chi_so_ket_qua if chu_de_kho[i] == chu_de_muc_tieu)


TAI_LIEU_BOSS = (
    "may tinh hien dai co the chay duoc nhieu ung dung cung mot luc. "
    "phan mem tot giup moi ung dung chay muot ma hon truoc. "
    "moi nguoi deu can mot chiec may tinh va mot phan mem thuc su tot. "
    "mon an ngon phu thuoc vao gia vi va cong thuc nau chuan. "
    "ung dung nau an tren may tinh dang ngay cang pho bien voi nhieu nguoi "
    "moi ung dung nau an thuong huong dan cong thuc va goi y gia vi phu hop "
    "mot chiec may tinh cu cung du de chay tot cac ung dung nau an don gian "
    "gia dinh nao cung nen co it nhat mot may tinh de tim cong thuc nau an hay. "
    "vi xu ly manh giup thiet bi chay nhanh hon rat nhieu. "
    "dau bep gioi luon lam viec can than trong nha bep sach se. "
    "lap trinh vien dung thuat toan toi uu de xu ly du lieu nhanh hon. "
    "mon trang mieng ngot ngao thuong duoc dung sau bua an chinh."
)
CHU_DE = [
    "cong_nghe", "cong_nghe", "cong_nghe", "am_thuc", "am_thuc",
    "cong_nghe", "am_thuc", "cong_nghe", "am_thuc",
]
CAU_HOI = "may tinh chay ung dung va phan mem nao tot nhat"

doan = chia_theo_cau(TAI_LIEU_BOSS)
vector_cau_hoi = tinh_vector_dem_tu(CAU_HOI, TU_VUNG, CUM_TU_GOC)
vector_kho = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in doan]

chi_so_cosine = tim_k_lan_can_vet_can(vector_cau_hoi, vector_kho, 3)
chi_so_dot_tho = tim_k_lan_can_theo_dot_tho(vector_cau_hoi, vector_kho, 3)

doan_lien_quan = [doan[i] for i in chi_so_cosine]

so_dung_cosine = dem_dung_chu_de(chi_so_cosine, CHU_DE, "cong_nghe")
so_dung_dot_tho = dem_dung_chu_de(chi_so_dot_tho, CHU_DE, "cong_nghe")

print(len(doan))
print(chi_so_cosine)
print(so_dung_cosine, "/", 3)
print(chi_so_dot_tho)
print(so_dung_dot_tho, "/", 3)
```

```python title=test
assert len(doan) == 9, f"tai lieu BOSS phai chia thanh 9 doan -- dang ra {len(doan)}"
assert chi_so_cosine == [0, 1, 2], f"k=3 lan can theo COSINE phai la [0, 1, 2] -- dang ra {chi_so_cosine}"
assert so_dung_cosine == 3, f"ca 3 ket qua COSINE phai dung chu de cong nghe -- dang ra {so_dung_cosine}"
assert chi_so_dot_tho == [4, 0, 1], f"k=3 lan can theo DOT THO phai la [4, 0, 1] -- dang ra {chi_so_dot_tho}"
assert so_dung_dot_tho == 2, f"DOT THO phai SAI it nhat 1/3 (doan chi so 4 la am_thuc lot vao) -- dang ra {so_dung_dot_tho}"
assert doan_lien_quan[0] == doan[0], "phai lay lai DUNG noi dung van ban qua chi so, khong duoc bien doi gi ca"
assert all(CHU_DE[i] == "cong_nghe" for i in chi_so_cosine), "MOI ket qua tra ve boi cosine deu phai dung chu de cong nghe"
assert any(CHU_DE[i] != "cong_nghe" for i in chi_so_dot_tho), "PHAI co it nhat mot ket qua SAI chu de khi dung dot product tho"
assert CHU_DE[chi_so_dot_tho[0]] == "am_thuc", f"doan xep hang 1 theo dot tho phai la doan chi so 4 (am_thuc, lap tu nhieu) -- dang ra chi so {chi_so_dot_tho[0]}"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc
kho_nho = [[1, 1], [5, 5], [0, 0]]
assert tim_k_lan_can_theo_dot_tho([1, 1], kho_nho, 1) == [1], "dot tho phai chon vector [5,5] (dot=10) truoc [1,1] (dot=2)"
assert tim_k_lan_can_vet_can([1, 1], kho_nho, 1) == [0], "cosine phai coi [1,1] va [5,5] la NHU NHAU (cung huong, cosine=1.0) va chon chi so xuat hien TRUOC (0)"
assert dem_dung_chu_de([0, 1], ["a", "b", "a"], "a") == 1, f"dem_dung_chu_de dem sai -- dang ra {dem_dung_chu_de([0, 1], ['a', 'b', 'a'], 'a')}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `tim_k_lan_can_theo_dot_tho`) là biểu thức bên trong list comprehension — GIỐNG HỆT `tim_k_lan_can_vet_can` của bài `3`, nhưng gọi `tich_vo_huong` (dot product thô) thay vì `tuong_dong_cosine`. Chỗ hai (trong `dem_dung_chu_de`) là điều kiện của generator expression bên trong `sum(...)` — so sánh xem chủ đề của đoạn thứ `i` có BẰNG chủ đề mục tiêu hay không.
- kind: strategy
  body: 'Chỗ đầu: `tich_vo_huong(vector_cau_hoi, v)` — dùng dot product thô, không chuẩn hoá. Chỗ hai: `chu_de_kho[i] == chu_de_muc_tieu` — so sánh bằng `==`, đếm số lần `True`.'
- kind: one-line
  body: 'Chỗ đầu là `tich_vo_huong(vector_cau_hoi, v)`, chỗ hai là `chu_de_kho[i] == chu_de_muc_tieu`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT tich_vo_huong(vector_cau_hoi, v) (dot product THO, khong phai tuong_dong_cosine) cho TUNG vector trong kho; VA cho trong hai phai SO SANH BANG (==) chu de cua doan thu i voi chu de muc tieu
  requireAst:
  - kind: uses-call, target: tich_vo_huong, min: 2
  - kind: uses-operator, target: "==", min: 3
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [2, 3] cho hai
  # luat theo dung thu tu khai bao o tren.
  # tich_vo_huong=2 (TONG THAT, da xac nhan bang cong cu, khong doan tay):
  # MOT lan CO SAN trong boilerplate cua tuong_dong_cosine ("return
  # tich_vo_huong(v1, v2) / (d1 * d2)", da day tu bai 2), MOT lan CHINH la
  # cho trong dau. Neu chi dat min=1 (ngay tho), mot mutant dien
  # "do_tuong_dong = [tuong_dong_cosine(vector_cau_hoi, v) for v in ...]"
  # (goi NHAM ham cosine thay vi dot tho, xoa mat diem khac biet ma bai nay
  # muon day) van qua duoc vi con lai 1 lan goi tich_vo_huong trong
  # boilerplate cua tuong_dong_cosine -- GOTCHA "boilerplate-threshold-
  # masking"; da tu kiem chung: mutant nay lam chi_so_dot_tho tro thanh
  # [0, 1, 2] (giong het chi_so_cosine, mat het diem doi chieu), bi bat CA
  # boi static (voi min=2, dung) LAN boi assertion "chi_so_dot_tho == [4, 0,
  # 1]" va "so_dung_dot_tho == 2" (neu chi dung static min=1 thi van bi bat
  # boi assertion, nhung static se sai lam qua som).
  # "=="=3 (TONG THAT): HAI lan CO SAN trong boilerplate cua tuong_dong_cosine
  # ("if d1 == 0 or d2 == 0:" -- day la HAI nut Compare rieng biet, moi nut
  # mang mot toan tu Eq, nen dem la 2), MOT lan CHINH la cho trong hai. Neu
  # chi dat min=1, mot mutant dien "chu_de_kho[i] != chu_de_muc_tieu" (dao
  # nguoc dieu kien) se lam mat toan tu '==' o cho trong hai nhung VAN con 2
  # lan '==' tu boilerplate -- nhung dung min=3 (TONG THAT) thi mutant nay
  # tut xuong 2, duoi nguong, bi chan boi static; da tu kiem chung dong thoi
  # no cung bi bat boi assertion "so_dung_cosine == 3" (dao nguoc dieu kien
  # se dem NGUOC, cho ra 0 thay vi 3).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua ham
  # _dem de xac nhan, khong doan tay): dien "chu_de_kho[i] ==
  # chu_de_muc_tieu" vao cho trong dau ("do_tuong_dong = [chu_de_kho[i] ==
  # chu_de_muc_tieu for v in danh_sach_vector_kho]" trong
  # tim_k_lan_can_theo_dot_tho) VA dien "tich_vo_huong(vector_cau_hoi, v)"
  # vao cho trong hai ("return sum(1 for i in chi_so_ket_qua if
  # tich_vo_huong(vector_cau_hoi, v))" trong dem_dung_chu_de) -- ket qua AST
  # van la [2, 3], Y HET ban dung (tong so lan goi tich_vo_huong va tong so
  # lan '==' khong doi, chi doi VI TRI). Static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': cho trong dau dung 'chu_de_kho' va
  # 'chu_de_muc_tieu' -- nhung hai ten nay CHUA HE TON TAI trong scope cua
  # tim_k_lan_can_theo_dot_tho (ham nay chi nhan tham so vector_cau_hoi,
  # danh_sach_vector_kho, k) -- da tu chay THAT mutant nay qua python3, xac
  # nhan no nem NameError: name 'chu_de_kho' is not defined ngay khi ham
  # duoc goi -- bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat them GOTCHA #6 cho luat uses-call (tich_vo_huong): trong
  # pham vi list comprehension o cho trong dau, bien 'v' la bien vong lap
  # DUY NHAT trong scope, khong co danh sach nao khac cung do dai voi
  # danh_sach_vector_kho de mot mutant "dung sai bien" co the tinh co khop
  # -- rui ro nay da duoc ra soat va KHONG ap dung o day.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^9\\n\\[0, 1, 2\\]\\n3 / 3\\n\\[4, 0, 1\\]\\n2 / 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3/3` đúng chủ đề bằng cosine, chỉ `2/3` bằng dot product thô — một minh
chứng số học cụ thể, không suy đoán, cho việc tại sao chuẩn hoá theo độ
dài là cần thiết. Quest `tim-kiem-vector-va-do-tuong-dong` (q8.5b) đóng tại
`5/5` bài.
::::

::::reflect{#nghi-lai}
Quest này mở ra với một câu hỏi cụ thể: tích vô hướng thô (q8.5a) có đủ để
tìm đoạn văn liên quan không? Năm bài đã trả lời từng mảnh một:

> **`on-lai-dot-product-va-han-che-do-dai`** (bài `1`) — đo bằng số một lỗ
> hổng đã được cảnh báo nhưng chưa đo: đoạn dài lặp từ (`261` từ, dot
> product `= 6`) thắng đoạn ngắn khớp sát (`19` từ, dot product `= 2`).
>
> **`chuan-hoa-va-cosine-similarity`** (bài `2`) — sửa đúng chỗ lệch: chia
> cho tích độ dài hai vector đảo ngược kết quả (`0,7071 > 0,5`), và xử lý
> đúng vector toàn số `0`.
>
> **`tim-k-lan-can-gan-nhat-vet-can`** (bài `3`) — vét cạn trên một kho
> `8` đoạn, tìm đúng `k=3` chỉ số công nghệ (`[1, 3, 5]`), không lẫn đoạn
> ẩm thực nào.
>
> **`do-toc-do-va-lam-chuan-doi-chieu`** (bài `4`) — đo bằng số tất định:
> vét cạn luôn cần đúng `N` phép so sánh, bất kể `k`; đây là chuẩn đối
> chiếu độ chính xác cho quest sau.
>
> **`boss-tim-dung-k-doan-lien-quan`** (bài `5`, quest này) — ráp cả bốn
> bước, đo `3/3` đúng chủ đề bằng cosine, chỉ `2/3` bằng dot product thô.

Một điều CỐ Ý xuyên suốt cả `5` bài: mọi phép đo "gần nhau" đều là một hàm
Python thuần, tất định — không mô hình embedding thật nào đứng sau, không
mạng nơ-ron, không gọi API. Track `T8.5` "RAG từ số `0`" tiếp tục từ đây
với `hnsw-tu-cai` (q8.5c): vét cạn (bài `3`/`4`) luôn đúng nhưng tốn `N`
phép so sánh cho mỗi câu hỏi — quest sau xây một chỉ mục HNSW tự cài, mục
tiêu là GIẢM số phép so sánh xuống dưới `N` mà vẫn giữ độ chính xác cao,
đối chiếu với đúng kết quả vét cạn đã đo ở quest này làm "đáp án chuẩn".
::::

::::checkpoint{mastery=0.9}
::::
