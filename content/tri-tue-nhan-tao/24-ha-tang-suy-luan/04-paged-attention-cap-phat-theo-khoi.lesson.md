---
id: tri-tue-nhan-tao.ha-tang-suy-luan.paged-attention-cap-phat-theo-khoi
title: "PagedAttention rút gọn: cấp phát bộ nhớ theo khối, không cấp cố định"
summary: "lang_phi_cap_phat_co_dinh(danh_sach_do_dai_can, do_dai_cap_phat_co_dinh) cong don (do_dai_cap_phat_co_dinh - do_dai_can) cho TUNG request -- cap CO DINH bang do dai request DAI NHAT co the co, du request that su can it hon nhieu. lang_phi_cap_phat_theo_khoi(danh_sach_do_dai_can, kich_thuoc_khoi) chia theo KHOI co dinh (kich_thuoc_khoi), moi request chi duoc cap DUNG so khoi can (ceil(do_dai_can/kich_thuoc_khoi)), lang phi toi da la kich_thuoc_khoi-1 MOI request. Tren [10,100,45,8,60] voi do_dai_cap_phat_co_dinh=100, kich_thuoc_khoi=16: cach cu lang phi 277 don vi, cach moi (theo khoi) chi lang phi 33 don vi -- giam hon 8 lan."
locale: vi
track: tri-tue-nhan-tao
module: ha-tang-suy-luan
order: 4
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.paged-attention-cap-phat-theo-khoi]
requires: [ai.gop-lo-tang-toc-do]
concepts: [ai.paged-attention-cap-phat-theo-khoi]
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
Hai bài trước đo THỜI GIAN — gộp lô nhanh hơn tuần tự `5,0` lần. Nhưng
`Chương 39.2` nói tới một điểm nghẽn KHÁC hẳn: HuggFace `pipeline` mặc định
xử lý được RẤT ÍT request đồng thời — không phải vì chậm, mà vì hết BỘ NHỚ
GPU. Vấn đề LÀ **KV-cache bị cấp phát lãng phí**.
::::

