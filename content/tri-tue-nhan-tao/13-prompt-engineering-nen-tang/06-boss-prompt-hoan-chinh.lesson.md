---
id: tri-tue-nhan-tao.prompt-engineering-nen-tang.boss-prompt-hoan-chinh
title: "BOSS — Ráp một prompt hoàn chỉnh, đo cải thiện so với prompt trần trụi"
summary: "Rap CA BON ky thuat cua quest nay thanh MOT prompt: vai tro message (bai 1, system rieng cho rang buoc) + rang buoc ro rang (bai 3, dinh dang so tran trui) + few-shot (bai 2, 2 vi du) + CoT (bai 5, cum 'hay suy nghi tung buoc'). Do tren CUNG bo 5 cau hoi kiem thu da dung o bai 5, tieu chi 'dung' la CA dinh dang (chuoi toan chu so) LAN gia tri (bang tong that). Prompt tran trui (chi cau hoi): 0/5. Prompt day du (ca bon ky thuat): 5/5. Cai thien +5/5, tu 0% len 100%, do that bang Python -- kem hai thi nghiem doi chung rieng le (chi rang buoc: 2/5; chi few-shot: 0/5) chung minh dinh dang va gia tri la HAI truc doc lap, can CA HAI ky thuat moi dat toi da."
locale: vi
track: tri-tue-nhan-tao
module: prompt-engineering-nen-tang
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [ai.boss-prompt-hoan-chinh]
requires: [ai.do-thuc-nghiem-anh-huong-prompt]
concepts: [ai.boss-prompt-hoan-chinh]
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
Năm bài: vai trò message, few-shot, ràng buộc rõ ràng, prompt template,
đo A/B với CoT. Bài này ráp TẤT CẢ lại thành MỘT prompt duy nhất — và đo
bằng số xem nó cải thiện được bao nhiêu so với một prompt trần trụi.
::::

