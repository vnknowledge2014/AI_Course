---
id: tri-tue-nhan-tao.ha-tang-suy-luan.gop-lo-tang-toc-do
title: "Gộp lô (continuous batching): MAX chứ không phải TỔNG"
summary: "tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo) chia danh sach thanh cac lo kich thuoc co dinh, moi lo chi tra chi_phi_khoi_dong MOT LAN roi cong max(lo) (thoi gian tinh LON NHAT trong lo, vi chay SONG SONG tren cung GPU -- khong phai cong don). Voi 5 request 10 don vi, chi_phi_khoi_dong=200, kich_thuoc_lo=5 (mot lo gom ca 5): tong_thoi_gian_gop_lo=200+max([10]*5)=210 -- so voi tong_thoi_gian_tuan_tu=1050 (bai truoc), nhanh gap DUNG 1050/210=5 lan. Voi lo khong deu [5,20,8] mot lo kich thuoc 3: 200+max(5,20,8)=200+20=220 -- KHONG PHAI 200+sum(5,20,8)=233."
locale: vi
track: tri-tue-nhan-tao
module: ha-tang-suy-luan
order: 3
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.gop-lo-tang-toc-do]
requires: [ai.pipeline-tuan-tu-lang-phi]
concepts: [ai.gop-lo-tang-toc-do]
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
Bài trước: `5` request nhỏ, xử lý TUẦN TỰ, tổng `1050` — trong đó `1000`
chỉ LÀ chi phí khởi động bị trả `5` lần. Câu hỏi CÒN NGUYÊN từ cuối bài
trước: nếu gộp cả `5` request vào MỘT lô, chi phí đó có cần trả `5` lần
nữa không?
::::

