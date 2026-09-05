---
id: tri-tue-nhan-tao.dau-ra-co-cau-truc-va-cong-cu.rang-buoc-theo-schema-va-retry
title: "Ràng buộc theo schema và retry: kiểm rồi yêu cầu lại"
summary: "Mot 'schema' don gian la tap hop field bat buoc: ham kiem_tra_schema(dap_an, cac_field_bat_buoc) kiem dap_an la dict VA co DU moi field, khong thieu field nao. LLM mo phong llm_mo_phong_tra_gia(ten_hang, lan_goi) mo phong loi that: lan goi dau tra ve JSON THIEU truong 'don_vi' (qua tham so lan_goi, khong phai ngau nhien that), lan goi sau moi day du. Ham goi_va_kiem_schema kiem tra roi RETRY khi sai: banh_mi/ca_phe dung dung 2 lan goi thi dat schema hop le; mot mat hang (tra_sua) mo phong loi DAI DANG -- het ca 3 lan van thieu truong, tra ve None. Ca hai con so (2 lan, 3 lan het han) deu chay that."
locale: vi
track: tri-tue-nhan-tao
module: dau-ra-co-cau-truc-va-cong-cu
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 9
teaches: [ai.rang-buoc-theo-schema-va-retry]
requires: [ai.vi-sao-can-dau-ra-co-cau-truc]
concepts: [ai.rang-buoc-theo-schema-va-retry]
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
Bài trước: đầu ra có cấu trúc luôn đọc đúng trường — miễn là trường đó THẬT
SỰ có mặt. Nhưng một LLM (mô phỏng hay thật) đôi khi trả về JSON THIẾU một
trường bắt buộc. Bài này thêm một bước kiểm tra: đúng SCHEMA chưa, và nếu
chưa thì YÊU CẦU LẠI.
::::

