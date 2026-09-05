---
id: tri-tue-nhan-tao.prompt-engineering-nen-tang.vai-tro-message
title: "Vai trò message: system, user, assistant"
summary: "T8.3 khép lại bằng một Transformer tự viết từ số 0. T8.4 mở đầu bằng một CHUYỂN HƯỚNG: không tự xây mô hình nữa, mà học DÙNG một mô hình cho đúng cách -- qua một LLM MÔ PHỎNG (hàm Python thuần, tra bảng/luật cố định, KHÔNG mạng nơ-ron, KHÔNG gọi API thật). Bài này dạy cấu trúc message: mỗi lượt là một dict có role (system/user/assistant) và content. Đo bằng số thật: cùng một câu persona, đặt ở role=system thì đổi được văn phong câu trả lời (thêm lời chào trang trọng); đặt CÙNG câu đó ở role=user thì bị bỏ qua hoàn toàn -- chứng minh cơ chế đọc theo TRƯỜNG role, không phải theo nội dung chữ."
locale: vi
track: tri-tue-nhan-tao
module: prompt-engineering-nen-tang
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.vai-tro-message]
requires: [ai.boss-transformer-tu-so-0]
concepts: [ai.vai-tro-message]
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
T8.3 vừa đóng lại bằng một Transformer tự viết từ số `0`. T8.4 rẽ sang một
hướng khác hẳn: không XÂY mô hình nữa, mà học DÙNG một mô hình cho đúng
cách. Điểm khởi đầu: một lượt gọi mô hình không phải một câu văn liền
mạch — nó là một DANH SÁCH message có cấu trúc.
::::

