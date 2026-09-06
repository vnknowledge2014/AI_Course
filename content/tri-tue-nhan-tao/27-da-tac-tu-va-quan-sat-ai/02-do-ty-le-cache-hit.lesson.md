---
id: tri-tue-nhan-tao.da-tac-tu-va-quan-sat-ai.do-ty-le-cache-hit
title: "Semantic cache: hỏi cosine TRƯỚC khi gọi LLM, đo tỉ lệ cache hit"
summary: "tim_trong_cache(cau_hoi_vector, cache, nguong_cosine) DUYET tung muc {cau_hoi, vector, tra_loi} theo THU TU trong cache, tinh tuong_dong_cosine (tai dung nguyen van tu q8.5a/b), tra ve tra_loi cua muc DAU TIEN co cosine VUOT qua nguong_cosine (dung > nghiem ngat, dung 0,95 theo §39 Bai tap 2), hoac None neu khong muc nao dat. do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine) chay MOI cau hoi qua tim_trong_cache, dem so lan khac None chia tong so cau hoi. Tren CACHE 3 muc da tra loi (ao thun/doi tra/cham soc khach hang) va 5 cau hoi moi (2 cau gan giong cosine ~0,999, 1 cau hoan toan khac cosine ~0,577 o CA BA muc, 1 cau gan giong con lai, 1 cau chi dat cosine 0,7071 -- duoi nguong 0,95): ty_le_cache_hit = 3/5 = 0,6 -- DUNG 60%, khong phai 100% hay 0%. Doi nguong_cosine tu 0,95 xuong 0,7: cau thu 5 (0,7071 > 0,7) THANH hit, ty le tang len 4/5 = 0,8 -- nguong THAT SU rang buoc ket qua."
locale: vi
track: tri-tue-nhan-tao
module: da-tac-tu-va-quan-sat-ai
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.do-ty-le-cache-hit]
requires: [ai.da-tac-tu-qua-su-kien]
concepts: [ai.do-ty-le-cache-hit]
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
`Chương 39` Bài tập `2`: "Cài semantic cache: trước khi gọi LLM, tìm trong
vector store xem có prompt tương tự (cosine > `0,95`) đã trả lời chưa. Đo
tỉ lệ cache hit trên một tập câu hỏi thật." Gọi LLM tốn tiền VÀ tốn thời
gian (§39.1: độ trễ tính bằng giây). Nếu câu hỏi này GẦN GIỐNG một câu đã
trả lời, tại sao phải hỏi lại?
::::