::::explain{#schema_va_retry_la_gi}
Một **schema** — ở mức đơn giản nhất — là một danh sách các **trường bắt
buộc** mà một câu trả lời có cấu trúc phải có đủ. Hàm
`kiem_tra_schema(dap_an, cac_field_bat_buoc)` kiểm ĐÚNG hai điều: `dap_an`
có phải một `dict` không, và với MỖI trường trong `cac_field_bat_buoc`,
trường đó có nằm trong `dap_an` không. Thiếu một trường THÔI cũng đủ để
kết quả là "sai schema".

LLM (mô phỏng hay thật) không phải lúc nào cũng trả về đủ trường ngay lần
đầu — đây là một lỗi có thật, không phải chuyện hiếm: đôi khi phản hồi bị
cắt cụt, hoặc bỏ sót một trường phụ. LLM mô phỏng của bài này tái hiện đúng
lỗi đó bằng một luật CỐ ĐỊNH, không ngẫu nhiên: hàm nhận thêm một tham số
`lan_goi` (đếm từ `0`) nói rõ đây là LẦN GỌI thứ mấy, và trả về JSON đủ
trường hay thiếu trường tuỳ theo con số đó — tất định hoàn toàn, không có
gì "may rủi".

**Retry** là hành động gọi LẠI, dựa trên kết quả kiểm schema: nếu
`kiem_tra_schema` báo sai, gọi lại LLM (với `lan_goi` tăng thêm `1`) thay vì
chấp nhận một câu trả lời thiếu trường. Vòng lặp dừng khi ĐẠT schema hợp lệ,
hoặc khi hết số lần thử cho phép — trường hợp sau nghĩa là LLM (mô phỏng
hay thật) LIÊN TỤC sai, và pipeline phải từ bỏ thay vì lặp vô hạn.
::::

::::example{#do_retry_that}
Ba mặt hàng: hai mặt hàng chỉ cần retry MỘT lần là đạt schema, một mặt hàng
LUÔN thiếu trường (lỗi dai dẳng) nên hết sạch số lần thử vẫn thất bại:

```python title=readonly
BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}


def llm_mo_phong_tra_gia(ten_hang, lan_goi):
    gia = BANG_GIA[ten_hang]
    if ten_hang == "tra_sua":
        # mo phong mot loi DAI DANG: moi lan goi deu thieu "don_vi"
        return {"gia": gia}
    if lan_goi == 0:
        # mo phong loi THAT cua LLM: lan dau thieu field "don_vi"
        return {"gia": gia}
    return {"gia": gia, "don_vi": "dong"}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, dict):
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def goi_va_kiem_schema(ten_hang, cac_field_bat_buoc, so_lan_toi_da=3):
    lan_goi = 0
    while lan_goi < so_lan_toi_da:
        dap_an = llm_mo_phong_tra_gia(ten_hang, lan_goi)
        if kiem_tra_schema(dap_an, cac_field_bat_buoc):
            return dap_an, lan_goi + 1
        lan_goi += 1
    return None, lan_goi


CAC_FIELD_BAT_BUOC = ["gia", "don_vi"]

dap_banh_mi, so_lan_banh_mi = goi_va_kiem_schema("banh_mi", CAC_FIELD_BAT_BUOC)
dap_tra_sua, so_lan_tra_sua = goi_va_kiem_schema("tra_sua", CAC_FIELD_BAT_BUOC)

print(dap_banh_mi, so_lan_banh_mi)
print(dap_tra_sua, so_lan_tra_sua)
```

```text title=readonly
{'gia': 15000, 'don_vi': 'dong'} 2
None 3
```

`banh_mi`: lần gọi đầu (`lan_goi=0`) thiếu `"don_vi"`, schema SAI, retry;
lần gọi thứ hai (`lan_goi=1`) đủ trường, schema ĐÚNG — dừng lại, tổng cộng
`2` lần gọi. `tra_sua`: MỌI lần gọi đều thiếu `"don_vi"` (lỗi dai dẳng, mô
phỏng một trường hợp LLM không bao giờ tự sửa) — hết `3` lần thử cho phép
(`so_lan_toi_da` mặc định), vòng lặp dừng, trả về `None` — pipeline biết
chắc chắn là ĐÃ THỬ nhưng KHÔNG THÀNH CÔNG, không âm thầm dùng một câu trả
lời thiếu trường.
::::

::::predict{#doan_gioi_han_lan_thu commitOnce}
`goi_va_kiem_schema("tra_sua", CAC_FIELD_BAT_BUOC, so_lan_toi_da=3)` trả về
`(None, 3)` — hết `3` lần thử vẫn thiếu trường.

**Trước khi chạy thử**, bạn đoán: nếu gọi
`goi_va_kiem_schema("banh_mi", CAC_FIELD_BAT_BUOC, so_lan_toi_da=1)` (giới
hạn CHỈ `1` lần thử, biết rằng `banh_mi` cần đúng `2` lần mới đạt schema),
kết quả sẽ là gì?

:::opt{correct}
`(None, 1)` — vòng lặp `while lan_goi < so_lan_toi_da` chỉ chạy ĐÚNG một
lần (`lan_goi=0`, thiếu `"don_vi"`, schema sai), sau đó `lan_goi` tăng lên
`1`, điều kiện `1 < 1` là `False`, vòng lặp dừng và hàm trả về `(None,
lan_goi)` với `lan_goi=1` — CHƯA kịp tới lần gọi thứ hai (nơi `banh_mi` mới
đủ trường)
:::

:::opt
`({'gia': 15000}, 1)` — hàm vẫn trả về câu trả lời NHẬN ĐƯỢC ở lần thử cuối
cùng, dù thiếu trường, vì ít nhất còn CÓ MỘT phản hồi để dùng tạm
::why
Gần đúng ở trực giác "còn dữ liệu thì nên dùng tạm" — một chiến lược có
thể hợp lý trong một số ngữ cảnh khác.

Chỗ lệch: `goi_va_kiem_schema` không có nhánh nào trả về một `dap_an` CHƯA
qua được `kiem_tra_schema`. Vòng lặp chỉ `return dap_an, lan_goi + 1` ở
đúng nhánh `if kiem_tra_schema(...)`; khi vòng lặp kết thúc mà KHÔNG đi vào
nhánh đó, dòng cuối cùng `return None, lan_goi` luôn chạy — không có
đường nào để một `dap_an` thiếu trường lọt ra ngoài.
::
:::

:::opt
`(None, 3)` — giống hệt kết quả của `tra_sua`, vì mọi lần gọi thất bại đều
dừng ở đúng `3` lần thử
::why
Gần đúng ở việc bạn nhớ đúng con số `3` xuất hiện ở ví dụ của `tra_sua` —
nhưng con số đó tới từ tham số `so_lan_toi_da` MẶC ĐỊNH (`= 3`) của
`goi_va_kiem_schema`, không phải một hằng số cố định trong toàn hàm.

Chỗ lệch: câu hỏi này gọi hàm với `so_lan_toi_da=1` — một giá trị KHÁC.
Vòng lặp `while lan_goi < so_lan_toi_da` dừng theo giá trị THAM SỐ được
truyền vào ở MỖI lần gọi, không phải một con số cố định — đổi tham số thì
đổi luôn số lần thử tối đa.
::
:::
::::

::::code{#viet_kiem_tra_schema_va_retry}
Hoàn thiện `kiem_tra_schema` (kiểm `dap_an` đúng kiểu `dict`) và
`goi_va_kiem_schema` (tăng `lan_goi` sau mỗi lần thử thất bại).

```python title=starter
BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}


def llm_mo_phong_tra_gia(ten_hang, lan_goi):
    gia = BANG_GIA[ten_hang]
    if ten_hang == "tra_sua":
        return {"gia": gia}
    if lan_goi == 0:
        return {"gia": gia}
    return {"gia": gia, "don_vi": "dong"}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, ___):                  # dict
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def goi_va_kiem_schema(ten_hang, cac_field_bat_buoc, so_lan_toi_da=3):
    lan_goi = 0
    while lan_goi < so_lan_toi_da:
        dap_an = llm_mo_phong_tra_gia(ten_hang, lan_goi)
        if kiem_tra_schema(dap_an, cac_field_bat_buoc):
            return dap_an, lan_goi + 1
        lan_goi ___ 1                                  # += 
    return None, lan_goi


CAC_FIELD_BAT_BUOC = ["gia", "don_vi"]

dap_banh_mi, so_lan_banh_mi = goi_va_kiem_schema("banh_mi", CAC_FIELD_BAT_BUOC)
dap_tra_sua, so_lan_tra_sua = goi_va_kiem_schema("tra_sua", CAC_FIELD_BAT_BUOC)

print(dap_banh_mi, so_lan_banh_mi)
print(dap_tra_sua, so_lan_tra_sua)
```

```python title=solution
BANG_GIA = {
    "banh_mi": 15000,
    "ca_phe": 25000,
    "tra_sua": 42000,
}


def llm_mo_phong_tra_gia(ten_hang, lan_goi):
    gia = BANG_GIA[ten_hang]
    if ten_hang == "tra_sua":
        return {"gia": gia}
    if lan_goi == 0:
        return {"gia": gia}
    return {"gia": gia, "don_vi": "dong"}


def kiem_tra_schema(dap_an, cac_field_bat_buoc):
    if not isinstance(dap_an, dict):
        return False
    for field in cac_field_bat_buoc:
        if field not in dap_an:
            return False
    return True


def goi_va_kiem_schema(ten_hang, cac_field_bat_buoc, so_lan_toi_da=3):
    lan_goi = 0
    while lan_goi < so_lan_toi_da:
        dap_an = llm_mo_phong_tra_gia(ten_hang, lan_goi)
        if kiem_tra_schema(dap_an, cac_field_bat_buoc):
            return dap_an, lan_goi + 1
        lan_goi += 1
    return None, lan_goi


CAC_FIELD_BAT_BUOC = ["gia", "don_vi"]

dap_banh_mi, so_lan_banh_mi = goi_va_kiem_schema("banh_mi", CAC_FIELD_BAT_BUOC)
dap_tra_sua, so_lan_tra_sua = goi_va_kiem_schema("tra_sua", CAC_FIELD_BAT_BUOC)

print(dap_banh_mi, so_lan_banh_mi)
print(dap_tra_sua, so_lan_tra_sua)
```

```python title=test
assert kiem_tra_schema({"gia": 1, "don_vi": "dong"}, CAC_FIELD_BAT_BUOC) == True, "dict du truong phai qua schema"
assert kiem_tra_schema({"gia": 1}, CAC_FIELD_BAT_BUOC) == False, "dict thieu 'don_vi' phai KHONG qua schema"
assert kiem_tra_schema("khong phai dict", CAC_FIELD_BAT_BUOC) == False, f"mot chuoi khong phai dict phai la False -- dang ra {kiem_tra_schema('khong phai dict', CAC_FIELD_BAT_BUOC)}"
assert kiem_tra_schema([1, 2], CAC_FIELD_BAT_BUOC) == False, "mot list khong phai dict phai la False"

assert dap_banh_mi == {"gia": 15000, "don_vi": "dong"}, f"banh_mi phai dat schema du -- dang ra {dap_banh_mi!r}"
assert so_lan_banh_mi == 2, f"banh_mi phai can dung 2 lan goi -- dang ra {so_lan_banh_mi}"
assert dap_tra_sua is None, f"tra_sua phai het han sau khi retry, tra ve None -- dang ra {dap_tra_sua!r}"
assert so_lan_tra_sua == 3, f"tra_sua phai dung het 3 lan thu -- dang ra {so_lan_tra_sua}"

dap_ca_phe, so_lan_ca_phe = goi_va_kiem_schema("ca_phe", CAC_FIELD_BAT_BUOC)
assert dap_ca_phe == {"gia": 25000, "don_vi": "dong"}, f"ca_phe phai dat schema du -- dang ra {dap_ca_phe!r}"
assert so_lan_ca_phe == 2, f"ca_phe phai can dung 2 lan goi -- dang ra {so_lan_ca_phe}"

# bien: gioi han so_lan_toi_da=1 -- khong du de banh_mi dat schema (can 2
# lan), phai tra ve None sau DUNG 1 lan thu, khong duoc chay tiep
dap_1_lan, so_lan_1_lan = goi_va_kiem_schema("banh_mi", CAC_FIELD_BAT_BUOC, so_lan_toi_da=1)
assert dap_1_lan is None, f"gioi han 1 lan thu phai tra ve None -- dang ra {dap_1_lan!r}"
assert so_lan_1_lan == 1, f"gioi han 1 lan thu phai dung o so lan=1 -- dang ra {so_lan_1_lan}"
```

:::hints
- kind: attention
  body: Hai chỗ trống. Chỗ đầu là kiểu dữ liệu cần kiểm trong `isinstance(dap_an, ___)` — kiểu đại diện cho một đối tượng JSON đã phân tích, trong Python là kiểu tập hợp khoá-giá trị. Chỗ hai là phép TĂNG bộ đếm `lan_goi` thêm `1` sau mỗi lần thử thất bại — dùng phép gán cộng dồn, không phải gán lại một hằng số.
- kind: strategy
  body: 'Chỗ đầu: `dict` (cho dòng `isinstance(dap_an, dict)`). Chỗ hai: `+=` (cho dòng `lan_goi += 1`).'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `dict` và `+=`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: kiem_tra_schema phai kiem tra dung kieu "dict" (khong duoc doi thanh "list" hay kieu khac), VA lan_goi phai duoc TANG bang "+=" sau moi lan thu that bai (khong duoc gan lai mot hang so co dinh)
  requireAst:
  - kind: uses-name, target: "dict", min: 1
  - kind: uses-operator, target: "+", min: 2
  # Da thu that (goi _dem tren code trich tu solution, khong doan tay).
  # uses-name "dict"=1: DUY NHAT mot lan, chinh la cho trong 1 -- khong co
  # occurrence "dict" nao khac trong toan bo solution. Dien bua thanh "list"
  # hay bat ky kieu khac lam so nay tut ve 0 -- duoi nguong min=1, bi chan;
  # dong thoi bi chan boi tests (kiem_tra_schema({"gia":1,"don_vi":"dong"},
  # ...) se tra ve False sai, vi mot dict khong bao gio la instance cua
  # "list").
  # "+"=2: mot lan CO SAN (return dap_an, lan_goi + 1 -- BinOp Add), mot
  # lan la cho trong 2 (lan_goi += 1 la AugAssign voi toan tu Add, cung
  # khop target "+"). Neu chi dat min=1 (ngay tho), mot mutant xoa bo cho
  # trong 2 (vi du thay bang "lan_goi = 1", gan lai hang so co dinh thay vi
  # tang don) VAN qua duoc vi con lai 1 lan "+" o dong return -- day la
  # GOTCHA "boilerplate-threshold-masking"; dat dung min=2 (tong THAT) moi
  # chan duoc mutant nay. Mutant "lan_goi = 1" con gay VONG LAP VO HAN cho
  # tra_sua (lan_goi luon quay ve 1, khong bao gio dat 3) -- bi chan CA boi
  # tier run/tests (vuot thoi gian cho phep) neu lot qua static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^\\{'gia': 15000, 'don_vi': 'dong'\\} 2\\nNone 3\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`banh_mi`: đúng `2` lần gọi thì đạt schema. `tra_sua`: hết `3` lần thử vẫn
thiếu trường, trả về `None` — pipeline biết rõ đã thất bại, không âm thầm
dùng dữ liệu thiếu. Bài sau đổi hướng: LLM mô phỏng không trả lời trực
tiếp nữa, mà trả về một CHỈ THỊ GỌI HÀM — và code phía người gọi phải THỰC
SỰ gọi đúng hàm đó để lấy dữ liệu thật.
::::

::::reflect{#nghi-lai}
Schema + retry không sửa được một LLM hay quên trường — nó chỉ đảm bảo
pipeline KHÔNG BAO GIỜ dùng một câu trả lời chưa đạt chuẩn mà không biết.
`banh_mi`/`ca_phe` cho thấy retry "có tác dụng": lần sau LLM mô phỏng tự
sửa. `tra_sua` cho thấy retry KHÔNG PHẢI phép màu: nếu lỗi dai dẳng, vòng
lặp phải có một giới hạn (`so_lan_toi_da`) và một lối thoát rõ ràng
(`None`), không được lặp vô hạn hy vọng lần sau sẽ khác. Bài sau chuyển
sang một dạng đầu ra có cấu trúc khác hẳn: thay vì trả lời trực tiếp, LLM
mô phỏng trả về một CHỈ THỊ GỌI HÀM (`{"goi_ham": ..., "tham_so": {...}}`)
— và code phía người gọi phải đọc chỉ thị đó rồi THỰC SỰ gọi đúng hàm
tương ứng để lấy dữ liệu, thay vì để LLM tự bịa ra câu trả lời.
::::

::::checkpoint{mastery=0.85}
::::
