---
id: tri-tue-nhan-tao.ha-tang-suy-luan.kien-truc-ba-tang-ai-stack
title: "Kiến trúc 3 tầng của AI Stack: Inference, Gateway, Orchestration"
summary: "phan_loai_tang(ten_thanh_phan) tra ve BANG_TANG.get(ten_thanh_phan, 'khong_xac_dinh') -- MOT bang tra cuu CO DINH, so khop CHINH XAC ten, khong suy luan ngu nghia. dem_theo_tang(danh_sach_ten) dem so thanh phan moi tang bang cach goi phan_loai_tang cho tung ten. HE_THONG_MAU gom 9 thanh phan (vLLM, FastAPI Gateway, RAG Pipeline, TensorRT-LLM, Redis Rate Limiter, Prompt Template Engine, Ollama, Xac Thuc API Key, Dieu Phoi Agent) duoc phan DUNG 3/3/3 vao ba tang inference/gateway/orchestration. phan_loai_tang('Thanh Phan La') (ten khong co trong bang) tra ve 'khong_xac_dinh' -- bang tra cuu khong doan ten la gi, chi tra dung nhung gi da khai bao san."
locale: vi
track: tri-tue-nhan-tao
module: ha-tang-suy-luan
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.kien-truc-ba-tang-ai-stack]
requires: [ai.boss-rag-tu-so-0]
concepts: [ai.kien-truc-ba-tang-ai-stack]
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
`T8.5` đóng tại `34/34`: chunking, HNSW, BM25, hybrid, prompt — cả một RAG
pipeline chạy được TỪ SỐ 0. `T8.6` (track CUỐI CÙNG của `R8`) hỏi một câu
khác hẳn: pipeline ĐÓ chạy được trên MÁY CỦA BẠN, nhưng phục vụ HÀNG NGHÌN
người dùng cùng lúc thì cần THÊM những gì? Câu trả lời không phải "thêm
thuật toán" — mà LÀ một cách TỔ CHỨC lại chính những gì đã học.
::::

