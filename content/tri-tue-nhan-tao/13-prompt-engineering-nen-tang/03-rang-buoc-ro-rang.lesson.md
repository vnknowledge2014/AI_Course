---
id: tri-tue-nhan-tao.prompt-engineering-nen-tang.rang-buoc-ro-rang
title: "Ràng buộc rõ ràng: thu hẹp không gian câu trả lời"
summary: "LLM mo phong tinh tong so luong trong mot cau hoi, nhung LAN NAY truc do la DINH DANG cau tra loi, khong phai gia tri so. Khong co rang buoc: chon MOT trong 3 cach dien dat mo ho co san, chon theo do dai danh sach so (vi du 2 so -> 'Tong la 5, nhung ban nen kiem tra lai.'; 3 so -> 'Toi nghi tong khoang 7.'). Co dung mot cau rang buoc tuong minh (system hoac user chua 'chi tra loi bang mot con so'): CA HAI cau hoi cung sup ve DUNG MOT dinh dang -- chuoi so nguyen tran, '5' va '7'. Bon con so nay deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: prompt-engineering-nen-tang
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.rang-buoc-ro-rang]
requires: [ai.zero-shot-va-few-shot]
concepts: [ai.rang-buoc-ro-rang]
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

::::byte{trigger=enter mood=curious pose=point-editor}
Bài trước đo được GIÁ TRỊ số đúng hay sai. Bài này đo một trục khác hẳn:
ĐỊNH DẠNG của câu trả lời. Không chỉ thị định dạng, LLM mô phỏng trả lời
bằng một trong vài cách diễn đạt khác nhau — mơ hồ. Thêm một câu chỉ thị
tường minh, và mọi câu hỏi sụp về ĐÚNG MỘT định dạng.
::::

