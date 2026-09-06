---
id: tri-tue-nhan-tao.ha-tang-suy-luan.pipeline-tuan-tu-lang-phi
title: "Pipeline tuần tự: chi phí khởi động bị nhân lên N lần"
summary: "tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong) cong don (chi_phi_khoi_dong + thoi_gian_tinh) cho TUNG request trong danh sach -- vi xu ly tuan tu tra chi phi khoi dong RIENG cho MOI request, khong chia se. Voi 5 request, moi request can 10 (don vi mo phong) tinh toan THAT, chi phi khoi dong co dinh 200 (nap model/kich hoat GPU): tong_thoi_gian_tuan_tu([10,10,10,10,10], 200) = 5 * (200+10) = 1050 -- trong do 1000/1050 (95,24%) la chi phi khoi dong LAP LAI, chi 50/1050 la tinh toan THAT su can. Doi chi phi khoi dong xuong 50: tong_thoi_gian_tuan_tu([10,10,10,10,10], 50) = 5*(50+10) = 300 -- chi phi khoi dong cang lon, phan lang phi cang chiem ty trong lon hon."
locale: vi
track: tri-tue-nhan-tao
module: ha-tang-suy-luan
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [ai.pipeline-tuan-tu-lang-phi]
requires: [ai.kien-truc-ba-tang-ai-stack]
concepts: [ai.pipeline-tuan-tu-lang-phi]
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
Bài trước gọi tầng `Inference` LÀ "tầng tính toán GPU thấp nhất" — nhưng
không nói RÕ vì sao tầng đó, nếu cài NGÂY THƠ, lại lãng phí đến mức
`Chương 39` phải dành nguyên một mục để giải quyết. Câu trả lời bắt đầu từ
một con số duy nhất: **chi phí khởi động**.
::::

