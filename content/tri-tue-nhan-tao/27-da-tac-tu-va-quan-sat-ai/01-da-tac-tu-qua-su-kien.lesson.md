---
id: tri-tue-nhan-tao.da-tac-tu-va-quan-sat-ai.da-tac-tu-qua-su-kien
title: "Đa tác tử qua sự kiện: Coder viết, Tester chấm, Coder sửa — không gọi trực tiếp"
summary: "xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua) mo phong §39.5: neu su_kien la CodeWrittenEvent, chay_test_tat_dinh (so sanh ket_qua_tinh voi ket_qua_mong_doi) roi day vao hang_doi_ket_qua MOT TestPassedEvent (dung) hoac TestFailedEvent (sai); neu la TestFailedEvent, day vao MOT CodeWrittenEvent moi voi ket_qua_tinh = ket_qua_tinh cu + 1 (Coder sua). xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap) xu ly TUNG su kien mot tu dau hang doi, noi dai hang doi voi su kien moi sinh, dung khi hang doi RONG HOAC dat gioi_han_vong_lap. Kich ban 1 (bug ket_qua_tinh=4 sai voi mong_doi=5, gioi_han=10): 4 su kien theo dung thu tu CodeWrittenEvent-TestFailedEvent-CodeWrittenEvent-TestPassedEvent, hang doi con lai RONG (0). Kich ban 2 (bug ket_qua_tinh=0 con cach mong_doi=100 rat xa, gioi_han=5): dung DUNG tai 5 su kien, hang doi con lai VAN CON 1 phan tu -- gioi_han_vong_lap THAT SU chan vong lap, khong doi tu nhien."
locale: vi
track: tri-tue-nhan-tao
module: da-tac-tu-va-quan-sat-ai
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [ai.da-tac-tu-qua-su-kien]
requires: [ai.boss-do-toan-ven-cach-ly]
concepts: [ai.da-tac-tu-qua-su-kien]
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
`q8.6c` "RAG quy mô lớn" đã đóng tại `6/6`. `T8.6` chuyển sang quest CUỐI:
`q8.6d` "Đa tác tử và quan sát AI". `Chương 39.5` mô tả ba tác tử — Coder,
Tester, Reviewer — cần nói chuyện với nhau. Câu hỏi mở đầu: KHÔNG được gọi
trực tiếp (HTTP) vì Agent "nghĩ" rất lâu — vậy chúng phối hợp bằng cách nào?
::::