::::explain{#rang_buoc_thu_hep_dinh_dang}
Một chỉ thị MƠ HỒ (ví dụ chỉ hỏi "tổng là bao nhiêu?") không nói rõ câu
trả lời phải trông NHƯ THẾ NÀO — có thể là một câu đầy đủ, một số kèm lời
rào đón, hay một con số trần trụi. Một LLM thật, không được yêu cầu định
dạng cụ thể, có thể trả lời theo bất kỳ cách nào trong số đó — và với một
pipeline cần PARSE câu trả lời bằng code (không phải người đọc bằng mắt),
đây là vấn đề thật: một hàm chờ đợi `"5"` sẽ vỡ nếu nhận về
`"Tôi nghĩ tổng khoảng 5."`.

**Ràng buộc rõ ràng** — một chỉ thị nêu ĐÚNG định dạng mong muốn, ví dụ
`"Chỉ trả lời bằng một con số nguyên, không kèm chữ nào khác."` — thu hẹp
không gian câu trả lời hợp lệ từ NHIỀU cách diễn đạt xuống còn MỘT.

LLM mô phỏng ở bài này tái hiện đúng hai thái cực đó bằng luật cố định:

> **Không có ràng buộc tường minh** — chọn MỘT trong `3` cách diễn đạt cố
> định đã liệt kê sẵn (`CAC_CAU_TRA_LOI_MO_HO`), chọn cách nào phụ thuộc
> vào SỐ LƯỢNG đại lượng tìm thấy trong câu hỏi (một quy luật xác định,
> không phải chọn ngẫu nhiên — nhưng người gọi không cách nào ĐOÁN TRƯỚC
> câu trả lời sẽ ở định dạng nào, trừ khi biết trước quy luật đó).
>
> **Có ràng buộc tường minh** (một chuỗi cụ thể xuất hiện ở bất kỳ message
> nào, `system` hay `user`) — trả về ĐÚNG một định dạng: chuỗi số nguyên
> trần trụi, ví dụ `"5"`.

Đây là cách đo MÁY MÓC được hiệu ứng "ràng buộc thu hẹp không gian câu trả
lời" — không cần một LLM thật, chỉ cần một hàm tất định mô phỏng đúng hai
thái cực của nó.
::::

::::example{#do_rang_buoc}
Hai câu hỏi KHÁC nhau (một có `2` đại lượng, một có `3`), mỗi câu hỏi
thử với và không có ràng buộc tường minh:

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


cau_hoi_1 = "Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?"
cau_hoi_2 = "Tuan co 1 qua tao, 2 qua cam va 4 qua le. Hoi Tuan co tat ca bao nhieu qua?"

messages_mo_ho_1 = [{"role": "user", "content": cau_hoi_1}]
messages_mo_ho_2 = [{"role": "user", "content": cau_hoi_2}]

messages_rang_buoc_1 = [
    {"role": "system", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_1},
]
messages_rang_buoc_2 = [
    {"role": "system", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_2},
]

print(llm_dem_tong_co_rang_buoc(messages_mo_ho_1))
print(llm_dem_tong_co_rang_buoc(messages_mo_ho_2))
print(llm_dem_tong_co_rang_buoc(messages_rang_buoc_1))
print(llm_dem_tong_co_rang_buoc(messages_rang_buoc_2))
```

```text title=readonly
Tong la 5, nhung ban nen kiem tra lai.
Toi nghi tong khoang 7.
5
7
```

Không có ràng buộc: hai câu hỏi (`2` và `3` đại lượng) rơi vào HAI cách
diễn đạt KHÁC NHAU trong danh sách `CAC_CAU_TRA_LOI_MO_HO` — không đoán
trước được định dạng nếu không biết luật. Có ràng buộc: CẢ HAI câu hỏi
sụp về ĐÚNG MỘT định dạng — chuỗi số trần trụi (`"5"`, `"7"`) — dù nội
dung câu hỏi khác hẳn nhau.
::::

::::predict{#doan_rang_buoc_vi_tri commitOnce}
`co_rang_buoc_ro_rang` quét TẤT CẢ message trong danh sách, bất kể
`role`, tìm chuỗi `"chi tra loi bang mot con so"`.

**Trước khi đọc lại code**, bạn đoán: nếu đặt câu ràng buộc đó vào một
message `role="user"` ĐỨNG TRƯỚC câu hỏi thật (thay vì `role="system"`),
kết quả có còn là chuỗi số trần trụi không?

:::opt{correct}
Có — `co_rang_buoc_ro_rang` chỉ kiểm tra `tin["content"]` của MỌI message
trong danh sách, không lọc theo `tin["role"]`; câu ràng buộc nằm ở message
nào, vai trò nào, cũng được phát hiện như nhau
:::

:::opt
Không — chỉ ràng buộc đặt ở `role="system"` mới có hiệu lực, giống bài
trước (persona chỉ đọc từ `system`)
::why
Gần đúng ở việc NHỚ ĐÚNG bài trước: `llm_mo_phong_don_gian` (bài `1`) THẬT
SỰ chỉ đọc persona từ message `role="system"` — đó là một cơ chế có thật.

Chỗ lệch: không phải MỌI hàm mô phỏng trong track này đều lọc theo `role`
giống nhau — mỗi hàm có luật riêng, và bài học nằm ở việc ĐỌC ĐÚNG luật
của hàm đang xét, không suy diễn từ hàm khác. `co_rang_buoc_ro_rang` ở
bài này duyệt `for tin in messages` rồi kiểm `tin["content"]` mà không hề
chạm tới `tin["role"]` — ràng buộc có hiệu lực dù nằm ở `system` hay
`user`.
::
:::

:::opt
Không xác định được nếu không chạy thử — logic này quá phức tạp để đọc
trước
::why
Gần đúng ở tinh thần thận trọng — chạy thử để xác nhận luôn là bước cuối
cùng đúng đắn, và bài học này cũng sẽ xác nhận lại bằng code thật.

Chỗ lệch: `co_rang_buoc_ro_rang` chỉ có `4` dòng, không có nhánh rẽ nào
phụ thuộc `role` — đọc thẳng phần thân hàm đã đủ để trả lời trước khi
chạy, không cần đoán mò.
::
:::
::::

::::code{#viet_llm_dem_tong_co_rang_buoc}
Hoàn thiện `llm_dem_tong_co_rang_buoc`: khi KHÔNG có ràng buộc tường minh,
chọn một cách diễn đạt bằng phép CHIA LẤY DƯ trên số lượng đại lượng, rồi
điền số vào đúng vị trí `{tong}` của cách diễn đạt đó.

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
    chi_so = len(so_list) ___ len(CAC_CAU_TRA_LOI_MO_HO)     # %
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].___(tong=tong)       # format


cau_hoi_1 = "Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?"
cau_hoi_2 = "Tuan co 1 qua tao, 2 qua cam va 4 qua le. Hoi Tuan co tat ca bao nhieu qua?"

messages_mo_ho_1 = [{"role": "user", "content": cau_hoi_1}]
messages_mo_ho_2 = [{"role": "user", "content": cau_hoi_2}]

messages_rang_buoc_1 = [
    {"role": "system", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_1},
]
messages_rang_buoc_2 = [
    {"role": "system", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_2},
]

print(llm_dem_tong_co_rang_buoc(messages_mo_ho_1))
print(llm_dem_tong_co_rang_buoc(messages_mo_ho_2))
print(llm_dem_tong_co_rang_buoc(messages_rang_buoc_1))
print(llm_dem_tong_co_rang_buoc(messages_rang_buoc_2))
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


cau_hoi_1 = "Mai co 2 qua tao va 3 qua cam. Hoi Mai co tat ca bao nhieu qua?"
cau_hoi_2 = "Tuan co 1 qua tao, 2 qua cam va 4 qua le. Hoi Tuan co tat ca bao nhieu qua?"

messages_mo_ho_1 = [{"role": "user", "content": cau_hoi_1}]
messages_mo_ho_2 = [{"role": "user", "content": cau_hoi_2}]

messages_rang_buoc_1 = [
    {"role": "system", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_1},
]
messages_rang_buoc_2 = [
    {"role": "system", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_2},
]

print(llm_dem_tong_co_rang_buoc(messages_mo_ho_1))
print(llm_dem_tong_co_rang_buoc(messages_mo_ho_2))
print(llm_dem_tong_co_rang_buoc(messages_rang_buoc_1))
print(llm_dem_tong_co_rang_buoc(messages_rang_buoc_2))
```

```python title=test
assert llm_dem_tong_co_rang_buoc(messages_mo_ho_1) == "Tong la 5, nhung ban nen kiem tra lai.", f"cau hoi 1 khong rang buoc sai -- dang ra {llm_dem_tong_co_rang_buoc(messages_mo_ho_1)!r}"
assert llm_dem_tong_co_rang_buoc(messages_mo_ho_2) == "Toi nghi tong khoang 7.", f"cau hoi 2 khong rang buoc sai -- dang ra {llm_dem_tong_co_rang_buoc(messages_mo_ho_2)!r}"
assert llm_dem_tong_co_rang_buoc(messages_rang_buoc_1) == "5", f"cau hoi 1 co rang buoc phai la '5' -- dang ra {llm_dem_tong_co_rang_buoc(messages_rang_buoc_1)!r}"
assert llm_dem_tong_co_rang_buoc(messages_rang_buoc_2) == "7", f"cau hoi 2 co rang buoc phai la '7' -- dang ra {llm_dem_tong_co_rang_buoc(messages_rang_buoc_2)!r}"

# bien: rang buoc dat o role="user" (khong phai "system") van phai co
# hieu luc -- ham nay khong loc theo role
messages_rang_buoc_user = [
    {"role": "user", "content": "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."},
    {"role": "user", "content": cau_hoi_1},
]
assert llm_dem_tong_co_rang_buoc(messages_rang_buoc_user) == "5", "rang buoc o role=user van phai co hieu luc"

# bien: mot cau hoi co DUNG 3 dai luong khac (Constant khong doi) -- xac
# nhan chi_so tinh dung bang % chu khong phai mot hang so co dinh
cau_hoi_3 = "Vy co 1 qua tao, 1 qua cam va 1 qua le. Hoi Vy co tat ca bao nhieu qua?"
messages_mo_ho_3 = [{"role": "user", "content": cau_hoi_3}]
assert llm_dem_tong_co_rang_buoc(messages_mo_ho_3) == "Toi nghi tong khoang 3.", f"cau hoi 3 dai luong khac phai cung dinh dang voi cau hoi 2 -- dang ra {llm_dem_tong_co_rang_buoc(messages_mo_ho_3)!r}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu tính `chi_so` bằng phép CHIA LẤY DƯ (`%`) giữa số lượng đại lượng (`len(so_list)`) và số cách diễn đạt có sẵn (`len(CAC_CAU_TRA_LOI_MO_HO)`) — đảm bảo `chi_so` luôn là một chỉ số HỢP LỆ trong danh sách. Chỗ hai điền `tong` vào đúng vị trí `{tong}` của chuỗi mẫu đã chọn — dùng phương thức chuỗi chuyên điền theo tên tham số.
- kind: strategy
  body: 'Chỗ đầu: `%` (cho dòng `chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)`). Chỗ hai: `format` (cho dòng `CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `%` và `format`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: chi_so phai duoc tinh bang phep CHIA LAY DU "%" (khong duoc dung mot hang so co dinh hay toan tu khac), VA cau tra loi mo ho phai duoc dien bang ".format(tong=tong)" (khong duoc noi chuoi thu cong)
  requireAst:
  - kind: uses-operator, target: "%", min: 1
  - kind: uses-call, target: format, min: 1
  # Da thu that (goi _dem tren code trich tu solution). "%"=1: DUY NHAT
  # mot lan, chinh la cho trong 1. Dien bua "chi_so = 0" (hang so co dinh)
  # lam "%"=0 -- duoi nguong min=1, bi chan; dong thoi bi chan boi tests vi
  # ca hai cau hoi mo ho se cho CUNG mot cach dien dat (chi_so=0 luon luon)
  # thay vi hai cach khac nhau nhu ky vong.
  # uses-call "format"=1: DUY NHAT mot lan, chinh la cho trong 2. Dien bua
  # bang noi chuoi thu cong (vi du f-string) lam "format"=0 -- duoi nguong
  # min=1, bi chan boi static (du output co the van dung neu noi dung
  # dung dinh dang).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^Tong la 5, nhung ban nen kiem tra lai\\.\\nToi nghi tong khoang 7\\.\\n5\\n7\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không ràng buộc: hai câu hỏi, hai cách diễn đạt khác nhau. Có ràng buộc:
cả hai sụp về đúng một định dạng số trần trụi. Bài sau đóng gói câu hỏi
VÀ ràng buộc này vào một KHUÔN MẪU có tham số — tái dùng được cho bất kỳ
tên người/danh sách đại lượng nào, không phải viết lại câu văn mỗi lần.
::::

::::reflect{#nghi-lai}
Ràng buộc rõ ràng không đổi GIÁ TRỊ số được tính — nó thu hẹp KHÔNG GIAN
ĐỊNH DẠNG của câu trả lời, từ "một trong nhiều cách diễn đạt hợp lý" xuống
"đúng một khuôn". Với một pipeline cần PARSE câu trả lời bằng code (không
phải một người đọc bằng mắt), sự thu hẹp đó không phải chi tiết thẩm mỹ —
nó là điều kiện để pipeline chạy được mà không cần đoán mọi biến thể diễn
đạt có thể xảy ra. Bài sau đóng gói cả câu hỏi lẫn ràng buộc vào một
CHUỖI KHUÔN MẪU có tham số, để tạo lại prompt này cho nhiều tình huống
khác nhau mà không cần viết lại thủ công.
::::

::::checkpoint{mastery=0.85}
::::