::::explain{#chi_phi_khoi_dong_tra_rieng_moi_request}
Mỗi lần một GPU xử lý MỘT request suy luận, nó không chỉ tốn thời gian
TÍNH TOÁN THẬT (nhân ma trận, forward pass qua các tầng của mô hình) — nó
còn tốn một **chi phí khởi động cố định** (fixed overhead): nạp trọng số
mô hình vào bộ nhớ GPU nếu chưa có sẵn, kích hoạt kernel tính toán, chuẩn
bị bộ nhớ KV-cache cho request đó. Chi phí này KHÔNG phụ thuộc request dài
hay ngắn — nó LÀ một hằng số CỘNG THÊM cho mỗi lần gọi.

Một pipeline "ngây thơ" — xử lý request THEO THỨ TỰ, MỘT request tại một
thời điểm, không hề gộp lại — phải trả chi phí khởi động này **RIÊNG cho
MỖI request**, dù request đó có giống hệt request trước hay không:

```
tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong)
  = SUM(chi_phi_khoi_dong + thoi_gian_tinh) cho TUNG request trong danh sach
```

Với `N` request, chi phí khởi động bị nhân lên ĐÚNG `N` lần — dù bản thân
công việc tính toán KHÔNG hề tăng lên `N` lần. Đây LÀ khoản lãng phí thuần
tuý do CÁCH XỬ LÝ (tuần tự), không phải do bản chất công việc.
::::

::::example{#nam_request_nho_chi_phi_khoi_dong_lon}
```python title=readonly
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


CHI_PHI_KHOI_DONG = 200
DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]

tong = tong_thoi_gian_tuan_tu(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG)

print(tong)
```

```text title=readonly
1050
```

`5` request, mỗi request chỉ cần `10` đơn vị thời gian mô phỏng để TÍNH
TOÁN THẬT — tổng thời gian tính toán thật chỉ LÀ `5 * 10 = 50`. Nhưng vì xử
lý TUẦN TỰ, chi phí khởi động `200` bị trả RIÊNG cho MỖI request, `5` lần:
`5 * 200 = 1000`. Tổng cộng: `1000 + 50 = 1050`. Chi phí khởi động LẶP LẠI
chiếm `1000/1050 ≈ 95,24%` tổng thời gian — GẦN NHƯ TOÀN BỘ thời gian xử lý
bị "đốt" vào việc nạp lại/kích hoạt lại, không phải vào việc tính toán mà
người dùng thật sự cần.
::::

::::predict{#doan_tong_tuan_tu_5_request commitOnce}
Xét đúng `DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]` VÀ
`CHI_PHI_KHOI_DONG = 200` — `5` request, xử lý TUẦN TỰ (không gộp lô).

**Trước khi chạy thử**, bạn đoán: `tong_thoi_gian_tuan_tu` trả về bao
nhiêu?

:::opt{correct}
`1050` — chi phí khởi động `200` bị trả RIÊNG cho từng request trong cả
`5` request (`5 * 200 = 1000`), CỘNG tổng thời gian tính toán thật
(`5 * 10 = 50`); `1000 + 50 = 1050`
:::

:::opt
`250` — chi phí khởi động chỉ cần trả MỘT LẦN cho cả `5` request
(`200 + 5*10 = 250`), vì hệ thống chỉ cần "khởi động" một lần rồi xử lý
liên tiếp
::why
Gần đúng ở việc `250` LÀ một con số hợp lý — NẾU hệ thống chia sẻ được chi
phí khởi động giữa các request. Đó CHÍNH LÀ điều mà GỘP LÔ (batching, bài
sau) làm được.

Chỗ lệch: `tong_thoi_gian_tuan_tu` mô phỏng đúng nghĩa "TUẦN TỰ" — MỖI vòng
lặp `for thoi_gian_tinh in danh_sach_thoi_gian_tinh` cộng THÊM
`chi_phi_khoi_dong` một lần nữa, không hề nhớ rằng nó đã trả chi phí này ở
vòng lặp trước. Xử lý tuần tự (bài này) và gộp lô (bài sau) LÀ hai chiến
lược KHÁC NHAU — con số `250` là kết quả của chiến lược THỨ HAI, không phải
chiến lược mà hàm này đang cài.
::
:::

:::opt
`50` — chi phí khởi động không nên tính vào "thời gian xử lý" thực sự,
chỉ tính thời gian TÍNH TOÁN THẬT của các request (`5 * 10`)
::why
Gần đúng ở việc `50` LÀ đúng tổng thời gian TÍNH TOÁN THẬT (bỏ qua chi phí
khởi động) — quan sát về mặt "công việc hữu ích" đó đúng.

Chỗ lệch: chi phí khởi động LÀ một chi phí THẬT, không phải một khoản có
thể bỏ qua — nó chiếm dụng GPU thật sự (nạp trọng số, kích hoạt kernel).
Công thức `tong_thoi_gian_tuan_tu` cộng nó vào MỖI vòng lặp một cách CỐ Ý,
đúng để LÀM RÕ nó lớn tới mức nào khi bị lặp lại không cần thiết — bỏ nó ra
sẽ che mất chính điều bài này muốn chỉ ra.
::
:::
::::

::::code{#viet_tong_thoi_gian_tuan_tu}
Hoàn thiện `tong_thoi_gian_tuan_tu`: với MỖI request trong danh sách, cộng
dồn `chi_phi_khoi_dong + thoi_gian_tinh` vào tổng.

```python title=starter
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += ___                                          # chi_phi_khoi_dong + thoi_gian_tinh
    return tong


CHI_PHI_KHOI_DONG = 200
DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]

tong = tong_thoi_gian_tuan_tu(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG)

print(tong)
```

```python title=solution
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


CHI_PHI_KHOI_DONG = 200
DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]

tong = tong_thoi_gian_tuan_tu(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG)

print(tong)
```

```python title=test
assert tong_thoi_gian_tuan_tu([10, 10, 10, 10, 10], 200) == 1050, f"5 request 10 don vi, chi phi khoi dong 200 phai la 1050 -- dang ra {tong_thoi_gian_tuan_tu([10, 10, 10, 10, 10], 200)}"
assert tong_thoi_gian_tuan_tu([10], 200) == 210, f"1 request 10 don vi, chi phi khoi dong 200 phai la 210 -- dang ra {tong_thoi_gian_tuan_tu([10], 200)}"
assert tong_thoi_gian_tuan_tu([10, 10, 10, 10, 10], 50) == 300, f"doi chi phi khoi dong tu 200 xuong 50 (cung 5 request) phai doi ket qua thanh 300 -- dang ra {tong_thoi_gian_tuan_tu([10, 10, 10, 10, 10], 50)}"
assert tong_thoi_gian_tuan_tu([], 200) == 0, f"danh sach rong (khong co request nao) tong phai la 0 -- dang ra {tong_thoi_gian_tuan_tu([], 200)}"
assert tong == 1050, f"bien tong (demo chinh cua bai) phai la 1050 -- dang ra {tong}"
```

:::hints
- kind: attention
  body: "Mot cho trong duy nhat, ben trong vong lap for. Moi vong lap phai cong THEM ca chi_phi_khoi_dong LAN thoi_gian_tinh cua request hien tai -- day la diem mau chot: chi_phi_khoi_dong xuat hien LAP LAI o MOI vong lap, khong phai chi mot lan truoc vong lap."
- kind: strategy
  body: "chi_phi_khoi_dong + thoi_gian_tinh -- cong hai tham so lai, gan vao tong qua toan tu +=. Khong dung thoi_gian_tinh mot minh (thieu chi phi khoi dong) va khong dung chi_phi_khoi_dong mot minh (thieu phan tinh toan that)."
- kind: one-line
  body: "tong += chi_phi_khoi_dong + thoi_gian_tinh"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong phai CONG (dung toan tu '+') CA HAI tham so chi_phi_khoi_dong VA thoi_gian_tinh -- khong duoc chi dung mot trong hai, va khong duoc dung lai thoi_gian_tinh hai lan
  requireAst:
  - kind: uses-operator, target: "+", min: 2
  - kind: uses-name, target: "chi_phi_khoi_dong", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay) -- ket qua dung du kien:
  #   "+"=2: dong "tong += chi_phi_khoi_dong + thoi_gian_tinh" la MOT
  #   AugAssign (op Add, tu no da la 1 lan khop "+") CONG MOT BinOp
  #   (chi_phi_khoi_dong + thoi_gian_tinh, la 1 lan khop "+" nua) -- tong
  #   THAT la 2 tren toan bo solution (khong con "+" nao khac trong file).
  #   uses-name("chi_phi_khoi_dong")=1: DUY NHAT o cho trong -- tham so nay
  #   khong xuat hien lai o dau khac trong solution.
  # Dien bua "True" vao cho trong (tong += True) cho "+"=1 (chi con
  # AugAssign, mat BinOp) VA uses-name=0 -- CA HAI deu duoi nguong toi
  # thieu, static CHAN DUNG.
  #
  # GOTCHA "bien sai nhung cung do dai/gia tri tinh co" (da tu ra soat va
  # tu CHAY THAT): mot dap an sai PHO BIEN la dung nham thoi_gian_tinh HAI
  # LAN thay vi mot lan chi_phi_khoi_dong mot lan thoi_gian_tinh --
  # "tong += thoi_gian_tinh + thoi_gian_tinh". Da tu dung mutant nay va
  # CHAY THAT qua kiemAst: dem "+" van la 2 (AugAssign + BinOp, KHONG DOI --
  # rui ro that neu chi dat luat "+"), nhung uses-name("chi_phi_khoi_dong")
  # TUT xuong 0 (mutant nay khong he doc ten chi_phi_khoi_dong o dau ca) --
  # duoi nguong toi thieu 1, static CHAN DUNG nho luat thu hai. Neu KHONG co
  # luat uses-name nay, mutant se qua sach tier static (dung "+"=2) roi
  # SAI ve mat SO HOC tren du lieu demo (5 request, thoi_gian_tinh=10 --
  # mutant cho 5*(10+10)=100, khac han 1050 mong doi) nen van bi tier
  # 'tests'/'output' bat -- nhung luat uses-name giup static bat SOM hon,
  # dung tinh than "khong doan tay, tu kiem qua cong cu that".
- tier: tests
  timeoutMs: 6000
- tier: output
  match: exact
  expect: "1050"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`1050` — trong đó `1000` (`95,24%`) chỉ LÀ chi phí khởi động LẶP LẠI `5`
lần. Câu hỏi tự nhiên tiếp theo: nếu GỘP cả `5` request lại thành MỘT lô,
chi phí khởi động có cần trả `5` lần nữa không?
::::

::::reflect{#nghi-lai}
`tong_thoi_gian_tuan_tu` không sai về mặt CÚ PHÁP — nó cộng dồn đúng những
gì được yêu cầu. Cái "sai" ở đây LÀ một quyết định KIẾN TRÚC: xử lý TỪNG
request một, không chia sẻ gì giữa các lần gọi. Với `5` request nhỏ VÀ một
chi phí khởi động lớn, quyết định đó khiến `95,24%` thời gian bị "đốt" vào
việc lặp lại một công việc đáng lẽ chỉ cần làm MỘT LẦN. Bài tiếp theo giữ
nguyên đúng những con số này (`200`, `[10,10,10,10,10]`) nhưng đổi CHIẾN
LƯỢC: gộp nhiều request vào một lô, để chi phí khởi động chỉ còn trả một
lần cho CẢ LÔ.
::::

::::checkpoint{mastery=0.75}
::::