::::explain{#event_sourcing_khong_goi_truc_tiep}
`Chương 39.5` nêu rõ lý do: Agent Coder có thể mất `5` phút để viết xong một
đoạn code. Nếu Agent Tester gọi thẳng Coder qua HTTP và CHỜ phản hồi (đồng
bộ), cả hệ thống đứng khựng lại trong `5` phút đó. Giải pháp là **Event
Sourcing** (đã học ở R7): mỗi Agent KHÔNG gọi Agent khác — nó chỉ BẮN một
**sự kiện**, rồi đi làm việc khác. Một Agent KHÁC, đang lắng nghe hàng đợi
sự kiện, bắt được sự kiện đó và phản ứng:

1. **Agent Coder** viết code xong → bắn `CodeWrittenEvent(task_id)`.
2. **Agent Tester** bắt được → chạy test sandbox → bắn `TestFailedEvent`
   (hoặc `TestPassedEvent` nếu qua).
3. **Agent Coder** nhận `TestFailedEvent` → sửa code → bắn lại
   `CodeWrittenEvent` MỚI.

Sandbox này không có Kafka/Temporal thật (`Chương 39.5` nhắc tới cả hai).
Nhưng cốt lõi — "một sự kiện được XỬ LÝ, có thể SINH ra sự kiện MỚI, sự kiện
mới đó lại được xử lý tiếp, theo ĐÚNG thứ tự" — mô phỏng được bằng một
`list` Python đóng vai trò hàng đợi, xử lý bằng `.pop(0)`, và sự kiện mới
được `.extend()` vào CUỐI hàng đợi đó:

```
xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
  neu su_kien la CodeWrittenEvent:
    neu chay_test_tat_dinh(su_kien):        # so sanh ket_qua_tinh == ket_qua_mong_doi
      day TestPassedEvent vao hang_doi_ket_qua
    khac:
      day TestFailedEvent vao hang_doi_ket_qua
  neu su_kien la TestFailedEvent:
    day MOT CodeWrittenEvent MOI (Coder da "sua") vao hang_doi_ket_qua

xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
  hang_doi = BAN SAO cua hang_doi_ban_dau
  da_xu_ly = []
  so_vong = 0
  trong khi hang_doi CON PHAN TU VA so_vong < gioi_han_vong_lap:
    su_kien = hang_doi.pop(0)
    da_xu_ly.append(su_kien)
    xu_ly_mot_su_kien(su_kien, ...) roi noi ket qua vao CUOI hang_doi
    so_vong += 1
  tra ve da_xu_ly, hang_doi   # hang_doi con lai -- rong nghia la xong THAT
```

Điểm dạy quan trọng nhất: một `TestFailedEvent` không phải "lỗi cuối cùng"
— nó SINH RA một sự kiện MỚI (`CodeWrittenEvent`), và vòng lặp phải xử lý
tiếp sự kiện đó. Nhưng nếu Coder cứ sửa mãi mà KHÔNG bao giờ đúng, vòng lặp
sẽ chạy VÔ HẠN — nên `gioi_han_vong_lap` là một trần BẮT BUỘC, không phải
tùy chọn.
::::

::::example{#coder_sai_roi_sua_dung}
```python title=readonly
def chay_test_tat_dinh(su_kien):
    return su_kien["ket_qua_tinh"] == su_kien["ket_qua_mong_doi"]


def xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
    loai = su_kien["loai_su_kien"]
    if loai == "CodeWrittenEvent":
        if chay_test_tat_dinh(su_kien):
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestPassedEvent",
                "task_id": su_kien["task_id"],
            })
        else:
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestFailedEvent",
                "task_id": su_kien["task_id"],
                "ket_qua_tinh": su_kien["ket_qua_tinh"],
                "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
            })
    elif loai == "TestFailedEvent":
        hang_doi_ket_qua.append({
            "loai_su_kien": "CodeWrittenEvent",
            "task_id": su_kien["task_id"],
            "ket_qua_tinh": su_kien["ket_qua_tinh"] + 1,
            "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
        })


def xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
    hang_doi = list(hang_doi_ban_dau)
    da_xu_ly = []
    so_vong = 0
    while hang_doi and so_vong < gioi_han_vong_lap:
        su_kien = hang_doi.pop(0)
        da_xu_ly.append(su_kien)
        ket_qua_moi = []
        xu_ly_mot_su_kien(su_kien, ket_qua_moi)
        hang_doi.extend(ket_qua_moi)
        so_vong += 1
    return da_xu_ly, hang_doi


HANG_DOI_1 = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 1, "ket_qua_tinh": 4, "ket_qua_mong_doi": 5},
]
HANG_DOI_2 = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 2, "ket_qua_tinh": 0, "ket_qua_mong_doi": 100},
]

KET_QUA_1, CON_LAI_1 = xu_ly_toan_bo_hang_doi(HANG_DOI_1, 10)
KET_QUA_2, CON_LAI_2 = xu_ly_toan_bo_hang_doi(HANG_DOI_2, 5)

print([sk["loai_su_kien"] for sk in KET_QUA_1])
print(len(KET_QUA_1))
print(len(CON_LAI_1))
print([sk["loai_su_kien"] for sk in KET_QUA_2])
print(len(KET_QUA_2))
print(len(CON_LAI_2))
```

```text title=readonly
['CodeWrittenEvent', 'TestFailedEvent', 'CodeWrittenEvent', 'TestPassedEvent']
4
0
['CodeWrittenEvent', 'TestFailedEvent', 'CodeWrittenEvent', 'TestFailedEvent', 'CodeWrittenEvent']
5
1
```

`HANG_DOI_1`: task `1` viết code với `ket_qua_tinh = 4`, nhưng
`ket_qua_mong_doi = 5` — SAI. Vòng `0`: rút `CodeWrittenEvent(4)`, test
sai, sinh `TestFailedEvent(4)`. Vòng `1`: rút `TestFailedEvent(4)`, Coder
"sửa" — sinh `CodeWrittenEvent(ket_qua_tinh = 4 + 1 = 5)`. Vòng `2`: rút
`CodeWrittenEvent(5)`, test ĐÚNG (`5 == 5`), sinh `TestPassedEvent`. Vòng
`3`: rút `TestPassedEvent` — không sinh gì thêm (task đã xong). Hàng đợi
RỖNG — dừng tự nhiên. Đúng `4` sự kiện, đúng thứ tự
`CodeWrittenEvent → TestFailedEvent → CodeWrittenEvent → TestPassedEvent`,
`CON_LAI_1` rỗng (`0`) — xác nhận nó dừng vì XONG, không phải vì hết giới
hạn.

`HANG_DOI_2`: task `2` bắt đầu RẤT xa đáp án (`ket_qua_tinh = 0`,
`ket_qua_mong_doi = 100`) — mỗi lần sửa chỉ `+1`, cần tới `100` lần mới
đúng. Với `gioi_han_vong_lap = 5`: sau đúng `5` vòng (`CodeWrittenEvent(0) →
TestFailedEvent → CodeWrittenEvent(1) → TestFailedEvent →
CodeWrittenEvent(2)`), vòng lặp DỪNG vì `so_vong` chạm giới hạn — KHÔNG phải
vì hàng đợi rỗng. `CON_LAI_2` còn đúng `1` phần tử
(`TestFailedEvent` chưa được xử lý) — bằng chứng cụ thể rằng
`gioi_han_vong_lap` đã thật sự CẮT một vòng lặp còn dang dở, không để nó
chạy vô hạn.
::::

::::predict{#doan_doi_gioi_han_vong_lap commitOnce}
Xét lại `HANG_DOI_2` ở ví dụ trên (task bắt đầu từ `ket_qua_tinh = 0`, cần
tới `100` mới đúng — không thể xong trong vài vòng). Ở ví dụ,
`gioi_han_vong_lap = 5` cho ra `len(KET_QUA_2) = 5` và `len(CON_LAI_2) = 1`.

**Trước khi chạy thử**, bạn đoán: nếu đổi `gioi_han_vong_lap` từ `5` XUỐNG
`4` (giữ nguyên `HANG_DOI_2`), `len(KET_QUA_2)` và `len(CON_LAI_2)` MỚI LÀ
bao nhiêu?

:::opt{correct}
`4` và `1` — vòng lặp dừng SỚM hơn đúng `1` vòng (`CodeWrittenEvent(0) →
TestFailedEvent → CodeWrittenEvent(1) → TestFailedEvent`, dừng ở vòng `4`);
hàng đợi còn lại vẫn có đúng `1` phần tử chưa xử lý
(`CodeWrittenEvent(2)` chưa kịp sinh ra)
:::

:::opt
`5` và `1`, không đổi gì — vì `HANG_DOI_2` (dữ liệu đầu vào) không hề thay
đổi, chỉ đổi một tham số phụ
::why
Gần đúng ở việc `HANG_DOI_2` THẬT SỰ không đổi — quan sát đó đúng.

Chỗ lệch: `gioi_han_vong_lap` không phải "tham số phụ" — nó là điều kiện
DỪNG của vòng `while` (`so_vong < gioi_han_vong_lap`), và với task này (cần
`100` vòng mới xong), vòng lặp LUÔN dừng vì chạm giới hạn, không bao giờ vì
hàng đợi rỗng. Đổi giới hạn từ `5` xuống `4` đổi TRỰC TIẾP số vòng đã chạy
trước khi dừng.
::
:::

:::opt
`4` và `0` — giảm giới hạn nghĩa là hàng đợi phải được xử lý HẾT nhanh hơn,
nên không còn phần tử nào sót lại
::why
Gần đúng ở việc số sự kiện ĐÃ xử lý giảm xuống `4` — quan sát đó đúng.

Chỗ lệch: giảm giới hạn làm vòng lặp dừng SỚM HƠN, tức là CÀNG NHIỀU việc
còn dang dở, không phải ít hơn. Với task cần `100` vòng, cắt ở vòng `4`
vẫn để lại đúng `1` sự kiện chưa xử lý trong hàng đợi — y hệt lý do
`gioi_han_vong_lap = 5` để lại `1` sự kiện trong ví dụ gốc.
::
:::
::::

::::code{#viet_xu_ly_su_kien_va_hang_doi}
Hoàn thiện `xu_ly_mot_su_kien` (nhánh `TestFailedEvent`: Coder "sửa" bằng
cách cộng thêm `1` vào `ket_qua_tinh` cũ) và `xu_ly_toan_bo_hang_doi`
(điều kiện DỪNG của vòng `while`: hàng đợi còn phần tử VÀ chưa chạm giới
hạn vòng lặp).

```python title=starter
def chay_test_tat_dinh(su_kien):
    return su_kien["ket_qua_tinh"] == su_kien["ket_qua_mong_doi"]


def xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
    loai = su_kien["loai_su_kien"]
    if loai == "CodeWrittenEvent":
        if chay_test_tat_dinh(su_kien):
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestPassedEvent",
                "task_id": su_kien["task_id"],
            })
        else:
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestFailedEvent",
                "task_id": su_kien["task_id"],
                "ket_qua_tinh": su_kien["ket_qua_tinh"],
                "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
            })
    elif loai == "TestFailedEvent":
        hang_doi_ket_qua.append({
            "loai_su_kien": "CodeWrittenEvent",
            "task_id": su_kien["task_id"],
            "ket_qua_tinh": ___,                                # su_kien["ket_qua_tinh"] + 1
            "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
        })


def xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
    hang_doi = list(hang_doi_ban_dau)
    da_xu_ly = []
    so_vong = 0
    while ___:                                                  # hang_doi and so_vong < gioi_han_vong_lap
        su_kien = hang_doi.pop(0)
        da_xu_ly.append(su_kien)
        ket_qua_moi = []
        xu_ly_mot_su_kien(su_kien, ket_qua_moi)
        hang_doi.extend(ket_qua_moi)
        so_vong += 1
    return da_xu_ly, hang_doi


HANG_DOI_1 = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 1, "ket_qua_tinh": 4, "ket_qua_mong_doi": 5},
]
HANG_DOI_2 = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 2, "ket_qua_tinh": 0, "ket_qua_mong_doi": 100},
]

KET_QUA_1, CON_LAI_1 = xu_ly_toan_bo_hang_doi(HANG_DOI_1, 10)
KET_QUA_2, CON_LAI_2 = xu_ly_toan_bo_hang_doi(HANG_DOI_2, 5)

print([sk["loai_su_kien"] for sk in KET_QUA_1])
print(len(KET_QUA_1))
print(len(CON_LAI_1))
print([sk["loai_su_kien"] for sk in KET_QUA_2])
print(len(KET_QUA_2))
print(len(CON_LAI_2))
```

```python title=solution
def chay_test_tat_dinh(su_kien):
    return su_kien["ket_qua_tinh"] == su_kien["ket_qua_mong_doi"]


def xu_ly_mot_su_kien(su_kien, hang_doi_ket_qua):
    loai = su_kien["loai_su_kien"]
    if loai == "CodeWrittenEvent":
        if chay_test_tat_dinh(su_kien):
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestPassedEvent",
                "task_id": su_kien["task_id"],
            })
        else:
            hang_doi_ket_qua.append({
                "loai_su_kien": "TestFailedEvent",
                "task_id": su_kien["task_id"],
                "ket_qua_tinh": su_kien["ket_qua_tinh"],
                "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
            })
    elif loai == "TestFailedEvent":
        hang_doi_ket_qua.append({
            "loai_su_kien": "CodeWrittenEvent",
            "task_id": su_kien["task_id"],
            "ket_qua_tinh": su_kien["ket_qua_tinh"] + 1,
            "ket_qua_mong_doi": su_kien["ket_qua_mong_doi"],
        })


def xu_ly_toan_bo_hang_doi(hang_doi_ban_dau, gioi_han_vong_lap):
    hang_doi = list(hang_doi_ban_dau)
    da_xu_ly = []
    so_vong = 0
    while hang_doi and so_vong < gioi_han_vong_lap:
        su_kien = hang_doi.pop(0)
        da_xu_ly.append(su_kien)
        ket_qua_moi = []
        xu_ly_mot_su_kien(su_kien, ket_qua_moi)
        hang_doi.extend(ket_qua_moi)
        so_vong += 1
    return da_xu_ly, hang_doi


HANG_DOI_1 = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 1, "ket_qua_tinh": 4, "ket_qua_mong_doi": 5},
]
HANG_DOI_2 = [
    {"loai_su_kien": "CodeWrittenEvent", "task_id": 2, "ket_qua_tinh": 0, "ket_qua_mong_doi": 100},
]

KET_QUA_1, CON_LAI_1 = xu_ly_toan_bo_hang_doi(HANG_DOI_1, 10)
KET_QUA_2, CON_LAI_2 = xu_ly_toan_bo_hang_doi(HANG_DOI_2, 5)

print([sk["loai_su_kien"] for sk in KET_QUA_1])
print(len(KET_QUA_1))
print(len(CON_LAI_1))
print([sk["loai_su_kien"] for sk in KET_QUA_2])
print(len(KET_QUA_2))
print(len(CON_LAI_2))
```

```python title=test
assert [sk["loai_su_kien"] for sk in KET_QUA_1] == [
    "CodeWrittenEvent", "TestFailedEvent", "CodeWrittenEvent", "TestPassedEvent",
], f"KET_QUA_1 sai thu tu su kien -- dang ra {[sk['loai_su_kien'] for sk in KET_QUA_1]}"
assert len(KET_QUA_1) == 4, f"KET_QUA_1 phai co dung 4 su kien -- dang ra {len(KET_QUA_1)}"
assert CON_LAI_1 == [], f"CON_LAI_1 phai RONG (task 1 xong tu nhien) -- dang ra {CON_LAI_1}"

assert [sk["loai_su_kien"] for sk in KET_QUA_2] == [
    "CodeWrittenEvent", "TestFailedEvent", "CodeWrittenEvent", "TestFailedEvent", "CodeWrittenEvent",
], f"KET_QUA_2 sai thu tu su kien -- dang ra {[sk['loai_su_kien'] for sk in KET_QUA_2]}"
assert len(KET_QUA_2) == 5, f"KET_QUA_2 phai co dung 5 su kien (dung gioi_han_vong_lap) -- dang ra {len(KET_QUA_2)}"
assert len(CON_LAI_2) == 1, f"CON_LAI_2 phai con DUNG 1 phan tu chua xu ly -- dang ra {len(CON_LAI_2)}"

# bien: gioi_han_vong_lap THAT SU rang buoc ket qua (khong phai tham so thua)
KET_QUA_2B, CON_LAI_2B = xu_ly_toan_bo_hang_doi(HANG_DOI_2, 4)
assert len(KET_QUA_2B) == 4, f"doi gioi_han_vong_lap tu 5 xuong 4 phai cho 4 su kien -- dang ra {len(KET_QUA_2B)}"
assert len(CON_LAI_2B) == 1, f"CON_LAI_2B phai van con dung 1 phan tu -- dang ra {len(CON_LAI_2B)}"

# kiem tra truc tiep xu_ly_mot_su_kien, tach khoi xu_ly_toan_bo_hang_doi
r_pass = []
xu_ly_mot_su_kien({"loai_su_kien": "CodeWrittenEvent", "task_id": 9, "ket_qua_tinh": 1, "ket_qua_mong_doi": 1}, r_pass)
assert r_pass == [{"loai_su_kien": "TestPassedEvent", "task_id": 9}], f"code dung phai sinh TestPassedEvent -- dang ra {r_pass}"

r_fail = []
xu_ly_mot_su_kien({"loai_su_kien": "CodeWrittenEvent", "task_id": 9, "ket_qua_tinh": 1, "ket_qua_mong_doi": 2}, r_fail)
assert r_fail == [{
    "loai_su_kien": "TestFailedEvent", "task_id": 9, "ket_qua_tinh": 1, "ket_qua_mong_doi": 2,
}], f"code sai phai sinh TestFailedEvent giu nguyen ket_qua_tinh cu -- dang ra {r_fail}"

r_sua = []
xu_ly_mot_su_kien({"loai_su_kien": "TestFailedEvent", "task_id": 9, "ket_qua_tinh": 1, "ket_qua_mong_doi": 2}, r_sua)
assert r_sua == [{
    "loai_su_kien": "CodeWrittenEvent", "task_id": 9, "ket_qua_tinh": 2, "ket_qua_mong_doi": 2,
}], f"TestFailedEvent phai sinh CodeWrittenEvent moi voi ket_qua_tinh = cu + 1 -- dang ra {r_sua}"

# bien: hang doi rong (chua co su kien nao) khong lam gi, khong loi
rong_ket_qua, rong_con_lai = xu_ly_toan_bo_hang_doi([], 10)
assert rong_ket_qua == [] and rong_con_lai == [], "hang doi rong phai cho ca hai ket qua RONG"
```

:::hints
- kind: attention
  body: Hai chỗ trống, ở hai hàm khác nhau. Chỗ đầu (trong `xu_ly_mot_su_kien`, nhánh `TestFailedEvent`) là GIÁ TRỊ gán cho `ket_qua_tinh` của sự kiện `CodeWrittenEvent` MỚI — một phép TÍNH trên `ket_qua_tinh` của sự kiện SAI vừa nhận (biến `su_kien`). Chỗ hai (trong `xu_ly_toan_bo_hang_doi`) là ĐIỀU KIỆN của vòng `while` — hai điều kiện nối bằng `and`, cả hai đều phải đúng để vòng lặp CÒN chạy tiếp.
- kind: strategy
  body: 'Chỗ đầu: `su_kien["ket_qua_tinh"] + 1` — Coder "sửa" bằng cách cộng thêm `1` vào con số cũ (mô phỏng tối giản việc sửa code, không phải sửa thật). Chỗ hai: `hang_doi and so_vong < gioi_han_vong_lap` — `hang_doi` (còn phần tử, tức chưa xong) VÀ `so_vong < gioi_han_vong_lap` (chưa chạm trần); thiếu MỘT trong hai là phải dừng.'
- kind: one-line
  body: 'Chỗ đầu là `su_kien["ket_qua_tinh"] + 1`, chỗ hai là `hang_doi and so_vong < gioi_han_vong_lap`.'
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: static
  onFail: cho trong dau phai la phep TINH su_kien["ket_qua_tinh"] + 1 (khong duoc chep san mot con so); cho trong hai phai la DIEU KIEN hang_doi and so_vong < gioi_han_vong_lap (ca hai ve noi bang and, khong duoc dung rieng mot ve hay dung or)
  requireAst:
  - kind: uses-operator, target: "+", min: 2
  - kind: uses-operator, target: "and", min: 1
  # Da CHAY THAT qua kiemAst() (node + pyodide, dung dist build that tai
  # packages/exec-python/dist/kiem-ast.js, trich CHINH XAC khoi solution cua
  # file nay) -- xac nhan DUNG CHINH XAC (min VA min+1): "+"=2, "and"=1.
  # "+"=2 (TONG THAT tren toan bo solution, KHONG phai boilerplate lon): 1
  # lan o cho trong dau (su_kien["ket_qua_tinh"] + 1), CONG 1 lan o
  # "so_vong += 1" trong xu_ly_toan_bo_hang_doi (AugAssign CUNG duoc
  # _khop_toan_tu dem la mot phep "+"). Khong noi nao khac trong solution
  # dung "+". "and"=1: DUY NHAT o cho trong hai -- khong noi nao khac trong
  # solution dung "and" (vong while ngoai cung, "hang_doi or hang_doi_mien_phi"
  # KHONG xuat hien trong bai nay).
  # Dien bua "True" vao ca hai cho trong ("ket_qua_tinh": True va "while
  # True:") cho "+"=1 (duoi 2, chi con so_vong += 1) VA "and"=0 (duoi 1) --
  # CA HAI luat CHAN DUNG (da CHAY THAT xac nhan qua kiemAst). Rieng "while
  # True:" con bi chan THEM boi tier 'run': da tu chay THAT qua python3
  # (gioi han buoc mo phong 300.000), xac nhan no ROI VAO VONG LAP VO HAN
  # THAT (RuntimeError "vuot han muc buoc") vi hang_doi khong bao gio rong
  # va so_vong khong duoc kiem tra.
  #
  # 🔴🔴🔴🔴 GOTCHA "hoan doi ca cum" (da tu dung mutant hoan doi CA HAI cho
  # trong -- xac dinh ranh gioi TU CHINH khoi starter, khong doan tay -- va
  # CHAY THAT qua kiemAst() THAT VA python3 THAT): dien "hang_doi and
  # so_vong < gioi_han_vong_lap" vao cho trong dau ("ket_qua_tinh": hang_doi
  # and so_vong < gioi_han_vong_lap trong xu_ly_mot_su_kien) VA dien
  # "su_kien[\"ket_qua_tinh\"] + 1" vao cho trong hai ("while
  # su_kien[\"ket_qua_tinh\"] + 1:" trong xu_ly_toan_bo_hang_doi) -- da CHAY
  # THAT qua kiemAst(): tong so lan "+" VA tong so lan "and" tren TOAN BO
  # solution DEU KHONG DOI (van dung 2 va 1, chi doi VI TRI) -- static
  # KHONG bat duoc mutant nay.
  # Mutant nay BI BAT boi tier 'run': ben trong xu_ly_mot_su_kien (tham so
  # la su_kien, hang_doi_ket_qua -- KHONG CO "hang_doi"/"so_vong"/
  # "gioi_han_vong_lap" nao trong scope nay), bieu thuc moi doc CA BA ten
  # CHUA TON TAI -- da tu chay THAT qua python3, xac nhan NameError "name
  # 'hang_doi' is not defined" ngay khi mot TestFailedEvent duoc xu ly lan
  # dau. Ben trong xu_ly_toan_bo_hang_doi, bieu thuc moi "while
  # su_kien[\"ket_qua_tinh\"] + 1:" doc ten "su_kien" -- ten nay CO duoc GAN
  # o dong "su_kien = hang_doi.pop(0)" BEN TRONG than vong while, nen Python
  # coi no la bien LOCAL cua ham; doc no TRUOC lan gan dau tien (ngay dieu
  # kien while, truoc khi vong lap chay lan nao) nem UnboundLocalError -- da
  # tu chay THAT xac nhan thong bao "cannot access local variable 'su_kien'
  # where it is not associated with a value". Ca hai bi chan boi tier
  # 'run', doc lap voi static.
  # Da tu ra soat GOTCHA #6: "hang_doi", "so_vong", "gioi_han_vong_lap" chi
  # ton tai trong pham vi xu_ly_toan_bo_hang_doi; "su_kien" la ten THAM SO
  # cua xu_ly_mot_su_kien VA cung la ten BIEN CUC BO (gan qua .pop(0)) cua
  # xu_ly_toan_bo_hang_doi -- day LA truong hop can luu y (cung ten, hai
  # ham) nhung da tu CHAY THAT xac nhan no van bi chan (boi UnboundLocalError
  # do thu tu gan/doc trong CHINH than ham, khong lien quan gi toi ham kia),
  # nen khong can them rule uses-name bo sung -- ghi lai ro co che thay vi
  # bo qua.
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: "^\\['CodeWrittenEvent', 'TestFailedEvent', 'CodeWrittenEvent', 'TestPassedEvent'\\]\\n4\\n0\\n\\['CodeWrittenEvent', 'TestFailedEvent', 'CodeWrittenEvent', 'TestFailedEvent', 'CodeWrittenEvent'\\]\\n5\\n1\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4` sự kiện, đúng thứ tự — Coder sai, Tester bắt, Coder sửa, Tester chấm lại
đúng. Không HTTP nào bị gọi trực tiếp; mọi thứ đi qua MỘT hàng đợi sự kiện.
Bài tiếp theo: TRƯỚC KHI một Agent "gọi LLM" (tốn tiền, tốn thời gian), có
cách nào NHỚ lại câu hỏi đã trả lời rồi không?
::::

::::reflect{#nghi-lai}
`xu_ly_mot_su_kien` không "biết" Coder hay Tester là ai — nó chỉ nhìn vào
TRƯỜNG `loai_su_kien` và phản ứng bằng một quy tắc CỐ ĐỊNH: `CodeWrittenEvent`
sinh ra một sự kiện chấm điểm, `TestFailedEvent` sinh ra một `CodeWrittenEvent`
mới. Đó CHÍNH LÀ ý nghĩa của "không gọi trực tiếp" — không Agent nào cần
biết Agent kia đang làm gì hay mất bao lâu; chúng chỉ phản ứng với SỰ KIỆN
xuất hiện trong hàng đợi. Đo được trên `HANG_DOI_1`: đúng `4` sự kiện, đúng
thứ tự, dừng vì XONG (`CON_LAI_1 = []`). Trên `HANG_DOI_2`: một bug không
thể sửa xong trong `5` vòng — `gioi_han_vong_lap` cắt vòng lặp ở đúng vòng
thứ `5`, để lại `1` sự kiện dang dở, chứng minh trần vòng lặp là một BỘ
PHẬN AN TOÀN bắt buộc, không phải trang trí. Bài tiếp theo chuyển sang một
mảnh khác của `Chương 39` — Bài tập `2`: semantic cache, đo tỉ lệ cache hit
TRƯỚC khi một Agent thật sự "gọi LLM".
::::

::::checkpoint{mastery=0.76}
::::
