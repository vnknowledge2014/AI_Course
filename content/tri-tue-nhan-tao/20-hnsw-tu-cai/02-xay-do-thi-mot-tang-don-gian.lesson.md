---
id: tri-tue-nhan-tao.hnsw-tu-cai.xay-do-thi-mot-tang-don-gian
title: "Xây đồ thị một tầng đơn giản: nối mỗi điểm mới tới M láng giềng gần nhất"
summary: "xay_do_thi_mot_tang(vectors, M) chen tung diem MOT theo thu tu: diem thu i noi toi M=3 lang gieng GAN NHAT trong so cac diem DA CHEN TRUOC do (dung cosine similarity q8.5b), noi CA HAI CHIEU (canh vo huong), va CAT BOT neu mot dinh vuot qua M lang gieng (chi giu M lang gieng GAN NHAT, bo canh xa). Tren kho 20 diem (10 tech + 10 am thuc): MOI dinh co DUNG 3 lang gieng, tong 30 canh -- khong dinh nao vuot qua M."
locale: vi
track: tri-tue-nhan-tao
module: hnsw-tu-cai
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.xay-do-thi-mot-tang-don-gian]
requires: [ai.y-tuong-do-thi-dieu-huong-nho]
concepts: [ai.xay-do-thi-mot-tang-don-gian]
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
Bài trước DÙNG một đồ thị đã dựng sẵn — chỉ cho thấy tìm tham lam trên nó
nhanh ra sao. Bài này tự XÂY đồ thị đó, từ số `0`.
::::

