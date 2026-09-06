---
id: tri-tue-nhan-tao.chunking-va-embedding-tu-che.do-tuong-dong-hai-doan-van
title: "Đo tương đồng hai đoạn văn: tích vô hướng (dot product)"
summary: "Dung lai DUNG bo tu vung 16 muc va ba vector vA1/vA2/vB1 cua bai truoc. tich_vo_huong(vA1, vA2) = 4 (cung chu de cong nghe), tich_vo_huong(vA1, vB1) = 0 (khac chu de) -- 4 > 0, do bang MOT con so DUY NHAT thay vi dem tho so chi so khac 0. Day la tich vo huong THO (dot product), CHUA chuan hoa do dai vector -- cosine similarity day du de danh cho quest sau (q8.5b)."
locale: vi
track: tri-tue-nhan-tao
module: chunking-va-embedding-tu-che
order: 5
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.do-tuong-dong-hai-doan-van]
requires: [ai.vector-tu-che-dem-tu]
concepts: [ai.do-tuong-dong-hai-doan-van]
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
Bài trước đếm SỐ CHỈ SỐ cùng khác `0` giữa hai vector — một phép đo thô, chỉ
nói "có bao nhiêu từ chung". Bài này thay nó bằng MỘT con số duy nhất, tính
đến CẢ số lần lặp lại của từng từ: tích vô hướng.
::::