::::explain{#rap_toan_bo_prompt}
Một prompt hoàn chỉnh gồm ĐÚNG những gì bốn bài trước đã xây, không thêm
gì mới:

> **Vai trò message** (`vai-tro-message`, bài `1`) — một message
> `role="system"` riêng biệt mang chỉ thị cấu hình, tách khỏi câu hỏi của
> người dùng.
>
> **Ràng buộc rõ ràng** (`rang-buoc-ro-rang`, bài `3`) — chỉ thị format
> cụ thể (`"Chi tra loi bang mot con so nguyen, khong kem chu nao
> khac."`) đặt trong message `system` đó, thu hẹp câu trả lời về ĐÚNG một
> định dạng: chuỗi số nguyên trần trụi.
>
> **Few-shot** (`zero-shot-va-few-shot`, bài `2`) — đúng `2` cặp message
> `user`/`assistant` mẫu, đặt TRƯỚC câu hỏi thật, đủ để vượt ngưỡng
> `so_vi_du >= 2`.
>
> **Chain-of-thought** (`do-thuc-nghiem-anh-huong-prompt`, bài `5`) —
> cụm `" Hay suy nghi tung buoc."` nối vào cuối câu hỏi thật.

Cả bốn kỹ thuật GHÉP lại vào MỘT hàm `llm_mo_phong_day_du`, dùng lại
NGUYÊN VẸN luật của từng bài: `dung_day_du` (đúng GIÁ TRỊ, quyết định bởi
few-shot HOẶC CoT — bài `2` và `5`) và `co_rang_buoc_ro_rang` (đúng ĐỊNH
DẠNG, quyết định bởi chỉ thị tường minh — bài `3`) là HAI trục HOÀN TOÀN
độc lập trong code — không nhánh nào phụ thuộc nhánh kia.

Phép đo trung tâm: một câu trả lời chỉ tính là **"đúng"** khi ĐỒNG THỜI
đúng cả hai trục — `dap_an.isdigit()` (định dạng: chuỗi toàn chữ số,
không lẫn chữ) VÀ `int(dap_an) == tong_dung` (giá trị: đúng tổng thật).
Trên CÙNG bộ `5` câu hỏi kiểm thử đã dùng ở bài `5`, so sánh **prompt
trần trụi** (chỉ câu hỏi, không hệ thống, không few-shot, không CoT) với
**prompt đầy đủ** (cả bốn kỹ thuật).
::::

::::example{#boss_do_cai_thien}
Chạy cả hai phiên bản prompt trên bộ `5` câu hỏi kiểm thử:

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


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_mo_phong_day_du(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    tong = sum(so_list) if dung_day_du else sum(so_list[:2])
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."
HAU_TO_COT = " Hay suy nghi tung buoc."

BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

VI_DU_CAU_HOI_1 = "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"
VI_DU_DAP_AN_1 = "3"
VI_DU_CAU_HOI_2 = "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"
VI_DU_DAP_AN_2 = "8"


def tao_prompt_tran_trui(cau_hoi):
    return [{"role": "user", "content": cau_hoi}]


def tao_prompt_day_du(cau_hoi):
    return [
        {"role": "system", "content": CAU_RANG_BUOC},
        {"role": "user", "content": VI_DU_CAU_HOI_1},
        {"role": "assistant", "content": VI_DU_DAP_AN_1},
        {"role": "user", "content": VI_DU_CAU_HOI_2},
        {"role": "assistant", "content": VI_DU_DAP_AN_2},
        {"role": "user", "content": cau_hoi + HAU_TO_COT},
    ]


def dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
    return dap_an.isdigit() and int(dap_an) == tong_dung


def chay_bo_kiem_thu(tao_prompt):
    so_dung = 0
    for cau_hoi in BO_CAU_HOI_KIEM_THU:
        messages = tao_prompt(cau_hoi)
        dap_an = llm_mo_phong_day_du(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
            so_dung += 1
    return so_dung


so_dung_tran_trui = chay_bo_kiem_thu(tao_prompt_tran_trui)
so_dung_day_du = chay_bo_kiem_thu(tao_prompt_day_du)

print(so_dung_tran_trui, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_day_du, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_day_du - so_dung_tran_trui)
```

```text title=readonly
0 / 5
5 / 5
5
```

Prompt trần trụi: `0/5` — không một câu nào đạt CẢ HAI tiêu chí, vì thiếu
ràng buộc khiến định dạng luôn là một trong ba cách diễn đạt mơ hồ (không
phải chuỗi số thuần), dù giá trị số bên trong câu chữ có đúng hay không.
Prompt đầy đủ (cả bốn kỹ thuật): `5/5` — ĐÚNG hoàn toàn. Cải thiện đo
được: từ `0` lên `5` trên `5` câu, tức từ `0%` lên `100%`.
::::

::::example{#boss_doi_chung_rieng_le}
Để xác nhận định dạng và giá trị là HAI trục ĐỘC LẬP (không phải một kỹ
thuật ngẫu nhiên "làm được tất cả"), chạy thêm hai đối chứng: CHỈ có ràng
buộc (không few-shot, không CoT), và CHỈ có few-shot (không ràng buộc,
không CoT):

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


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_mo_phong_day_du(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    tong = sum(so_list) if dung_day_du else sum(so_list[:2])
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


def dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
    return dap_an.isdigit() and int(dap_an) == tong_dung


CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."

BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

VI_DU_CAU_HOI_1 = "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"
VI_DU_DAP_AN_1 = "3"
VI_DU_CAU_HOI_2 = "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"
VI_DU_DAP_AN_2 = "8"


def chay_bo_kiem_thu(tao_prompt):
    so_dung = 0
    for cau_hoi in BO_CAU_HOI_KIEM_THU:
        messages = tao_prompt(cau_hoi)
        dap_an = llm_mo_phong_day_du(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
            so_dung += 1
    return so_dung


def tao_prompt_chi_rang_buoc(cau_hoi):
    return [
        {"role": "system", "content": CAU_RANG_BUOC},
        {"role": "user", "content": cau_hoi},
    ]


def tao_prompt_chi_fewshot(cau_hoi):
    return [
        {"role": "user", "content": VI_DU_CAU_HOI_1},
        {"role": "assistant", "content": VI_DU_DAP_AN_1},
        {"role": "user", "content": VI_DU_CAU_HOI_2},
        {"role": "assistant", "content": VI_DU_DAP_AN_2},
        {"role": "user", "content": cau_hoi},
    ]


so_dung_chi_rang_buoc = chay_bo_kiem_thu(tao_prompt_chi_rang_buoc)
so_dung_chi_fewshot = chay_bo_kiem_thu(tao_prompt_chi_fewshot)

print(so_dung_chi_rang_buoc, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_chi_fewshot, "/", len(BO_CAU_HOI_KIEM_THU))
```

```text title=readonly
2 / 5
0 / 5
```

**Chỉ ràng buộc** (không few-shot, không CoT): `2/5` — định dạng LUÔN
đúng (chuỗi số trần trụi ở cả `5` câu), nhưng giá trị chỉ đúng ở `2` câu
có `<= 2` đại lượng (luật đi tắt trùng với tổng thật); `3` câu còn lại có
định dạng đúng nhưng SỐ SAI. **Chỉ few-shot** (không ràng buộc): `0/5` —
giá trị số ĐÚNG ở cả `5` câu (few-shot đã kích hoạt cộng toàn bộ), nhưng
KHÔNG câu nào đạt vì thiếu ràng buộc khiến định dạng luôn là một cách
diễn đạt mơ hồ, không phải chuỗi số thuần — dù con số ĐÚNG nằm ngay trong
câu chữ, `dung_dinh_dang_va_gia_tri` vẫn từ chối vì `dap_an.isdigit()` là
`False`. Hai đối chứng này xác nhận: định dạng và giá trị là HAI trục
tách biệt hoàn toàn trong code, và CHỈ prompt đầy đủ (cả bốn kỹ thuật)
mới đạt tối đa trên CẢ HAI.
::::

::::predict{#doan_ablation commitOnce}
Dựa theo hai đối chứng vừa đo (`chỉ ràng buộc` = `2/5`, `chỉ few-shot` =
`0/5`), và biết `dung_day_du` được kích hoạt bởi few-shot HOẶC CoT (toán
tử `or`, bài `5`).

**Trước khi chạy thử**, bạn đoán: một prompt CHỈ có ràng buộc VÀ CoT
(không few-shot) — tức thêm `HAU_TO_COT` vào câu hỏi của
`tao_prompt_chi_rang_buoc`, không thêm ví dụ mẫu nào — sẽ đạt bao nhiêu
trên bộ `5` câu kiểm thử?

:::opt{correct}
`5/5` — ràng buộc đảm bảo định dạng luôn đúng (như đối chứng `chỉ ràng
buộc`), và CoT một mình đã đủ kích hoạt `dung_day_du = True` (giống hiệu
ứng đo ở bài `5`, không cần few-shot) nên giá trị cũng đúng ở cả `5` câu —
cả hai trục đều đạt tối đa, không cần few-shot tham gia
:::

:::opt
`2/5` — giống hệt đối chứng `chỉ ràng buộc`, vì CoT chỉ có tác dụng khi đi
kèm few-shot, không đứng một mình được
::why
Gần đúng ở việc bạn NHỚ ĐÚNG con số `2/5` của đối chứng `chỉ ràng buộc` —
đó thật sự là kết quả khi không có few-shot LẪN không có CoT.

Chỗ lệch: bài `5` (`do-thuc-nghiem-anh-huong-prompt`) đã đo TRỰC TIẾP rằng
CoT một mình (không few-shot nào) đủ để đạt `5/5` đúng giá trị trên chính
bộ `5` câu hỏi này — `dung_day_du = co_yeu_cau_cot(messages) or so_vi_du
>= 2` dùng `or`, nên CoT đứng một mình vẫn kích hoạt được nhánh cộng toàn
bộ, không cần few-shot đi kèm.
::
:::

:::opt
Không xác định được nếu không biết tỉ lệ đóng góp của ràng buộc so với
CoT
::why
Gần đúng ở tinh thần thận trọng khi kết hợp nhiều yếu tố — trong nhiều hệ
thống thực tế, hiệu ứng kết hợp CẦN đo thực nghiệm vì không cộng tuyến
tính đơn giản.

Chỗ lệch: ở LLM mô phỏng này, hai trục được cài đặt tách biệt hoàn toàn
trong code — `co_rang_buoc_ro_rang` quyết định ĐỊNH DẠNG, `dung_day_du`
quyết định GIÁ TRỊ, không nhánh nào đọc kết quả của nhánh kia. Đọc thẳng
thân hàm `llm_mo_phong_day_du` đã đủ để suy ra kết quả trước khi chạy,
không cần đo thêm.
::
:::
::::

::::code{#viet_boss_ket_luan}
Hoàn thiện phần cuối: gọi `chay_bo_kiem_thu` với đúng hàm dựng prompt cho
mỗi phiên bản, rồi tổng hợp CẢ HAI bằng chứng (có cải thiện VÀ đạt tối đa)
thành một kết luận duy nhất.

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


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_mo_phong_day_du(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    tong = sum(so_list) if dung_day_du else sum(so_list[:2])
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."
HAU_TO_COT = " Hay suy nghi tung buoc."

BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

VI_DU_CAU_HOI_1 = "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"
VI_DU_DAP_AN_1 = "3"
VI_DU_CAU_HOI_2 = "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"
VI_DU_DAP_AN_2 = "8"


def tao_prompt_tran_trui(cau_hoi):
    return [{"role": "user", "content": cau_hoi}]


def tao_prompt_day_du(cau_hoi):
    return [
        {"role": "system", "content": CAU_RANG_BUOC},
        {"role": "user", "content": VI_DU_CAU_HOI_1},
        {"role": "assistant", "content": VI_DU_DAP_AN_1},
        {"role": "user", "content": VI_DU_CAU_HOI_2},
        {"role": "assistant", "content": VI_DU_DAP_AN_2},
        {"role": "user", "content": cau_hoi + HAU_TO_COT},
    ]


def dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
    return dap_an.isdigit() and int(dap_an) == tong_dung


def chay_bo_kiem_thu(tao_prompt):
    so_dung = 0
    for cau_hoi in BO_CAU_HOI_KIEM_THU:
        messages = tao_prompt(cau_hoi)
        dap_an = llm_mo_phong_day_du(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
            so_dung += 1
    return so_dung


so_dung_tran_trui = chay_bo_kiem_thu(___)                # tao_prompt_tran_trui
so_dung_day_du = chay_bo_kiem_thu(___)                    # tao_prompt_day_du

co_cai_thien = so_dung_day_du > so_dung_tran_trui
dat_toi_da = so_dung_day_du == len(BO_CAU_HOI_KIEM_THU)
ket_luan = ___                                            # co_cai_thien and dat_toi_da

print(so_dung_tran_trui, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_day_du, "/", len(BO_CAU_HOI_KIEM_THU))
print(co_cai_thien, dat_toi_da)
print(ket_luan)
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


def dem_vi_du_fewshot(messages):
    return sum(1 for tin in messages if tin["role"] == "assistant")


def co_rang_buoc_ro_rang(messages):
    for tin in messages:
        if "chi tra loi bang mot con so" in tin["content"].lower():
            return True
    return False


def co_yeu_cau_cot(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    return "hay suy nghi tung buoc" in cau_hoi.lower()


CAC_CAU_TRA_LOI_MO_HO = [
    "Toi nghi tong khoang {tong}.",
    "Co le la {tong}.",
    "Tong la {tong}, nhung ban nen kiem tra lai.",
]


def llm_mo_phong_day_du(messages):
    cau_hoi = lay_cau_hoi_cuoi(messages)
    so_list = trich_so(cau_hoi)
    so_vi_du = dem_vi_du_fewshot(messages)
    dung_day_du = co_yeu_cau_cot(messages) or so_vi_du >= 2
    tong = sum(so_list) if dung_day_du else sum(so_list[:2])
    if co_rang_buoc_ro_rang(messages):
        return str(tong)
    chi_so = len(so_list) % len(CAC_CAU_TRA_LOI_MO_HO)
    return CAC_CAU_TRA_LOI_MO_HO[chi_so].format(tong=tong)


CAU_RANG_BUOC = "Chi tra loi bang mot con so nguyen, khong kem chu nao khac."
HAU_TO_COT = " Hay suy nghi tung buoc."

BO_CAU_HOI_KIEM_THU = [
    "An co 2 qua tao va 3 qua cam. Hoi An co tat ca bao nhieu qua?",
    "Binh co 1 qua tao, 2 qua le va 4 qua cam. Hoi Binh co tat ca bao nhieu qua?",
    "Chi co 5 qua cam va 1 qua tao. Hoi Chi co tat ca bao nhieu qua?",
    "Dung co 2 qua tao, 3 qua cam, 1 qua le va 4 qua buoi. Hoi Dung co tat ca bao nhieu qua?",
    "Em co 3 qua tao, 2 qua cam va 5 qua le. Hoi Em co tat ca bao nhieu qua?",
]

VI_DU_CAU_HOI_1 = "Nam co 1 qua tao va 2 qua cam. Hoi Nam co tat ca bao nhieu qua?"
VI_DU_DAP_AN_1 = "3"
VI_DU_CAU_HOI_2 = "Hoa co 4 qua tao, 1 qua cam va 3 qua le. Hoi Hoa co tat ca bao nhieu qua?"
VI_DU_DAP_AN_2 = "8"


def tao_prompt_tran_trui(cau_hoi):
    return [{"role": "user", "content": cau_hoi}]


def tao_prompt_day_du(cau_hoi):
    return [
        {"role": "system", "content": CAU_RANG_BUOC},
        {"role": "user", "content": VI_DU_CAU_HOI_1},
        {"role": "assistant", "content": VI_DU_DAP_AN_1},
        {"role": "user", "content": VI_DU_CAU_HOI_2},
        {"role": "assistant", "content": VI_DU_DAP_AN_2},
        {"role": "user", "content": cau_hoi + HAU_TO_COT},
    ]


def dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
    return dap_an.isdigit() and int(dap_an) == tong_dung


def chay_bo_kiem_thu(tao_prompt):
    so_dung = 0
    for cau_hoi in BO_CAU_HOI_KIEM_THU:
        messages = tao_prompt(cau_hoi)
        dap_an = llm_mo_phong_day_du(messages)
        tong_dung = sum(trich_so(cau_hoi))
        if dung_dinh_dang_va_gia_tri(dap_an, tong_dung):
            so_dung += 1
    return so_dung


so_dung_tran_trui = chay_bo_kiem_thu(tao_prompt_tran_trui)
so_dung_day_du = chay_bo_kiem_thu(tao_prompt_day_du)

co_cai_thien = so_dung_day_du > so_dung_tran_trui
dat_toi_da = so_dung_day_du == len(BO_CAU_HOI_KIEM_THU)
ket_luan = co_cai_thien and dat_toi_da

print(so_dung_tran_trui, "/", len(BO_CAU_HOI_KIEM_THU))
print(so_dung_day_du, "/", len(BO_CAU_HOI_KIEM_THU))
print(co_cai_thien, dat_toi_da)
print(ket_luan)
```

```python title=test
assert so_dung_tran_trui == 0, f"prompt tran trui phai la 0/5 -- dang ra {so_dung_tran_trui}"
assert so_dung_day_du == 5, f"prompt day du phai la 5/5 -- dang ra {so_dung_day_du}"
assert co_cai_thien == True, "co_cai_thien phai la True"
assert dat_toi_da == True, "dat_toi_da phai la True"
assert ket_luan == True, "ket_luan phai la True -- CA HAI bang chung deu phai dung"

# bien: doi chung rieng le, xac nhan dinh dang va gia tri la HAI truc doc
# lap (khong phai mot con so ngau nhien)
def tao_prompt_chi_rang_buoc(cau_hoi):
    return [
        {"role": "system", "content": CAU_RANG_BUOC},
        {"role": "user", "content": cau_hoi},
    ]

def tao_prompt_chi_fewshot(cau_hoi):
    return [
        {"role": "user", "content": VI_DU_CAU_HOI_1},
        {"role": "assistant", "content": VI_DU_DAP_AN_1},
        {"role": "user", "content": VI_DU_CAU_HOI_2},
        {"role": "assistant", "content": VI_DU_DAP_AN_2},
        {"role": "user", "content": cau_hoi},
    ]

assert chay_bo_kiem_thu(tao_prompt_chi_rang_buoc) == 2, f"chi rang buoc phai la 2/5 -- dang ra {chay_bo_kiem_thu(tao_prompt_chi_rang_buoc)}"
assert chay_bo_kiem_thu(tao_prompt_chi_fewshot) == 0, f"chi few-shot phai la 0/5 -- dang ra {chay_bo_kiem_thu(tao_prompt_chi_fewshot)}"

# GOTCHA "and" vs "or": tren du lieu THAT cua bai nay, co_cai_thien (True)
# va dat_toi_da (True) deu True nen "and"/"or" cho CUNG ket qua -- chi
# static rieng moi phan biet duoc bien the toan tu nay, khong output/tests
# nao bat duoc vi ca hai deu cho ket_luan=True.
assert dung_dinh_dang_va_gia_tri("5", 5) == True, "'5' voi tong dung 5 phai la True"
assert dung_dinh_dang_va_gia_tri("Toi nghi tong khoang 5.", 5) == False, "chuoi khong phai so thuan phai la False du gia tri dung"
assert dung_dinh_dang_va_gia_tri("7", 5) == False, "so dung dinh dang nhung SAI gia tri phai la False"
```

:::hints
- kind: attention
  body: Ba chỗ trống. Chỗ đầu và chỗ hai truyền ĐÚNG hàm dựng prompt (không gọi nó, chỉ truyền TÊN hàm) cho `chay_bo_kiem_thu` — `tao_prompt_tran_trui` cho phiên bản trần trụi, `tao_prompt_day_du` cho phiên bản đầy đủ. Chỗ ba tổng hợp HAI điều kiện (`co_cai_thien`, `dat_toi_da`) bằng toán tử kết hợp CẢ HAI phải đúng.
- kind: strategy
  body: 'Chỗ đầu: `tao_prompt_tran_trui`. Chỗ hai: `tao_prompt_day_du`. Chỗ ba: `co_cai_thien and dat_toi_da`.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `tao_prompt_tran_trui`, `tao_prompt_day_du`, và `co_cai_thien and dat_toi_da`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: hai loi goi chay_bo_kiem_thu phai dung DUNG hai ham dung khac nhau (tao_prompt_tran_trui va tao_prompt_day_du, khong duoc dao nguoc hay goi trung mot ham), VA ket_luan phai la phep 'and' THAT giua ca hai dieu kien (khong duoc chep san True, khong doi thanh 'or')
  requireAst:
  - kind: uses-call, target: chay_bo_kiem_thu, min: 2
  - kind: uses-name, target: tao_prompt_tran_trui, min: 1
  - kind: uses-name, target: tao_prompt_day_du, min: 1
  - kind: uses-name, target: co_cai_thien, min: 2
  - kind: uses-operator, target: and, min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # chay_bo_kiem_thu=2: dung hai lan goi, chinh la cho trong 1 va 2 --
  # dinh nghia ham (def chay_bo_kiem_thu(...):) khong tinh la Call. Dien
  # bua chep san "so_dung_tran_trui = 0" (bo qua goi ham THAT) lam so nay
  # tut xuong duoi 2 -- bi chan; dong thoi bi chan boi tests.
  # tao_prompt_tran_trui=1, tao_prompt_day_du=1: moi ten CHI xuat hien
  # dung 1 lan, o dung cho trong tuong ung. Dao nguoc hai cho trong cho
  # nhau (truyen tao_prompt_day_du vao cho 1, tao_prompt_tran_trui vao
  # cho 2) khong doi so dem AST nao (van moi ten 1 lan) NHUNG lam
  # so_dung_tran_trui va so_dung_day_du hoan doi gia tri cho nhau -- bi
  # chan boi tests (so_dung_tran_trui phai la 0, se thanh 5 neu dao
  # nguoc).
  # uses-name "co_cai_thien"=2: mot lan CO SAN trong dong "print(co_cai_thien,
  # dat_toi_da)" (luon chay, khong bi cho trong), mot lan la CHINH cho trong
  # 3 (doc ten co_cai_thien trong "co_cai_thien and dat_toi_da"). Neu chi
  # dat min=1 (ngay tho), mot mutant thay ten "co_cai_thien" bang mot ten
  # KHAC luon-dung-True o vi tri do (vi du doi thanh "chay_bo_kiem_thu" --
  # mot ham, luon truthy) van qua duoc vi con lai 1 lan doc "co_cai_thien"
  # trong print -- day la GOTCHA "boilerplate-threshold-masking" THAT SU
  # da bi cong cu dot bien tu dong bat duoc luc dau (truoc khi them luat
  # nay); dat dung min=2 (tong THAT) moi chan duoc mutant nay. Mutant nay
  # KHONG bi bat boi output/tests vi ca hai gia tri (co_cai_thien=True va
  # chay_bo_kiem_thu, mot ham luon truthy) deu lam "X and dat_toi_da" tra
  # ve dat_toi_da=True nhu nhau -- CHI static rieng moi bat duoc.
  # and=2: mot lan CO SAN trong dung_dinh_dang_va_gia_tri ("dap_an.isdigit()
  # and int(dap_an) == tong_dung"), mot lan la cho trong 3. Neu chi dat
  # min=1 (ngay tho), mot mutant chep san "ket_luan = True" (bo hoan toan
  # BoolOp) VAN qua duoc vi con lai 1 lan "and" trong boilerplate -- day la
  # GOTCHA "boilerplate-threshold-masking"; dat dung min=2 (tong THAT) moi
  # chan duoc mutant nay -- va no cung bi chan boi tests (ket_luan van la
  # True nen khong bi bat boi output/tests, CHI static rieng moi bat
  # duoc). Doi "and" thanh "or" o cho trong 3 cung bi chan boi static
  # (BoolOp doi tu And sang Or, khong con khop target "and", "and" tut ve
  # 1 duoi nguong 2) MAC DU tren du lieu THAT ca hai dieu kien deu True
  # nen "or" cho CUNG ket_luan=True -- khong output/tests nao bat duoc
  # bien the nay, chi static rieng moi bat.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^0 / 5\\n5 / 5\\nTrue True\\nTrue\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Prompt trần trụi: `0/5`. Prompt đầy đủ (vai trò + ràng buộc + few-shot +
CoT): `5/5`. Cải thiện `+5`, từ `0%` lên `100%`, đo bằng số thật trên
cùng một bộ câu hỏi. Quest `prompt-engineering-nen-tang` (q8.4a) khép lại
tại đây.
::::

::::reflect{#nghi-lai}
Sáu bài, một domain xuyên suốt (đếm số lượng quả trong một câu hỏi), bốn
kỹ thuật ráp dần vào một LLM mô phỏng:

> **`vai-tro-message`** (bài `1`) — cấu trúc message (`role`/`content`)
> quyết định phần nào của một lượt gọi được đọc như chỉ thị cấu hình,
> phần nào là câu hỏi.
>
> **`zero-shot-va-few-shot`** (bài `2`) — số lượng ví dụ mẫu đặt trước
> câu hỏi đổi được hành vi TÍNH TOÁN, qua một ngưỡng cụ thể (`>= 2`).
>
> **`rang-buoc-ro-rang`** (bài `3`) — một chỉ thị định dạng tường minh
> thu hẹp không gian câu trả lời từ nhiều cách diễn đạt xuống còn một.
>
> **`prompt-template`** (bài `4`) — đóng gói cấu trúc câu hỏi (và ràng
> buộc kèm theo) vào một khuôn có tham số, tái dùng được cho nhiều đầu
> vào.
>
> **`do-thuc-nghiem-anh-huong-prompt`** (bài `5`) — đo A/B trên một bộ
> câu hỏi cố định, và một kỹ thuật thứ tư (CoT) cải thiện đo được `+3/5`
> mà không cần ví dụ mẫu.
>
> **`boss-prompt-hoan-chinh`** (bài `6`, quest này) — ráp cả bốn kỹ
> thuật, đo cải thiện `0/5 → 5/5` trên cùng một bộ kiểm thử, và tách bạch
> hai trục định dạng/giá trị bằng hai đối chứng riêng.

Toàn bộ quest dùng đúng MỘT nguyên tắc: một LLM MÔ PHỎNG — hàm Python
thuần, tất định, không mạng nơ-ron, không gọi API thật — vẫn đủ để đo
ĐƯỢC BẰNG SỐ những cơ chế cốt lõi của prompt engineering: cấu trúc message
thay đổi hành vi ra sao, ví dụ mẫu thu hẹp không gian ra sao, ràng buộc
tường minh thu hẹp định dạng ra sao, và một chỉ thị suy luận từng bước có
thể sửa lỗi mà không cần ví dụ nào. Quest tiếp theo, `chien-luoc-giai-
ma-va-lay-mau` (q8.4b), rẽ sang một hướng khác: quay lại dùng THẬT
`Tensor`/`khoi_transformer` của T8.3 (không còn mô phỏng), và học cách
MỘT phân phối xác suất đầu ra CÓ THẬT được biến thành văn bản qua các
chiến lược giải mã khác nhau — greedy, nhiệt độ, top-k, top-p.
::::

::::checkpoint{mastery=0.9}
::::