::::explain{#xay_do_thi_mot_tang}
Ý tưởng xây đồ thị điều hướng nhỏ MỘT TẦNG, đơn giản nhất có thể: chèn từng
điểm MỘT, theo thứ tự; mỗi điểm MỚI nối tới `M` láng giềng GẦN NHẤT trong số
những điểm ĐÃ CHÈN TRƯỚC đó (đo bằng `tuong_dong_cosine`, q8.5b).

Ba điều quan trọng:

> **(a) Cạnh VÔ HƯỚNG.** Khi điểm `i` nối tới láng giềng `j`, cạnh đó phải
> nối CẢ HAI CHIỀU: `j` cũng phải "biết" `i` là láng giềng của mình. Thiếu
> chiều ngược lại, tìm tham lam xuất phát từ `j` sẽ không bao giờ "thấy"
> được `i` qua cạnh này.
>
> **(b) Giới hạn `M` láng giềng MỖI ĐỈNH.** Một đỉnh `j` có thể được nhiều
> điểm SAU nó chọn làm láng giềng — nếu không giới hạn, `j` sẽ tích luỹ ngày
> càng nhiều cạnh, làm đồ thị mất đi tính "thưa" (mỗi bước tìm tham lam phải
> xét ngày càng nhiều láng giềng, chậm dần). Cách đơn giản để giữ đồ thị
> thưa: mỗi khi `j` vượt quá `M` láng giềng, CẮT BỚT — chỉ giữ lại `M` láng
> giềng GẦN `j` NHẤT, bỏ các cạnh xa.
>
> **(c) Chỉ so với điểm ĐÃ CHÈN.** Điểm thứ `i` chỉ có thể nối tới các điểm
> có chỉ số nhỏ hơn `i` (đã tồn tại trong đồ thị trước nó) — không thể nối
> tới một điểm CHƯA được chèn.

Kết quả: một đồ thị mà MỌI đỉnh có TỐI ĐA `M` láng giềng — đúng tính chất
"thưa" cần thiết để tìm tham lam nhanh hơn vét cạn (bài trước).
::::

::::example{#xay_do_thi_20_diem}
Kho `20` điểm của bài trước (`10` điểm công nghệ, `10` điểm ẩm thực), xây
đồ thị một tầng với `M=3`:

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


def xay_do_thi_mot_tang(vectors, M):
    do_thi = {i: [] for i in range(len(vectors))}
    for i in range(1, len(vectors)):
        ung_vien = list(range(i))
        sap_xep = sorted(((tuong_dong_cosine(vectors[i], vectors[j]), j) for j in ung_vien), reverse=True)
        chon = [j for _, j in sap_xep[:M]]
        for j in chon:
            do_thi[i].append(j)
            do_thi[j].append(i)
            if len(do_thi[j]) > M:
                lg = sorted(((tuong_dong_cosine(vectors[j], vectors[n]), n) for n in do_thi[j]), reverse=True)
                do_thi[j] = [n for _, n in lg[:M]]
    return do_thi


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

do_thi = xay_do_thi_mot_tang(VEC, 3)
so_lang_gieng = [len(do_thi[i]) for i in range(N)]
tong_canh = sum(so_lang_gieng) // 2

print(so_lang_gieng)
print(tong_canh)
print(max(so_lang_gieng) <= 3)
print(do_thi[8])
```

```text title=readonly
[3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
30
True
[5, 4, 2]
```

Sau khi chèn xong cả `20` điểm: MỖI đỉnh có ĐÚNG `3` láng giềng (bằng `M`),
tổng cộng `30` cạnh — khớp với công thức `N × M / 2 = 20 × 3 / 2 = 30`
(mỗi cạnh được đếm từ CẢ HAI đầu). Không đỉnh nào vượt quá `M=3` (bước cắt
bớt đã hoạt động đúng). Đỉnh `8` (`"du lieu duoc may tinh xu ly tu dong"`)
nối tới các đỉnh `[5, 4, 2]` — ba đỉnh công nghệ khác gần nó nhất.
::::

::::predict{#doan_khong_vuot_qua_M commitOnce}
Xét đúng ví dụ trên: `20` điểm được chèn LẦN LƯỢT, mỗi điểm mới nối tới
`M=3` láng giềng gần nhất trong số các điểm ĐÃ CHÈN TRƯỚC.

**Trước khi chạy thử**, bạn đoán: sau khi chèn HẾT `20` điểm, có đỉnh nào
kết thúc với NHIỀU HƠN `3` láng giềng không?

:::opt{correct}
Không — dù một đỉnh có thể được NHIỀU điểm sau nó chọn làm láng giềng (tích
luỹ nhiều hơn `3` cạnh tạm thời), bước CẮT BỚT (`if len(do_thi[j]) > M:`)
luôn đưa nó về lại ĐÚNG `M=3` láng giềng gần nhất, mỗi khi nó vượt ngưỡng
:::

:::opt
Có — những đỉnh được chèn SỚM (chỉ số nhỏ, ví dụ đỉnh `0` hay `1`) sẽ được
NHIỀU điểm chèn sau chọn làm láng giềng hơn, nên chúng tích luỹ nhiều cạnh
hơn `M` và không có cơ chế nào giới hạn lại
::why
Gần đúng ở việc đỉnh chèn sớm CÓ THỂ được nhiều điểm sau chọn làm láng
giềng hơn — quan sát về "cơ hội được chọn nhiều lần" đó đúng.

Chỗ lệch: mỗi lần một đỉnh `j` được chọn thêm (vượt quá `M` láng giềng),
đoạn mã LUÔN kiểm tra `len(do_thi[j]) > M` VÀ cắt bớt về đúng `M` láng
giềng gần `j` nhất — không có ngoại lệ nào cho đỉnh chèn sớm hay muộn. Cơ
chế cắt bớt này áp dụng ĐỀU cho mọi đỉnh, mọi lúc.
::
:::

:::opt
Có — vì tổng số cạnh phải là số nguyên, mà `N × M = 20 × 3 = 60` là số
chẵn nhưng khi chia cho `2` một vài đỉnh phải "gánh" thêm cạnh lẻ để tổng
khớp
::why
Gần đúng ở việc quan sát TỔNG SỐ CẠNH có ràng buộc số học thật (mỗi cạnh
được đếm từ hai đầu) — chi tiết đó đúng.

Chỗ lệch: `N × M = 60` đã là số CHẴN, `60 / 2 = 30` là một số nguyên tròn,
không có phần dư nào cần "gánh" — nếu MỌI đỉnh có đúng `M=3` láng giềng thì
tổng cạnh khớp chính xác, không cần đỉnh nào vượt ngưỡng.
::
:::
::::

::::code{#viet_xay_do_thi_mot_tang}
Hoàn thiện `xay_do_thi_mot_tang`: nối cạnh VÔ HƯỚNG (điểm `j` cũng phải
"biết" `i`), và cắt bớt về đúng `M` láng giềng khi vượt ngưỡng.

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
    return math.sqrt(sum(x ** 2 for x in v))


def tuong_dong_cosine(v1, v2):
    d1 = do_dai_vector(v1)
    d2 = do_dai_vector(v2)
    if d1 == 0 or d2 == 0:
        return 0.0
    return tich_vo_huong(v1, v2) / (d1 * d2)


def xay_do_thi_mot_tang(vectors, M):
    do_thi = {i: [] for i in range(len(vectors))}
    for i in range(1, len(vectors)):
        ung_vien = list(range(i))
        sap_xep = sorted(((tuong_dong_cosine(vectors[i], vectors[j]), j) for j in ung_vien), reverse=True)
        chon = [j for _, j in sap_xep[:M]]
        for j in chon:
            do_thi[i].append(j)
            do_thi[j].append(___)                # i
            if len(do_thi[j]) > ___:              # M
                lg = sorted(((tuong_dong_cosine(vectors[j], vectors[n]), n) for n in do_thi[j]), reverse=True)
                do_thi[j] = [n for _, n in lg[:M]]
    return do_thi


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

do_thi = xay_do_thi_mot_tang(VEC, 3)
so_lang_gieng = [len(do_thi[i]) for i in range(N)]
tong_canh = sum(so_lang_gieng) // 2

print(so_lang_gieng)
print(tong_canh)
print(max(so_lang_gieng) <= 3)
print(do_thi[8])
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


def xay_do_thi_mot_tang(vectors, M):
    do_thi = {i: [] for i in range(len(vectors))}
    for i in range(1, len(vectors)):
        ung_vien = list(range(i))
        sap_xep = sorted(((tuong_dong_cosine(vectors[i], vectors[j]), j) for j in ung_vien), reverse=True)
        chon = [j for _, j in sap_xep[:M]]
        for j in chon:
            do_thi[i].append(j)
            do_thi[j].append(i)
            if len(do_thi[j]) > M:
                lg = sorted(((tuong_dong_cosine(vectors[j], vectors[n]), n) for n in do_thi[j]), reverse=True)
                do_thi[j] = [n for _, n in lg[:M]]
    return do_thi


DIEM = [
    "may tinh chay phan mem manh", "lap trinh vien viet thuat toan hay", "vi xu ly xu ly du lieu nhanh",
    "ket noi mang giup ung dung on dinh", "thuat toan sap xep du lieu chuan", "ung dung may tinh xu ly du lieu lon",
    "vi xu ly manh giup ung dung nhanh", "phan mem lap trinh toi uu thuat toan", "du lieu duoc may tinh xu ly tu dong",
    "ung dung ket noi mang truyen du lieu",
    "mon an ngon can gia vi cong thuc", "dau bep nau an trong nha bep sach", "thuc pham tuoi giup mon an ngon",
    "cong thuc nau an don gian de lam", "mon trang mieng ngot ngao sau bua an", "nha bep sach se giup dau bep thoai mai",
    "gia vi dam da giup mon an hap dan", "dau bep gioi che bien thuc pham tuoi", "mon an voi cong thuc gia vi vua an",
    "thuc pham nau thanh mon trang mieng ngon",
]
VEC = [tinh_vector_dem_tu(d, TU_VUNG, CUM_TU_GOC) for d in DIEM]
N = len(VEC)

do_thi = xay_do_thi_mot_tang(VEC, 3)
so_lang_gieng = [len(do_thi[i]) for i in range(N)]
tong_canh = sum(so_lang_gieng) // 2

print(so_lang_gieng)
print(tong_canh)
print(max(so_lang_gieng) <= 3)
print(do_thi[8])
```

```python title=test
assert so_lang_gieng == [3] * 20, f"moi dinh phai co DUNG 3 lang gieng -- dang ra {so_lang_gieng}"
assert tong_canh == 30, f"tong so canh phai la 30 (= 20*3/2) -- dang ra {tong_canh}"
assert max(so_lang_gieng) <= 3, "khong dinh nao duoc vuot qua M=3 lang gieng"
assert do_thi[8] == [5, 4, 2], f"dinh 8 phai noi toi [5, 4, 2] -- dang ra {do_thi[8]}"

# kiem tra truc tiep tren mot vi du nho, tu tinh tay duoc: 4 diem, M=1
vec_nho = [[1, 0], [1, 0], [0, 1], [0, 1]]
do_thi_nho = xay_do_thi_mot_tang(vec_nho, 1)
assert do_thi_nho[0] == [1], f"diem 0 chi co 1 ung vien (diem 1, cosine=1.0) -- dang ra {do_thi_nho[0]}"
assert all(len(v) <= 1 for v in do_thi_nho.values()), "voi M=1, KHONG dinh nao duoc co qua 1 lang gieng"

# bien: diem dau tien (chi so 0) khong co ung vien nao de noi (chua co diem nao chen truoc no)
assert do_thi[0] != [], "diem 0 van phai co lang gieng, vi CAC DIEM SAU no se noi NGUOC lai voi no"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là đối số của `do_thi[j].append(...)` — đây chính là bước tạo cạnh NGƯỢC (từ `j` về lại điểm đang chèn), để cạnh trở thành VÔ HƯỚNG; giá trị cần thêm vào là chỉ số của điểm ĐANG được chèn. Chỗ hai là NGƯỠNG so sánh trong điều kiện cắt bớt — số láng giềng tối đa mà một đỉnh được phép giữ.
- kind: strategy
  body: 'Chỗ đầu: `i` — điểm đang được chèn (biến vòng lặp ngoài cùng). Chỗ hai: `M` — tham số giới hạn số láng giềng, truyền vào hàm.'
- kind: one-line
  body: 'Chỗ đầu là `i`, chỗ hai là `M`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la bien 'i' (diem dang chen, de tao canh NGUOC lam canh vo huong) -- khong duoc dung 'j' hay mot ten khac; cho trong hai phai la bien 'M' (nguong cat bot) -- khong duoc chep san mot so; VA toan tu so sanh '>' o dieu kien cat bot phai giu NGUYEN (khong duoc doi thanh '>=')
  requireAst:
  - kind: uses-name, target: i, min: 6
  - kind: uses-name, target: M, min: 3
  - kind: uses-operator, target: ">", min: 1
  # Da thu that (dung ban sao _dem trich tu kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [6, 3, 1] cho ba
  # luat theo dung thu tu khai bao o tren.
  #
  # 🔴🔴🔴 LO CHAM DIEM PHAT HIEN BOI CONG CU DOT BIEN TU DONG (tools/
  # kiem_dot_bien.mjs), KHONG PHAI tu tay/agent dung: chay cong cu that qua
  # `node tools/kiem_dot_bien.mjs` bao "doi MOI dau > thanh >= (1 cho) --
  # van qua sach moi tang cham" tren dong CO chua cho trong ("if
  # len(do_thi[j]) > ___:"). Dieu tra: voi ">=", buoc cat bot kich hoat SOM
  # HON mot nhip (ngay khi do dai DUNG BANG M, khong can VUOT qua M), nhung
  # vi buoc cat luon lay dung `lg[:M]` (M lang gieng GAN NHAT trong so cac
  # ung vien hien co, sap xep lai theo cosine that), tap hop lang gieng CUOI
  # CUNG hoi tu ve dung mot ket qua -- tren BO DU LIEU CO DINH nay, ket qua
  # SAU KHI chen xong 20 diem la Y HET nhau du dung '>' hay '>=' (da tu xac
  # nhan bang cach chay THAT ca hai bien the qua python3: so_lang_gieng,
  # tong_canh, do_thi[8] deu giong het nhau) -- nen KHONG assertion/output
  # nao (tests/output) phan biet duoc hai bien the. Luat static moi them o
  # tren (`uses-operator ">" min: 1`) dong lo nay: '>=' la mot lop AST khac
  # han '>' (ast.GtE khac ast.Gt), nen dem AST cua '>' tut xuong 0 (duoi
  # nguong min=1), bi static chan NGAY -- da tu xac nhan lai bang cong cu
  # dot bien THAT sau khi them luat: `node tools/kiem_dot_bien.mjs` khong
  # con bao lo nao cho buoc nay.
  # i=6 (TONG THAT, da xac nhan bang cong cu, khong doan tay): doc (Load) o
  # {i: [] for i in range(...)} (1 -- key cua dict-comp doc lai bien vong lap
  # cua CHINH no), "list(range(i))" (1), "vectors[i]" trong sorted(...) (1),
  # "do_thi[i].append(j)" (1), VA cho trong dau "do_thi[j].append(i)" (1) --
  # cong 5, nhung cong cu bao 6; kiem lai bang python3 that (khong doan) xac
  # nhan dung 6, mot lan doc nam o vi tri khac chua liet ke het o day, khong
  # anh huong toi viec dat nguong dung bang TONG THAT do cong cu bao.
  # M=3 (TONG THAT): "sap_xep[:M]" (chon M ung vien, 1), cho trong hai "> M"
  # (1), VA "lg[:M]" (cat con M lang gieng, 1). Neu chi dat min=1 cho tung
  # luat (ngay tho), mot mutant BO SOT hoan toan cho trong (vi du dien "0"
  # vao ca hai) van co the qua nho cac lan xuat hien CO SAN trong boilerplate
  # -- GOTCHA "boilerplate-threshold-masking"; da tu kiem chung bang cach
  # dien "0" vao ca hai cho trong: static (voi min=6/3 dung) chan duoc ngay
  # vi tong giam xuong duoi nguong, VA rieng viec dien "0" vao cho trong dau
  # con lam do_thi[j].append(0) (them mot "lang gieng" gia, sai hoan toan)
  # -- ca hai tang deu bat duoc, doc lap nhau.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant va CHAY THAT qua
  # python3 de xac nhan, khong doan tay): dien "M" vao cho trong dau
  # ("do_thi[j].append(M)") VA dien "i" vao cho trong hai ("if
  # len(do_thi[j]) > i:") -- ket qua AST la [6, 3], Y HET ban dung (tong so
  # lan doc 'i' va 'M' khong doi, chi doi VI TRI: 'M' gio duoc doc o cho
  # dau thay vi cho hai, 'i' gio duoc doc o cho hai thay vi cho dau). Static
  # KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run'/'tests'/'output': da tu chay THAT qua
  # python3, xac nhan no KHONG nem loi (ca "M" -- mot so nguyen -- lan "i" --
  # cung mot so nguyen -- deu la gia tri HOP LE de append/so sanh, khong gay
  # TypeError) nhung cho ra do_thi HOAN TOAN SAI: so_lang_gieng tro thanh
  # [4, 5, 7, 7, 6, 6, 3, 4, 6, 6, 8, 8, 8, 6, 5, 5, 4, 4, 3, 3] (khong con
  # dong nhat = 3 nua), tong_canh = 54 (khong phai 30), max(so_lang_gieng) <=
  # 3 tro thanh False, va do_thi[8] = [5, 4, 2, 3, 3, 3] (6 phan tu, khong
  # phai 3) -- MOI assertion o tren deu bat duoc mutant nay, doc lap voi
  # static.
  # Da tu ra soat rieng mot bien the "dien bua": dien CA HAI cho trong bang
  # `True` (`do_thi[j].append(True)`, `if len(do_thi[j]) > True:`) -- da tu
  # chay THAT qua python3 va phat hien no VAN CHO RA output DUNG HET (vi
  # `True == 1`, nguong cat bot tro nen chat hon nhung buoc cat van luon lay
  # dung M=3 lang gieng gan nhat trong slice `lg[:M]` khong doi, nen ket qua
  # CUOI CUNG tinh co trung khop) -- day la mot ca "dien bua qua duoc output"
  # THAT, nhung bi chan boi CHINH tang 'static': dem AST cho ca hai cho
  # trong dien True la [5, 2] (thay vi [6, 3] dung), duoi nguong min o tren,
  # nen static day loi TRUOC KHI kip chay toi output -- xac nhan static la
  # tang BAT BUOC cho truong hop nay, khong phai tests/output.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3\\]\\n30\\nTrue\\n\\[5, 4, 2\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`30` cạnh, mỗi đỉnh đúng `M=3` láng giềng — đồ thị thưa đã tự xây xong. Bài
sau viết thuật toán TÌM KIẾM tham lam đầy đủ trên đồ thị này.
::::

::::reflect{#nghi-lai}
Xây đồ thị một tầng không cần thuật toán phức tạp — chỉ cần MỘT quy tắc lặp
lại cho từng điểm mới: tìm `M` láng giềng gần nhất trong số các điểm đã có,
nối cạnh CẢ HAI CHIỀU, và cắt bớt nếu một đỉnh vượt quá `M`. Kết quả là một
đồ thị THƯA có kiểm soát — không đỉnh nào phình to không giới hạn, dù thứ tự
chèn có ưu ái đỉnh nào. Đây chính là cấu trúc dữ liệu mà bài trước đã DÙNG
như một hộp đen; giờ nó không còn là hộp đen nữa. Bài sau viết thuật toán
TÌM KIẾM tham lam đầy đủ (không chỉ một bước, mà lặp tới khi hội tụ), và đo
số bước nhảy lẫn số phép so sánh cho một truy vấn cụ thể.
::::

::::checkpoint{mastery=0.85}
::::
