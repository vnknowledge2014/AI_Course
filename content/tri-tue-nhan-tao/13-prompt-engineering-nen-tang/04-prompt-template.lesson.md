---
id: tri-tue-nhan-tao.prompt-engineering-nen-tang.prompt-template
title: "Prompt template: khuôn mẫu có tham số, tái dùng được"
summary: "Dong goi cau hoi 'X co Y. Hoi X co tat ca bao nhieu qua?' thanh MOT chuoi khuon mau MAU_PROMPT co hai tham so {ten}/{mo_ta}, dien bang .format(). Ham tao_tin_nhan(ten, mo_ta, co_rang_buoc) sinh danh sach message DAY DU (kem hoac khong kem message he thong rang buoc) tu DUNG mot khuon, khong viet lai cau van moi lan. Chay qua LLM mo phong cua bai truoc: cung khuon, khac tham so, cho hai ket qua khac nhau -- khong rang buoc ra 'Tong la 5, nhung ban nen kiem tra lai.', co rang buoc ra '7' -- ca hai deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: prompt-engineering-nen-tang
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.prompt-template]
requires: [ai.rang-buoc-ro-rang]
concepts: [ai.prompt-template]
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

::::byte{trigger=enter mood=curious pose=point-stage}
Ba bài trước, mỗi câu hỏi được gõ tay từng chữ. Bài này đóng gói CẤU TRÚC
của câu hỏi (và cả ràng buộc kèm theo) vào MỘT khuôn có chỗ trống — điền
tên và số liệu khác nhau vào cùng một khuôn, không viết lại câu văn.
::::

