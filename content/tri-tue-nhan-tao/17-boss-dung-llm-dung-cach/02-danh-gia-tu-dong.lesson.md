---
id: tri-tue-nhan-tao.boss-dung-llm-dung-cach.danh-gia-tu-dong
title: "Đánh giá tự động: khớp tuyệt đối so với độ tương đồng"
summary: "Tu cai (KHONG dung thu vien ngoai) hai do do don gian: khop_chinh_xac (so chuoi bang nhau tuyet doi) va do_tuong_dong_tu (Jaccard don gian tren tap tu -- so tu chung chia tong so tu rieng biet giua hai cau). Tren 4 cap cau tra loi mo hinh vs dap an chuan: khop tuyet doi chi 1/4 (dung mot cap y het tung chu); do tuong dong tu cho [1.0, 1.0, 0.625, 0.111] -- cap thu hai (cung tu, khac thu tu) khop_chinh_xac=False nhung do_tuong_dong_tu=1.0, phan biet ro 'khop tuyet doi' khoi 'gan giong nhung khong khop het'. Tat ca con so deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: boss-dung-llm-dung-cach
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.danh-gia-tu-dong]
requires: [ai.ao-giac-va-kiem-tra-ngu-canh]
concepts: [ai.danh-gia-tu-dong]
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
Bài trước kiểm một CON SỐ có nằm trong phạm vi cho phép không. Nhưng chấm
điểm một câu trả lời thường cần so nó với một đáp án chuẩn — CẢ CÂU, không
chỉ một con số. Bài này tự cài hai cách đo tự động, không dùng thư viện
ngoài.
::::