::::explain{#tich_vo_huong_la_gi}
Đếm "chỉ số cùng khác `0`" (bài trước) chỉ trả lời được câu hỏi CÓ/KHÔNG cho
từng vị trí — không phân biệt được một từ xuất hiện `1` lần hay `5` lần.
**Tích vô hướng** (dot product) sửa đúng chỗ đó: nhân TỪNG CẶP thành phần
tương ứng của hai vector, rồi CỘNG DỒN tất cả lại thành MỘT con số duy nhất:

```
tich_vo_huong(v1, v2) = v1[0]*v2[0] + v1[1]*v2[1] + ... + v1[n-1]*v2[n-1]
```

Trực giác: nếu cả `v1` VÀ `v2` đều khác `0` ở CÙNG một chỉ số (cùng chứa một
từ vựng), tích của cặp đó góp một số DƯƠNG vào tổng — càng nhiều từ CHUNG,
và mỗi từ chung xuất hiện càng NHIỀU lần ở cả hai đoạn, tổng càng LỚN. Nếu
một chỉ số bằng `0` ở BẤT KỲ vector nào (từ đó không xuất hiện ở ít nhất một
trong hai đoạn), tích của cặp đó là `0` — không đóng góp gì vào tổng. Hai
đoạn hoàn toàn KHÁC chủ đề (không từ vựng nào chung, như bài trước đã đo)
cho tích vô hướng bằng đúng `0`.

*(Ghi chú: đây là tích vô hướng THÔ, CHƯA chuẩn hoá theo độ dài (độ lớn) của
vector — một đoạn văn DÀI hơn, dùng từ vựng nhiều lần hơn, sẽ tự nhiên có
tích vô hướng LỚN hơn một đoạn ngắn, dù mức độ "cùng chủ đề" tương đương.
Cách sửa vấn đề này — chuẩn hoá bằng độ dài, gọi là độ tương đồng cô-sin
(cosine similarity) — dành cho quest sau, `tim-kiem-vector-va-do-tuong-dong`
(q8.5b). Bài này chỉ cần tích vô hướng thô để SO SÁNH TƯƠNG ĐỐI hai cặp đoạn
trên CÙNG một bộ từ vựng — đủ để phân biệt "cùng chủ đề" khỏi "khác chủ
đề".)*
::::

::::example{#tich_vo_huong_hai_cap_doan}
Dùng lại đúng bộ từ vựng và ba vector của bài trước — `vA1`, `vA2` (cùng chủ
đề công nghệ), `vB1` (chủ đề ẩm thực):

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


DOAN_A1 = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh xu ly du lieu qua vi xu ly manh me"
DOAN_A2 = "lap trinh vien viet phan mem de may tinh xu ly du lieu nhanh hon ung dung thuat toan toi uu"
DOAN_B1 = "mon an ngon phu thuoc vao gia vi va cong thuc nau dau bep gioi trong nha bep sach se tao ra mon trang mieng hap dan"

vA1 = tinh_vector_dem_tu(DOAN_A1, TU_VUNG, CUM_TU_GOC)
vA2 = tinh_vector_dem_tu(DOAN_A2, TU_VUNG, CUM_TU_GOC)
vB1 = tinh_vector_dem_tu(DOAN_B1, TU_VUNG, CUM_TU_GOC)

dot_cung_chu_de = tich_vo_huong(vA1, vA2)
dot_khac_chu_de = tich_vo_huong(vA1, vB1)

print(dot_cung_chu_de)
print(dot_khac_chu_de)
print(dot_cung_chu_de > dot_khac_chu_de)
```

```text title=readonly
4
0
True
```

`tich_vo_huong(vA1, vA2) = 4` — cả `vA1` và `vA2` khác `0` ở đúng `4` chỉ số
(`may_tinh`, `phan_mem`, `lap_trinh`, `du_lieu`), mỗi chỉ số đó đều bằng
`1` ở CẢ hai vector (`1*1 = 1`, cộng dồn `4` chỉ số bằng `4`).
`tich_vo_huong(vA1, vB1) = 0` — không chỉ số nào khác `0` ở CẢ HAI vector
cùng lúc, nên mọi tích thành phần đều bằng `0`. Kết quả: `4 > 0` — MỘT con
số duy nhất, không cần đếm tay từng chỉ số, đã phân biệt được cặp cùng chủ
đề khỏi cặp khác chủ đề.
::::

::::predict{#doan_dot_product_hai_cap commitOnce}
Xét đúng ba vector `vA1 = [1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0]`,
`vB1 = [0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,1]` ở ví dụ trên (không chỉ số nào
khác `0` ở cả hai cùng lúc).

**Trước khi chạy thử**, bạn đoán: `tich_vo_huong(vA1, vB1)` là bao nhiêu?

:::opt{correct}
`0` — với MỌI chỉ số `i`, ít nhất một trong hai giá trị `vA1[i]` hoặc
`vB1[i]` bằng `0` (không chỉ số nào cả hai đều khác `0`), nên MỌI tích
thành phần `vA1[i] * vB1[i]` đều bằng `0`, và tổng của toàn số `0` vẫn là
`0`
:::

:::opt
`10` — vì cộng tổng số thành phần khác `0` của cả hai vector lại (`5` ở
`vA1`, cộng `6` ở `vB1`) sẽ ra tổng mức độ "hoạt động" của cặp đoạn này
::why
Gần đúng ở việc `5` và `6` đúng là số thành phần khác `0` của từng vector
riêng lẻ — quan sát đó đúng.

Chỗ lệch: tích vô hướng không CỘNG các vector lại — nó NHÂN từng cặp thành
phần TƯƠNG ỨNG (cùng chỉ số) rồi mới cộng các TÍCH đó. Vì hai tập chỉ số
khác `0` của `vA1` và `vB1` không hề giao nhau, mọi tích thành phần đều là
`(số khác 0) * 0 = 0`, hoặc `0 * (số khác 0) = 0` — không có phép cộng
trực tiếp hai vector nào xảy ra ở đây cả.
::
:::

:::opt
Một số ÂM — vì hai đoạn hoàn toàn KHÁC chủ đề nên phải có một "khoảng
cách" lớn, và khoảng cách trong toán học thường được biểu diễn bằng số âm
khi hai thứ đối lập nhau
::why
Gần đúng ở TRỰC GIÁC "khác chủ đề nên phải có một tín hiệu ĐỐI LẬP rõ rệt"
— một cách nghĩ hợp lý khi liên tưởng tới các phép đo "khoảng cách" khác.

Chỗ lệch: vector đếm từ ở đây chỉ chứa số ĐẾM — luôn là số nguyên KHÔNG ÂM
(`0` hoặc dương, không có "đếm âm"). Tích của hai số không âm luôn KHÔNG
ÂM (`0` hoặc dương), nên tổng của chúng — tích vô hướng — cũng không bao
giờ ÂM với loại vector này. "Khác chủ đề" ở đây thể hiện qua giá trị THẤP
NHẤT có thể (`0`), không phải một giá trị âm.
::
:::
::::

::::code{#viet_tich_vo_huong}
Hoàn thiện `tich_vo_huong` (nhân từng cặp thành phần, cộng dồn) và
`cao_hon_han` (so sánh hai tích vô hướng).

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


def cao_hon_han(d_cung_chu_de, d_khac_chu_de):
    return ___                                              # d_cung_chu_de > d_khac_chu_de


DOAN_A1 = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh xu ly du lieu qua vi xu ly manh me"
DOAN_A2 = "lap trinh vien viet phan mem de may tinh xu ly du lieu nhanh hon ung dung thuat toan toi uu"
DOAN_B1 = "mon an ngon phu thuoc vao gia vi va cong thuc nau dau bep gioi trong nha bep sach se tao ra mon trang mieng hap dan"

vA1 = tinh_vector_dem_tu(DOAN_A1, TU_VUNG, CUM_TU_GOC)
vA2 = tinh_vector_dem_tu(DOAN_A2, TU_VUNG, CUM_TU_GOC)
vB1 = tinh_vector_dem_tu(DOAN_B1, TU_VUNG, CUM_TU_GOC)

dot_cung_chu_de = tich_vo_huong(vA1, vA2)
dot_khac_chu_de = tich_vo_huong(vA1, vB1)

print(dot_cung_chu_de)
print(dot_khac_chu_de)
print(cao_hon_han(dot_cung_chu_de, dot_khac_chu_de))
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


def cao_hon_han(d_cung_chu_de, d_khac_chu_de):
    return d_cung_chu_de > d_khac_chu_de


DOAN_A1 = "may tinh hien dai chay phan mem duoc viet bang ngon ngu lap trinh xu ly du lieu qua vi xu ly manh me"
DOAN_A2 = "lap trinh vien viet phan mem de may tinh xu ly du lieu nhanh hon ung dung thuat toan toi uu"
DOAN_B1 = "mon an ngon phu thuoc vao gia vi va cong thuc nau dau bep gioi trong nha bep sach se tao ra mon trang mieng hap dan"

vA1 = tinh_vector_dem_tu(DOAN_A1, TU_VUNG, CUM_TU_GOC)
vA2 = tinh_vector_dem_tu(DOAN_A2, TU_VUNG, CUM_TU_GOC)
vB1 = tinh_vector_dem_tu(DOAN_B1, TU_VUNG, CUM_TU_GOC)

dot_cung_chu_de = tich_vo_huong(vA1, vA2)
dot_khac_chu_de = tich_vo_huong(vA1, vB1)

print(dot_cung_chu_de)
print(dot_khac_chu_de)
print(cao_hon_han(dot_cung_chu_de, dot_khac_chu_de))
```

```python title=test
assert dot_cung_chu_de == 4, f"tich vo huong cua cap cung chu de phai la 4 -- dang ra {dot_cung_chu_de}"
assert dot_khac_chu_de == 0, f"tich vo huong cua cap khac chu de phai la 0 -- dang ra {dot_khac_chu_de}"
assert cao_hon_han(dot_cung_chu_de, dot_khac_chu_de) == True, "cao_hon_han phai la True -- cung chu de phai cao hon han khac chu de"

# kiem tra truc tiep ham tich_vo_huong tren vector nho, tu tinh tay duoc
assert tich_vo_huong([1, 2, 3], [4, 5, 6]) == 32, f"1*4+2*5+3*6=32 -- dang ra {tich_vo_huong([1, 2, 3], [4, 5, 6])}"
assert tich_vo_huong([0, 0, 0], [1, 2, 3]) == 0, "tich vo huong voi vector toan so 0 phai la 0"
assert tich_vo_huong([2, 0], [0, 3]) == 0, "khong chi so nao cung khac 0 thi tich vo huong phai la 0"

# tinh doi xung: tich_vo_huong(v1, v2) phai bang tich_vo_huong(v2, v1)
assert tich_vo_huong(vA1, vA2) == tich_vo_huong(vA2, vA1), "tich vo huong phai doi xung: doi thu tu hai vector khong doi ket qua"

# bien cao_hon_han: gia tri bang nhau phai cho False (khong PHAI cao hon han)
assert cao_hon_han(3, 3) == False, "hai gia tri BANG NHAU khong duoc coi la 'cao hon han'"
assert cao_hon_han(2, 5) == False, "gia tri THAP hon khong duoc coi la 'cao hon han'"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `tich_vo_huong`) là biểu thức bên trong `sum(... for a, b in zip(v1, v2))` — với mỗi cặp `(a, b)` (một thành phần từ `v1`, một thành phần TƯƠNG ỨNG từ `v2`), tính TÍCH của chúng. Chỗ hai (trong `cao_hon_han`) là một phép SO SÁNH đơn giản giữa hai đối số — trả về `True` khi đối số ĐẦU thật sự LỚN HƠN đối số sau (không phải lớn hơn-hoặc-bằng).
- kind: strategy
  body: 'Chỗ đầu: `a * b` — nhân hai thành phần tương ứng. Chỗ hai: `d_cung_chu_de > d_khac_chu_de` — so sánh nghiêm ngặt bằng toán tử `>`.'
- kind: one-line
  body: 'Chỗ đầu là `a * b`, chỗ hai là `d_cung_chu_de > d_khac_chu_de`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai NHAN (dung toan tu '*') hai thanh phan tuong ung a, b -- khong duoc cong hay chep san mot so; VA cho trong hai phai SO SANH THAT bang toan tu '>' (khong duoc luon tra ve True/False chep san)
  requireAst:
  - kind: uses-operator, target: "*", min: 1
  - kind: uses-operator, target: ">", min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1] cho hai
  # luat theo dung thu tu khai bao o tren.
  # "*"=1: CHI mot lan duy nhat trong toan bo solution, dung o cho trong dau
  # ("a * b" ben trong generator expression cua sum(...)). Dien "a + b" (cong
  # thay vi nhan) lam so nay tut ve 0 -- bi chan boi static; da tu kiem
  # chung: dien nhu vay lam tich_vo_huong([1,2,3],[4,5,6]) tro thanh
  # 1+2+3+4+5+6=21 thay vi 32 (dung ra phai la tich), bi bat CA boi static
  # LAN boi assertion rieng ve vi du nho nay.
  # ">"=1: CHI mot lan duy nhat, dung o cho trong hai ("d_cung_chu_de >
  # d_khac_chu_de"). Dien "return True" (chep san, dung tren DUNG bo du lieu
  # nay vi 4>0 THAT la True) lam so nay tut ve 0 -- bi chan boi static; da tu
  # kiem chung: tren du lieu chinh cua bai hoc dien "True" van cho ket qua
  # dung (vi 4>0 THAT la True), nen day la mot cheat CHI static rieng moi
  # bat duoc -- nhung assertion rieng "cao_hon_han(3, 3) == False" va
  # "cao_hon_han(2, 5) == False" (o duoi) cung bat duoc no doc lap, vi
  # "return True" chep san se sai tren HAI ca do.
  #
  # Hai cho trong nay KHONG hoan doi duoc cho nhau: cho dau la mot bieu thuc
  # SO (dung trong sum/generator, kieu int), cho hai la mot bieu thuc BOOL
  # dat trong vi tri return cua MOT HAM KHAC HAN (cao_hon_han nhan hai doi so
  # d_cung_chu_de/d_khac_chu_de, khong lien quan gi toi a/b cua
  # tich_vo_huong) -- khac ham, khac bien, khong the "hoan doi ca cum" theo
  # dung nghia GOTCHA #5 (hai gia tri phai o CUNG mot cho ky cu phap tuong
  # duong moi hoan doi duoc). Da tu kiem tra logic nay truoc khi ket luan,
  # khong bo qua buoc xac minh.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^4\\n0\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tich_vo_huong(vA1, vA2) = 4`, `tich_vo_huong(vA1, vB1) = 0` — MỘT con số
duy nhất phân biệt cùng chủ đề khỏi khác chủ đề. Bài BOSS ráp lại toàn bộ
quest: chia một tài liệu, tính vector mỗi đoạn, đo mọi cặp.
::::

::::reflect{#nghi-lai}
Tích vô hướng không "hiểu" nội dung — nó chỉ nhân và cộng những con số đã
đếm sẵn. Nhưng ráp đúng ba mảnh đã xây (chia đoạn → vector đếm từ → tích vô
hướng), nó cho ra một con số DUY NHẤT phân biệt được "cùng chủ đề" khỏi
"khác chủ đề" mà không cần đọc lại văn bản gốc lần nào nữa — đúng nền tảng
của bước TRUY XUẤT trong RAG (bài `1`): so mọi đoạn trong kho tài liệu với
câu hỏi, lấy đoạn có tích vô hướng CAO NHẤT. Bài BOSS tiếp theo ráp lại toàn
bộ năm bài của quest này trên một tài liệu MỚI, tự nghĩ: chia thành đoạn,
tính vector từng đoạn, và đo MỌI cặp — xác nhận bằng số rằng cơ chế này thật
sự hoạt động, không chỉ trên ví dụ đã dựng sẵn.
::::

::::checkpoint{mastery=0.85}
::::