::::explain{#tim_truoc_khi_goi_lai}
Semantic cache khác cache thông thường (khớp CHÍNH XÁC chuỗi văn bản) — nó
khớp theo Ý NGHĨA. "Áo thun xanh giá bao nhiêu" và "ao thun mau xanh gia
bao nhieu tien" là hai CHUỖI khác nhau, nhưng cùng một Ý — vector embedding
của chúng gần nhau, cosine cao. `Chương 39` Bài tập `2` nói cụ thể: ngưỡng
`0,95` — chỉ coi LÀ "đã trả lời rồi" khi cosine VƯỢT qua mức đó, không phải
"na ná".

`tuong_dong_cosine` đã viết ở `q8.5a`/`q8.5b` — TÁI DÙNG NGUYÊN VĂN, không
viết lại. Cache LÀ một danh sách các mục đã lưu, MỖI mục nhớ CẢ BA thứ: câu
hỏi gốc, vector của nó, và câu trả lời đã có:

```
tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
  cho MOI muc TRONG cache (theo DUNG thu tu):
    do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
    neu do_tuong_dong > nguong_cosine:
      tra ve muc["tra_loi"]                 # DUNG luon, khong xet tiep
  tra ve None                                # khong muc nao dat

do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
  so_hit = dem so cau hoi ma tim_trong_cache(...) KHAC None
  tra ve so_hit / tong so cau hoi
```

Điểm mấu chốt: `tim_trong_cache` dừng lại ở mục ĐẦU TIÊN vượt ngưỡng — nó
không đi tìm mục "tốt nhất" trong cache, chỉ cần một mục ĐỦ TỐT là trả lời
ngay, đúng tinh thần "tránh gọi LLM càng sớm càng tốt".
::::

::::example{#do_ty_le_hit_tren_5_cau_hoi}
```python title=readonly
import math


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


def tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
    for muc in cache:
        do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
        if do_tuong_dong > nguong_cosine:
            return muc["tra_loi"]
    return None


def do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
    so_hit = 0
    for v in danh_sach_cau_hoi_vector:
        if tim_trong_cache(v, cache, nguong_cosine) is not None:
            so_hit += 1
    return so_hit / len(danh_sach_cau_hoi_vector)


CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]

CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0],
    [0.02, 0.98, 0.0],
    [0.3, 0.3, 0.3],
    [0.0, 0.02, 0.99],
    [0.5, 0.5, 0.0],
]

KET_QUA_TIM = [tim_trong_cache(v, CACHE, 0.95) for v in CAC_VECTOR_MOI]
TY_LE_HIT = do_ty_le_cache_hit(CAC_VECTOR_MOI, CACHE, 0.95)

print(KET_QUA_TIM)
print(TY_LE_HIT)
```

```text title=readonly
['Ao thun xanh gia 150000 dong', 'Doi tra trong vong 7 ngay', None, 'Goi hotline 1900 xxxx', None]
0.6
```

`CACHE` có `3` mục đã trả lời. `5` câu hỏi mới: vector `[0.99, 0.05, 0.0]`
GẦN giống mục `0` (cosine ≈ `0,9987`, vượt `0,95`) → HIT, trả lời cũ. Vector
`[0.02, 0.98, 0.0]` gần giống mục `1` (cosine ≈ `0,9998`) → HIT. Vector
`[0.3, 0.3, 0.3]` KHÁC HẲN cả `3` mục (cosine ≈ `0,5774` với MỌI mục, đều
dưới `0,95`) → `None`, KHÔNG hit — phải gọi LLM thật. Vector
`[0.0, 0.02, 0.99]` gần giống mục `2` (cosine ≈ `0,9998`) → HIT. Vector
`[0.5, 0.5, 0.0]` có cosine ≈ `0,7071` với mục `0` VÀ mục `1` — GẦN nhưng
KHÔNG ĐỦ (dưới ngưỡng `0,95`) → `None`. Tổng: `3` hit trên `5` câu —
`TY_LE_HIT = 3 / 5 = 0,6` — một tỉ lệ CỤ THỂ, không phải toàn trúng hay
toàn trượt.
::::

::::predict{#doan_doi_nguong_cosine commitOnce}
Xét đúng `5` câu hỏi và `CACHE` ở ví dụ trên. Câu hỏi thứ `5`
(`[0.5, 0.5, 0.0]`) có cosine ≈ `0,7071` với mục `0` — KHÔNG đủ ngưỡng
`0,95` nên bị tính LÀ trượt (`None`).

**Trước khi chạy thử**, bạn đoán: nếu đổi `nguong_cosine` từ `0,95` XUỐNG
`0,7` (giữ nguyên mọi thứ khác), `TY_LE_HIT` MỚI LÀ bao nhiêu?

:::opt{correct}
`0,8` (`4/5`) — câu hỏi thứ `5` giờ có cosine (`0,7071`) VƯỢT ngưỡng mới
(`0,7`), nên nó chuyển thành HIT; `4` trên `5` câu hỏi giờ đạt, tỉ lệ tăng
từ `0,6` lên `0,8`
:::

:::opt
Vẫn là `0,6` — hạ ngưỡng chỉ làm cache "khoan dung" hơn với những câu ĐÃ
hit, không ảnh hưởng câu đã trượt
::why
Gần đúng ở việc hạ ngưỡng THẬT SỰ không đổi những câu ĐÃ vượt ngưỡng cũ
(chúng vẫn hit) — quan sát đó đúng.

Chỗ lệch: hạ ngưỡng làm ĐIỀU KIỆN `do_tuong_dong > nguong_cosine` dễ đạt
HƠN cho MỌI câu hỏi, kể cả câu ĐÃ trượt ở ngưỡng cũ. Câu hỏi thứ `5` có
cosine `0,7071` — lớn hơn `0,7` — nên với ngưỡng mới nó ĐỦ điều kiện, dù
KHÔNG đủ ở ngưỡng `0,95` cũ. `nguong_cosine` là tham số của CHÍNH điều kiện
so sánh, không phải một hằng số cố định.
::
:::

:::opt
`1,0` (`5/5`) — hạ ngưỡng đủ thấp thì MỌI câu hỏi đều được coi LÀ hit, vì
cosine của bất kỳ hai vector nào cũng dương
::why
Gần đúng ở việc hạ ngưỡng làm NHIỀU câu hỏi hit hơn — hướng thay đổi đó
đúng.

Chỗ lệch: câu hỏi thứ `3` (`[0.3, 0.3, 0.3]`) có cosine ≈ `0,5774` với CẢ
BA mục trong `CACHE` — THẤP HƠN cả ngưỡng mới `0,7`. Hạ ngưỡng xuống `0,7`
CHƯA đủ thấp để câu này hit; nó cần một ngưỡng dưới `0,5774` mới đạt. Không
phải MỌI mức hạ ngưỡng đều biến hết mọi câu thành hit.
::
:::
::::

::::code{#viet_tim_va_do_ty_le_cache}
Hoàn thiện `tim_trong_cache` (điều kiện SO SÁNH cosine với ngưỡng, quyết
định một mục có "đủ tốt" để trả lời ngay không) và `do_ty_le_cache_hit`
(phép CHIA số lần hit cho tổng số câu hỏi).

```python title=starter
import math


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


def tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
    for muc in cache:
        do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
        if ___:                                              # do_tuong_dong > nguong_cosine
            return muc["tra_loi"]
    return None


def do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
    so_hit = 0
    for v in danh_sach_cau_hoi_vector:
        if tim_trong_cache(v, cache, nguong_cosine) is not None:
            so_hit += 1
    return ___                                                # so_hit / len(danh_sach_cau_hoi_vector)


CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]

CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0],
    [0.02, 0.98, 0.0],
    [0.3, 0.3, 0.3],
    [0.0, 0.02, 0.99],
    [0.5, 0.5, 0.0],
]

KET_QUA_TIM = [tim_trong_cache(v, CACHE, 0.95) for v in CAC_VECTOR_MOI]
TY_LE_HIT = do_ty_le_cache_hit(CAC_VECTOR_MOI, CACHE, 0.95)

print(KET_QUA_TIM)
print(TY_LE_HIT)
```

```python title=solution
import math


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


def tim_trong_cache(cau_hoi_vector, cache, nguong_cosine):
    for muc in cache:
        do_tuong_dong = tuong_dong_cosine(cau_hoi_vector, muc["vector"])
        if do_tuong_dong > nguong_cosine:
            return muc["tra_loi"]
    return None


def do_ty_le_cache_hit(danh_sach_cau_hoi_vector, cache, nguong_cosine):
    so_hit = 0
    for v in danh_sach_cau_hoi_vector:
        if tim_trong_cache(v, cache, nguong_cosine) is not None:
            so_hit += 1
    return so_hit / len(danh_sach_cau_hoi_vector)


CACHE = [
    {"cau_hoi": "ao thun mau xanh gia bao nhieu", "vector": [1.0, 0.0, 0.0], "tra_loi": "Ao thun xanh gia 150000 dong"},
    {"cau_hoi": "chinh sach doi tra trong bao lau", "vector": [0.0, 1.0, 0.0], "tra_loi": "Doi tra trong vong 7 ngay"},
    {"cau_hoi": "lam sao lien he cham soc khach hang", "vector": [0.0, 0.0, 1.0], "tra_loi": "Goi hotline 1900 xxxx"},
]

CAC_VECTOR_MOI = [
    [0.99, 0.05, 0.0],
    [0.02, 0.98, 0.0],
    [0.3, 0.3, 0.3],
    [0.0, 0.02, 0.99],
    [0.5, 0.5, 0.0],
]

KET_QUA_TIM = [tim_trong_cache(v, CACHE, 0.95) for v in CAC_VECTOR_MOI]
TY_LE_HIT = do_ty_le_cache_hit(CAC_VECTOR_MOI, CACHE, 0.95)

print(KET_QUA_TIM)
print(TY_LE_HIT)
```

```python title=test
assert KET_QUA_TIM == [
    "Ao thun xanh gia 150000 dong", "Doi tra trong vong 7 ngay", None,
    "Goi hotline 1900 xxxx", None,
], f"KET_QUA_TIM sai -- dang ra {KET_QUA_TIM}"
assert TY_LE_HIT == 0.6, f"TY_LE_HIT phai la 0.6 (3 hit tren 5 cau) -- dang ra {TY_LE_HIT}"

# bien: nguong_cosine THAT SU rang buoc ket qua
KET_QUA_TIM_07 = [tim_trong_cache(v, CACHE, 0.7) for v in CAC_VECTOR_MOI]
TY_LE_HIT_07 = do_ty_le_cache_hit(CAC_VECTOR_MOI, CACHE, 0.7)
assert KET_QUA_TIM_07[4] == "Ao thun xanh gia 150000 dong", f"ha nguong xuong 0.7 phai lam cau thu 5 thanh HIT -- dang ra {KET_QUA_TIM_07[4]}"
assert TY_LE_HIT_07 == 0.8, f"ha nguong xuong 0.7 phai cho ty le 0.8 (4/5) -- dang ra {TY_LE_HIT_07}"
assert TY_LE_HIT_07 != TY_LE_HIT, "doi nguong_cosine phai doi TY_LE_HIT"

# kiem tra truc tiep tim_trong_cache tren cache RONG -- phai la None, khong loi
assert tim_trong_cache([1.0, 0.0, 0.0], [], 0.95) is None, "cache rong phai tra ve None"

# kiem tra truc tiep: cau hoi TRUNG KHOP TUYET DOI voi mot muc da co (cosine = 1.0)
assert tim_trong_cache([1.0, 0.0, 0.0], CACHE, 0.95) == "Ao thun xanh gia 150000 dong", "cau hoi trung khop tuyet doi voi mot muc phai HIT"

# bien: mot cau hoi duy nhat, hit tuyet doi -> ty le phai la 1.0
assert do_ty_le_cache_hit([[1.0, 0.0, 0.0]], CACHE, 0.95) == 1.0, "mot cau hoi hit tuyet doi phai cho ty le 1.0"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `tim_trong_cache`) là ĐIỀU KIỆN của `if` — so sánh độ tương đồng vừa tính (`do_tuong_dong`) với ngưỡng. Chỗ hai (trong `do_ty_le_cache_hit`) là GIÁ TRỊ TRẢ VỀ CUỐI CÙNG — một phép CHIA.
- kind: strategy
  body: 'Chỗ đầu: `do_tuong_dong > nguong_cosine` — dùng `>` NGHIÊM NGẶT (không phải `>=`), đúng cách `Chương 39` Bài tập `2` viết "cosine > 0,95". Chỗ hai: `so_hit / len(danh_sach_cau_hoi_vector)` — số lần hit chia cho TỔNG số câu hỏi đã hỏi, cho ra một tỉ lệ giữa `0` và `1`.'
- kind: one-line
  body: 'Chỗ đầu là `do_tuong_dong > nguong_cosine`, chỗ hai là `so_hit / len(danh_sach_cau_hoi_vector)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la SO SANH do_tuong_dong > nguong_cosine (dung > nghiem ngat, khong duoc dung >= hay chep san True/False); cho trong hai phai la PHEP CHIA so_hit / len(danh_sach_cau_hoi_vector) (khong duoc chep san mot con so co dinh)
  requireAst:
  - kind: uses-operator, target: ">", min: 1
  - kind: uses-operator, target: "/", min: 2
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # CHINH XAC khoi solution cua file nay) -- xac nhan DUNG CHINH XAC (min
  # VA min+1): ">"=1, "/"=2.
  # ">"=1: DUY NHAT o cho trong dau -- khong noi nao khac trong solution
  # dung phep >.
  # "/"=2 (TONG THAT, khong phai boilerplate lon): 1 lan trong
  # tuong_dong_cosine (tich_vo_huong(v1, v2) / (d1 * d2), ham tai dung tu
  # q8.5a/b), CONG 1 lan o cho trong hai (so_hit / len(...)). Khong noi nao
  # khac trong solution dung phep /.
  # Dien bua "True" vao ca hai cho trong ("if True" va "return True") cho
  # ">"=0 VA "/"=1 (duoi 2, chi con phep chia trong tuong_dong_cosine) --
  # CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien "so_hit / len(danh_sach_cau_hoi_vector)"
  # vao cho trong dau ("if so_hit / len(danh_sach_cau_hoi_vector):" trong
  # tim_trong_cache) VA dien "do_tuong_dong > nguong_cosine" vao cho trong
  # hai ("return do_tuong_dong > nguong_cosine" trong do_ty_le_cache_hit) --
  # da CHAY THAT qua kiemAst(): tong so lan ">" VA tong so lan "/" tren
  # TOAN BO solution DEU KHONG DOI (van dung 1 va 2, chi doi VI TRI) --
  # static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong tim_trong_cache (tham so la
  # cau_hoi_vector, cache, nguong_cosine -- KHONG CO "so_hit"/
  # "danh_sach_cau_hoi_vector" nao trong scope nay), bieu thuc moi doc CA
  # HAI ten CHUA TON TAI -- da tu chay THAT qua python3, xac nhan NameError
  # "name 'so_hit' is not defined" ngay khi tim_trong_cache duoc goi lan
  # dau (tu chinh do_ty_le_cache_hit). Ben trong do_ty_le_cache_hit (KHONG
  # CO bien don le "do_tuong_dong" nao trong scope nay, chi co "do_tuong_dong"
  # LA BIEN CUC BO cua tim_trong_cache, khac ham), bieu thuc moi "return
  # do_tuong_dong > nguong_cosine" doc ten "do_tuong_dong" CHUA TON TAI --
  # nhung vi tim_trong_cache (goi TRUOC do_ty_le_cache_hit chay toi dong
  # return) da nem NameError truoc, loi thuc te quan sat duoc LA NameError
  # ve "so_hit", khong phai ve "do_tuong_dong" -- da tu chay THAT xac nhan
  # ca hai nhanh deu la loi runtime, doc lap voi static.
  # Da tu ra soat GOTCHA #6: "so_hit" va "do_tuong_dong" la hai bien CUC BO
  # RIENG cua hai ham khac nhau, khong trung ten voi tham so nao cua ham
  # kia -- khong co rui ro nham lan tinh co ve HINH DANG.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\['Ao thun xanh gia 150000 dong', 'Doi tra trong vong 7 ngay', None, 'Goi hotline 1900 xxxx', None\\]\\n0\\.6\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0,6` — đúng `3` trên `5` câu hỏi được trả lời NGAY, không hề chạm tới LLM.
Bài tiếp theo chuyển sang Bài tập `3` của `Chương 39`: khi nhiều request
cùng chờ, ai được phục vụ trước — người trả phí, hay người xếp hàng lâu
nhất?
::::

::::reflect{#nghi-lai}
`tim_trong_cache` không "hiểu" ngôn ngữ tự nhiên — nó chỉ so hai vector
bằng cosine, đúng cách `q8.5a`/`q8.5b` đã dạy. Cái MỚI ở bài này là VỊ TRÍ
của phép so sánh đó trong một hệ thống lớn hơn: `Chương 39` Bài tập `2` đặt
nó NGAY TRƯỚC bước gọi LLM — một bước rẻ (vài phép cộng/nhân) chặn trước
một bước đắt (giây, xu tiền). Đo được cụ thể: `3/5 = 0,6` — không phải
mọi câu hỏi lặp lại đều được cache bắt (câu `3` khác hẳn, đúng phải gọi
LLM thật), và không phải ngưỡng nào cũng cho cùng tỉ lệ — hạ `nguong_cosine`
từ `0,95` xuống `0,7` đẩy tỉ lệ lên `0,8`, đổi THẬT sự cân bằng giữa "tiết
kiệm" và "trả lời sai vì tưởng giống mà không giống". Bài tiếp theo giữ
nguyên tinh thần "quyết định TRƯỚC khi tốn tài nguyên", áp dụng cho một bài
toán khác: request nào được xử lý TRƯỚC khi GPU rảnh ra.
::::

::::checkpoint{mastery=0.78}
::::