::::explain{#danh_gia_tu_dong_la_gi}
**Đánh giá tự động** (automated evaluation) là việc chấm điểm đầu ra của
một LLM (mô phỏng hay thật) bằng MỘT CÔNG THỨC tất định, không cần con
người đọc từng câu. Bài này cài hai độ đo đơn giản nhất, khác nhau về mức
độ NGHIÊM KHẮC:

> **Khớp chính xác** (exact match) — so hai chuỗi có BẰNG NHAU TUYỆT ĐỐI
> không, từng ký tự một. Nghiêm khắc nhất: chỉ cần khác MỘT ký tự, MỘT dấu
> cách, MỘT thứ tự từ — đã tính là KHÔNG khớp.
>
> **Độ tương đồng từ** (word similarity) — một dạng Jaccard đơn giản: tách
> cả hai câu thành TẬP HỢP các từ (bỏ trùng lặp, bỏ thứ tự), rồi tính tỉ lệ
> `số từ CHUNG chia cho tổng số từ RIÊNG BIỆT` (hợp của hai tập). Điểm số
> nằm trong khoảng `0,0` (không chung từ nào) tới `1,0` (hai tập từ giống
> hệt nhau).

Hai độ đo này ĐO KHÁC NHAU: khớp chính xác quan tâm tới HÌNH DẠNG chuỗi
(kể cả thứ tự từ); độ tương đồng từ chỉ quan tâm NỘI DUNG từ vựng, bỏ qua
thứ tự. Hai câu viết cùng những từ, khác thứ tự, có thể khớp chính xác
`False` nhưng tương đồng từ `1,0` — "gần giống" theo nghĩa từ vựng, dù
không "khớp hệt" theo nghĩa chuỗi.
::::

::::example{#do_hai_do_do_that}
Bốn cặp (câu trả lời mô hình, đáp án chuẩn), chấm bằng cả hai độ đo:

```python title=readonly
def khop_chinh_xac(cau_tra_loi, dap_an_chuan):
    return cau_tra_loi == dap_an_chuan


def tach_tu(cau):
    return set(cau.split())


def do_tuong_dong_tu(cau_tra_loi, dap_an_chuan):
    tu_tra_loi = tach_tu(cau_tra_loi)
    tu_dap_an = tach_tu(dap_an_chuan)
    giao = tu_tra_loi & tu_dap_an
    hop = tu_tra_loi | tu_dap_an
    if not hop:
        return 1.0
    return len(giao) / len(hop)


CAC_CAP = [
    ("gia banh mi la 15000 dong", "gia banh mi la 15000 dong"),
    ("la 15000 dong gia banh mi", "gia banh mi la 15000 dong"),
    ("gia banh mi khoang 15000 dong thoi", "gia banh mi la 15000 dong"),
    ("toi khong biet gia", "gia ca phe la 25000 dong"),
]


def danh_gia_tu_dong(cac_cap):
    so_khop_tuyet_doi = 0
    danh_sach_diem = []
    for cau_tra_loi, dap_an_chuan in cac_cap:
        if khop_chinh_xac(cau_tra_loi, dap_an_chuan):
            so_khop_tuyet_doi += 1
        diem = do_tuong_dong_tu(cau_tra_loi, dap_an_chuan)
        danh_sach_diem.append(round(diem, 3))
    return so_khop_tuyet_doi, danh_sach_diem


so_khop, diem_ds = danh_gia_tu_dong(CAC_CAP)
print(so_khop, "/", len(CAC_CAP))
print(diem_ds)
```

```text title=readonly
1 / 4
[1.0, 1.0, 0.625, 0.111]
```

Khớp chính xác: chỉ `1/4` — cặp đầu tiên khớp Y HỆT từng ký tự. Độ tương
đồng từ: `[1,0; 1,0; 0,625; 0,111]`. Cặp THỨ HAI đáng chú ý nhất: cùng
những từ (`gia`, `banh`, `mi`, `la`, `15000`, `dong`), chỉ khác THỨ TỰ —
khớp chính xác là `False` (chuỗi không bằng nhau), nhưng độ tương đồng từ
là `1,0` (tập từ giống hệt) — phân biệt rõ "khớp tuyệt đối" khỏi "gần
giống nhưng không khớp hệt".
::::

::::predict{#doan_cap_thu_hai commitOnce}
Xét cặp thứ hai ở ví dụ trên: `"la 15000 dong gia banh mi"` (câu trả lời)
so với `"gia banh mi la 15000 dong"` (đáp án chuẩn) — cùng SÁU từ, chỉ khác
THỨ TỰ.

**Trước khi chạy thử**, bạn đoán: `khop_chinh_xac` và `do_tuong_dong_tu`
trên cặp này cho ra gì?

:::opt{correct}
`khop_chinh_xac` là `False`, `do_tuong_dong_tu` là `1,0` — hai chuỗi
KHÔNG bằng nhau (thứ tự từ khác nhau, nên phép so `==` cho `False`), nhưng
`tach_tu` biến cả hai câu thành CÙNG một tập hợp sáu từ (thứ tự không quan
trọng với `set`), nên `giao` và `hop` bằng nhau, tỉ lệ đúng bằng `1,0`
:::

:::opt
Cả hai đều là `True`/`1,0` — vì hai câu chứa đúng CÙNG những từ, nên bất kỳ
độ đo hợp lý nào cũng phải coi chúng là khớp nhau
::why
Gần đúng ở việc bạn nhận ra đúng hai câu này CÙNG một nội dung từ vựng —
quan sát đó không sai.

Chỗ lệch: `khop_chinh_xac` dùng phép so chuỗi `==`, so từng KÝ TỰ theo
đúng THỨ TỰ xuất hiện — `"la 15000 dong gia banh mi"` và
`"gia banh mi la 15000 dong"` là hai CHUỖI khác nhau (ký tự đầu tiên đã
khác: `'l'` so với `'g'`), nên `==` cho `False`, dù nội dung từ vựng giống
hệt.
::
:::

:::opt
Cả hai đều là `False`/`0,0` — vì "giống thứ tự" mới tính là khớp, còn đảo
thứ tự coi như một câu HOÀN TOÀN khác
::why
Gần đúng ở việc bạn đúng khi cho rằng `khop_chinh_xac` sẽ là `False` —
phần đó đúng.

Chỗ lệch: `do_tuong_dong_tu` không quan tâm thứ tự — nó gọi `tach_tu` biến
câu thành một `set`, và `set` không có khái niệm "thứ tự". Hai câu có
CÙNG sáu từ (dù viết khác thứ tự) cho ra CÙNG một tập hợp, nên `giao`
bằng `hop`, tỉ lệ là `1,0` chứ không phải `0,0`.
::
:::
::::

::::code{#viet_hai_do_do}
Hoàn thiện `khop_chinh_xac` (so hai chuỗi bằng nhau tuyệt đối) và
`do_tuong_dong_tu` (xử lý trường hợp biên: hai câu đều KHÔNG có từ nào,
định nghĩa độ tương đồng là `1,0`).

```python title=starter
def khop_chinh_xac(cau_tra_loi, dap_an_chuan):
    return cau_tra_loi ___ dap_an_chuan              # ==


def tach_tu(cau):
    return set(cau.split())


def do_tuong_dong_tu(cau_tra_loi, dap_an_chuan):
    tu_tra_loi = tach_tu(cau_tra_loi)
    tu_dap_an = tach_tu(dap_an_chuan)
    giao = tu_tra_loi & tu_dap_an
    hop = tu_tra_loi | tu_dap_an
    if not hop:
        return ___                                    # 1.0
    return len(giao) / len(hop)


CAC_CAP = [
    ("gia banh mi la 15000 dong", "gia banh mi la 15000 dong"),
    ("la 15000 dong gia banh mi", "gia banh mi la 15000 dong"),
    ("gia banh mi khoang 15000 dong thoi", "gia banh mi la 15000 dong"),
    ("toi khong biet gia", "gia ca phe la 25000 dong"),
]


def danh_gia_tu_dong(cac_cap):
    so_khop_tuyet_doi = 0
    danh_sach_diem = []
    for cau_tra_loi, dap_an_chuan in cac_cap:
        if khop_chinh_xac(cau_tra_loi, dap_an_chuan):
            so_khop_tuyet_doi += 1
        diem = do_tuong_dong_tu(cau_tra_loi, dap_an_chuan)
        danh_sach_diem.append(round(diem, 3))
    return so_khop_tuyet_doi, danh_sach_diem


so_khop, diem_ds = danh_gia_tu_dong(CAC_CAP)
print(so_khop, "/", len(CAC_CAP))
print(diem_ds)
```

```python title=solution
def khop_chinh_xac(cau_tra_loi, dap_an_chuan):
    return cau_tra_loi == dap_an_chuan


def tach_tu(cau):
    return set(cau.split())


def do_tuong_dong_tu(cau_tra_loi, dap_an_chuan):
    tu_tra_loi = tach_tu(cau_tra_loi)
    tu_dap_an = tach_tu(dap_an_chuan)
    giao = tu_tra_loi & tu_dap_an
    hop = tu_tra_loi | tu_dap_an
    if not hop:
        return 1.0
    return len(giao) / len(hop)


CAC_CAP = [
    ("gia banh mi la 15000 dong", "gia banh mi la 15000 dong"),
    ("la 15000 dong gia banh mi", "gia banh mi la 15000 dong"),
    ("gia banh mi khoang 15000 dong thoi", "gia banh mi la 15000 dong"),
    ("toi khong biet gia", "gia ca phe la 25000 dong"),
]


def danh_gia_tu_dong(cac_cap):
    so_khop_tuyet_doi = 0
    danh_sach_diem = []
    for cau_tra_loi, dap_an_chuan in cac_cap:
        if khop_chinh_xac(cau_tra_loi, dap_an_chuan):
            so_khop_tuyet_doi += 1
        diem = do_tuong_dong_tu(cau_tra_loi, dap_an_chuan)
        danh_sach_diem.append(round(diem, 3))
    return so_khop_tuyet_doi, danh_sach_diem


so_khop, diem_ds = danh_gia_tu_dong(CAC_CAP)
print(so_khop, "/", len(CAC_CAP))
print(diem_ds)
```

```python title=test
assert so_khop == 1, f"khop tuyet doi phai la 1/4 -- dang ra {so_khop}"
assert diem_ds == [1.0, 1.0, 0.625, 0.111], f"danh sach diem tuong dong sai -- dang ra {diem_ds}"

# xac nhan truc tiep tren cap thu hai: khop_chinh_xac=False nhung
# do_tuong_dong_tu=1.0 -- day la bang chung TRUNG TAM cua bai nay
assert khop_chinh_xac("la 15000 dong gia banh mi", "gia banh mi la 15000 dong") == False, "cau dao thu tu tu KHONG duoc khop chinh xac"
assert do_tuong_dong_tu("la 15000 dong gia banh mi", "gia banh mi la 15000 dong") == 1.0, "cau dao thu tu tu (cung tap tu) phai co do tuong dong la 1.0"

# bien: hai chuoi rong -- do_tuong_dong_tu phai la 1.0 (khong chia cho 0)
assert do_tuong_dong_tu("", "") == 1.0, f"hai chuoi rong phai cho do tuong dong 1.0 -- dang ra {do_tuong_dong_tu('', '')}"
assert khop_chinh_xac("", "") == True, "hai chuoi rong phai khop chinh xac (bang nhau)"

# bien: hai cau HOAN TOAN khac nhau (khong chung tu nao) -- do tuong dong 0.0
assert do_tuong_dong_tu("banh mi", "ca phe") == 0.0, f"hai cau khong chung tu nao phai cho 0.0 -- dang ra {do_tuong_dong_tu('banh mi', 'ca phe')}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu so hai chuỗi có BẰNG NHAU TUYỆT ĐỐI không — dùng toán tử so sánh bằng nhau (`==`). Chỗ hai xử lý trường hợp CẢ HAI câu đều không có từ nào (`hop` rỗng) — quy ước độ tương đồng của hai câu rỗng là "hoàn toàn giống nhau", tức giá trị lớn nhất có thể (`1.0`), tránh chia cho `0`.
- kind: strategy
  body: 'Chỗ đầu: `==` (cho dòng `return cau_tra_loi == dap_an_chuan`). Chỗ hai: `1.0` (cho dòng `return 1.0` khi `hop` rỗng).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `==` và `1.0`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: khop_chinh_xac phai dung toan tu "==" de so hai chuoi (khong duoc doi hay bo phep so sanh), VA truong hop hai cau rong phai tra ve dung gia tri "1.0" (khong duoc doi thanh "0.0" hay mot so khac)
  requireAst:
  - kind: uses-operator, target: "==", min: 1
  - kind: has-literal, target: "1.0", min: 1
  # Da thu that (goi kiemAst that -- trich nguyen ham _dem tu kiem-ast.ts,
  # chay qua python3 tren code trich tu solution, khong doan tay).
  # "=="=1: XUAT HIEN DUY NHAT o cho trong 1 (return cau_tra_loi ==
  # dap_an_chuan) -- khong co "==" nao khac trong toan bo solution (khong
  # co dieu kien "==" nao khac o dau ca). Dien bua xoa phep so sanh (vi du
  # "return True") lam so nay tut ve 0 -- duoi nguong min=1, bi chan; dong
  # thoi bi chan boi tests (so_khop se tinh sai thanh 4 vi moi cap deu
  # "khop").
  # has-literal "1.0"=1: XUAT HIEN DUY NHAT o cho trong 2 -- khong co gia
  # tri 1.0 nao khac trong solution. Dien bua thanh "0.0" lam so nay tut ve
  # 0 -- duoi nguong min=1, bi chan; dong thoi bi chan boi tests
  # (do_tuong_dong_tu("", "") se tra ve 0.0 thay vi 1.0).
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^1 / 4\\n\\[1\\.0, 1\\.0, 0\\.625, 0\\.111\\]\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khớp chính xác: `1/4`. Độ tương đồng từ: `[1,0; 1,0; 0,625; 0,111]` — cặp
thứ hai cho thấy rõ hai độ đo KHÔNG PHẢI cùng một thứ: khớp tuyệt đối khắt
khe với thứ tự, tương đồng từ chỉ quan tâm nội dung từ vựng. Bài sau đặt
MỘT LLM mô phỏng khác vào vai giám khảo — chấm theo một RUBRIC (luật) cố
định, không phải đếm từ chung.
::::

::::reflect{#nghi-lai}
Hai độ đo này KHÔNG thay thế nhau — chúng đo hai thứ khác nhau và cả hai
đều có giới hạn. Khớp chính xác nghiêm khắc tới mức một câu ĐÚNG Ý NGHĨA
nhưng viết khác thứ tự vẫn bị tính là sai (`False` ở cặp `2`). Độ tương
đồng từ khoan dung hơn về thứ tự, nhưng cũng có nguy cơ ngược lại: hai câu
CHIA SẺ nhiều từ chung (kể cả một con số SAI) vẫn có thể được điểm cao —
bài BOSS cuối quest sẽ cho thấy đúng tình huống đó, khi một câu trả lời SAI
GIÁ TRỊ vẫn đạt độ tương đồng từ khá cao chỉ vì phần lớn CÂU CHỮ xung quanh
vẫn đúng. Đây chính là lý do cần MỘT lớp đánh giá khác — không đếm từ
chung, mà áp một LUẬT rõ ràng lên đúng chỗ quan trọng (số tiền, đơn vị) —
bài sau xây đúng lớp đó: một LLM mô phỏng đóng vai GIÁM KHẢO.
::::

::::checkpoint{mastery=0.85}
::::
