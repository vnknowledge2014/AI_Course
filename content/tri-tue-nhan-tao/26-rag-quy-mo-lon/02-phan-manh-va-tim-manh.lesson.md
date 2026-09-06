---
id: tri-tue-nhan-tao.rag-quy-mo-lon.phan-manh-va-tim-manh
title: "Phân mảnh (sharding): round-robin chia kho lớn, KHÔNG mất KHÔNG trùng"
summary: "tim_manh_chua(chi_so_tai_lieu, so_manh) = chi_so_tai_lieu % so_manh -- ham bam CO DINH, round-robin sharding. phan_manh(danh_sach_tai_lieu, so_manh) chia danh sach thanh so_manh mang con, moi tai lieu chi_so i roi DUNG vao manh tim_manh_chua(i, so_manh). Tren kho 10 'doan' (DOAN = doan_0..doan_9), so_manh=3: manh 0 nhan chi so 0,3,6,9 (4 tai lieu), manh 1 nhan 1,4,7 (3), manh 2 nhan 2,5,8 (3) -- tong 4+3+3=10, BANG DUNG tong goc, khong mat khong trung. tim_manh_chua(7,3)=1 nhat quan voi viec doan_7 nam trong manh 1. Doi so_manh tu 3 sang 4: tim_manh_chua(7,4)=3 (khac han 1) -- tham so so_manh THAT SU rang buoc ket qua, va tong tren moi manh VAN LA 10 du chia kieu nao."
locale: vi
track: tri-tue-nhan-tao
module: rag-quy-mo-lon
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [ai.phan-manh-va-tim-manh]
requires: [ai.nap-du-lieu-theo-su-kien]
concepts: [ai.phan-manh-va-tim-manh]
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
`1` triệu tài liệu không nằm gọn trong MỘT máy, MỘT cấu trúc dữ liệu.
`Chương 39.4` mục `2` gọi đây LÀ **Vector Database Sharding** — chia một kho
khổng lồ thành nhiều MẢNH nhỏ hơn. Câu hỏi: chia THEO CÁCH nào để không mất,
không trùng, và LUÔN biết tài liệu nào nằm ở mảnh nào?
::::

