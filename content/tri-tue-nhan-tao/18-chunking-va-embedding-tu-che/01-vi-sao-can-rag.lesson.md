---
id: tri-tue-nhan-tao.chunking-va-embedding-tu-che.vi-sao-can-rag
title: "Vì sao cần RAG: kiến thức đóng băng, ngữ cảnh có trần"
summary: "Mot LLM mo phong tra loi cau hoi ve 'san pham x' (mot su kien XAY RA SAU thoi diem 'huan luyen') qua bang tra cuu KIEN_THUC_DA_HOC co dinh (chi co 3 muc, khong chua san pham x). KHONG co ngu canh: tra loi ao giac co dinh 'toi nghi la khoang 10000000 dong' (SAI). CO ngu canh (mot doan tim duoc tu KHO_TAI_LIEU, dua vao TRUOC khi hoi): trich dung so that tu doan do, tra loi 'con so that (15000000 dong)' (DUNG). Cung MOT cau hoi, hai ket qua doi lap — do bang so, khong suy doan."
locale: vi
track: tri-tue-nhan-tao
module: chunking-va-embedding-tu-che
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.vi-sao-can-rag]
requires: [ai.boss-tro-ly-dung-llm-dung-cach]
concepts: [ai.vi-sao-can-rag]
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
T8.4 khép lại ở `28/28`: kiểm ảo giác, chấm điểm, giám khảo mô phỏng, chống
injection, che PII. T8.5 mở ra với một câu hỏi mới: nếu LLM không biết một
sự kiện, làm sao cho nó biết mà KHÔNG cần huấn luyện lại?
::::

