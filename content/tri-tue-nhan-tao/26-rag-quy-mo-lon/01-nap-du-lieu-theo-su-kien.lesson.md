---
id: tri-tue-nhan-tao.rag-quy-mo-lon.nap-du-lieu-theo-su-kien
title: "Nạp dữ liệu theo sự kiện: hàng đợi xử lý dần, KHÔNG đổi kết quả cuối"
summary: "xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly) mo phong Kafka/RabbitMQ (§39.4 muc 1): sao chep hang doi (khong sua danh sach goc), LAN LUOT .pop(0) tung tai lieu, ap ham_xu_ly (chia_theo_cau tai dung tu q8.5a), gom KET QUA vao MOT danh sach phang. nap_tat_ca_cung_luc(danh_sach_tai_lieu, ham_xu_ly) lam CUNG viec bang list comprehension long. Tren HANG_DOI_GOC gom 4 tai lieu (moi tai lieu 2 cau, phan cach bang dau cham): ca hai ham cho DUNG CUNG ket qua -- danh sach 8 cau phang, giong het tung phan tu -- chi khac CACH xu ly (tuan tu qua hang doi so voi list comprehension mot lan), khong khac KET QUA. Sau khi goi xu_ly_hang_doi_nap, HANG_DOI_GOC van con nguyen 4 tai lieu (ham tu sao chep truoc khi pop, khong sua danh sach nguoi goi truyen vao)."
locale: vi
track: tri-tue-nhan-tao
module: rag-quy-mo-lon
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.nap-du-lieu-theo-su-kien]
requires: [ai.boss-cong-ai-day-du]
concepts: [ai.nap-du-lieu-theo-su-kien]
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
`T8.6` chuyển sang track cuối cùng của `R8`: `q8.6c` "RAG quy mô lớn" —
`Chương 39.4` mô tả một công ty với `1` triệu trang tài liệu, không thể tải
hết vào bộ nhớ. Bài đầu tiên: khi tài liệu MỚI liên tục đổ về, nạp chúng
theo kiểu nào?
::::