::::explain{#bam_co_dinh_round_robin}
Qdrant, Milvus, Pinecone (tên nêu ở `Chương 39.4`) đều chia một kho lớn
thành nhiều **mảnh** (shard) — mỗi mảnh có thể nằm trên một máy vật lý khác
nhau. Cách chia đơn giản nhất, VẪN đủ để hiểu ý tưởng cốt lõi: **round-robin
sharding** — tài liệu thứ `0` vào mảnh `0`, tài liệu thứ `1` vào mảnh `1`,
… cho tới khi hết số mảnh thì QUAY LẠI mảnh `0`.

Công thức: một phép **chia lấy dư** (modulo) CỐ ĐỊNH.

```
tim_manh_chua(chi_so_tai_lieu, so_manh) = chi_so_tai_lieu % so_manh
```

Vì `%` luôn cho ra CÙNG một kết quả với CÙNG một cặp `(chi_so_tai_lieu,
so_manh)` — không có yếu tố ngẫu nhiên, không có trạng thái nào thay đổi
theo thời gian — công thức này bảo đảm HAI bất biến quan trọng:

1. **Không mất, không trùng**: mỗi tài liệu rơi vào ĐÚNG MỘT mảnh (không
   phải không mảnh nào, không phải nhiều hơn một mảnh) — tổng số tài liệu
   trên MỌI mảnh cộng lại LUÔN bằng đúng tổng số tài liệu gốc.
2. **Nhất quán**: gọi `tim_manh_chua` nhiều lần với CÙNG chỉ số và CÙNG số
   mảnh luôn cho CÙNG một câu trả lời — một hệ thống thật cần điều này để
   biết ĐI TÌM một tài liệu cụ thể ở mảnh nào, không cần dò cả `so_manh`
   mảnh.

`phan_manh(danh_sach_tai_lieu, so_manh)` dùng đúng công thức đó để XẾP từng
tài liệu vào mảng con tương ứng.
::::

::::example{#chia_10_doan_thanh_3_manh}
```python title=readonly
def tim_manh_chua(chi_so_tai_lieu, so_manh):
    return chi_so_tai_lieu % so_manh


def phan_manh(danh_sach_tai_lieu, so_manh):
    cac_manh = [[] for _ in range(so_manh)]
    for chi_so, tai_lieu in enumerate(danh_sach_tai_lieu):
        chi_so_manh = tim_manh_chua(chi_so, so_manh)
        cac_manh[chi_so_manh].append(tai_lieu)
    return cac_manh


DOAN = [f"doan_{i}" for i in range(10)]

CAC_MANH = phan_manh(DOAN, 3)
tong_tren_moi_manh = [len(m) for m in CAC_MANH]
tong_tat_ca = sum(tong_tren_moi_manh)
manh_cua_doan_7 = tim_manh_chua(7, 3)

print(CAC_MANH)
print(tong_tren_moi_manh)
print(tong_tat_ca)
print(manh_cua_doan_7)
```

```text title=readonly
[['doan_0', 'doan_3', 'doan_6', 'doan_9'], ['doan_1', 'doan_4', 'doan_7'], ['doan_2', 'doan_5', 'doan_8']]
[4, 3, 3]
10
1
```

`DOAN` có đúng `10` tài liệu (chỉ số `0` tới `9`). Với `so_manh = 3`: tài
liệu chỉ số `0, 3, 6, 9` (mọi chỉ số chia hết cho `3`) rơi vào mảnh `0` —
`4` tài liệu. Chỉ số `1, 4, 7` rơi vào mảnh `1` — `3` tài liệu. Chỉ số
`2, 5, 8` rơi vào mảnh `2` — `3` tài liệu. `tong_tren_moi_manh = [4, 3, 3]`,
cộng lại `4 + 3 + 3 = 10` — ĐÚNG bằng `len(DOAN)`, không thiếu không thừa
một tài liệu nào. `manh_cua_doan_7 = tim_manh_chua(7, 3) = 1` khớp CHÍNH XÁC
với việc `"doan_7"` thật sự nằm trong `CAC_MANH[1]` — công thức và kết quả
thực tế NHẤT QUÁN với nhau.
::::

::::predict{#doan_doi_so_manh_thanh_4 commitOnce}
Xét việc gọi lại `tim_manh_chua(7, so_manh)` — CÙNG chỉ số tài liệu (`7`) —
nhưng đổi `so_manh` từ `3` (như ví dụ trên) THÀNH `4`.

**Trước khi chạy thử**, bạn đoán: `tim_manh_chua(7, 4)` trả về mấy?

:::opt{correct}
`3` — vì `7 % 4 = 3` (`7` chia `4` được `1`, dư `3`); đổi `so_manh` đổi
NGAY kết quả, vì công thức là `chi_so_tai_lieu % so_manh`, phụ thuộc TRỰC
TIẾP vào cả hai tham số
:::

:::opt
Vẫn là `1`, giống hệt `tim_manh_chua(7, 3)` ở ví dụ trên — vì tài liệu chỉ
số `7` luôn thuộc về đúng MỘT mảnh cố định, không phụ thuộc có bao nhiêu
mảnh đang tồn tại
::why
Gần đúng ở việc "một tài liệu luôn thuộc về đúng MỘT mảnh" — bất biến đó
đúng, NHƯNG chỉ đúng với một `so_manh` CỐ ĐỊNH đã chọn. Đổi tổng số mảnh
(`so_manh`) THAY ĐỔI HẲN cách chia — round-robin phân bố lại toàn bộ khi số
mảnh đổi.

Chỗ lệch: `tim_manh_chua` không "nhớ" tài liệu `7` từng ở mảnh nào trước đó
— nó tính LẠI từ đầu bằng công thức `chi_so_tai_lieu % so_manh` với chính
`so_manh` được truyền VÀO lần gọi đó. `7 % 3 = 1` và `7 % 4 = 3` là hai phép
tính HOÀN TOÀN khác nhau, không liên quan tới nhau.
::
:::

:::opt
`4`, vì phép chia lấy dư đảo ngược thứ tự hai toán hạng: `so_manh % chi_so_tai_lieu`
tức `4 % 7 = 4`
::why
Gần đúng ở việc phép `%` THẬT SỰ nhạy với THỨ TỰ hai toán hạng (`a % b`
khác `b % a` nói chung) — quan sát đó đúng về bản chất toán học của `%`.

Chỗ lệch: công thức của `tim_manh_chua` LUÔN đặt `chi_so_tai_lieu` (tham số
ĐẦU) TRƯỚC dấu `%`, và `so_manh` (tham số HAI) SAU dấu `%` — thứ tự này cố
định trong định nghĩa hàm, không đảo theo cách gọi. `tim_manh_chua(7, 4)`
tính đúng `7 % 4`, không phải `4 % 7`.
::
:::
::::

::::code{#viet_tim_manh_va_phan_manh}
Hoàn thiện `tim_manh_chua` (công thức băm cố định `%`) và `phan_manh` (dùng
LẠI `tim_manh_chua` để xếp mỗi tài liệu vào đúng mảng con).

```python title=starter
def tim_manh_chua(chi_so_tai_lieu, so_manh):
    return ___                                    # chi_so_tai_lieu % so_manh


def phan_manh(danh_sach_tai_lieu, so_manh):
    cac_manh = [[] for _ in range(so_manh)]
    for chi_so, tai_lieu in enumerate(danh_sach_tai_lieu):
        chi_so_manh = ___                          # tim_manh_chua(chi_so, so_manh)
        cac_manh[chi_so_manh].append(tai_lieu)
    return cac_manh


DOAN = [f"doan_{i}" for i in range(10)]

CAC_MANH = phan_manh(DOAN, 3)
tong_tren_moi_manh = [len(m) for m in CAC_MANH]
tong_tat_ca = sum(tong_tren_moi_manh)
manh_cua_doan_7 = tim_manh_chua(7, 3)

print(CAC_MANH)
print(tong_tren_moi_manh)
print(tong_tat_ca)
print(manh_cua_doan_7)
```

```python title=solution
def tim_manh_chua(chi_so_tai_lieu, so_manh):
    return chi_so_tai_lieu % so_manh


def phan_manh(danh_sach_tai_lieu, so_manh):
    cac_manh = [[] for _ in range(so_manh)]
    for chi_so, tai_lieu in enumerate(danh_sach_tai_lieu):
        chi_so_manh = tim_manh_chua(chi_so, so_manh)
        cac_manh[chi_so_manh].append(tai_lieu)
    return cac_manh


DOAN = [f"doan_{i}" for i in range(10)]

CAC_MANH = phan_manh(DOAN, 3)
tong_tren_moi_manh = [len(m) for m in CAC_MANH]
tong_tat_ca = sum(tong_tren_moi_manh)
manh_cua_doan_7 = tim_manh_chua(7, 3)

print(CAC_MANH)
print(tong_tren_moi_manh)
print(tong_tat_ca)
print(manh_cua_doan_7)
```

```python title=test
assert tim_manh_chua(0, 3) == 0, f"0 % 3 = 0 -- dang ra {tim_manh_chua(0, 3)}"
assert tim_manh_chua(7, 3) == 1, f"7 % 3 = 1 -- dang ra {tim_manh_chua(7, 3)}"
assert tim_manh_chua(9, 3) == 0, f"9 % 3 = 0 -- dang ra {tim_manh_chua(9, 3)}"
assert tim_manh_chua(7, 4) == 3, f"doi so_manh tu 3 sang 4: 7 % 4 = 3, PHAI khac ket qua cu -- dang ra {tim_manh_chua(7, 4)}"

assert CAC_MANH == [
    ["doan_0", "doan_3", "doan_6", "doan_9"],
    ["doan_1", "doan_4", "doan_7"],
    ["doan_2", "doan_5", "doan_8"],
], f"CAC_MANH sai -- dang ra {CAC_MANH}"
assert tong_tren_moi_manh == [4, 3, 3], f"tong_tren_moi_manh phai la [4, 3, 3] -- dang ra {tong_tren_moi_manh}"
assert tong_tat_ca == len(DOAN), f"tong so tai lieu tren MOI manh cong lai phai BANG DUNG tong goc (10) -- dang ra {tong_tat_ca}"
assert manh_cua_doan_7 == 1, f"manh_cua_doan_7 phai la 1 -- dang ra {manh_cua_doan_7}"

# bien: doi so_manh THAT SU rang buoc phan_manh -- tong VAN la 10 nhung
# cach chia KHAC han
CAC_MANH_4 = phan_manh(DOAN, 4)
assert sum(len(m) for m in CAC_MANH_4) == 10, f"doi so_manh sang 4, tong VAN phai la 10 -- dang ra {sum(len(m) for m in CAC_MANH_4)}"
assert CAC_MANH_4 != CAC_MANH, "doi so_manh tu 3 sang 4 phai cho ra CACH CHIA khac (so manh khac nhau, phan bo khac nhau)"
assert len(CAC_MANH_4) == 4, f"phan_manh(DOAN, 4) phai co DUNG 4 manh -- dang ra {len(CAC_MANH_4)}"

# bien: moi tai lieu chi nam DUNG mot manh duy nhat -- khong trung
for chi_so, tai_lieu in enumerate(DOAN):
    manh_dung = tim_manh_chua(chi_so, 3)
    so_lan_xuat_hien = sum(1 for m in CAC_MANH if tai_lieu in m)
    assert so_lan_xuat_hien == 1, f"{tai_lieu} phai xuat hien DUNG 1 lan trong toan bo CAC_MANH -- dang ra {so_lan_xuat_hien}"
    assert tai_lieu in CAC_MANH[manh_dung], f"{tai_lieu} phai nam trong manh {manh_dung} theo dung cong thuc tim_manh_chua"

# bien: hang doi rong cho ra cac manh RONG, khong loi
CAC_MANH_RONG = phan_manh([], 3)
assert CAC_MANH_RONG == [[], [], []], f"danh sach rong phai cho 3 manh rong -- dang ra {CAC_MANH_RONG}"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `tim_manh_chua`) là GIÁ TRỊ TRẢ VỀ của cả hàm — một phép TÍNH, không phải một lời gọi hàm nào khác. Chỗ hai (trong `phan_manh`) là GIÁ TRỊ GÁN cho `chi_so_manh` — một lời GỌI LẠI hàm vừa viết ở chỗ đầu, KHÔNG được viết lại công thức `%` một lần nữa ở đây.
- kind: strategy
  body: 'Chỗ đầu: `chi_so_tai_lieu % so_manh` — phép chia lấy dư, tài liệu chỉ số CÀNG lớn thì càng "quay vòng" qua các mảnh. Chỗ hai: `tim_manh_chua(chi_so, so_manh)` — TÁI SỬ DỤNG hàm vừa viết, truyền đúng biến vòng lặp `chi_so` (không phải `tai_lieu`) và tham số `so_manh` của `phan_manh`.'
- kind: one-line
  body: 'Chỗ đầu là `chi_so_tai_lieu % so_manh`, chỗ hai là `tim_manh_chua(chi_so, so_manh)`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la phep TINH chi_so_tai_lieu % so_manh (khong duoc goi ham nao khac); cho trong hai phai GOI LAI tim_manh_chua(chi_so, so_manh) (khong duoc viet lai cong thuc % mot lan nua trong phan_manh)
  requireAst:
  - kind: uses-operator, target: "%", min: 1
  - kind: uses-call, target: tim_manh_chua, min: 2
  # Da CHAY THAT qua kiemAst() (node + pyodide, dist build that, trich
  # solution TU CHINH file nay) -- xac nhan CHINH XAC (min VA min+1): "%"=1,
  # tim_manh_chua=2. "%"=1: DUY NHAT o cho trong dau (chi_so_tai_lieu %
  # so_manh) -- khong noi nao khac trong solution dung phep %.
  # tim_manh_chua=2 (TONG THAT, khong phai boilerplate lon): 1 lan o cho
  # trong hai (goi tu phan_manh), CONG 1 lan o dong demo cuoi file
  # (manh_cua_doan_7 = tim_manh_chua(7, 3)) -- ham nay KHONG tu goi lai
  # chinh no (khong de quy).
  # Dien bua "True" vao ca hai cho trong ("return True" va "chi_so_manh =
  # True") cho "%"=0 VA tim_manh_chua=1 (duoi nguong 2, vi luc do chi con
  # dong demo goi no) -- CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua
  # kiemAst).
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter -- va CHAY THAT qua
  # kiemAst() THAT VA python3 THAT): dien "tim_manh_chua(chi_so, so_manh)"
  # vao cho trong dau ("return tim_manh_chua(chi_so, so_manh)" trong
  # tim_manh_chua) VA dien "chi_so_tai_lieu % so_manh" vao cho trong hai
  # ("chi_so_manh = chi_so_tai_lieu % so_manh" trong phan_manh) -- da CHAY
  # THAT qua kiemAst(): tong so lan "%" VA tong so lan goi tim_manh_chua
  # tren TOAN BO solution DEU KHONG DOI (van dung 1 va 2, chi doi VI TRI) --
  # static KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong tim_manh_chua (tham so la
  # chi_so_tai_lieu, so_manh -- KHONG CO bien "chi_so" nao trong scope
  # nay), bieu thuc moi "tim_manh_chua(chi_so, so_manh)" doc ten "chi_so"
  # CHUA TON TAI -- da tu chay THAT qua python3, xac nhan NameError "name
  # 'chi_so' is not defined" ngay khi tim_manh_chua duoc goi lan dau (dong
  # demo cuoi file). Ben trong phan_manh (bien vong lap la chi_so, tai_lieu
  # -- KHONG CO "chi_so_tai_lieu" nao trong scope nay), bieu thuc moi
  # "chi_so_tai_lieu % so_manh" doc ten "chi_so_tai_lieu" CHUA TON TAI --
  # da tu chay THAT xac nhan NameError "name 'chi_so_tai_lieu' is not
  # defined" ngay vong lap dau tien cua phan_manh. Ca hai bi chan boi tier
  # 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: "chi_so_tai_lieu" va "chi_so" la hai ten KHAC
  # NHAU (khong trung), "so_manh" la ten THAM SO chung o CA HAI ham (dung y
  # -- cung mot y nghia "tong so manh" o moi noi no xuat hien) nen khong co
  # rui ro nham lan tinh co ve HINH DANG.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\[\\['doan_0', 'doan_3', 'doan_6', 'doan_9'\\], \\['doan_1', 'doan_4', 'doan_7'\\], \\['doan_2', 'doan_5', 'doan_8'\\]\\]\\n\\[4, 3, 3\\]\\n10\\n1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4 + 3 + 3 = 10` — không thiếu, không thừa một tài liệu nào; `tim_manh_chua`
luôn nói ĐÚNG mảnh nào chứa tài liệu nào. Bài tiếp theo hỏi câu hỏi QUAN
TRỌNG HƠN sharding: khi kho có NHIỀU khách hàng khác nhau, dữ liệu của họ có
được phép "lẫn" vào nhau qua tìm kiếm không?
::::

::::reflect{#nghi-lai}
`tim_manh_chua` không "thông minh" — nó chỉ là một phép `%` duy nhất, không
quan tâm nội dung tài liệu, không cân bằng tải theo kích thước thật. Đó
CHÍNH LÀ điểm mạnh của round-robin sharding: một công thức CỐ ĐỊNH, không
trạng thái, luôn cho CÙNG câu trả lời với CÙNG đầu vào — nên một hệ thống
LỚN không cần "nhớ" tài liệu nào ở đâu, chỉ cần TÍNH LẠI từ chỉ số của nó.
Bất biến đo được trên `10` tài liệu chia `3` mảnh: tổng `4 + 3 + 3 = 10`,
khớp đúng gốc. Đổi `so_manh` từ `3` sang `4` đổi HẲN cách phân bố (tổng vẫn
`10`, nhưng phân bố khác) — sharding và số lượng mảnh là một QUYẾT ĐỊNH vận
hành, không phải một hằng số cố định của dữ liệu.

Sharding giải quyết bài toán "kho quá LỚN cho một máy". Bài tiếp theo giải
quyết một bài toán KHÁC hẳn, dù nghe có vẻ liên quan: "kho có NHIỀU khách
hàng, dữ liệu của họ không được LẪN vào nhau" — `Chương 39.4` gọi đây LÀ
**Tenant Isolation**.
::::

::::checkpoint{mastery=0.77}
::::
