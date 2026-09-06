---
id: tri-tue-nhan-tao.da-tac-tu-va-quan-sat-ai.bang-tra-cuu-su-co
title: "Bảng tra cứu sự cố: năm triệu chứng của Chương 39, tra bảng không đoán"
summary: "BANG_SU_CO la mot dict CO DINH anh xa 5 TRIEU CHUNG (gpu_oom_tai_cao, do_tre_cao_gpu_ranh, chi_phi_tang_bat_thuong, rag_lech_chu_de, timeout_gateway -- dung nguyen bang troubleshooting cuoi chuong 39, dich sang tieng Viet) sang mot dict con {nguyen_nhan, cach_xu_ly}. tra_cuu_su_co(trieu_chung) = BANG_SU_CO.get(trieu_chung, {nguyen_nhan: khong_xac_dinh, cach_xu_ly: can kiem tra them}) -- dung khuon .get cua q8.6a bai 1/q8.6b bai 3, KHONG nem loi cho trieu chung la. tra_cuu_nhieu_trieu_chung(danh_sach_trieu_chung) ap tra_cuu_su_co cho TUNG triệu chứng, tra ve danh sach dict. Tren 6 trieu chung dau vao (5 trieu chung THAT + 1 trieu chung la 'loi_la_chua_gap_bao_gio'): dung 5 nguyen nhan khop CHINH XAC bang goc, so_khong_xac_dinh = 1 (DUNG trieu chung la, khong hon khong kem)."
locale: vi
track: tri-tue-nhan-tao
module: da-tac-tu-va-quan-sat-ai
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.bang-tra-cuu-su-co]
requires: [ai.hang-doi-uu-tien-chong-doi]
concepts: [ai.bang-tra-cuu-su-co]
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
Ba bài đầu mô phỏng BA cơ chế của `Chương 39`. Cuối chương còn một bảng
không mô phỏng gì — nó chỉ LÀ một bảng TRA CỨU: **Troubleshooting**. Khi
một trong ba cơ chế trên "báo động" (cache hit thấp bất thường, hàng đợi
ùn ứ...), làm sao BIẾT ngay nguyên nhân VÀ cách xử lý, không đoán mò?
::::