::::explain{#hang_doi_su_kien_khong_lam_mat_gi}
`Chương 39.4` mục `1` mô tả **Data Ingestion Pipeline (Event-Driven)**: người
dùng tải lên một tài liệu → hệ thống bắn một **sự kiện** vào một hàng đợi
(Kafka/RabbitMQ) → một worker đứng nghe hàng đợi đó, XỬ LÝ TỪNG sự kiện một,
KHÔNG chặn (không chờ xử lý xong tài liệu này mới NHẬN tài liệu tiếp theo vào
hàng đợi).

Sandbox này không có Kafka thật. Nhưng ý tưởng cốt lõi — "xử lý TỪNG tài liệu
một, tuần tự, qua một hàng đợi" — mô phỏng được bằng một `list` Python đóng
vai trò hàng đợi, và một vòng lặp `while` rút phần tử ĐẦU (`.pop(0)`) cho tới
khi hàng đợi cạn:

```
xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
  hang_doi = BAN SAO cua hang_doi_tai_lieu   # khong sua danh sach nguoi goi truyen vao
  ket_qua = []
  trong khi hang_doi con phan tu:
    tai_lieu = hang_doi.pop(0)               # lay PHAN TU DAU, giong hang doi that
    ket_qua.extend(ham_xu_ly(tai_lieu))       # ap ham xu ly, gom PHANG vao ket_qua
  tra ve ket_qua
```

Đối chiếu bắt buộc: cách "xử lý TỪNG tài liệu một qua hàng đợi" này PHẢI cho
ra **CÙNG kết quả cuối cùng** với cách nạp NGÂY THƠ — xử lý TẤT CẢ tài liệu
CÙNG LÚC bằng một `list comprehension` đơn giản, không hề có khái niệm "hàng
đợi":

```
nap_tat_ca_cung_luc(danh_sach_tai_lieu, ham_xu_ly):
  tra ve [doan CHO MOI tai_lieu TRONG danh_sach_tai_lieu CHO MOI doan TRONG ham_xu_ly(tai_lieu)]
```

Đây là điểm dạy quan trọng nhất của bài này: "event-driven" không đổi **KẾT
QUẢ** — nó chỉ đổi **CÁCH xử lý theo thời gian** (không chặn, xử lý dần thay
vì xử lý hết một lượt). Trong một mô phỏng TẤT ĐỊNH (không có đồng hồ thật,
không có độ trễ mạng thật), sự khác biệt "không chặn" không đo được trực
tiếp bằng số — nhưng vẫn dạy được qua chính CẤU TRÚC của hai hàm: một hàm
dùng hàng đợi + vòng lặp `while` + `.pop(0)`, một hàm dùng `list comprehension`
một lượt — và bài học là: dù cấu trúc khác hẳn nhau, **kết quả phải giống
hệt nhau**.
::::

::::example{#doi_chieu_hang_doi_va_cung_luc}
```python title=readonly
def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
    hang_doi = list(hang_doi_tai_lieu)
    ket_qua = []
    while hang_doi:
        tai_lieu = hang_doi.pop(0)
        ket_qua.extend(ham_xu_ly(tai_lieu))
    return ket_qua


def nap_tat_ca_cung_luc(danh_sach_tai_lieu, ham_xu_ly):
    return [doan for tai_lieu in danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)]


HANG_DOI_GOC = [
    "san pham a co bao hanh 12 thang. doi tra trong 7 ngay dau.",
    "san pham b dung pin sac nhanh. thoi luong pin keo dai 2 ngay.",
    "san pham c ho tro ket noi wifi. ho tro ca bluetooth 5.",
    "san pham d co man hinh amoled. do phan giai cao.",
]

ket_qua_hang_doi = xu_ly_hang_doi_nap(HANG_DOI_GOC, chia_theo_cau)
ket_qua_cung_luc = nap_tat_ca_cung_luc(HANG_DOI_GOC, chia_theo_cau)
giong_nhau = ket_qua_hang_doi == ket_qua_cung_luc
so_luong_ban_dau = len(HANG_DOI_GOC)

print(ket_qua_hang_doi)
print(giong_nhau)
print(len(ket_qua_hang_doi))
print(so_luong_ban_dau)
```

```text title=readonly
['san pham a co bao hanh 12 thang', 'doi tra trong 7 ngay dau', 'san pham b dung pin sac nhanh', 'thoi luong pin keo dai 2 ngay', 'san pham c ho tro ket noi wifi', 'ho tro ca bluetooth 5', 'san pham d co man hinh amoled', 'do phan giai cao']
True
8
4
```

`HANG_DOI_GOC` có đúng `4` tài liệu, mỗi tài liệu có `2` câu (phân cách bằng
dấu chấm). `xu_ly_hang_doi_nap` xử lý qua hàng đợi: rút tài liệu `0` ra, tách
thành `2` câu, gom vào `ket_qua`; rút tài liệu `1`, tách, gom tiếp; lặp lại
cho tới khi hàng đợi rỗng. `nap_tat_ca_cung_luc` không có khái niệm "rút ra
từng cái" — nó duyệt cả `4` tài liệu trong MỘT biểu thức, tách VÀ gom cùng
lúc. `giong_nhau = True` xác nhận: dù CÁCH xử lý khác hẳn nhau (tuần tự qua
hàng đợi so với một lượt), **KẾT QUẢ** — đúng `8` câu, đúng thứ tự — giống hệt
nhau. `len(ket_qua_hang_doi) = 8` (`4` tài liệu × `2` câu), còn
`so_luong_ban_dau = 4` — hai con số này khác nhau vì một đo số TÀI LIỆU gốc,
một đo số CÂU sau khi tách, không phải cùng một đại lượng.
::::

::::predict{#doan_hang_doi_goc_sau_khi_goi commitOnce}
Xét NGAY SAU khi gọi `xu_ly_hang_doi_nap(HANG_DOI_GOC, chia_theo_cau)` ở ví dụ
trên (hàm này dùng `.pop(0)` để rút TỪNG tài liệu ra khỏi một biến `hang_doi`
bên trong nó).

**Trước khi chạy thử**, bạn đoán: `len(HANG_DOI_GOC)` (biến GỐC, truyền VÀO
hàm) đo được LÀ bao nhiêu, ngay sau lần gọi đó?

:::opt{correct}
`4` — không đổi gì cả; `xu_ly_hang_doi_nap` tạo một BẢN SAO
(`hang_doi = list(hang_doi_tai_lieu)`) NGAY ĐẦU hàm, rồi CHỈ `.pop(0)` trên
bản sao đó — `HANG_DOI_GOC` (biến gốc bên ngoài) không hề bị đụng tới
:::

:::opt
`0` — vì `.pop(0)` LẦN LƯỢT rút hết mọi phần tử ra khỏi `HANG_DOI_GOC` cho tới
khi nó rỗng, đúng như một hàng đợi thật bị "tiêu thụ" dần
::why
Gần đúng ở việc một hàng đợi THẬT bị tiêu thụ dần khi xử lý — quan sát đó
đúng CHO BẢN SAO bên trong hàm (biến `hang_doi` cục bộ).

Chỗ lệch: `hang_doi = list(hang_doi_tai_lieu)` tạo ra một danh sách MỚI,
HOÀN TOÀN tách biệt khỏi `HANG_DOI_GOC` — gọi `.pop(0)` trên `hang_doi`
không hề chạm tới `HANG_DOI_GOC`. Nếu hàm KHÔNG có dòng sao chép này (chỉ
viết thẳng `hang_doi_tai_lieu.pop(0)`), khi đó `HANG_DOI_GOC` mới thật sự bị
rỗng dần — nhưng đó không phải cách `xu_ly_hang_doi_nap` được viết ở đây.
::
:::

:::opt
`8` — bằng đúng số CÂU đã tách ra, vì hàm "biến" tài liệu gốc thành các câu
đã xử lý
::why
Gần đúng ở con số `8` — đó CHÍNH LÀ độ dài của `ket_qua_hang_doi` (kết quả
TRẢ VỀ của hàm), không phải chuyện gì đó xảy ra với `HANG_DOI_GOC`.

Chỗ lệch: `HANG_DOI_GOC` và `ket_qua_hang_doi` là HAI biến khác nhau hoàn
toàn — một là đối số ĐẦU VÀO (không đổi), một là giá trị hàm TRẢ VỀ (danh
sách `8` câu). Đo `len` của biến đầu vào sau khi gọi hàm không liên quan gì
tới độ dài của giá trị hàm trả về.
::
:::
::::

::::code{#viet_xu_ly_hang_doi_va_cung_luc}
Hoàn thiện `xu_ly_hang_doi_nap` (gom kết quả CỦA `ham_xu_ly` áp lên tài liệu
vừa rút ra, PHẲNG vào `ket_qua`) và `nap_tat_ca_cung_luc` (dùng một
`list comprehension` LỒNG để làm CÙNG việc trong một lượt, không qua hàng
đợi).

```python title=starter
def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
    hang_doi = list(hang_doi_tai_lieu)
    ket_qua = []
    while hang_doi:
        tai_lieu = hang_doi.pop(0)
        ket_qua.extend(___)                                  # ham_xu_ly(tai_lieu)
    return ket_qua


def nap_tat_ca_cung_luc(danh_sach_tai_lieu, ham_xu_ly):
    return ___                                               # [doan for tai_lieu in danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)]


HANG_DOI_GOC = [
    "san pham a co bao hanh 12 thang. doi tra trong 7 ngay dau.",
    "san pham b dung pin sac nhanh. thoi luong pin keo dai 2 ngay.",
    "san pham c ho tro ket noi wifi. ho tro ca bluetooth 5.",
    "san pham d co man hinh amoled. do phan giai cao.",
]

ket_qua_hang_doi = xu_ly_hang_doi_nap(HANG_DOI_GOC, chia_theo_cau)
ket_qua_cung_luc = nap_tat_ca_cung_luc(HANG_DOI_GOC, chia_theo_cau)
giong_nhau = ket_qua_hang_doi == ket_qua_cung_luc
so_luong_ban_dau = len(HANG_DOI_GOC)

print(ket_qua_hang_doi)
print(giong_nhau)
print(len(ket_qua_hang_doi))
print(so_luong_ban_dau)
```

```python title=solution
def chia_theo_cau(van_ban):
    cac_cau = van_ban.split(".")
    ra = []
    for cau in cac_cau:
        cau_sach = cau.strip()
        if cau_sach != "":
            ra.append(cau_sach)
    return ra


def xu_ly_hang_doi_nap(hang_doi_tai_lieu, ham_xu_ly):
    hang_doi = list(hang_doi_tai_lieu)
    ket_qua = []
    while hang_doi:
        tai_lieu = hang_doi.pop(0)
        ket_qua.extend(ham_xu_ly(tai_lieu))
    return ket_qua


def nap_tat_ca_cung_luc(danh_sach_tai_lieu, ham_xu_ly):
    return [doan for tai_lieu in danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)]


HANG_DOI_GOC = [
    "san pham a co bao hanh 12 thang. doi tra trong 7 ngay dau.",
    "san pham b dung pin sac nhanh. thoi luong pin keo dai 2 ngay.",
    "san pham c ho tro ket noi wifi. ho tro ca bluetooth 5.",
    "san pham d co man hinh amoled. do phan giai cao.",
]

ket_qua_hang_doi = xu_ly_hang_doi_nap(HANG_DOI_GOC, chia_theo_cau)
ket_qua_cung_luc = nap_tat_ca_cung_luc(HANG_DOI_GOC, chia_theo_cau)
giong_nhau = ket_qua_hang_doi == ket_qua_cung_luc
so_luong_ban_dau = len(HANG_DOI_GOC)

print(ket_qua_hang_doi)
print(giong_nhau)
print(len(ket_qua_hang_doi))
print(so_luong_ban_dau)
```

```python title=test
assert ket_qua_hang_doi == [
    "san pham a co bao hanh 12 thang", "doi tra trong 7 ngay dau",
    "san pham b dung pin sac nhanh", "thoi luong pin keo dai 2 ngay",
    "san pham c ho tro ket noi wifi", "ho tro ca bluetooth 5",
    "san pham d co man hinh amoled", "do phan giai cao",
], f"ket_qua_hang_doi sai -- dang ra {ket_qua_hang_doi}"
assert ket_qua_cung_luc == ket_qua_hang_doi, f"nap CUNG LUC phai cho DUNG CUNG ket qua voi nap qua HANG DOI -- dang ra {ket_qua_cung_luc}"
assert giong_nhau is True, f"giong_nhau phai la True -- hai cach nap khac CACH xu ly nhung phai cung KET QUA -- dang ra {giong_nhau}"
assert len(ket_qua_hang_doi) == 8, f"tong so cau sau khi tach phai la 8 (4 tai lieu x 2 cau) -- dang ra {len(ket_qua_hang_doi)}"
assert so_luong_ban_dau == 4, f"so_luong_ban_dau phai la 4 (so TAI LIEU goc, khong phai so cau) -- dang ra {so_luong_ban_dau}"
assert len(HANG_DOI_GOC) == 4, f"HANG_DOI_GOC phai VAN CON 4 tai lieu sau khi goi xu_ly_hang_doi_nap -- ham khong duoc sua danh sach goc -- dang ra {len(HANG_DOI_GOC)}"

# bien: hang doi RONG phai cho ket qua RONG, ca hai cach
assert xu_ly_hang_doi_nap([], chia_theo_cau) == [], "hang doi rong phai cho ket qua rong"
assert nap_tat_ca_cung_luc([], chia_theo_cau) == [], "danh sach rong phai cho ket qua rong"

# kiem tra truc tiep tren mot ham_xu_ly khac (khong phai chia_theo_cau) --
# xac nhan xu_ly_hang_doi_nap that su GOI ham_xu_ly, khong chi tra ve nguyen tai lieu
def nhan_doi_ky_tu(van_ban):
    return [van_ban, van_ban]

assert xu_ly_hang_doi_nap(["x", "y"], nhan_doi_ky_tu) == ["x", "x", "y", "y"], "phai ap ham_xu_ly len TUNG tai lieu roi gom PHANG, khong chi lap lai tai lieu goc"
assert nap_tat_ca_cung_luc(["x", "y"], nhan_doi_ky_tu) == ["x", "x", "y", "y"], "nap_tat_ca_cung_luc phai cho CUNG ket qua voi ham_xu_ly khac"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `xu_ly_hang_doi_nap`) nằm TRONG `ket_qua.extend(___)` — cần GIÁ TRỊ áp `ham_xu_ly` lên tài liệu VỪA rút ra (biến `tai_lieu`). Chỗ hai (trong `nap_tat_ca_cung_luc`) là GIÁ TRỊ TRẢ VỀ của cả hàm — một `list comprehension` LỒNG hai vòng `for`, làm CÙNG việc "tách rồi gom phẳng" nhưng KHÔNG qua hàng đợi.
- kind: strategy
  body: 'Chỗ đầu: `ham_xu_ly(tai_lieu)` — áp hàm xử lý lên tài liệu vừa `.pop(0)` ra, `.extend(...)` gom kết quả (một danh sách con) vào `ket_qua` theo kiểu PHẲNG (không lồng). Chỗ hai: `[doan for tai_lieu in danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)]` — vòng `for` ngoài duyệt từng tài liệu, vòng `for` trong duyệt từng phần tử mà `ham_xu_ly(tai_lieu)` trả về, kết quả đã PHẲNG sẵn nhờ cấu trúc lồng của comprehension.'
- kind: one-line
  body: 'Chỗ đầu là `ham_xu_ly(tai_lieu)`, chỗ hai là `[doan for tai_lieu in danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)]`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai GOI THAT ham_xu_ly(tai_lieu) ben trong extend(...) (khong duoc goi lai chia_theo_cau truc tiep hay chep san mot danh sach); cho trong hai phai la MOT list comprehension LONG hai vong for goi ham_xu_ly(tai_lieu) (khong duoc dung vong lap while hay goi lai xu_ly_hang_doi_nap)
  requireAst:
  - kind: uses-call, target: ham_xu_ly, min: 2
  - kind: comprehension, min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dung dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich solution TU CHINH file nay)
  # -- xac nhan qua ca hai min VA min+1: dung CHINH XAC 2 va 1 (khong phai
  # "it nhat"). ham_xu_ly=2 (TONG THAT tren toan bo solution): 1 lan o cho
  # trong dau (ket_qua.extend(ham_xu_ly(tai_lieu))), 1 lan o cho trong hai
  # (trong list comprehension). Khong noi nao khac trong solution goi
  # ham_xu_ly. comprehension=1 (TONG THAT): CHI list comprehension o cho
  # trong hai -- ham xu_ly_hang_doi_nap dung vong lap while/for thuong,
  # khong co comprehension nao.
  # Dien bua "True" vao ca hai cho trong ("ket_qua.extend(True)" va
  # "return True") cho ham_xu_ly=0 VA comprehension=0 -- CA HAI luat CHAN
  # DUNG (da CHAY THAT xac nhan qua kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter, khong doan tay -- va
  # CHAY THAT qua kiemAst() THAT VA python3 THAT): dien
  # "[doan for tai_lieu in danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)]"
  # vao cho trong dau ("ket_qua.extend([doan for tai_lieu in
  # danh_sach_tai_lieu for doan in ham_xu_ly(tai_lieu)])" trong
  # xu_ly_hang_doi_nap) VA dien "ham_xu_ly(tai_lieu)" vao cho trong hai
  # ("return ham_xu_ly(tai_lieu)" trong nap_tat_ca_cung_luc) -- da CHAY
  # THAT qua kiemAst(): tong so lan goi ham_xu_ly VA tong so comprehension
  # tren TOAN BO solution DEU KHONG DOI (van dung 2 va 1, chi doi VI TRI) --
  # static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong xu_ly_hang_doi_nap (tham so
  # la hang_doi_tai_lieu, KHONG CO bien "danh_sach_tai_lieu" nao trong scope
  # nay), bieu thuc moi doc ten "danh_sach_tai_lieu" CHUA HE TON TAI --
  # NameError NGAY khi dong nay chay (trong vong lap while dau tien). Da tu
  # chay THAT qua python3, xac nhan thong bao "name 'danh_sach_tai_lieu' is
  # not defined". Ben trong nap_tat_ca_cung_luc (tham so la
  # danh_sach_tai_lieu, KHONG CO bien "tai_lieu" don le nao trong scope nay
  # -- chi co ten do LAM THAM SO cua ham, khong phai bien cuc bo), bieu thuc
  # moi "ham_xu_ly(tai_lieu)" doc ten "tai_lieu" CHUA TON TAI -- da tu chay
  # THAT xac nhan NameError "name 'tai_lieu' is not defined" ngay khi ham
  # duoc goi. Ca hai bi chan boi tier 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: "ham_xu_ly" la ten THAM SO duy nhat trong ca hai
  # ham, khong trung voi bien nao khac trong pham vi bai nay; "comprehension"
  # (khong co target) dem MOI dang comprehension trong solution -- solution
  # nay chi co DUNG MOT, nen khong co rui ro nham lan voi comprehension khac.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\['san pham a co bao hanh 12 thang', 'doi tra trong 7 ngay dau', 'san pham b dung pin sac nhanh', 'thoi luong pin keo dai 2 ngay', 'san pham c ho tro ket noi wifi', 'ho tro ca bluetooth 5', 'san pham d co man hinh amoled', 'do phan giai cao'\\]\\nTrue\\n8\\n4\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`True` — hai cách nạp khác hẳn nhau về CÁCH xử lý (hàng đợi tuần tự so với
một lượt) nhưng cho ĐÚNG cùng `8` câu. Bài tiếp theo: kho vector LỚN không
nằm gọn trong một cấu trúc — nó phải được CHIA thành nhiều mảnh nhỏ.
::::

::::reflect{#nghi-lai}
`xu_ly_hang_doi_nap` không nhanh hơn `nap_tat_ca_cung_luc` — trong một mô
phỏng tất định, không có "worker chạy song song" hay "độ trễ mạng" nào để đo
tốc độ thật. Điều nó DẠY được là một sự thật kiến trúc: một hệ thống
event-driven xử lý dữ liệu THEO THỜI GIAN khác (dần dần, không chặn, mỗi
tài liệu một sự kiện) nhưng **không được phép** đổi KẾT QUẢ cuối cùng so với
xử lý gộp một lượt — nếu đổi, đó là một lỗi (mất tài liệu, trùng tài liệu,
sai thứ tự), không phải một đặc điểm của kiến trúc event-driven. `giong_nhau
= True` xác nhận bất biến đó trên `8` câu tách ra từ `4` tài liệu. Bài tiếp
theo: một kho vector triệu tài liệu không nằm gọn trong một cấu trúc dữ liệu
— `Chương 39.4` mục `2` gọi đó là **Vector Database Sharding**, chia kho lớn
thành nhiều mảnh nhỏ hơn.
::::

::::checkpoint{mastery=0.75}
::::