::::explain{#ba-tang-ai-stack}
Một hệ thống AI production thật KHÔNG phải một file `script.py` gọi thẳng
LLM. Nó chia thành ba tầng, mỗi tầng một trách nhiệm RIÊNG:

1. **Inference Layer** — tầng tính toán GPU thấp nhất. Đây LÀ nơi mô hình
   thật sự CHẠY (`vLLM`, `Ollama`, `TensorRT-LLM`, `Triton Inference
   Server`).
2. **AI Gateway Layer** — đứng TRƯỚC tầng Inference, làm nhiệm vụ bảo mật
   VÀ định tuyến (`FastAPI Gateway`, `Redis Rate Limiter`, xác thực API
   key).
3. **Orchestration Layer** — quản lý LOGIC nghiệp vụ: RAG pipeline,
   reranking, quản lý prompt (đúng những gì `T8.4`/`T8.5` đã xây).

Ba tầng này không phải một BẢNG PHÂN LOẠI cần "hiểu" tên gọi — nó LÀ một
**bảng tra cứu cố định** (table lookup). Cho một tên thành phần, tra NÓ
trong bảng, trả về ĐÚNG tầng đã khai báo — không suy luận "tên nghe giống
gì", không đoán theo ngữ nghĩa của chuỗi ký tự:

```
phan_loai_tang(ten_thanh_phan) = BANG_TANG.get(ten_thanh_phan, "khong_xac_dinh")
```

`BANG_TANG` là một `dict` CỐ ĐỊNH, viết tay một lần: mỗi tên thành phần ánh
xạ THẲNG tới đúng MỘT tầng trong ba tầng. `.get(..., "khong_xac_dinh")` là
điểm mấu chốt: một tên KHÔNG có trong bảng không làm chương trình NỔ (như
`BANG_TANG[ten_thanh_phan]` sẽ làm với `KeyError`) — nó trả về một giá trị
mặc định TƯỜNG MINH, nói THẲNG rằng "bảng này chưa biết thành phần này",
thay vì đoán bừa một tầng nào đó.
::::

::::example{#dem_theo_tang_chin_thanh_phan}
```python title=readonly
BANG_TANG = {
    "vLLM": "inference",
    "TensorRT-LLM": "inference",
    "Ollama": "inference",
    "FastAPI Gateway": "gateway",
    "Redis Rate Limiter": "gateway",
    "Xac Thuc API Key": "gateway",
    "RAG Pipeline": "orchestration",
    "Prompt Template Engine": "orchestration",
    "Dieu Phoi Agent": "orchestration",
}


def phan_loai_tang(ten_thanh_phan):
    return BANG_TANG.get(ten_thanh_phan, "khong_xac_dinh")


def dem_theo_tang(danh_sach_ten):
    dem = {"inference": 0, "gateway": 0, "orchestration": 0, "khong_xac_dinh": 0}
    for ten in danh_sach_ten:
        tang = phan_loai_tang(ten)
        dem[tang] += 1
    return dem


HE_THONG_MAU = [
    "vLLM", "FastAPI Gateway", "RAG Pipeline",
    "TensorRT-LLM", "Redis Rate Limiter", "Prompt Template Engine",
    "Ollama", "Xac Thuc API Key", "Dieu Phoi Agent",
]

ket_qua = dem_theo_tang(HE_THONG_MAU)

print(ket_qua)
print(phan_loai_tang("vLLM"))
print(phan_loai_tang("Thanh Phan La"))
```

```text title=readonly
{'inference': 3, 'gateway': 3, 'orchestration': 3, 'khong_xac_dinh': 0}
inference
khong_xac_dinh
```

`HE_THONG_MAU` gồm đúng `9` thành phần — cố ý xếp `3` thành phần cho MỖI
tầng, nhưng liệt kê XEN KẼ thứ tự (không nhóm theo tầng) để `dem_theo_tang`
phải THẬT SỰ tra bảng cho từng cái một, không thể "đếm theo khối". Kết quả
`{'inference': 3, 'gateway': 3, 'orchestration': 3, 'khong_xac_dinh': 0}`
xác nhận: đúng `3` thành phần rơi vào MỖI tầng, và không thành phần nào bị
xếp sai. `phan_loai_tang("Thanh Phan La")` — một tên KHÔNG có trong
`BANG_TANG` — trả về `"khong_xac_dinh"` ngay lập tức, không hề "đoán" xem
cái tên lạ đó CÓ THỂ thuộc tầng nào.
::::

::::predict{#doan_ollama_server commitOnce}
`BANG_TANG` có khoá `"Ollama"` (ánh xạ tới `"inference"`), nhưng KHÔNG có
khoá `"Ollama Server"` — một tên gần giống, chỉ thêm một từ.

**Trước khi chạy thử**, bạn đoán: `phan_loai_tang("Ollama Server")` trả về
gì?

:::opt{correct}
`"khong_xac_dinh"` — `BANG_TANG.get(...)` so khớp CHÍNH XÁC từng ký tự của
chuỗi khoá; `"Ollama Server"` và `"Ollama"` LÀ hai chuỗi khác nhau (khác độ
dài, khác nội dung), nên `.get()` không tìm thấy khoá nào khớp và trả về
giá trị mặc định
:::

:::opt
`"inference"` — vì `"Ollama Server"` trông giống một biến thể của `"Ollama"`
(cùng một hệ thống, chỉ thêm từ mô tả), nên nó vẫn nên được xếp cùng tầng
::why
Gần đúng ở trực giác rằng `"Ollama Server"` và `"Ollama"` CÙNG nói về một
sản phẩm thật ngoài đời — quan sát đó hợp lý cho MỘT NGƯỜI đọc, người biết
suy luận NGỮ NGHĨA giữa hai tên gần giống nhau.

Chỗ lệch: `phan_loai_tang` KHÔNG phải một người đọc — nó LÀ một bảng tra
cứu CỨNG (`dict.get`), so khớp CHÍNH XÁC chuỗi ký tự, không hề "hiểu" rằng
hai tên có liên quan. Đây chính LÀ điểm khác biệt cốt lõi giữa TRA BẢNG (mà
bài này dạy) và một hệ thống suy luận ngôn ngữ tự nhiên (mà bài này KHÔNG
làm) — thêm một khoảng trắng và một từ là đủ để trở thành một chuỗi hoàn
toàn khác trong mắt `dict.get`.
::
:::

:::opt
Chương trình sẽ NỔ với `KeyError`, vì `"Ollama Server"` không tồn tại trong
`BANG_TANG`
::why
Gần đúng ở việc `"Ollama Server"` THẬT SỰ không có trong `BANG_TANG` —
quan sát đó đúng.

Chỗ lệch: `KeyError` chỉ xảy ra với `BANG_TANG["Ollama Server"]` (truy cập
trực tiếp bằng dấu ngoặc vuông). `phan_loai_tang` dùng `BANG_TANG.get(...,
"khong_xac_dinh")` — phương thức `.get` với đối số thứ hai LÀ giá trị mặc
định KHÔNG BAO GIỜ ném lỗi khi khoá không tồn tại; nó trả về giá trị mặc
định đó một cách êm thấm.
::
:::
::::

::::code{#viet_phan_loai_va_dem_tang}
Hoàn thiện `phan_loai_tang` (tra `BANG_TANG` qua `.get`, mặc định
`"khong_xac_dinh"`) và `dem_theo_tang` (gọi `phan_loai_tang` cho từng tên,
cộng dồn vào bộ đếm đúng tầng).

```python title=starter
BANG_TANG = {
    "vLLM": "inference",
    "TensorRT-LLM": "inference",
    "Ollama": "inference",
    "FastAPI Gateway": "gateway",
    "Redis Rate Limiter": "gateway",
    "Xac Thuc API Key": "gateway",
    "RAG Pipeline": "orchestration",
    "Prompt Template Engine": "orchestration",
    "Dieu Phoi Agent": "orchestration",
}


def phan_loai_tang(ten_thanh_phan):
    return ___                                              # BANG_TANG.get(ten_thanh_phan, "khong_xac_dinh")


def dem_theo_tang(danh_sach_ten):
    dem = {"inference": 0, "gateway": 0, "orchestration": 0, "khong_xac_dinh": 0}
    for ten in danh_sach_ten:
        tang = ___                                          # phan_loai_tang(ten)
        dem[tang] += 1
    return dem


HE_THONG_MAU = [
    "vLLM", "FastAPI Gateway", "RAG Pipeline",
    "TensorRT-LLM", "Redis Rate Limiter", "Prompt Template Engine",
    "Ollama", "Xac Thuc API Key", "Dieu Phoi Agent",
]

ket_qua = dem_theo_tang(HE_THONG_MAU)

print(ket_qua)
print(phan_loai_tang("vLLM"))
print(phan_loai_tang("Thanh Phan La"))
```

```python title=solution
BANG_TANG = {
    "vLLM": "inference",
    "TensorRT-LLM": "inference",
    "Ollama": "inference",
    "FastAPI Gateway": "gateway",
    "Redis Rate Limiter": "gateway",
    "Xac Thuc API Key": "gateway",
    "RAG Pipeline": "orchestration",
    "Prompt Template Engine": "orchestration",
    "Dieu Phoi Agent": "orchestration",
}


def phan_loai_tang(ten_thanh_phan):
    return BANG_TANG.get(ten_thanh_phan, "khong_xac_dinh")


def dem_theo_tang(danh_sach_ten):
    dem = {"inference": 0, "gateway": 0, "orchestration": 0, "khong_xac_dinh": 0}
    for ten in danh_sach_ten:
        tang = phan_loai_tang(ten)
        dem[tang] += 1
    return dem


HE_THONG_MAU = [
    "vLLM", "FastAPI Gateway", "RAG Pipeline",
    "TensorRT-LLM", "Redis Rate Limiter", "Prompt Template Engine",
    "Ollama", "Xac Thuc API Key", "Dieu Phoi Agent",
]

ket_qua = dem_theo_tang(HE_THONG_MAU)

print(ket_qua)
print(phan_loai_tang("vLLM"))
print(phan_loai_tang("Thanh Phan La"))
```

```python title=test
kq_test1 = dem_theo_tang(["vLLM", "FastAPI Gateway", "RAG Pipeline"])
assert kq_test1 == {"inference": 1, "gateway": 1, "orchestration": 1, "khong_xac_dinh": 0}, f"dem_theo_tang phai dem DUNG 1 cho moi tang tren 3 thanh phan nay -- dang ra {kq_test1}"

kq_rong = dem_theo_tang([])
assert kq_rong == {"inference": 0, "gateway": 0, "orchestration": 0, "khong_xac_dinh": 0}, f"danh sach rong phai cho MOI tang bang 0 -- dang ra {kq_rong}"

assert phan_loai_tang("vLLM") == "inference", f"vLLM phai duoc xep vao inference -- dang ra {phan_loai_tang('vLLM')}"
assert phan_loai_tang("FastAPI Gateway") == "gateway", f"FastAPI Gateway phai duoc xep vao gateway -- dang ra {phan_loai_tang('FastAPI Gateway')}"
assert phan_loai_tang("RAG Pipeline") == "orchestration", f"RAG Pipeline phai duoc xep vao orchestration -- dang ra {phan_loai_tang('RAG Pipeline')}"
assert phan_loai_tang("Thanh Phan La") == "khong_xac_dinh", f"ten khong co trong BANG_TANG phai tra ve khong_xac_dinh -- dang ra {phan_loai_tang('Thanh Phan La')}"

assert ket_qua == {"inference": 3, "gateway": 3, "orchestration": 3, "khong_xac_dinh": 0}, f"HE_THONG_MAU co 9 thanh phan (3 moi tang) phai cho ket qua 3/3/3 -- dang ra {ket_qua}"
```

:::hints
- kind: attention
  body: "Hai cho trong, o hai ham khac nhau. Cho dau (trong phan_loai_tang) la GIA TRI TRA VE cua ham -- tra BANG_TANG bang .get, kem gia tri mac dinh khi khong tim thay. Cho hai (trong dem_theo_tang) la GIA TRI DUOC GAN cho bien tang trong vong lap -- goi lai chinh ham phan_loai_tang vua viet o cho dau."
- kind: strategy
  body: "Cho dau: BANG_TANG.get(ten_thanh_phan, \"khong_xac_dinh\") -- dung .get voi hai doi so, doi so hai la gia tri mac dinh khi khoa khong ton tai. Cho hai: phan_loai_tang(ten) -- goi ham vua dinh nghia, truyen bien vong lap ten."
- kind: one-line
  body: "Cho dau la BANG_TANG.get(ten_thanh_phan, \"khong_xac_dinh\"), cho hai la phan_loai_tang(ten)."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai dung BANG_TANG.get(ten_thanh_phan, "khong_xac_dinh") (tra bang qua .get, KHONG dung [] truc tiep); cho trong hai phai GOI LAI phan_loai_tang(ten) -- khong duoc tra bang truc tiep o day
  requireAst:
  - kind: uses-call, target: "get", min: 1
  - kind: uses-call, target: "phan_loai_tang", min: 3
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh file
  # nay) -- ket qua dung nhu du kien:
  #   "get"=1: DUY NHAT mot lan trong toan bo solution, o cho trong dau
  #   (BANG_TANG.get(...)). Khong noi nao khac trong file goi .get.
  #   "phan_loai_tang"=3: 1 lan o cho trong hai (ben trong dem_theo_tang),
  #   CONG 2 lan o phan demo ben ngoai ham (goi truc tiep
  #   phan_loai_tang("vLLM") va phan_loai_tang("Thanh Phan La")) -- tong
  #   THAT la 3, dat dung boilerplate-threshold (khong doan tay).
  # Dien bua "True" vao CA HAI cho trong (thay ___ bang True) cho ca hai
  # dem ve 0/0 -- duoi ca hai nguong toi thieu, static CHAN DUNG.
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong va
  # CHAY THAT qua kiemAst): dien "phan_loai_tang(ten)" vao cho trong dau
  # ("return phan_loai_tang(ten)" trong phan_loai_tang) VA dien
  # "BANG_TANG.get(ten_thanh_phan, \"khong_xac_dinh\")" vao cho trong hai
  # ("tang = BANG_TANG.get(ten_thanh_phan, ...)" trong dem_theo_tang) --
  # tong so lan "get" VA "phan_loai_tang" trong TOAN BO solution KHONG DOI
  # (van la 1 va 3, chi doi VI TRI hai bieu thuc cho nhau) -- static KHONG
  # bat duoc mutant nay, da xac nhan CHAY THAT qua kiemAst.
  # Mutant nay BI BAT boi tier 'run': ben trong phan_loai_tang (tham so la
  # ten_thanh_phan), bieu thuc moi "phan_loai_tang(ten)" dung ten 'ten' --
  # CHUA HE TON TAI trong scope cua phan_loai_tang -- NameError ngay khi ham
  # duoc goi lan dau (tu dem_theo_tang, dong "tang = phan_loai_tang(ten)"
  # sau khi da bi hoan doi thanh goi BANG_TANG.get(ten_thanh_phan,...) --
  # ten_thanh_phan cung KHONG TON TAI trong scope cua dem_theo_tang, bien
  # vong lap o do ten la 'ten'). Da tu chay THAT ca hai huong hoan doi qua
  # python3: xac nhan NameError "name 'ten_thanh_phan' is not defined" ngay
  # khi dem_theo_tang(["vLLM"]) duoc goi -- bi chan boi tier 'run', doc lap
  # voi static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{'inference': 3, 'gateway': 3, 'orchestration': 3, 'khong_xac_dinh': 0}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3/3/3` — mọi thành phần rơi ĐÚNG tầng, và một cái tên lạ trả về
`"khong_xac_dinh"` thay vì đoán liều. Ba tầng đã được ĐẶT TÊN. Câu hỏi tiếp
theo LÀ: bên trong tầng `Inference`, tại sao gọi model TUẦN TỰ — một
request tại một thời điểm — lại lãng phí đến vậy?
::::

::::reflect{#nghi-lai}
`phan_loai_tang` không "biết" gì về AI cả — nó chỉ tra một `dict` cố định.
Đó chính LÀ điểm quan trọng: kiến trúc 3 tầng của một hệ AI, dù nghe có vẻ
trừu tượng, hoàn toàn có thể mô tả bằng một PHÉP TRA BẢNG tất định. Không
cần "hiểu" `vLLM` là gì để biết nó thuộc tầng `Inference` — chỉ cần bảng
tra cứu đã khai báo đúng một lần. Bài tiếp theo rời khỏi câu hỏi "thành
phần nào thuộc tầng nào" để hỏi câu hỏi vận hành đầu tiên bên TRONG tầng
`Inference`: xử lý các request MỘT CÁCH TUẦN TỰ tốn kém tới mức nào?
::::

::::checkpoint{mastery=0.75}
::::