::::explain{#gop_lo_chi_tra_mot_lan_va_chay_song_song}
`continuous batching` — gộp NHIỀU request vào MỘT lô (batch) trước khi đưa
vào GPU — thay đổi phép tính theo đúng HAI cách, không phải MỘT:

1. **Chi phí khởi động chỉ trả MỘT LẦN cho CẢ LÔ**, không phải riêng cho
   từng request trong lô đó. Nạp model, kích hoạt kernel — những việc này
   làm MỘT LẦN LÀ ĐỦ cho toàn bộ lô, dù lô có `1` hay `100` request.
2. **Thời gian tính toán của lô LÀ giá trị LỚN NHẤT (MAX)** trong số các
   request của lô đó — KHÔNG PHẢI tổng của chúng. Vì sao? Vì GPU tính toán
   các request trong CÙNG một lô **song song** (cùng một phép nhân ma trận
   lớn, xử lý nhiều request cùng lúc trên nhiều lõi) — lô CHỈ xong khi
   request CHẬM NHẤT trong lô xong, giống hệt việc một nhóm người rời khỏi
   phòng CÙNG một lúc: nhóm chỉ "xong" khi người CUỐI CÙNG bước ra, không
   phải tổng thời gian của TỪNG người cộng lại.

```
tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo)
  = SUM(chi_phi_khoi_dong + max(lo)) cho TUNG lo (kich thuoc kich_thuoc_lo)
```

Điểm thứ hai (`MAX` chứ không phải `SUM`) LÀ điểm dễ hiểu NHẦM nhất của cả
bài — và cũng LÀ điểm quan trọng nhất để phân biệt "gộp lô" với "cộng dồn
tuần tự".
::::

::::example{#doi_chieu_tuan_tu_va_gop_lo}
```python title=readonly
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


CHI_PHI_KHOI_DONG = 200
DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]
KICH_THUOC_LO = 5

tong_tuan_tu = tong_thoi_gian_tuan_tu(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG)
tong_gop_lo = tong_thoi_gian_gop_lo(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG, KICH_THUOC_LO)
toc_do_nhanh_gap = tong_tuan_tu / tong_gop_lo

print(tong_tuan_tu)
print(tong_gop_lo)
print(toc_do_nhanh_gap)
```

```text title=readonly
1050
210
5.0
```

Cùng `5` request, cùng chi phí khởi động `200` — chỉ đổi CHIẾN LƯỢC xử lý.
`KICH_THUOC_LO = 5` gộp CẢ `5` request vào ĐÚNG một lô duy nhất: chi phí
khởi động `200` giờ chỉ trả `1` lần (không phải `5` lần), và thời gian tính
toán của lô LÀ `max([10, 10, 10, 10, 10]) = 10` (không phải
`10+10+10+10+10=50`, vì cả `5` request chạy SONG SONG). Tổng:
`200 + 10 = 210`. So với tuần tự (`1050`), gộp lô nhanh gấp ĐÚNG
`1050 / 210 = 5,0` lần — CHÍNH XÁC bằng số request trong lô, vì chi phí
khởi động (vốn chiếm `95,24%` thời gian tuần tự) giờ chỉ còn xuất hiện MỘT
lần thay vì `5` lần.
::::

::::predict{#doan_lo_khong_deu commitOnce}
Xét một lô có ĐÚNG `3` request với thời gian tính toán thật lần lượt LÀ
`5`, `20`, `8` (đơn vị mô phỏng) — CHỈ một lô duy nhất, chi phí khởi động
`200`.

**Trước khi chạy thử**, bạn đoán: lô đó tốn TỔNG bao nhiêu thời gian?

:::opt{correct}
`220` — chi phí khởi động `200` (trả MỘT LẦN cho cả lô) CỘNG
`max(5, 20, 8) = 20` (thời gian của request CHẬM NHẤT trong lô, vì cả `3`
request chạy song song); `200 + 20 = 220`
:::

:::opt
`233` — chi phí khởi động `200` cộng TỔNG thời gian tính toán của cả `3`
request (`5+20+8=33`); `200+33=233`
::why
Gần đúng ở việc `33` LÀ tổng đúng của `5+20+8` — phép cộng đó không sai.

Chỗ lệch: các request trong CÙNG một lô chạy SONG SONG trên GPU, không
phải LẦN LƯỢT — nên thời gian của cả lô KHÔNG PHẢI tổng thời gian từng
request, mà LÀ thời gian của request CHẬM NHẤT (`max`, không phải `sum`).
Cộng dồn (`sum`) chính LÀ công thức của bài TRƯỚC (xử lý tuần tự) — nhầm nó
sang gộp lô là nhầm đúng điểm khác biệt cốt lõi giữa hai chiến lược.
::
:::

:::opt
`20` — chỉ tính thời gian của request chậm nhất, không cần cộng thêm chi
phí khởi động vì cả lô chỉ khởi động một lần nên coi như không đáng kể
::why
Gần đúng ở việc `max(5, 20, 8) = 20` LÀ đúng phần thời gian TÍNH TOÁN của
lô — quan sát đó đúng.

Chỗ lệch: "trả MỘT LẦN" không có nghĩa LÀ "bỏ qua" — chi phí khởi động
`200` VẪN phải trả, chỉ LÀ trả ĐÚNG một lần cho cả lô thay vì lặp lại cho
từng request. Bỏ hẳn nó ra khỏi phép tính là nhầm "trả một lần" thành
"miễn phí" — hai điều rất khác nhau.
::
:::
::::

::::code{#viet_tong_thoi_gian_gop_lo}
Hoàn thiện `tong_thoi_gian_gop_lo`: với mỗi lô kích thước `kich_thuoc_lo`,
tính thời gian bằng `chi_phi_khoi_dong + max(lo)` — KHÔNG phải tổng.

```python title=starter
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + ___]              # kich_thuoc_lo
        tong += chi_phi_khoi_dong + ___                        # max(lo)
    return tong


CHI_PHI_KHOI_DONG = 200
DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]
KICH_THUOC_LO = 5

tong_tuan_tu = tong_thoi_gian_tuan_tu(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG)
tong_gop_lo = tong_thoi_gian_gop_lo(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG, KICH_THUOC_LO)
toc_do_nhanh_gap = tong_tuan_tu / tong_gop_lo

print(tong_tuan_tu)
print(tong_gop_lo)
print(toc_do_nhanh_gap)
```

```python title=solution
def tong_thoi_gian_tuan_tu(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong):
    tong = 0
    for thoi_gian_tinh in danh_sach_thoi_gian_tinh:
        tong += chi_phi_khoi_dong + thoi_gian_tinh
    return tong


def tong_thoi_gian_gop_lo(danh_sach_thoi_gian_tinh, chi_phi_khoi_dong, kich_thuoc_lo):
    tong = 0
    for i in range(0, len(danh_sach_thoi_gian_tinh), kich_thuoc_lo):
        lo = danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo]
        tong += chi_phi_khoi_dong + max(lo)
    return tong


CHI_PHI_KHOI_DONG = 200
DANH_SACH_5_REQUEST = [10, 10, 10, 10, 10]
KICH_THUOC_LO = 5

tong_tuan_tu = tong_thoi_gian_tuan_tu(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG)
tong_gop_lo = tong_thoi_gian_gop_lo(DANH_SACH_5_REQUEST, CHI_PHI_KHOI_DONG, KICH_THUOC_LO)
toc_do_nhanh_gap = tong_tuan_tu / tong_gop_lo

print(tong_tuan_tu)
print(tong_gop_lo)
print(toc_do_nhanh_gap)
```

```python title=test
assert tong_thoi_gian_gop_lo([10, 10, 10, 10, 10], 200, 5) == 210, f"mot lo gom ca 5 request 10 don vi, chi phi khoi dong 200 phai la 210 -- dang ra {tong_thoi_gian_gop_lo([10, 10, 10, 10, 10], 200, 5)}"
assert tong_thoi_gian_gop_lo([5, 20, 8], 200, 3) == 220, f"mot lo [5,20,8] phai dung MAX (20), khong phai SUM (33) -- ket qua dung phai la 220, dang ra {tong_thoi_gian_gop_lo([5, 20, 8], 200, 3)}"
assert tong_thoi_gian_gop_lo([10, 20, 15, 5, 30, 10, 10], 200, 3) == 660, f"7 request chia lam 3 lo kich thuoc 3 (2 lo day, 1 lo con 1) phai la 660 -- dang ra {tong_thoi_gian_gop_lo([10, 20, 15, 5, 30, 10, 10], 200, 3)}"
assert tong_thoi_gian_gop_lo([10, 10, 10, 10, 10], 200, 2) == 630, f"cung 5 request nhung kich_thuoc_lo=2 (nhieu lo hon) phai la 630, KHAC voi kich_thuoc_lo=5 (210) -- xac nhan kich_thuoc_lo THAT SU anh huong ket qua, dang ra {tong_thoi_gian_gop_lo([10, 10, 10, 10, 10], 200, 2)}"
assert tong_tuan_tu == 1050, f"tong_tuan_tu phai giu nguyen 1050 nhu bai truoc -- dang ra {tong_tuan_tu}"
assert tong_gop_lo == 210, f"tong_gop_lo (kich_thuoc_lo=5) phai la 210 -- dang ra {tong_gop_lo}"
assert toc_do_nhanh_gap == 5.0, f"toc do nhanh gap phai la DUNG 5.0 (1050/210) -- dang ra {toc_do_nhanh_gap}"
```

:::hints
- kind: attention
  body: "Hai cho trong, cung mot dong for-loop cua ham tong_thoi_gian_gop_lo. Cho dau la GIOI HAN TREN cua phep cat lat (slice) -- lay dung kich_thuoc_lo phan tu ke tu vi tri i. Cho hai la THOI GIAN TINH CUA CA LO -- phai la GIA TRI LON NHAT trong lo (chay song song), KHONG PHAI tong cac gia tri."
- kind: strategy
  body: "Cho dau: danh_sach_thoi_gian_tinh[i:i + kich_thuoc_lo] -- cat mot doan dai kich_thuoc_lo bat dau tu i. Cho hai: max(lo) -- ham max() tren danh sach lo vua cat duoc o dong truoc."
- kind: one-line
  body: "Cho dau la kich_thuoc_lo, cho hai la max(lo)."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai la kich_thuoc_lo (gioi han tren cua slice); cho trong hai phai dung ham max(lo) -- KHONG duoc dung sum(lo), day la diem khac biet cot loi giua gop lo (chay song song) va cong don tuan tu
  requireAst:
  - kind: uses-call, target: "max", min: 1
  - kind: uses-name, target: "kich_thuoc_lo", min: 2
  forbidAst:
  - kind: uses-call, target: "sum"
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay) -- ket qua dung du kien:
  #   "max"=1: DUY NHAT o cho trong hai (max(lo)). Ham tong_thoi_gian_tuan_tu
  #   khong dung max o dau ca.
  #   uses-name("kich_thuoc_lo")=2: mot lan o phan da cho san trong starter
  #   (tham so thu ba cua ham range(), khong phai cho trong) CONG mot lan o
  #   cho trong dau (slice) -- tong THAT la 2.
  #   "sum" (forbid)=0: solution khong goi sum() o dau ca -- luat cam KHONG
  #   bi vi pham tren dap an dung.
  # Dien bua "True" vao CA HAI cho trong cho "max"=0 (duoi nguong 1) VA
  # uses-name("kich_thuoc_lo") TUT xuong 1 (chi con lan trong range(), mat
  # lan o slice -- duoi nguong 2) -- CA HAI luat CHAN DUNG.
  # Mutant "dung sum thay max" (loi quan niem PHO BIEN nhat cua bai nay):
  # da tu CHAY THAT qua kiemAst -- "max"=0 (duoi nguong, CHAN DUNG boi luat
  # requireAst), VA rieng "sum"=1 (VI PHAM luat forbidAst -- CHAN DUNG THEM
  # LAN NUA, doc lap). Hai lop chan nay day du, khong can dua vao
  # tests/output de bat loi quan niem trung tam cua bai.
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong
  # -- xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua
  # kiemAst): dien "max(lo)" vao cho trong dau (bien dong slice thanh
  # "danh_sach_thoi_gian_tinh[i:i + max(lo)]") VA dien "kich_thuoc_lo" vao
  # cho trong hai (bien dong cong don thanh "tong += chi_phi_khoi_dong +
  # kich_thuoc_lo") -- tong so lan "max" VA uses-name("kich_thuoc_lo") KHONG
  # DOI (van la 1 va 2, chi doi VI TRI) -- static KHONG bat duoc mutant nay,
  # da xac nhan CHAY THAT qua kiemAst.
  # Mutant nay BI BAT boi tier 'run': dong slice "lo =
  # danh_sach_thoi_gian_tinh[i:i + max(lo)]" THAM CHIEU chinh bien 'lo' o
  # VE PHAI TRUOC KHI no duoc gan xong o VE TRAI CUNG dong nay -- Python coi
  # 'lo' la bien cuc bo (vi co gan trong vong lap), nen doc no truoc lan gan
  # dau tien nem UnboundLocalError NGAY LAP TUC o vong lap dau tien. Da tu
  # chay THAT qua python3, xac nhan thong bao "cannot access local variable
  # 'lo' where it is not associated with a value" -- bi chan boi tier
  # 'run', doc lap voi static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^1050\\n210\\n5\\.0\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`5,0` lần nhanh hơn — cùng dữ liệu, chỉ đổi cách gộp. Nhưng gộp lô mới GIẢI
QUYẾT được BAO NHIÊU request cùng lúc — nó chưa nói gì về việc GPU có ĐỦ
BỘ NHỚ cho tất cả request trong lô hay không. Câu hỏi tiếp theo LÀ về bộ
nhớ: KV-cache của mỗi request nên được cấp phát NHƯ THẾ NÀO?
::::

::::reflect{#nghi-lai}
Điểm dễ nhầm nhất của gộp lô không phải "trả chi phí khởi động một lần" —
điều đó khá trực giác. Điểm dễ nhầm LÀ thời gian TÍNH TOÁN của cả lô: nó
KHÔNG cộng dồn như xử lý tuần tự, mà LÀ giá trị LỚN NHẤT trong lô, vì các
request chạy SONG SONG trên cùng một GPU. Nhầm `max` thành `sum` là nhầm
đúng bản chất "song song" thành "tuần tự nhưng gộp nhãn" — hai điều nghe
giống nhau nhưng cho ra hai kết quả rất khác. Bài này đo được TỐC ĐỘ tăng
lên nhờ gộp lô; bài tiếp theo đo một chiều khác của cùng vấn đề: BỘ NHỚ.
::::

::::checkpoint{mastery=0.78}
::::