::::explain{#kien_thuc_dong_bang_va_cua_so_co_tran}
Một LLM (mô phỏng, như xuyên suốt track này) học từ một khối dữ liệu CỐ ĐỊNH
tại một thời điểm nào đó — mọi thứ xảy ra SAU thời điểm đó, nó không thể
biết, vì kiến thức của nó đã "đóng băng" (frozen) từ lúc huấn luyện xong. Bài
`ao-giac-va-kiem-tra-ngu-canh` (q8.4e) đã đo điều này: hỏi ngoài phạm vi kiến
thức, LLM mô phỏng không từ chối mà BỊA một câu trả lời nghe hợp lý — đó là
ảo giác (hallucination).

Bài `dem-token-that` và `cua-so-ngu-canh-va-cat-bot` (q8.4d) đo thêm một giới
hạn khác: một lượt gọi LLM chỉ nhét được HỮU HẠN token vào ngữ cảnh (context
window). Hai giới hạn này cộng lại thành một vấn đề: không thể vừa "dạy lại"
LLM mỗi khi có tin mới (tốn kém, chậm), vừa không thể nhét NGUYÊN VẸN mọi tài
liệu liên quan vào một lượt hỏi (vượt trần token).

**RAG** (Retrieval-Augmented Generation — "sinh có tăng cường bằng truy
xuất") giải quyết bằng một ý tưởng đơn giản: đừng bắt LLM nhớ HẾT mọi thứ.
Thay vào đó, giữ tri thức trong một **kho tài liệu** (document store) bên
NGOÀI mô hình. Khi có câu hỏi:

> **1. Truy xuất (retrieve)** — tìm đoạn văn bản LIÊN QUAN nhất trong kho tài
> liệu cho câu hỏi đó.
>
> **2. Tăng cường (augment)** — đưa đoạn văn bản đó vào ngữ cảnh, NGAY TRƯỚC
> khi hỏi LLM.
>
> **3. Sinh (generate)** — LLM trả lời DỰA TRÊN đoạn văn bản vừa được đưa
> vào, không phải chỉ dựa vào trí nhớ nội tại đã đóng băng.

Toàn bộ track `T8.5` xây từng mảnh của quy trình ba bước này, TỰ CÀI hoàn
toàn (không gọi mô hình embedding thật đã huấn luyện trước — sandbox không có
mạng). Bài này đo bước quan trọng nhất trước tiên: có/không có bước truy
xuất tạo ra khác biệt gì trên CÙNG một câu hỏi.
::::

::::example{#do_ao_giac_khi_khong_co_ngu_canh}
Một LLM mô phỏng với kiến thức đóng băng nhỏ (`KIEN_THUC_DA_HOC`, chỉ `3`
mục) và một kho tài liệu ngoài mô hình (`KHO_TAI_LIEU`) chứa thông tin MỚI
hơn — hỏi về một sản phẩm không có trong kiến thức đóng băng:

```python title=readonly
import re

KIEN_THUC_DA_HOC = {
    "thu do phap": "paris",
    "thu do nhat ban": "tokyo",
    "so ngay trong tuan": "7",
}

KHO_TAI_LIEU = {
    "san_pham_x": "san pham x ra mat thang 3 nam 2025, gia ban le la 15000000 dong.",
    "san_pham_y": "san pham y ngung san xuat tu thang 6 nam 2024.",
}


def tim_doan_lien_quan(cau_hoi, kho_tai_lieu):
    cau_thuong = cau_hoi.lower()
    for ten_tai_lieu, noi_dung in kho_tai_lieu.items():
        tu_khoa = ten_tai_lieu.replace("_", " ")
        if tu_khoa in cau_thuong:
            return noi_dung
    return None


def tra_loi_khong_ngu_canh(cau_hoi):
    cau_thuong = cau_hoi.lower()
    for khoa, gia_tri in KIEN_THUC_DA_HOC.items():
        if khoa in cau_thuong:
            return gia_tri
    return "toi nghi la khoang 10000000 dong"


def trich_so_cuoi(van_ban):
    cac_so = re.findall(r"\d+", van_ban)
    return cac_so[-1] if cac_so else None


def tra_loi_co_ngu_canh(cau_hoi, ngu_canh):
    if ngu_canh is None:
        return tra_loi_khong_ngu_canh(cau_hoi)
    gia = trich_so_cuoi(ngu_canh)
    if gia is None:
        return tra_loi_khong_ngu_canh(cau_hoi)
    return gia + " dong"


CAU_HOI = "gia san pham x la bao nhieu"
DAP_AN_DUNG = "15000000 dong"

tra_loi_sai = tra_loi_khong_ngu_canh(CAU_HOI)
doan_lien_quan = tim_doan_lien_quan(CAU_HOI, KHO_TAI_LIEU)
tra_loi_dung = tra_loi_co_ngu_canh(CAU_HOI, doan_lien_quan)

print(tra_loi_sai)
print(doan_lien_quan)
print(tra_loi_dung)
print(tra_loi_sai == DAP_AN_DUNG, tra_loi_dung == DAP_AN_DUNG)
```

```text title=readonly
toi nghi la khoang 10000000 dong
san pham x ra mat thang 3 nam 2025, gia ban le la 15000000 dong.
15000000 dong
False True
```

`KIEN_THUC_DA_HOC` (kiến thức "đã học lúc huấn luyện") không có mục nào về
`"san pham x"` — hỏi trực tiếp, `tra_loi_khong_ngu_canh` không tìm thấy khoá
nào khớp, rơi vào nhánh cuối: bịa một con số cố định `"toi nghi la khoang
10000000 dong"`, SAI so với đáp án đúng `"15000000 dong"`. Khi TRUY XUẤT
trước — `tim_doan_lien_quan` tìm thấy đúng đoạn văn bản trong `KHO_TAI_LIEU`
chứa thông tin sản phẩm X (`"...gia ban le la 15000000 dong."`) — và đưa đoạn
đó vào `tra_loi_co_ngu_canh` làm ngữ cảnh, hàm trích được số THẬT nằm trong
đó, trả lời ĐÚNG. CÙNG một câu hỏi, khác nhau đúng MỘT thứ: có hay không có
đoạn văn bản liên quan trong ngữ cảnh trước khi trả lời.
::::

::::predict{#doan_tra_loi_khong_ngu_canh commitOnce}
Xét đúng ví dụ trên. `CAU_HOI = "gia san pham x la bao nhieu"` — sản phẩm X
ra mắt SAU thời điểm "huấn luyện" `KIEN_THUC_DA_HOC`, nên không có mục nào
khớp.

**Trước khi chạy thử**, bạn đoán: `tra_loi_khong_ngu_canh(CAU_HOI)` trả về
gì?

:::opt{correct}
`"toi nghi la khoang 10000000 dong"` — một câu trả lời BỊA cố định, SAI so
với giá thật (`15000000 dong`); hàm không tìm thấy khoá nào trong
`KIEN_THUC_DA_HOC` khớp với câu hỏi nên rơi thẳng vào nhánh cuối cùng, luôn
trả về đúng chuỗi đoán sẵn đó bất kể câu hỏi là gì
:::

:::opt
`"15000000 dong"` — vì đó là giá THẬT của sản phẩm X, và một LLM đủ "thông
minh" phải suy luận ra được con số đúng
::why
Gần đúng ở việc đó chính xác là con số THẬT — nhưng đề đang hỏi về hàm
`tra_loi_khong_ngu_canh`, một hàm KHÔNG nhận ngữ cảnh nào cả (chỉ nhận
`cau_hoi`). Nó không có cách nào "suy luận" ra một con số chưa từng xuất
hiện trong `KIEN_THUC_DA_HOC` — không có phép thuật nào giúp một bảng tra
cứu cố định chứa thông tin nó chưa từng có.
::
:::

:::opt
`"toi khong biet"` — vì một hệ thống AN TOÀN, khi không tìm thấy dữ liệu phù
hợp, phải từ chối trả lời thay vì đoán liều
::why
Gần đúng ở TRỰC GIÁC AN TOÀN — đó chính xác là hướng mà `kiem_tra_ao_giac`
của q8.4e đã dạy (từ chối khi câu trả lời nằm ngoài phạm vi cho phép).

Nhưng hàm `tra_loi_khong_ngu_canh` ở ĐÂY được viết đơn giản hơn, KHÔNG có
nhánh từ chối nào — nó chỉ có hai nhánh: khớp một khoá trong
`KIEN_THUC_DA_HOC`, hoặc rơi vào `return` cuối cùng — một chuỗi đoán CỐ ĐỊNH,
không phải một lời từ chối.
::
:::
::::

::::code{#viet_so_sanh_co_khong_rag}
Hoàn thiện `so_sanh_co_khong_rag`: TRUY XUẤT đoạn văn bản liên quan trước,
rồi dùng đúng đoạn đó làm ngữ cảnh khi hỏi lại.

```python title=starter
import re

KIEN_THUC_DA_HOC = {
    "thu do phap": "paris",
    "thu do nhat ban": "tokyo",
    "so ngay trong tuan": "7",
}

KHO_TAI_LIEU = {
    "san_pham_x": "san pham x ra mat thang 3 nam 2025, gia ban le la 15000000 dong.",
    "san_pham_y": "san pham y ngung san xuat tu thang 6 nam 2024.",
}


def tim_doan_lien_quan(cau_hoi, kho_tai_lieu):
    cau_thuong = cau_hoi.lower()
    for ten_tai_lieu, noi_dung in kho_tai_lieu.items():
        tu_khoa = ten_tai_lieu.replace("_", " ")
        if tu_khoa in cau_thuong:
            return noi_dung
    return None


def tra_loi_khong_ngu_canh(cau_hoi):
    cau_thuong = cau_hoi.lower()
    for khoa, gia_tri in KIEN_THUC_DA_HOC.items():
        if khoa in cau_thuong:
            return gia_tri
    return "toi nghi la khoang 10000000 dong"


def trich_so_cuoi(van_ban):
    cac_so = re.findall(r"\d+", van_ban)
    return cac_so[-1] if cac_so else None


def tra_loi_co_ngu_canh(cau_hoi, ngu_canh):
    if ngu_canh is None:
        return tra_loi_khong_ngu_canh(cau_hoi)
    gia = trich_so_cuoi(ngu_canh)
    if gia is None:
        return tra_loi_khong_ngu_canh(cau_hoi)
    return gia + " dong"


def so_sanh_co_khong_rag(cau_hoi, dap_an_dung, kho_tai_lieu):
    tra_loi_khong = tra_loi_khong_ngu_canh(cau_hoi)
    doan = ___                                            # tim_doan_lien_quan(cau_hoi, kho_tai_lieu)
    tra_loi_co = ___                                       # tra_loi_co_ngu_canh(cau_hoi, doan)
    return {
        "khong_rag": tra_loi_khong,
        "co_rag": tra_loi_co,
        "khong_rag_dung": tra_loi_khong == dap_an_dung,
        "co_rag_dung": tra_loi_co == dap_an_dung,
    }


CAU_HOI = "gia san pham x la bao nhieu"
DAP_AN_DUNG = "15000000 dong"

ket_qua = so_sanh_co_khong_rag(CAU_HOI, DAP_AN_DUNG, KHO_TAI_LIEU)
print(ket_qua["khong_rag"])
print(ket_qua["co_rag"])
print(ket_qua["khong_rag_dung"], ket_qua["co_rag_dung"])
```

```python title=solution
import re

KIEN_THUC_DA_HOC = {
    "thu do phap": "paris",
    "thu do nhat ban": "tokyo",
    "so ngay trong tuan": "7",
}

KHO_TAI_LIEU = {
    "san_pham_x": "san pham x ra mat thang 3 nam 2025, gia ban le la 15000000 dong.",
    "san_pham_y": "san pham y ngung san xuat tu thang 6 nam 2024.",
}


def tim_doan_lien_quan(cau_hoi, kho_tai_lieu):
    cau_thuong = cau_hoi.lower()
    for ten_tai_lieu, noi_dung in kho_tai_lieu.items():
        tu_khoa = ten_tai_lieu.replace("_", " ")
        if tu_khoa in cau_thuong:
            return noi_dung
    return None


def tra_loi_khong_ngu_canh(cau_hoi):
    cau_thuong = cau_hoi.lower()
    for khoa, gia_tri in KIEN_THUC_DA_HOC.items():
        if khoa in cau_thuong:
            return gia_tri
    return "toi nghi la khoang 10000000 dong"


def trich_so_cuoi(van_ban):
    cac_so = re.findall(r"\d+", van_ban)
    return cac_so[-1] if cac_so else None


def tra_loi_co_ngu_canh(cau_hoi, ngu_canh):
    if ngu_canh is None:
        return tra_loi_khong_ngu_canh(cau_hoi)
    gia = trich_so_cuoi(ngu_canh)
    if gia is None:
        return tra_loi_khong_ngu_canh(cau_hoi)
    return gia + " dong"


def so_sanh_co_khong_rag(cau_hoi, dap_an_dung, kho_tai_lieu):
    tra_loi_khong = tra_loi_khong_ngu_canh(cau_hoi)
    doan = tim_doan_lien_quan(cau_hoi, kho_tai_lieu)
    tra_loi_co = tra_loi_co_ngu_canh(cau_hoi, doan)
    return {
        "khong_rag": tra_loi_khong,
        "co_rag": tra_loi_co,
        "khong_rag_dung": tra_loi_khong == dap_an_dung,
        "co_rag_dung": tra_loi_co == dap_an_dung,
    }


CAU_HOI = "gia san pham x la bao nhieu"
DAP_AN_DUNG = "15000000 dong"

ket_qua = so_sanh_co_khong_rag(CAU_HOI, DAP_AN_DUNG, KHO_TAI_LIEU)
print(ket_qua["khong_rag"])
print(ket_qua["co_rag"])
print(ket_qua["khong_rag_dung"], ket_qua["co_rag_dung"])
```

```python title=test
assert ket_qua["khong_rag"] == "toi nghi la khoang 10000000 dong", f"khong_rag phai la cau ao giac co dinh -- dang ra {ket_qua['khong_rag']}"
assert ket_qua["co_rag"] == "15000000 dong", f"co_rag phai la gia THAT trich tu ngu canh -- dang ra {ket_qua['co_rag']}"
assert ket_qua["khong_rag_dung"] == False, "khong_rag_dung phai la False -- khong co ngu canh thi tra loi SAI"
assert ket_qua["co_rag_dung"] == True, "co_rag_dung phai la True -- co ngu canh thi tra loi DUNG"

# bien 1: cau hoi ve mot su kien DA co san trong KIEN_THUC_DA_HOC -- dung ca
# hai che do, vi khong can RAG cho nhung gi da biet tu truoc
kq_da_biet = so_sanh_co_khong_rag("thu do phap la gi", "paris", KHO_TAI_LIEU)
assert kq_da_biet["khong_rag"] == "paris", f"cau hoi da co san kien thuc phai dung ngay ca khong RAG -- dang ra {kq_da_biet['khong_rag']}"
assert kq_da_biet["co_rag"] == "paris", f"co RAG khong duoc lam hong cau tra loi da dung san -- dang ra {kq_da_biet['co_rag']}"
assert kq_da_biet["khong_rag_dung"] == True and kq_da_biet["co_rag_dung"] == True, "ca hai che do phai dung voi cau hoi da co san kien thuc"

# bien 2: cau hoi ve mot san pham KHONG co trong CA KIEN_THUC_DA_HOC LAN
# KHO_TAI_LIEU -- RAG khong the giup neu khong co tai lieu lien quan de tim
kq_khong_co_tai_lieu = so_sanh_co_khong_rag("gia san pham z la bao nhieu", "20000000 dong", KHO_TAI_LIEU)
assert kq_khong_co_tai_lieu["co_rag"] == "toi nghi la khoang 10000000 dong", f"khong co tai lieu lien quan thi RAG cung khong giup duoc -- dang ra {kq_khong_co_tai_lieu['co_rag']}"
assert kq_khong_co_tai_lieu["co_rag_dung"] == False, "khong tim thay tai lieu lien quan thi co_rag_dung phai la False"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu phải TRUY XUẤT (retrieve) đoạn văn bản liên quan từ `kho_tai_lieu` cho đúng `cau_hoi` — dùng lại hàm `tim_doan_lien_quan` đã có sẵn ở trên. Chỗ hai phải dùng ĐÚNG đoạn vừa tìm được (biến `doan`) làm ngữ cảnh khi gọi `tra_loi_co_ngu_canh` — không được gọi `tra_loi_khong_ngu_canh` ở đây, vì như vậy sẽ bỏ qua hoàn toàn bước truy xuất.
- kind: strategy
  body: 'Chỗ đầu: `tim_doan_lien_quan(cau_hoi, kho_tai_lieu)`. Chỗ hai: `tra_loi_co_ngu_canh(cau_hoi, doan)` — đối số thứ hai PHẢI là biến `doan` vừa gán ở chỗ đầu, không phải `None` hay một chuỗi khác.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `tim_doan_lien_quan(cau_hoi, kho_tai_lieu)` và `tra_loi_co_ngu_canh(cau_hoi, doan)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la mot loi GOI THAT toi tim_doan_lien_quan (khong duoc bo qua buoc truy xuat), VA cho trong hai phai dung DUNG bien 'doan' vua tim duoc lam ngu canh khi goi tra_loi_co_ngu_canh (khong duoc goi tra_loi_khong_ngu_canh thay the)
  requireAst:
  - kind: uses-call, target: tim_doan_lien_quan, min: 1
  - kind: uses-call, target: tra_loi_co_ngu_canh, min: 1
  - kind: uses-name, target: doan, min: 1
  # Da thu that (trich nguyen ham _dem cua kiem-ast.ts, chay qua python3 TREN
  # CHINH van ban solution da trich tu file nay) -- ket qua [1, 1, 1] cho ba
  # luat theo dung thu tu khai bao o tren.
  # tim_doan_lien_quan=1: dinh nghia ham (dong "def tim_doan_lien_quan(...)")
  # KHONG tinh la Call -- lan GOI THAT duy nhat nam dung o cho trong dau.
  # Dien "doan = None" (bo qua truy xuat) lam so nay tut ve 0 -- bi chan; da
  # tu kiem chung: dien nhu vay lam ket_qua["co_rag"] tro thanh cau ao giac
  # giong het khong_rag, bi bat CA boi static LAN boi assertion
  # "co_rag_dung == True".
  # tra_loi_co_ngu_canh=1: dinh nghia ham khong tinh; lan GOI THAT duy nhat
  # nam o cho trong hai. Dien "tra_loi_khong_ngu_canh(cau_hoi)" (bo qua ngu
  # canh) lam so nay tut ve 0 -- bi chan; da tu kiem chung: dien nhu vay lam
  # co_rag == khong_rag (ca hai deu la cau ao giac), bi bat boi assertion
  # "ket_qua['co_rag'] == '15000000 dong'".
  # uses-name "doan"=1: bien nay CHI duoc DOC (Load) dung mot lan, o cho
  # trong hai (`tra_loi_co_ngu_canh(cau_hoi, doan)`) -- dong gan "doan = ___"
  # la Store, khong duoc dem boi uses-name.
  #
  # 🔴 Hai cho trong nay KHONG hoan doi duoc cho nhau: cho dau tra ve mot
  # DOAN VAN BAN (hoac None), cho hai nhan HAI doi so (cau_hoi, doan) va tra
  # ve MOT CHUOI cau tra loi -- hai kieu du lieu/chu ky ham khac han nhau,
  # khong the "hoan doi ca cum" theo dung nghia GOTCHA #5 (hai gia tri phai
  # CUNG KIEU va cung vi tri cu phap moi hoan doi duoc). Vi vay KHONG can
  # dung mutant hoan doi rieng cho cap nay -- da tu kiem tra logic nay truoc
  # khi ket luan, khong bo qua buoc xac minh.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^toi nghi la khoang 10000000 dong\\n15000000 dong\\nFalse True\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một câu hỏi: không có ngữ cảnh — ảo giác (`10000000 dong`, sai). Có
ngữ cảnh — đúng (`15000000 dong`). Từ bài sau, quest này xây từng phần của
bước "truy xuất" đó: chia tài liệu, tính vector, đo độ gần.
::::

::::reflect{#nghi-lai}
RAG không phải một mô hình mới, một thuật toán bí ẩn nào cả — nó là một
QUY TRÌNH: giữ tri thức bên NGOÀI mô hình, tìm đúng phần liên quan, rồi mới
đưa vào ngữ cảnh trước khi hỏi. Bài này đã đo phần dễ nhất của quy trình đó
bằng một `tim_doan_lien_quan` cực đơn giản (khớp từ khoá trực tiếp). Vấn đề
thật của RAG bắt đầu từ đây: một kho tài liệu THẬT có hàng nghìn đoạn văn,
không thể khớp từ khoá tay như thế này mãi được. Năm bài còn lại của quest
`chunking-va-embedding-tu-che` xây chính xác phần còn thiếu: chia một tài
liệu dài thành nhiều đoạn nhỏ hợp lý (bài `2`, `3`), biến mỗi đoạn thành một
vector số đo được (bài `4`), rồi so khoảng cách giữa các vector đó để tìm ra
đoạn nào THẬT SỰ liên quan (bài `5`) — nền tảng của một bước "truy xuất"
tổng quát hơn nhiều so với khớp từ khoá.
::::

::::checkpoint{mastery=0.85}
::::