::::explain{#cap_phat_co_dinh_vs_theo_khoi}
Mỗi request suy luận cần một vùng bộ nhớ GPU gọi LÀ **KV-cache** — lưu lại
kết quả tính toán trung gian của các token ĐÃ xử lý, để không phải tính
lại từ đầu cho mỗi token MỚI. Câu hỏi LÀ: cấp phát vùng nhớ này NHƯ THẾ
NÀO?

**Cách cũ (cấp phát cố định)**: vì không biết TRƯỚC một request sẽ dài bao
nhiêu, hệ thống cấp phát MỘT khối liền, đủ lớn cho request DÀI NHẤT có thể
xảy ra — cho MỌI request, kể cả những request ngắn. Phần dư thừa (khối lớn
trừ đi phần thực sự dùng) bị khoá lại, không ai khác dùng được, dù request
đó có ngắn tới đâu.

**PagedAttention (rút gọn)** — vay mượn Ý TƯỞNG **Virtual Memory** của hệ
điều hành — chia bộ nhớ thành các **khối nhỏ CỐ ĐỊNH** (`kich_thuoc_khoi`).
Mỗi request chỉ được cấp ĐÚNG số khối nó cần — làm tròn LÊN (`ceil`), vì
một request cần `17` đơn vị VẪN cần trọn một khối thứ hai dù chỉ dùng có
`1` đơn vị trong đó:

```
so_khoi_can(do_dai_can, kich_thuoc_khoi) = ceil(do_dai_can / kich_thuoc_khoi)
lang_phi_1_request = so_khoi_can * kich_thuoc_khoi - do_dai_can
```

Lãng phí TỐI ĐA của MỖI request giờ chỉ còn `kich_thuoc_khoi - 1` (đúng
BẰNG khối cuối cùng còn dư) — thay vì có thể lãng phí gần HẾT một khối
"cấp cố định" khổng lồ như cách cũ.
::::

::::example{#doi_chieu_co_dinh_va_theo_khoi}
```python title=readonly
def lang_phi_cap_phat_co_dinh(danh_sach_do_dai_can, do_dai_cap_phat_co_dinh):
    tong = 0
    for do_dai_can in danh_sach_do_dai_can:
        tong += do_dai_cap_phat_co_dinh - do_dai_can
    return tong


def lang_phi_cap_phat_theo_khoi(danh_sach_do_dai_can, kich_thuoc_khoi):
    tong = 0
    for do_dai_can in danh_sach_do_dai_can:
        so_khoi = (do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi
        tong += so_khoi * kich_thuoc_khoi - do_dai_can
    return tong


DO_DAI_CAP_PHAT_CO_DINH = 100
KICH_THUOC_KHOI = 16
DANH_SACH_DO_DAI_CAN = [10, 100, 45, 8, 60]

lang_phi_cu = lang_phi_cap_phat_co_dinh(DANH_SACH_DO_DAI_CAN, DO_DAI_CAP_PHAT_CO_DINH)
lang_phi_moi = lang_phi_cap_phat_theo_khoi(DANH_SACH_DO_DAI_CAN, KICH_THUOC_KHOI)

print(lang_phi_cu)
print(lang_phi_moi)
```

```text title=readonly
277
33
```

`5` request cần lần lượt `10, 100, 45, 8, 60` đơn vị bộ nhớ THẬT SỰ — dài
nhất LÀ `100`, nên cách cũ cấp CỐ ĐỊNH `100` đơn vị cho CẢ `5` request:
lãng phí LÀ `(100-10)+(100-100)+(100-45)+(100-8)+(100-60) = 90+0+55+92+40
= 277`. Cách mới (theo khối `16`): request cần `10` chỉ tốn `1` khối (`16`
đơn vị, lãng phí `6`); request cần `100` tốn `7` khối (`112` đơn vị, lãng
phí `12`); tương tự cho `45` (`3` khối, lãng phí `3`), `8` (`1` khối, lãng
phí `8`), `60` (`4` khối, lãng phí `4`). Tổng lãng phí mới:
`6+12+3+8+4 = 33`. Từ `277` xuống `33` — giảm hơn `8` lần lãng phí, trên
CÙNG một tập request.
::::

::::predict{#doan_request_17_don_vi commitOnce}
Xét đúng MỘT request cần `do_dai_can = 17` đơn vị, `kich_thuoc_khoi = 16`
(mỗi khối chứa được `16` đơn vị).

**Trước khi chạy thử**, bạn đoán: request này cần bao nhiêu khối, VÀ lãng
phí bao nhiêu đơn vị?

:::opt{correct}
`2` khối (`32` đơn vị được cấp phát), lãng phí `15` đơn vị — `17` đơn vị
CHỈ nhỉnh hơn đúng `1` khối (`16`), nhưng phần dư (`1` đơn vị) VẪN cần
TRỌN một khối THỨ HAI, vì hệ thống không thể cấp "nửa khối"; khối thứ hai
đó chỉ dùng `1/16` dung lượng, dư ra `15`
:::

:::opt
`1` khối (`16` đơn vị) là đủ, vì `17` "gần bằng" `16` nên có thể làm tròn
XUỐNG — thiếu đúng `1` đơn vị không đáng kể
::why
Gần đúng ở việc `17` THẬT SỰ rất gần `16` — chỉ lệch đúng `1` đơn vị, quan
sát đó đúng về độ lớn.

Chỗ lệch: bộ nhớ KHÔNG THỂ cấp phát THIẾU — nếu chỉ cấp `1` khối (`16` đơn
vị) cho một request cần `17`, request đó sẽ THIẾU đúng `1` đơn vị bộ nhớ
để lưu trọn kết quả tính toán, dẫn tới lỗi hoặc mất dữ liệu. Công thức
dùng `ceil` (làm tròn LÊN), không phải làm tròn xuống hay làm tròn theo độ
gần — bất kỳ phần dư nào, dù chỉ `1` đơn vị, đều BẮT BUỘC một khối MỚI.
::
:::

:::opt
Đúng `17` đơn vị được cấp phát, không dư khối nào — vì hệt thống chỉ cần
cấp CHÍNH XÁC bằng nhu cầu thực tế
::why
Gần đúng ở việc "cấp đúng theo nhu cầu" LÀ mục tiêu chung của cả bài này
(so với cách cũ cấp CỐ ĐỊNH `100`) — tinh thần đó đúng.

Chỗ lệch: PagedAttention KHÔNG cấp phát THEO ĐƠN VỊ LẺ — nó cấp theo
**KHỐI** kích thước cố định (`kich_thuoc_khoi = 16`), giống hệt cách hệ
điều hành cấp bộ nhớ ảo theo TRANG (page), không theo byte lẻ. Cấp đúng
`17` đơn vị (không dư) là mô tả một hệ thống KHÔNG có ràng buộc khối —
đúng LÀ điều PagedAttention KHÔNG làm; nó vẫn còn lãng phí (tối đa
`kich_thuoc_khoi - 1` mỗi request), chỉ LÀ ít lãng phí hơn HẲN cách cấp cố
định bằng độ dài dài nhất.
::
:::
::::

::::code{#viet_lang_phi_cap_phat_theo_khoi}
Hoàn thiện `lang_phi_cap_phat_theo_khoi`: tính số khối cần bằng `ceil`
(viết dưới dạng `(do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi`),
rồi cộng dồn phần dư (`so_khoi * kich_thuoc_khoi - do_dai_can`).

```python title=starter
def lang_phi_cap_phat_co_dinh(danh_sach_do_dai_can, do_dai_cap_phat_co_dinh):
    tong = 0
    for do_dai_can in danh_sach_do_dai_can:
        tong += do_dai_cap_phat_co_dinh - do_dai_can
    return tong


def lang_phi_cap_phat_theo_khoi(danh_sach_do_dai_can, kich_thuoc_khoi):
    tong = 0
    for do_dai_can in danh_sach_do_dai_can:
        so_khoi = ___                                        # (do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi
        tong += ___                                          # so_khoi * kich_thuoc_khoi - do_dai_can
    return tong


DO_DAI_CAP_PHAT_CO_DINH = 100
KICH_THUOC_KHOI = 16
DANH_SACH_DO_DAI_CAN = [10, 100, 45, 8, 60]

lang_phi_cu = lang_phi_cap_phat_co_dinh(DANH_SACH_DO_DAI_CAN, DO_DAI_CAP_PHAT_CO_DINH)
lang_phi_moi = lang_phi_cap_phat_theo_khoi(DANH_SACH_DO_DAI_CAN, KICH_THUOC_KHOI)

print(lang_phi_cu)
print(lang_phi_moi)
```

```python title=solution
def lang_phi_cap_phat_co_dinh(danh_sach_do_dai_can, do_dai_cap_phat_co_dinh):
    tong = 0
    for do_dai_can in danh_sach_do_dai_can:
        tong += do_dai_cap_phat_co_dinh - do_dai_can
    return tong


def lang_phi_cap_phat_theo_khoi(danh_sach_do_dai_can, kich_thuoc_khoi):
    tong = 0
    for do_dai_can in danh_sach_do_dai_can:
        so_khoi = (do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi
        tong += so_khoi * kich_thuoc_khoi - do_dai_can
    return tong


DO_DAI_CAP_PHAT_CO_DINH = 100
KICH_THUOC_KHOI = 16
DANH_SACH_DO_DAI_CAN = [10, 100, 45, 8, 60]

lang_phi_cu = lang_phi_cap_phat_co_dinh(DANH_SACH_DO_DAI_CAN, DO_DAI_CAP_PHAT_CO_DINH)
lang_phi_moi = lang_phi_cap_phat_theo_khoi(DANH_SACH_DO_DAI_CAN, KICH_THUOC_KHOI)

print(lang_phi_cu)
print(lang_phi_moi)
```

```python title=test
assert lang_phi_cap_phat_theo_khoi([10], 16) == 6, f"1 request can 10, khoi 16 -- can 1 khoi (16), lang phi phai la 6 -- dang ra {lang_phi_cap_phat_theo_khoi([10], 16)}"
assert lang_phi_cap_phat_theo_khoi([16], 16) == 0, f"request can DUNG BANG 1 khoi (16) khong duoc lang phi gi ca -- dang ra {lang_phi_cap_phat_theo_khoi([16], 16)}"
assert lang_phi_cap_phat_theo_khoi([17], 16) == 15, f"request can 17 (vua qua 1 khoi) phai can 2 khoi (32), lang phi 15 -- day la phep kiem CEIL chu khong phai FLOOR -- dang ra {lang_phi_cap_phat_theo_khoi([17], 16)}"
assert lang_phi_cap_phat_theo_khoi([10, 100, 45, 8, 60], 16) == 33, f"ca 5 request cong lai phai lang phi DUNG 33 don vi -- dang ra {lang_phi_cap_phat_theo_khoi([10, 100, 45, 8, 60], 16)}"
assert lang_phi_cap_phat_co_dinh([10, 100, 45, 8, 60], 100) == 277, f"cach cu (cap co dinh 100) tren cung 5 request phai lang phi 277 -- dang ra {lang_phi_cap_phat_co_dinh([10, 100, 45, 8, 60], 100)}"
assert lang_phi_cu == 277, f"lang_phi_cu (bien demo) phai la 277 -- dang ra {lang_phi_cu}"
assert lang_phi_moi == 33, f"lang_phi_moi (bien demo) phai la 33 -- dang ra {lang_phi_moi}"
```

:::hints
- kind: attention
  body: "Hai cho trong, trong cung mot vong lap cua lang_phi_cap_phat_theo_khoi. Cho dau tinh SO KHOI can (lam tron LEN -- ceil -- vi mot phan du du nho cung can tron mot khoi moi). Cho hai tinh PHAN LANG PHI cua request do: tong dung luong da cap (so_khoi nhan kich_thuoc_khoi) tru di dung luong THAT SU can."
- kind: strategy
  body: "Cho dau: (do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi -- day la cong thuc ceil-division kinh dien bang so nguyen (cong them kich_thuoc_khoi-1 truoc khi chia lay phan nguyen). Cho hai: so_khoi * kich_thuoc_khoi - do_dai_can -- dung luong da cap tru dung luong can."
- kind: one-line
  body: "Cho dau la (do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi, cho hai la so_khoi * kich_thuoc_khoi - do_dai_can."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: cho trong dau phai dung DUNG cong thuc ceil-division (do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi -- KHONG duoc dung // don thuan (do la FLOOR, thieu bo nho cho request vua qua mot khoi); cho trong hai phai la so_khoi * kich_thuoc_khoi - do_dai_can
  requireAst:
  - kind: uses-operator, target: "//", min: 1
  - kind: uses-operator, target: "*", min: 1
  - kind: uses-operator, target: "-", min: 3
  - kind: uses-operator, target: "+", min: 3
  # Da CHAY THAT qua kiemAst() (node + pyodide, trich solution tu chinh
  # file nay, TOAN BO ca hai ham) -- ket qua dung du kien:
  #   "//"=1: DUY NHAT o cho trong dau (phep chia lay phan nguyen trong
  #   cong thuc ceil). Ham lang_phi_cap_phat_co_dinh khong dung // o dau.
  #   "*"=1: DUY NHAT o cho trong hai (so_khoi * kich_thuoc_khoi).
  #   "-"=3: 1 lan trong lang_phi_cap_phat_co_dinh (do_dai_cap_phat_co_dinh
  #   - do_dai_can, DA CHO SAN trong starter), CONG 1 lan trong cho trong
  #   dau (kich_thuoc_khoi - 1), CONG 1 lan trong cho trong hai
  #   (so_khoi*kich_thuoc_khoi - do_dai_can) -- tong THAT la 3.
  #   "+"=3: 1 lan AugAssign trong lang_phi_cap_phat_co_dinh (tong += ...,
  #   DA CHO SAN), 1 lan trong cho trong dau (do_dai_can + kich_thuoc_khoi),
  #   1 lan AugAssign trong cho trong hai (tong += ...) -- tong THAT la 3.
  # Dien bua "True" vao CA HAI cho trong cho "//"=0, "*"=0, "-"=0 (chi con
  # "-" cua ham co_dinh da cho san... nhung do la "-"=0 that su vi ham
  # co_dinh van con nguyen -- kiem tra rieng xac nhan ca bon luat deu duoi
  # nguong khi CA HAI cho trong bi thay True), "+"=1 (chi con AugAssign cua
  # ham co_dinh) -- tat ca bon luat CHAN DUNG.
  # Mutant "floor thay vi ceil" (loi quan niem PHO BIEN nhat: "so_khoi =
  # do_dai_can // kich_thuoc_khoi", bo han "+kich_thuoc_khoi-1"): da tu
  # CHAY THAT qua kiemAst tren TOAN BO solution (ca hai ham) -- "-" TUT
  # xuong 2 (mat di so hang "kich_thuoc_khoi - 1"), "+" TUT xuong 2 (mat di
  # so hang "do_dai_can + kich_thuoc_khoi") -- CA HAI duoi nguong toi
  # thieu 3, static CHAN DUNG. Mutant nay CON bi tier 'tests' bat rieng qua
  # test [17]->15 (floor cho so_khoi=1, alloc=16 < can=17, waste am -1,
  # khac han 15 mong doi) -- hai lop chan doc lap cho cung mot loi.
  #
  # GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho trong
  # -- xac dinh ranh gioi tu chinh khoi starter -- va CHAY THAT qua
  # kiemAst va python3): dien "so_khoi * kich_thuoc_khoi - do_dai_can" vao
  # cho trong dau (dong "so_khoi = so_khoi * kich_thuoc_khoi - do_dai_can")
  # VA dien "(do_dai_can + kich_thuoc_khoi - 1) // kich_thuoc_khoi" vao cho
  # trong hai (dong "tong += (do_dai_can + kich_thuoc_khoi - 1) //
  # kich_thuoc_khoi") -- tong so lan "//" , "*", "-", "+" tren TOAN BO
  # solution KHONG DOI (van la 1/1/3/3, chi doi VI TRI) -- static KHONG bat
  # duoc mutant nay, da xac nhan CHAY THAT qua kiemAst.
  # Mutant nay BI BAT boi tier 'run': dong "so_khoi = so_khoi *
  # kich_thuoc_khoi - do_dai_can" THAM CHIEU chinh bien 'so_khoi' o VE PHAI
  # TRUOC KHI no duoc gan xong o VE TRAI CUNG dong nay (gan PHANG, khong
  # phai AugAssign) -- Python coi 'so_khoi' la bien cuc bo cua vong lap, nen
  # doc no truoc lan gan dau tien nem UnboundLocalError NGAY o vong lap dau
  # tien. Da tu chay THAT qua python3, xac nhan thong bao "cannot access
  # local variable 'so_khoi' where it is not associated with a value" -- bi
  # chan boi tier 'run', doc lap voi static.
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: "^277\\n33\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`277` xuống `33` — giảm hơn `8` lần lãng phí, chỉ bằng cách cấp phát theo
KHỐI thay vì cấp cố định. Bốn bài đã đo: ba tầng kiến trúc, thời gian tuần
tự lãng phí, gộp lô tăng tốc, bộ nhớ theo khối giảm lãng phí. BOSS quý ráp
lại HAI trong số đó thành một hàng đợi hoàn chỉnh, đo bằng THÔNG LƯỢNG.
::::

::::reflect{#nghi-lai}
`lang_phi_cap_phat_theo_khoi` không loại bỏ HOÀN TOÀN lãng phí — nó vẫn có
thể lãng phí tới `kich_thuoc_khoi - 1` đơn vị cho MỖI request (khối cuối
cùng luôn có khả năng dư một phần). Điều nó làm được LÀ giới hạn phần
lãng phí đó ở một HẰNG SỐ NHỎ, thay vì để nó tỉ lệ với khoảng cách giữa độ
dài THẬT SỰ của một request và độ dài DÀI NHẤT có thể xảy ra trong cả hệ
thống — đó chính LÀ khác biệt giữa `277` VÀ `33` trên cùng một tập dữ
liệu. Bài tiếp theo (BOSS quý, đóng `q8.6a` tại `5/5`) không thêm khái
niệm mới — nó RÁP LẠI hai bài (`pipeline-tuan-tu-lang-phi`,
`gop-lo-tang-toc-do`) thành MỘT hàng đợi xử lý, đo bằng THÔNG LƯỢNG
(request phục vụ được trên MỖI đơn vị thời gian).
::::

::::checkpoint{mastery=0.78}
::::