::::explain{#prompt_template_la_gi}
Ba bài trước đều dùng những câu hỏi gõ tay, mỗi câu một chuỗi độc lập:
`"Lan co 3 qua tao..."`, `"Mai co 2 qua tao..."`, `"Tuan co 1 qua tao..."`.
Cấu trúc CÂU thì giống hệt nhau — chỉ tên người và danh sách số lượng khác
— nhưng phải gõ lại toàn bộ câu mỗi lần.

Một **prompt template** giải quyết đúng việc lặp lại đó: một chuỗi CỐ
ĐỊNH có vài **chỗ trống được đặt tên** (trong Python, cú pháp
`"{ten_tham_so}"` của `str.format`), và một hàm điền giá trị cụ thể vào
từng chỗ trống đó. Khuôn không đổi; chỉ tham số đổi.

```
MAU_PROMPT = "{ten} co {mo_ta}. Hoi {ten} co tat ca bao nhieu qua?"
```

Gọi `MAU_PROMPT.format(ten="Mai", mo_ta="2 qua tao va 3 qua cam")` cho ra
đúng câu `"Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu
qua?"` — không cần viết lại cấu trúc câu, chỉ cần cung cấp hai giá trị.

Lợi ích không chỉ là gõ ít hơn. Một khuôn CỐ ĐỊNH nghĩa là mọi câu hỏi
sinh ra từ nó đều có CÙNG cấu trúc — dễ kiểm tra, dễ thêm ràng buộc kèm
theo một cách NHẤT QUÁN (bài này ghép luôn message ràng buộc từ bài
trước vào hàm dựng message, có tuỳ chọn bật/tắt), và dễ tái sử dụng cho
hàng loạt câu hỏi khác nhau mà không phải sửa từng câu một.
::::

::::example{#dung_lai_mau_cho_nhieu_cau_hoi}
Một khuôn, hai bộ tham số khác nhau, có và không có ràng buộc:

```python title=readonly
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_dem_tong_co_rang_buoc(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    tong = sum(so_list)
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


MAU_PROMPT = "{ten} co {mo_ta}. Hoi {ten} co tat ca bao nhieu qua?"
CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."


def dien_mau(ten, mo_ta):
    return MAU_PROMPT.format(ten=ten, mo_ta=mo_ta)


def tao_tin_nhan(ten, mo_ta, co_rang_buoc=False):
    noi_dung = dien_mau(ten, mo_ta)
    tin_nhan = []
    if co_rang_buoc:
        tin_nhan.append({"role": "system", "content": CAU_RANG_BUOC})
    tin_nhan.append({"role": "user", "content": noi_dung})
    return tin_nhan


tin_nhan_mai = tao_tin_nhan("Mai", "2 qua tao va 3 qua cam")
tin_nhan_tuan = tao_tin_nhan("Tuan", "1 qua tao, 2 qua cam va 4 qua le", co_rang_buoc=True)

print(dien_mau("Mai", "2 qua tao va 3 qua cam"))
print(tin_nhan_mai)
print(llm_dem_tong_co_rang_buoc(tin_nhan_mai))
print(llm_dem_tong_co_rang_buoc(tin_nhan_tuan))
```

```text title=readonly
Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?
[{'role': 'user', 'content': 'Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?'}]
Tong la 5, nhung ban nen kiem tra lai.
7
```

`dien_mau("Mai", "2 qua tao va 3 qua cam")` tái tạo ĐÚNG câu hỏi đã gõ tay
ở bài trước — cùng khuôn, khác tham số cho `"Tuan"` với `co_rang_buoc=True`
gắn thêm một message `system` chứa `CAU_RANG_BUOC`, và câu trả lời sụp về
`"7"` — đúng số trần trụi, đúng như bài trước đã đo với ràng buộc.
::::

::::predict{#doan_them_tham_so commitOnce}
`tao_tin_nhan(ten, mo_ta, co_rang_buoc=False)` luôn thêm message `user`
chứa câu hỏi đã điền khuôn, và CHỈ thêm message `system` khi
`co_rang_buoc=True`.

**Trước khi đọc lại code**, bạn đoán: `len(tao_tin_nhan("An", "5 qua
xoai"))` (gọi KHÔNG truyền `co_rang_buoc`, dùng giá trị mặc định) bằng
bao nhiêu?

:::opt{correct}
`1` — vì `co_rang_buoc` mặc định là `False`, nhánh `if co_rang_buoc:`
không chạy, nên `tin_nhan` chỉ có ĐÚNG một phần tử được `append` — message
`user` chứa câu hỏi đã điền khuôn
:::

:::opt
`2` — vì hàm luôn thêm cả message hệ thống lẫn message câu hỏi, bất kể
tham số `co_rang_buoc`
::why
Gần đúng ở việc bạn nhớ đúng: hàm THẬT SỰ CÓ khả năng trả về danh sách
`2` phần tử — đúng khi gọi với `co_rang_buoc=True` (như `tin_nhan_tuan`
trong ví dụ).

Chỗ lệch: câu hỏi này gọi hàm KHÔNG truyền `co_rang_buoc`, nên tham số lấy
giá trị MẶC ĐỊNH khai trong chữ ký hàm — `co_rang_buoc=False`. Nhánh
`if co_rang_buoc:` chỉ chạy khi giá trị đó là `True`; mặc định `False`
nghĩa là nhánh đó bị bỏ qua hoàn toàn, danh sách chỉ còn `1` phần tử.
::
:::

:::opt
Không xác định được — phụ thuộc vào độ dài của `mo_ta`
::why
Gần đúng ở việc bạn để ý `mo_ta` LÀ một tham số ảnh hưởng tới NỘI DUNG câu
hỏi (`ten`/`mo_ta` được điền vào `MAU_PROMPT`).

Chỗ lệch: độ dài `mo_ta` chỉ ảnh hưởng NỘI DUNG chuỗi bên trong MỘT
message `user` — nó không thêm hay bớt SỐ LƯỢNG message trong danh sách.
Số lượng message chỉ phụ thuộc `co_rang_buoc` (một `if` duy nhất, không
liên quan gì tới độ dài chuỗi).
::
:::
::::

::::code{#viet_dien_mau_va_tao_tin_nhan}
Hoàn thiện `dien_mau` (điền khuôn bằng `.format`) và `tao_tin_nhan` (thêm
message hệ thống bằng `.append` khi có ràng buộc).

```python title=starter
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_dem_tong_co_rang_buoc(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    tong = sum(so_list)
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


MAU_PROMPT = "{ten} co {mo_ta}. Hoi {ten} co tat ca bao nhieu qua?"
CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."


def dien_mau(ten, mo_ta):
    return MAU_PROMPT.___(ten=ten, mo_ta=mo_ta)          # format


def tao_tin_nhan(ten, mo_ta, co_rang_buoc=False):
    noi_dung = dien_mau(ten, mo_ta)
    tin_nhan = []
    if co_rang_buoc:
        tin_nhan.___({"role": "system", "content": CAU_RANG_BUOC})   # append
    tin_nhan.append({"role": "user", "content": noi_dung})
    return tin_nhan


tin_nhan_mai = tao_tin_nhan("Mai", "2 qua tao va 3 qua cam")
tin_nhan_tuan = tao_tin_nhan("Tuan", "1 qua tao, 2 qua cam va 4 qua le", co_rang_buoc=True)

print(dien_mau("Mai", "2 qua tao va 3 qua cam"))
print(tin_nhan_mai)
print(llm_dem_tong_co_rang_buoc(tin_nhan_mai))
print(llm_dem_tong_co_rang_buoc(tin_nhan_tuan))
```

```python title=solution
import re


def trich_so(text):
    return [int(x) for x in re.findall(r"\d+", text)]


def lay_cau_hoi_cuoi(messages):
    cau_hoi = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
    return cau_hoi


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_dem_tong_co_rang_buoc(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    tong = sum(so_list)
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


MAU_PROMPT = "{ten} co {mo_ta}. Hoi {ten} co tat ca bao nhieu qua?"
CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."


def dien_mau(ten, mo_ta):
    return MAU_PROMPT.format(ten=ten, mo_ta=mo_ta)


def tao_tin_nhan(ten, mo_ta, co_rang_buoc=False):
    noi_dung = dien_mau(ten, mo_ta)
    tin_nhan = []
    if co_rang_buoc:
        tin_nhan.append({"role": "system", "content": CAU_RANG_BUOC})
    tin_nhan.append({"role": "user", "content": noi_dung})
    return tin_nhan


tin_nhan_mai = tao_tin_nhan("Mai", "2 qua tao va 3 qua cam")
tin_nhan_tuan = tao_tin_nhan("Tuan", "1 qua tao, 2 qua cam va 4 qua le", co_rang_buoc=True)

print(dien_mau("Mai", "2 qua tao va 3 qua cam"))
print(tin_nhan_mai)
print(llm_dem_tong_co_rang_buoc(tin_nhan_mai))
print(llm_dem_tong_co_rang_buoc(tin_nhan_tuan))
```

```python title=test
assert dien_mau("Mai", "2 qua tao va 3 qua cam") == "Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?", f"dien_mau sai -- dang ra {dien_mau('Mai', '2 qua tao va 3 qua cam')!r}"
assert len(tin_nhan_mai) == 1, f"khong rang buoc phai chi co 1 message -- dang ra {len(tin_nhan_mai)}"
assert len(tin_nhan_tuan) == 2, f"co rang buoc phai co 2 message -- dang ra {len(tin_nhan_tuan)}"
assert tin_nhan_tuan[0]["role"] == "system", f"message dau khi co rang buoc phai la role=system -- dang ra {tin_nhan_tuan[0]['role']!r}"
assert tin_nhan_tuan[0]["content"] == CAU_RANG_BUOC, "noi dung message he thong phai dung CAU_RANG_BUOC"
assert llm_dem_tong_co_rang_buoc(tin_nhan_mai) == "Tong la 5, nhung ban nen kiem tra lai.", f"cau Mai khong rang buoc sai -- dang ra {llm_dem_tong_co_rang_buoc(tin_nhan_mai)!r}"
assert llm_dem_tong_co_rang_buoc(tin_nhan_tuan) == "7", f"cau Tuan co rang buoc phai la '7' -- dang ra {llm_dem_tong_co_rang_buoc(tin_nhan_tuan)!r}"

# bien: goi tao_tin_nhan KHONG truyen co_rang_buoc -- phai dung mac dinh
# False, danh sach chi co 1 message
tin_nhan_mac_dinh = tao_tin_nhan("An", "5 qua xoai")
assert len(tin_nhan_mac_dinh) == 1, f"mac dinh khong truyen co_rang_buoc phai la 1 message -- dang ra {len(tin_nhan_mac_dinh)}"
assert tin_nhan_mac_dinh[0]["role"] == "user", "message duy nhat mac dinh phai la role=user"

# bien: khuon phai dung LAI dung ten o CA HAI cho -- kiem tra bang mot bo
# tham so khac hoan toan
assert dien_mau("Vy", "9 qua xoai") == "Vy co 9 qua xoai. Hoi Vy co tat ca bao nhieu qua?", f"dien_mau voi tham so khac sai -- dang ra {dien_mau('Vy', '9 qua xoai')!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu điền khuôn `MAU_PROMPT` bằng đúng hai tham số `ten`/`mo_ta` — dùng phương thức chuỗi chuyên điền theo tên tham số (đã dùng ở bài trước cho `CAC_CAU_TRA_LOI_MO_HO`). Chỗ hai thêm MỘT phần tử vào cuối danh sách `tin_nhan` — dùng phương thức danh sách chuyên thêm một phần tử.
- kind: strategy
  body: 'Chỗ đầu: `format` (cho dòng `MAU_PROMPT.format(ten=ten, mo_ta=mo_ta)`). Chỗ hai: `append` (cho dòng `tin_nhan.append({"role": "system", "content": CAU_RANG_BUOC})`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `format` và `append`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: dien_mau phai dung ".format(...)" de dien khuon (khong duoc noi chuoi thu cong), VA tao_tin_nhan phai dung ".append(...)" de them message he thong (khong duoc gan truc tiep vao danh sach bang chi so hay tao lai danh sach moi)
  requireAst:
  - kind: uses-call, target: format, min: 2
  - kind: uses-call, target: append, min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # format=2: MOT lan CO SAN trong llm_dem_tong_co_rang_buoc (boilerplate
  # tu bai truoc, ham nay van duoc COPY LAI de tu chua), MOT lan la cho
  # trong 1. Neu chi dat min=1 (ngay tho), mot mutant xoa bo cho trong 1
  # (vi du dien_mau tra ve MAU_PROMPT khong dien gi) VAN qua duoc vi con
  # lai 1 lan format trong boilerplate -- day la day du GOTCHA
  # "boilerplate-threshold-masking"; dat dung min=2 (tong THAT) moi chan
  # duoc mutant nay -- va no cung bi chan boi tests (dien_mau se tra ve
  # chuoi chua "{ten}"/"{mo_ta}" chua dien, khac han ky vong).
  # append=2: MOT lan CO SAN (dong tin_nhan.append({"role": "user", ...})
  # luon chay, khong bi cho trong), MOT lan la cho trong 2. Dien bua cho
  # trong 2 bang gan truc tiep "tin_nhan = [{"role": "system", ...}]" (ghi
  # de danh sach thay vi them vao) lam append=1 -- duoi nguong min=2, bi
  # chan; dong thoi bi chan boi tests vi thu tu/so luong message se sai
  # (message user bi mat).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^Mai co 2 qua tao va 3 qua cam\\. Hoi Mai co tat ca bao nhieu qua\\?\\n\\[\\{'role': 'user', 'content': 'Mai co 2 qua tao va 3 qua cam\\. Hoi Mai co tat ca bao nhieu qua\\?'\\}\\]\\nTong la 5, nhung ban nen kiem tra lai\\.\\n7\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một khuôn, hai bộ tham số, hai kết quả tái tạo đúng những gì đã đo thủ
công ở bài trước. Bài sau dùng khuôn này để chạy MỘT thực nghiệm A/B thật
sự: so sánh HAI phiên bản prompt trên CÙNG một bộ câu hỏi, và đo chênh
lệch bằng số.
::::

::::reflect{#nghi-lai}
Một prompt template không chỉ tiết kiệm việc gõ phím — nó biến "viết một
câu hỏi" thành "gọi một hàm với tham số", và điều đó mở khoá một việc mà
ba bài trước chưa làm được: chạy CÙNG một cấu trúc prompt trên NHIỀU đầu
vào khác nhau một cách có hệ thống, rồi đo kết quả hàng loạt thay vì đo
từng câu một. Bài sau dùng đúng khả năng đó: một khuôn, hai PHIÊN BẢN
prompt (có và không có toàn bộ kỹ thuật đã học), chạy trên một bộ câu hỏi
CỐ ĐỊNH, và đếm bằng số xem phiên bản nào cho nhiều câu trả lời đúng hơn.
::::

::::checkpoint{mastery=0.85}
::::