::::explain{#llm_mo_phong_va_vai_tro}
Nền tảng này chạy hoàn toàn trong trình duyệt (Pyodide/WASM), KHÔNG có
mạng — không lời giải nào trong track T8.4 gọi được một API LLM THẬT
(OpenAI, Anthropic, hay bất kỳ dịch vụ nào khác), và mọi phép chấm điểm
phải TẤT ĐỊNH (cùng input luôn cho cùng output). Vì vậy toàn bộ track này
dùng một **LLM MÔ PHỎNG**: một hàm Python THUẦN, KHÔNG mạng nơ-ron, KHÔNG
ngẫu nhiên, nhận vào danh sách message và trả lời bằng cách TRA BẢNG hoặc
ÁP LUẬT CỐ ĐỊNH đã biết trước. Mục tiêu là học CƠ CHẾ prompt engineering
— cấu trúc message, few-shot đổi hành vi ra sao, ràng buộc thu hẹp câu trả
lời ra sao — bằng những con số đo được cụ thể, không phải học cách trò
chuyện với một chatbot thật.

Một lời gọi LLM thật (và cả LLM mô phỏng của track này) không nhận một
đoạn văn bản liền mạch — nó nhận một danh sách **message**, mỗi message là
một `dict` có đúng hai trường bắt buộc:

> **`role`** — ai đang "nói" ở lượt này. Ba giá trị: `"system"` (quy tắc/
> persona đặt trước, do người lập trình cấu hình, không phải người dùng
> gõ), `"user"` (câu hỏi/lệnh của người dùng), `"assistant"` (câu trả lời
> mô hình đã đưa ra ở một lượt TRƯỚC — dùng làm ngữ cảnh lịch sử).
>
> **`content`** — nội dung văn bản của lượt đó.

Điểm mấu chốt: một hàm xử lý message đọc theo **trường `role`**, không
phải đọc TOÀN BỘ nội dung rồi đoán xem câu nào là "chỉ thị hệ thống". Nếu
đúng một câu persona (`"Bạn là một trợ lý trang trọng."`) được đặt vào
message có `role="system"`, hàm mô phỏng bên dưới đọc được nó và đổi văn
phong câu trả lời. Nếu CHÍNH CÂU ĐÓ bị đặt nhầm vào một message có
`role="user"`, hàm không còn cách nào phân biệt nó với một câu hỏi bình
thường — nó bị bỏ qua hoàn toàn với vai trò persona. Bài này đo đúng sự
khác biệt ấy bằng số thật.

Một chi tiết nữa: nếu có NHIỀU message `role="user"` trong cùng một lượt
gọi (ví dụ giữ lại vài câu hỏi cũ làm ngữ cảnh), quy ước trong track này là
chỉ CÂU HỎI CUỐI CÙNG mới là câu cần trả lời — các câu `user` trước đó chỉ
còn là lịch sử.
::::

::::example{#doc_vai_tro_va_doi_van_phong}
Một hàm mô phỏng tối giản: tra bảng câu hỏi cố định, và đổi văn phong dựa
theo `role="system"` (nếu có):

```python title=readonly
def dem_vai_tro(messages):
    dem = {}
    for tin in messages:
        vai_tro = tin["role"]
        dem[vai_tro] = dem.get(vai_tro, 0) + 1
    return dem


def llm_mo_phong_don_gian(messages):
    BANG = {
        "ban ten la gi": "Toi la tro ly AI mo phong.",
        "hom nay troi the nao": "Toi khong co du lieu thoi tiet that.",
    }
    cau_hoi = None
    he_thong = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
        elif tin["role"] == "system":
            he_thong = tin["content"]
    khoa = cau_hoi.strip().lower().rstrip("?.!") if cau_hoi else ""
    dap_an = BANG.get(khoa, "Toi khong co cau tra loi cho cau hoi nay.")
    if he_thong and "trang trong" in he_thong.lower():
        return "Kinh chao. " + dap_an
    return dap_an


messages_khong_he_thong = [
    {"role": "user", "content": "Ban ten la gi?"},
]

messages_co_he_thong = [
    {"role": "system", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]

messages_vai_tro_sai = [
    {"role": "user", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]

print(dem_vai_tro(messages_co_he_thong))
print(llm_mo_phong_don_gian(messages_khong_he_thong))
print(llm_mo_phong_don_gian(messages_co_he_thong))
print(llm_mo_phong_don_gian(messages_vai_tro_sai))
```

```text title=readonly
{'system': 1, 'user': 1}
Toi la tro ly AI mo phong.
Kinh chao. Toi la tro ly AI mo phong.
Toi la tro ly AI mo phong.
```

`messages_co_he_thong` có đúng `1` message `system` và `1` message `user`.
Với `system` đúng vai trò, câu trả lời đổi thành `"Kinh chao. ..."`. Với
`messages_vai_tro_sai` — CÙNG câu persona, nhưng đặt vào một message
`role="user"` thứ hai thay vì `role="system"` — hàm mô phỏng trả lại ĐÚNG
câu trả lời mặc định, không hề có lời chào trang trọng. Nội dung chữ giống
hệt nhau; chỉ khác trường `role`; kết quả khác nhau.
::::

::::predict{#doan_vai_tro_sai commitOnce}
Xét `messages_vai_tro_sai` ở ví dụ trên: message ĐẦU TIÊN có nội dung y hệt
câu persona (`"Ban la mot tro ly trang trong."`), nhưng mang `role="user"`
thay vì `role="system"`. Message THỨ HAI là câu hỏi thật (`role="user"`,
`"Ban ten la gi?"`).

**Trước khi đọc lại đoạn code**, bạn đoán: `llm_mo_phong_don_gian` trả lời
gì cho `messages_vai_tro_sai`?

:::opt{correct}
`"Toi la tro ly AI mo phong."` — vì hàm chỉ đọc persona từ message có
`role="system"`; ở đây KHÔNG có message nào mang `role` đó, nên `he_thong`
giữ nguyên `None`, và nhánh thêm lời chào trang trọng không bao giờ chạy.
Ngoài ra, vòng lặp gán `cau_hoi` cho MỌI message có `role="user"` mà nó gặp
— nên `cau_hoi` bị GHI ĐÈ, kết thúc bằng giá trị của message `user` CUỐI
CÙNG (`"Ban ten la gi?"`), không phải câu persona
:::

:::opt
`"Kinh chao. Toi la tro ly AI mo phong."` — vì nội dung câu persona vẫn
xuất hiện trong danh sách message, nên hàm vẫn nhận ra được nó
::why
Gần đúng ở việc nội dung chữ THẬT SỰ có mặt trong danh sách message — bạn
không đọc sai dữ liệu đầu vào.

Chỗ lệch: hàm mô phỏng không quét TOÀN BỘ nội dung để tìm cụm "trang
trọng" bất kể nó nằm ở đâu — nó chỉ đọc `tin["content"]` của những message
có `tin["role"] == "system"` (dòng `elif tin["role"] == "system":`). Một
message mang `role="user"` không bao giờ được gán vào biến `he_thong`, dù
nội dung của nó là gì — cơ chế đọc theo TRƯỜNG, không theo Ý NGHĨA câu chữ.
::
:::

:::opt
Hàm sẽ báo lỗi, vì có tới hai message cùng `role="user"` trong một danh
sách
::why
Gần đúng ở việc bạn để ý danh sách này có hai message `user` liên tiếp —
một cấu trúc khác thường so với ví dụ đầu (một `system` + một `user`).

Chỗ lệch: không có ràng buộc nào trong `llm_mo_phong_don_gian` cấm nhiều
message cùng `role`. Vòng lặp đơn giản là gán `cau_hoi = tin["content"]`
mỗi lần gặp một message `role="user"`, nên giá trị CUỐI CÙNG được gán mới
là giá trị còn lại sau vòng lặp — không có lỗi nào được ném ra.
::
:::
::::

::::code{#viet_llm_mo_phong_don_gian}
Hoàn thiện `llm_mo_phong_don_gian`: nhánh `elif` phải kiểm tra đúng vai trò
`"system"`, và câu trả lời khi có persona trang trọng phải được nối thêm
đúng lời chào `"Kinh chao. "` ở đầu.

```python title=starter
def dem_vai_tro(messages):
    dem = {}
    for tin in messages:
        vai_tro = tin["role"]
        dem[vai_tro] = dem.get(vai_tro, 0) + 1
    return dem


def llm_mo_phong_don_gian(messages):
    BANG = {
        "ban ten la gi": "Toi la tro ly AI mo phong.",
        "hom nay troi the nao": "Toi khong co du lieu thoi tiet that.",
    }
    cau_hoi = None
    he_thong = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
        elif tin["role"] ___:                        # == "system"
            he_thong = tin["content"]
    khoa = cau_hoi.strip().lower().rstrip("?.!") if cau_hoi else ""
    dap_an = BANG.get(khoa, "Toi khong co cau tra loi cho cau hoi nay.")
    if he_thong and "trang trong" in he_thong.lower():
        return ___ + dap_an                          # "Kinh chao. "
    return dap_an


messages_khong_he_thong = [
    {"role": "user", "content": "Ban ten la gi?"},
]

messages_co_he_thong = [
    {"role": "system", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]

messages_vai_tro_sai = [
    {"role": "user", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]

print(dem_vai_tro(messages_co_he_thong))
print(llm_mo_phong_don_gian(messages_khong_he_thong))
print(llm_mo_phong_don_gian(messages_co_he_thong))
print(llm_mo_phong_don_gian(messages_vai_tro_sai))
```

```python title=solution
def dem_vai_tro(messages):
    dem = {}
    for tin in messages:
        vai_tro = tin["role"]
        dem[vai_tro] = dem.get(vai_tro, 0) + 1
    return dem


def llm_mo_phong_don_gian(messages):
    BANG = {
        "ban ten la gi": "Toi la tro ly AI mo phong.",
        "hom nay troi the nao": "Toi khong co du lieu thoi tiet that.",
    }
    cau_hoi = None
    he_thong = None
    for tin in messages:
        if tin["role"] == "user":
            cau_hoi = tin["content"]
        elif tin["role"] == "system":
            he_thong = tin["content"]
    khoa = cau_hoi.strip().lower().rstrip("?.!") if cau_hoi else ""
    dap_an = BANG.get(khoa, "Toi khong co cau tra loi cho cau hoi nay.")
    if he_thong and "trang trong" in he_thong.lower():
        return "Kinh chao. " + dap_an
    return dap_an


messages_khong_he_thong = [
    {"role": "user", "content": "Ban ten la gi?"},
]

messages_co_he_thong = [
    {"role": "system", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]

messages_vai_tro_sai = [
    {"role": "user", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]

print(dem_vai_tro(messages_co_he_thong))
print(llm_mo_phong_don_gian(messages_khong_he_thong))
print(llm_mo_phong_don_gian(messages_co_he_thong))
print(llm_mo_phong_don_gian(messages_vai_tro_sai))
```

```python title=test
assert dem_vai_tro(messages_co_he_thong) == {"system": 1, "user": 1}, f"dem_vai_tro sai -- dang ra {dem_vai_tro(messages_co_he_thong)}"
assert llm_mo_phong_don_gian(messages_khong_he_thong) == "Toi la tro ly AI mo phong.", f"tra loi khong he thong sai -- dang ra {llm_mo_phong_don_gian(messages_khong_he_thong)!r}"
assert llm_mo_phong_don_gian(messages_co_he_thong) == "Kinh chao. Toi la tro ly AI mo phong.", f"tra loi co he thong sai -- dang ra {llm_mo_phong_don_gian(messages_co_he_thong)!r}"
assert llm_mo_phong_don_gian(messages_vai_tro_sai) == "Toi la tro ly AI mo phong.", f"tra loi vai tro sai phai GIONG voi khong he thong -- dang ra {llm_mo_phong_don_gian(messages_vai_tro_sai)!r}"

# bien: mot cuoc goi khong co message user nao ca -- cau_hoi phai la None,
# khoa phai la chuoi rong, tra ve cau mac dinh (khong duoc loi)
assert llm_mo_phong_don_gian([{"role": "system", "content": "Ban la mot tro ly trang trong."}]) == "Kinh chao. Toi khong co cau tra loi cho cau hoi nay.", "truong hop khong co user phai van chay duoc va van them loi chao"

# bien: system co mat nhung KHONG chua tu "trang trong" -- khong duoc them loi chao
assert llm_mo_phong_don_gian([
    {"role": "system", "content": "Ban la mot tro ly vui ve."},
    {"role": "user", "content": "Ban ten la gi?"},
]) == "Toi la tro ly AI mo phong.", "system khong chua 'trang trong' thi khong duoc them loi chao"

# bien: mot message role="assistant" (lich su mot luot truoc) KHONG duoc
# coi la persona he thong, du noi dung co chua "trang trong"
assert llm_mo_phong_don_gian([
    {"role": "assistant", "content": "Ban la mot tro ly trang trong."},
    {"role": "user", "content": "Ban ten la gi?"},
]) == "Toi la tro ly AI mo phong.", "role=assistant khong duoc coi la system, du noi dung chua 'trang trong'"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu nằm trong `elif tin["role"] ___:` — cần so sánh với đúng vai trò còn lại (không phải `"user"`, đã dùng ở nhánh `if` ngay trên). Chỗ hai là chuỗi ghép vào ĐẦU `dap_an` khi có persona trang trọng — xem đúng câu chữ được nhắc tới trong ví dụ phía trên (`"Kinh chao. ..."`).
- kind: strategy
  body: 'Chỗ đầu: `== "system"` — hoàn thiện điều kiện `elif tin["role"] == "system":`. Chỗ hai: `"Kinh chao. "` — chuỗi có dấu cách ở cuối, để khi nối với `dap_an` không bị dính chữ.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `== "system"` và `"Kinh chao. "`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: nhanh "elif" phai kiem tra dung vai tro "system" (khong duoc doi thanh vai tro khac hay bo qua so sanh); loi chao dung/sai duoc doi chieu chinh xac o tang tests/output
  requireAst:
  - kind: uses-operator, target: "==", min: 2
  - kind: has-literal, target: "system", min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # "=="=2: mot lan CO SAN (tin["role"]=="user" trong llm_mo_phong_don_gian
  # -- dem_vai_tro khong co phep so sanh nao ca, chi dung dict.get), mot
  # lan la cho trong 1 (elif tin["role"]=="system"). Dien bua cho trong 1
  # thanh "elif tin["role"]:" (bo so sanh, chi con kiem tra role co RONG
  # hay khong) lam "=="=1 -- duoi nguong min=2, bi chan boi static. Da tu
  # kiem chung bang Python that: mutant nay cho CUNG ket qua voi loi giai
  # dung tren moi bien vi role chi la "user" hoac "system" -- CHI test
  # bien "role=assistant" (them o duoi file test) moi phan biet duoc no
  # qua tests/output (mutant coi "assistant" nhu persona he thong, loi
  # giai dung thi khong), nen day la phong thu KEP that su, khong chi
  # static rieng le.
  # has-literal "system"=2: mot lan CO SAN trong du lieu
  # messages_co_he_thong ({"role": "system", ...}), mot lan trong cho
  # trong 1. Dien bua cho trong 1 thanh "assistant" hay bat ky vai tro nao
  # khac lam so nay tut xuong 1 -- duoi nguong min=2, bi chan; dong thoi bi
  # chan CA boi tests (llm_mo_phong_don_gian(messages_co_he_thong) se
  # khong con dung loi chao).
  # Cho trong 2 (chuoi "Kinh chao. " noi dau dap_an) KHONG co luat static
  # rieng: mot rang buoc has-literal voi target ket thuc bang dau cach bi
  # bo qua boi bo phan tich schema (dau cach cuoi chuoi YAML bi cat), nen
  # khong the kiem tra chinh xac chuoi nay o tang static. Dien bua thanh
  # mot chuoi khac (vi du "Xin chao. ") van CHAY duoc (qua tier run/static
  # ca hai luat tren) nhung bi chan CHAC CHAN boi tests/output, vi ca hai
  # deu doi chieu dung nguyen van chuoi "Kinh chao. Toi la tro ly AI mo
  # phong." -- da tu kiem chung bang Python that.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\{'system': 1, 'user': 1\\}\\nToi la tro ly AI mo phong\\.\\nKinh chao\\. Toi la tro ly AI mo phong\\.\\nToi la tro ly AI mo phong\\.\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một câu persona, hai `role` khác nhau, hai kết quả khác nhau — đo
được bằng số thật. Bài sau giữ nguyên `role` message nhưng thêm một trục
mới: bao nhiêu VÍ DỤ mẫu (few-shot) đặt trước câu hỏi, và điều đó đổi hành
vi của LLM mô phỏng ra sao.
::::

::::reflect{#nghi-lai}
Vai trò `system`/`user`/`assistant` không phải trang trí cú pháp — nó là
TRƯỜNG mà một hàm xử lý message đọc để quyết định "câu nào là chỉ thị cấu
hình, câu nào là câu hỏi thật, câu nào là lịch sử". Đặt nhầm vai trò không
làm nội dung biến mất khỏi dữ liệu, nhưng làm cơ chế đọc bỏ qua đúng phần
lẽ ra phải có hiệu lực. Bài sau giữ nguyên cấu trúc message này, nhưng đặt
THÊM các cặp `user`/`assistant` TRƯỚC câu hỏi cuối — biến chúng thành các
VÍ DỤ MẪU (few-shot), và đo xem có bao nhiêu ví dụ mới đủ để đổi hành vi
tính toán của LLM mô phỏng.
::::

::::checkpoint{mastery=0.8}
::::