::::explain{#tra_bang_khong_doan}
`Chương 39` kết thúc bằng một bảng cố định, ánh xạ MỖI triệu chứng tới
ĐÚNG một nguyên nhân và một cách xử lý:

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| GPU OOM khi tải cao | Batch size / context length quá lớn | Giảm `max_model_len`, bật paged attention |
| Độ trễ cao dù GPU rảnh | Request xử lý tuần tự | Bật continuous batching |
| Chi phí tăng bất thường | Không cache, prompt lặp lại nhiều | Thêm semantic cache + prompt caching |
| Kết quả RAG lệch chủ đề | Chunking kém hoặc thiếu re-rank | Giảm chunk size, thêm bước re-rank |
| Timeout ở tầng gateway | Request LLM dài hơn timeout mặc định | Tăng timeout VÀ chuyển sang streaming |

Đây là đúng khuôn đã dùng ở `q8.6a` bài `1` (`BANG_TANG.get(...)`) và
`q8.6b` bài `3` (`xac_thuc` — tra bảng, không đoán): một `dict` CỐ ĐỊNH,
tra bằng `.get(khoa, mac_dinh)`. Một triệu chứng LẠ (chưa từng gặp) KHÔNG
làm chương trình NỔ với `KeyError` — nó trả về một giá trị MẶC ĐỊNH tường
minh, để hệ thống biết "cái này tôi CHƯA CÓ câu trả lời", thay vì crash:

```
tra_cuu_su_co(trieu_chung) = BANG_SU_CO.get(trieu_chung,
    {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})
```

`tra_cuu_nhieu_trieu_chung` áp `tra_cuu_su_co` cho MỖI triệu chứng trong
một danh sách — hữu ích khi nhiều cảnh báo xuất hiện CÙNG lúc trên một hệ
thống lớn.
::::

::::example{#tra_cuu_sau_trieu_chung}
```python title=readonly
BANG_SU_CO = {
    "gpu_oom_tai_cao": {
        "nguyen_nhan": "batch size hoac do dai ngu canh qua lon",
        "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm",
    },
    "do_tre_cao_gpu_ranh": {
        "nguyen_nhan": "request xu ly tuan tu, khong gop lo",
        "cach_xu_ly": "bat continuous batching",
    },
    "chi_phi_tang_bat_thuong": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
    "rag_lech_chu_de": {
        "nguyen_nhan": "chunking kem hoac thieu buoc rerank",
        "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank",
    },
    "timeout_gateway": {
        "nguyen_nhan": "request llm dai hon timeout mac dinh",
        "cach_xu_ly": "tang timeout va chuyen sang streaming",
    },
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def tra_cuu_nhieu_trieu_chung(danh_sach_trieu_chung):
    return [tra_cuu_su_co(t) for t in danh_sach_trieu_chung]


CAC_TRIEU_CHUNG = [
    "gpu_oom_tai_cao", "do_tre_cao_gpu_ranh", "chi_phi_tang_bat_thuong",
    "rag_lech_chu_de", "timeout_gateway", "loi_la_chua_gap_bao_gio",
]

KET_QUA = tra_cuu_nhieu_trieu_chung(CAC_TRIEU_CHUNG)
CAC_NGUYEN_NHAN = [kq["nguyen_nhan"] for kq in KET_QUA]
SO_KHONG_XAC_DINH = sum(1 for kq in KET_QUA if kq["nguyen_nhan"] == "khong_xac_dinh")

print(CAC_NGUYEN_NHAN)
print(SO_KHONG_XAC_DINH)
```

```text title=readonly
['batch size hoac do dai ngu canh qua lon', 'request xu ly tuan tu, khong gop lo', 'khong co cache, prompt lap lai nhieu lan', 'chunking kem hoac thieu buoc rerank', 'request llm dai hon timeout mac dinh', 'khong_xac_dinh']
1
```

`CAC_TRIEU_CHUNG` có `6` mục: đúng `5` triệu chứng CÓ trong `BANG_SU_CO`
(nguyên văn cả năm hàng của bảng troubleshooting), cộng THÊM một triệu
chứng LẠ (`"loi_la_chua_gap_bao_gio"`, không hề tồn tại trong bảng).
`tra_cuu_nhieu_trieu_chung` áp `tra_cuu_su_co` cho TỪNG mục — năm mục ĐẦU
trả về ĐÚNG nguyên nhân đã khai trong `BANG_SU_CO`, mục CUỐI (triệu chứng
lạ) trả về `"khong_xac_dinh"` — không hề ném lỗi. `SO_KHONG_XAC_DINH = 1`
— ĐÚNG một triệu chứng lạ, không nhiều hơn (năm triệu chứng đầu ĐỀU khớp
bảng), không ít hơn (triệu chứng thứ sáu THẬT SỰ không có trong bảng).
::::

::::predict{#doan_trieu_chung_la_khac commitOnce}
Xét gọi `tra_cuu_su_co("gpu_qua_nong")` — một triệu chứng KHÔNG hề xuất
hiện trong `BANG_SU_CO` (khác cả CHỮ lẫn Ý so với năm triệu chứng đã khai).

**Trước khi chạy thử**, bạn đoán: kết quả trả về LÀ gì?

:::opt{correct}
`{"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"}` —
`BANG_SU_CO.get(...)` không tìm thấy khoá `"gpu_qua_nong"`, nên trả về
ĐÚNG giá trị mặc định đã khai ở đối số thứ hai của `.get`, không ném lỗi
:::

:::opt
Chương trình ném `KeyError`, vì `"gpu_qua_nong"` không có trong
`BANG_SU_CO`
::why
Gần đúng ở việc `"gpu_qua_nong"` THẬT SỰ không có trong `BANG_SU_CO` —
quan sát đó đúng.

Chỗ lệch: `tra_cuu_su_co` dùng `BANG_SU_CO.get(trieu_chung, mac_dinh)`,
KHÔNG dùng `BANG_SU_CO[trieu_chung]`. `.get` với đối số thứ hai KHÔNG BAO
GIỜ ném `KeyError` — nó trả về đối số thứ hai đó khi khoá không tồn tại.
`KeyError` chỉ xảy ra nếu dùng `[]` trực tiếp, đúng lỗi thiết kế mà
`q8.6a` bài `1` đã cảnh báo.
::
:::

:::opt
`{"nguyen_nhan": "gpu_oom_tai_cao", "cach_xu_ly": "..."}`  — vì
`"gpu_qua_nong"` gần giống chữ với `"gpu_oom_tai_cao"`, bảng sẽ tìm mục
GẦN GIỐNG nhất
::why
Gần đúng ở việc `"gpu_qua_nong"` và `"gpu_oom_tai_cao"` THẬT SỰ có vẻ
"liên quan" về ý nghĩa (cả hai đều nói về GPU) — quan sát về CHỦ ĐỀ đó
không sai.

Chỗ lệch: `dict.get` so khớp CHÍNH XÁC từng ký tự của khoá — nó không hề
"đoán" hay tìm chuỗi GẦN GIỐNG. `"gpu_qua_nong"` và `"gpu_oom_tai_cao"` là
hai CHUỖI khác nhau hoàn toàn theo `==`, nên `.get` coi khoá này LÀ không
tồn tại, dù về ý nghĩa con người đọc có thể thấy liên quan.
::
:::
::::

::::code{#viet_tra_cuu_su_co_va_nhieu_trieu_chung}
Hoàn thiện `tra_cuu_su_co` (tra bảng bằng `.get` với giá trị MẶC ĐỊNH tường
minh) và `tra_cuu_nhieu_trieu_chung` (áp hàm vừa viết cho TỪNG triệu chứng
trong một danh sách).

```python title=starter
BANG_SU_CO = {
    "gpu_oom_tai_cao": {
        "nguyen_nhan": "batch size hoac do dai ngu canh qua lon",
        "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm",
    },
    "do_tre_cao_gpu_ranh": {
        "nguyen_nhan": "request xu ly tuan tu, khong gop lo",
        "cach_xu_ly": "bat continuous batching",
    },
    "chi_phi_tang_bat_thuong": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
    "rag_lech_chu_de": {
        "nguyen_nhan": "chunking kem hoac thieu buoc rerank",
        "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank",
    },
    "timeout_gateway": {
        "nguyen_nhan": "request llm dai hon timeout mac dinh",
        "cach_xu_ly": "tang timeout va chuyen sang streaming",
    },
}


def tra_cuu_su_co(trieu_chung):
    return ___                                                  # BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def tra_cuu_nhieu_trieu_chung(danh_sach_trieu_chung):
    return ___                                                  # [tra_cuu_su_co(t) for t in danh_sach_trieu_chung]


CAC_TRIEU_CHUNG = [
    "gpu_oom_tai_cao", "do_tre_cao_gpu_ranh", "chi_phi_tang_bat_thuong",
    "rag_lech_chu_de", "timeout_gateway", "loi_la_chua_gap_bao_gio",
]

KET_QUA = tra_cuu_nhieu_trieu_chung(CAC_TRIEU_CHUNG)
CAC_NGUYEN_NHAN = [kq["nguyen_nhan"] for kq in KET_QUA]
SO_KHONG_XAC_DINH = sum(1 for kq in KET_QUA if kq["nguyen_nhan"] == "khong_xac_dinh")

print(CAC_NGUYEN_NHAN)
print(SO_KHONG_XAC_DINH)
```

```python title=solution
BANG_SU_CO = {
    "gpu_oom_tai_cao": {
        "nguyen_nhan": "batch size hoac do dai ngu canh qua lon",
        "cach_xu_ly": "giam max_model_len, bat paged attention cua vllm",
    },
    "do_tre_cao_gpu_ranh": {
        "nguyen_nhan": "request xu ly tuan tu, khong gop lo",
        "cach_xu_ly": "bat continuous batching",
    },
    "chi_phi_tang_bat_thuong": {
        "nguyen_nhan": "khong co cache, prompt lap lai nhieu lan",
        "cach_xu_ly": "them semantic cache va prompt caching",
    },
    "rag_lech_chu_de": {
        "nguyen_nhan": "chunking kem hoac thieu buoc rerank",
        "cach_xu_ly": "giam kich thuoc chunk, them buoc rerank",
    },
    "timeout_gateway": {
        "nguyen_nhan": "request llm dai hon timeout mac dinh",
        "cach_xu_ly": "tang timeout va chuyen sang streaming",
    },
}


def tra_cuu_su_co(trieu_chung):
    return BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})


def tra_cuu_nhieu_trieu_chung(danh_sach_trieu_chung):
    return [tra_cuu_su_co(t) for t in danh_sach_trieu_chung]


CAC_TRIEU_CHUNG = [
    "gpu_oom_tai_cao", "do_tre_cao_gpu_ranh", "chi_phi_tang_bat_thuong",
    "rag_lech_chu_de", "timeout_gateway", "loi_la_chua_gap_bao_gio",
]

KET_QUA = tra_cuu_nhieu_trieu_chung(CAC_TRIEU_CHUNG)
CAC_NGUYEN_NHAN = [kq["nguyen_nhan"] for kq in KET_QUA]
SO_KHONG_XAC_DINH = sum(1 for kq in KET_QUA if kq["nguyen_nhan"] == "khong_xac_dinh")

print(CAC_NGUYEN_NHAN)
print(SO_KHONG_XAC_DINH)
```

```python title=test
assert CAC_NGUYEN_NHAN == [
    "batch size hoac do dai ngu canh qua lon",
    "request xu ly tuan tu, khong gop lo",
    "khong co cache, prompt lap lai nhieu lan",
    "chunking kem hoac thieu buoc rerank",
    "request llm dai hon timeout mac dinh",
    "khong_xac_dinh",
], f"CAC_NGUYEN_NHAN sai -- dang ra {CAC_NGUYEN_NHAN}"
assert SO_KHONG_XAC_DINH == 1, f"SO_KHONG_XAC_DINH phai la 1 (dung mot trieu chung la) -- dang ra {SO_KHONG_XAC_DINH}"
assert len(KET_QUA) == 6, f"KET_QUA phai co dung 6 muc (dung so trieu chung dau vao) -- dang ra {len(KET_QUA)}"

# kiem tra truc tiep tung trieu chung THAT, tra dung ca nguyen_nhan lan cach_xu_ly
assert tra_cuu_su_co("timeout_gateway") == {
    "nguyen_nhan": "request llm dai hon timeout mac dinh",
    "cach_xu_ly": "tang timeout va chuyen sang streaming",
}, f"tra_cuu_su_co('timeout_gateway') sai -- dang ra {tra_cuu_su_co('timeout_gateway')}"
assert tra_cuu_su_co("chi_phi_tang_bat_thuong")["cach_xu_ly"] == "them semantic cache va prompt caching", "cach_xu_ly cua chi_phi_tang_bat_thuong sai"

# bien: trieu chung la KHAC (khong phai loi_la_chua_gap_bao_gio) van phai tra ve DUNG mac dinh, khong nem loi
assert tra_cuu_su_co("gpu_qua_nong") == {
    "nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them",
}, f"trieu chung la BAT KY phai tra ve dung mac dinh -- dang ra {tra_cuu_su_co('gpu_qua_nong')}"

# bien: danh sach rong phai cho ket qua rong, khong loi
assert tra_cuu_nhieu_trieu_chung([]) == [], "danh sach rong phai cho ket qua rong"

# bien: MOI trieu chung trong BANG_SU_CO deu duoc tra_cuu_su_co nhan dung, khong mot cai nao bi bao la
for trieu_chung_that in BANG_SU_CO:
    assert tra_cuu_su_co(trieu_chung_that)["nguyen_nhan"] != "khong_xac_dinh", f"{trieu_chung_that} la trieu chung THAT, khong duoc bao la khong_xac_dinh"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `tra_cuu_su_co`) là GIÁ TRỊ TRẢ VỀ CUỐI CÙNG — một lời gọi `.get` với HAI đối số. Chỗ hai (trong `tra_cuu_nhieu_trieu_chung`) cũng là GIÁ TRỊ TRẢ VỀ — một `list comprehension` gọi LẠI hàm vừa viết cho TỪNG phần tử.
- kind: strategy
  body: 'Chỗ đầu: `BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})` — đối số thứ hai của `.get` LÀ giá trị mặc định, trả về khi khoá không tồn tại, KHÔNG ném lỗi. Chỗ hai: `[tra_cuu_su_co(t) for t in danh_sach_trieu_chung]` — áp hàm `tra_cuu_su_co` cho TỪNG triệu chứng `t` trong danh sách.'
- kind: one-line
  body: 'Chỗ đầu là `BANG_SU_CO.get(trieu_chung, {"nguyen_nhan": "khong_xac_dinh", "cach_xu_ly": "can kiem tra them"})`, chỗ hai là `[tra_cuu_su_co(t) for t in danh_sach_trieu_chung]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai dung BANG_SU_CO.get(trieu_chung, {mac dinh}) (KHONG duoc dung [] truc tiep, se nem KeyError cho trieu chung la); cho trong hai phai GOI LAI tra_cuu_su_co(t) cho TUNG phan tu qua list comprehension (khong duoc tra ve thang BANG_SU_CO hay danh sach rong)
  requireAst:
  - kind: uses-call, target: "get", min: 1
  - kind: uses-call, target: tra_cuu_su_co, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # CHINH XAC khoi solution cua file nay) -- xac nhan DUNG CHINH XAC (min
  # VA min+1): get=1, tra_cuu_su_co=1.
  # get=1: DUY NHAT o cho trong dau -- khong noi nao khac trong solution
  # goi phuong thuc .get nao (BANG_SU_CO chi duoc doc qua .get o day, va
  # khong co dict nao khac trong bai goi .get).
  # tra_cuu_su_co=1 (TONG THAT, khong phai boilerplate lon): DUY NHAT o cho
  # trong hai (list comprehension trong tra_cuu_nhieu_trieu_chung) --
  # tra_cuu_su_co KHONG tu goi lai chinh no (khong de quy), va khong co
  # dong demo nao goi rieng le tra_cuu_su_co ben ngoai hai ham.
  # Dien bua "True" vao ca hai cho trong ("return True" va "return True")
  # cho get=0 VA tra_cuu_su_co=0 -- CA HAI luat CHAN DUNG (da CHAY THAT xac
  # nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien "[tra_cuu_su_co(t) for t in
  # danh_sach_trieu_chung]" vao cho trong dau ("return [tra_cuu_su_co(t) for
  # t in danh_sach_trieu_chung]" trong tra_cuu_su_co) VA dien
  # "BANG_SU_CO.get(trieu_chung, {...})" vao cho trong hai ("return
  # BANG_SU_CO.get(trieu_chung, {...})" trong tra_cuu_nhieu_trieu_chung) --
  # da CHAY THAT qua kiemAst(): tong so lan goi get VA tong so lan goi
  # tra_cuu_su_co tren TOAN BO solution DEU KHONG DOI (van dung 1 va 1, chi
  # doi VI TRI) -- static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong tra_cuu_su_co (tham so la
  # trieu_chung -- KHONG CO "danh_sach_trieu_chung" nao trong scope nay),
  # bieu thuc moi doc ten "danh_sach_trieu_chung" CHUA TON TAI -- da tu
  # chay THAT qua python3, xac nhan NameError "name 'danh_sach_trieu_chung'
  # is not defined" ngay khi tra_cuu_su_co duoc goi lan dau (tu chinh
  # tra_cuu_nhieu_trieu_chung). Ben trong tra_cuu_nhieu_trieu_chung (tham so
  # la danh_sach_trieu_chung -- KHONG CO "trieu_chung" don le nao trong
  # scope nay), bieu thuc moi "return BANG_SU_CO.get(trieu_chung, {...})"
  # doc ten "trieu_chung" CHUA TON TAI -- da tu chay THAT xac nhan NameError
  # "name 'trieu_chung' is not defined". Ca hai bi chan boi tier 'run',
  # doc lap voi static.
  # Da tu ra soat GOTCHA #6: "trieu_chung" va "danh_sach_trieu_chung" la hai
  # ten THAM SO khac nhau cua hai ham khac nhau, khong trung nhau -- khong
  # co rui ro nham lan tinh co ve HINH DANG.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\['batch size hoac do dai ngu canh qua lon', 'request xu ly tuan tu, khong gop lo', 'khong co cache, prompt lap lai nhieu lan', 'chunking kem hoac thieu buoc rerank', 'request llm dai hon timeout mac dinh', 'khong_xac_dinh'\\]\\n1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đúng `5` nguyên nhân khớp bảng, đúng `1` triệu chứng lạ được gắn cờ
`"khong_xac_dinh"` — không đoán mò, không crash. Bốn mảnh của `q8.6d` đã
xong: sự kiện tác tử, semantic cache, hàng đợi ưu tiên, tra cứu sự cố. BOSS
quý ráp CẢ BỐN thành một luồng "vận hành một ngày".
::::

::::reflect{#nghi-lai}
`tra_cuu_su_co` không "chẩn đoán" gì cả — nó chỉ tra một bảng đã viết SẴN,
đúng khuôn `.get(khoa, mac_dinh)` đã dùng ở `q8.6a` bài `1` và `q8.6b` bài
`3`. Điểm quan trọng là THÁI ĐỘ với dữ liệu LẠ: một triệu chứng chưa từng
gặp KHÔNG làm hệ thống crash — nó trả về một tín hiệu TƯỜNG MINH
(`"khong_xac_dinh"`) để người vận hành biết "đây là ca CHƯA CÓ trong sổ
tay", thay vì một `KeyError` bí ẩn giữa lúc hệ thống đang gặp sự cố thật.
Đo được cụ thể trên `6` triệu chứng: đúng `5` khớp bảng, đúng `1` lạ — con
số đó không đổi dù triệu chứng lạ LÀ gì (`"loi_la_chua_gap_bao_gio"` hay
`"gpu_qua_nong"`), miễn nó không có trong `BANG_SU_CO`.

Bốn bài của `q8.6d` đã mô phỏng bốn mảnh riêng của `Chương 39`: sự kiện tác
tử (§39.5), semantic cache (Bài tập `2`), hàng đợi ưu tiên (Bài tập `3`),
và bảng tra cứu sự cố (Troubleshooting cuối chương). BOSS quý — bài cuối
cùng — ráp CẢ BỐN thành một hàm mô phỏng "vận hành một ngày" của hệ thống,
đóng `q8.6d` tại `5/5`.
::::

::::checkpoint{mastery=0.82}
::::
