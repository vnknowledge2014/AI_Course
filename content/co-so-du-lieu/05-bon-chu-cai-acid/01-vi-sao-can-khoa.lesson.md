---
id: co-so-du-lieu.bon-chu-cai-acid.vi-sao-can-khoa
title: Vì sao cần khoá
summary: "Hai giao dịch CÙNG đọc một số dư TRƯỚC khi ai ghi, cộng thêm số riêng, RỒI ghi lại — giao dịch ghi SAU đè lên kết quả của giao dịch ghi TRƯỚC, dù cả hai đều 'thành công'. Đây là 'mất cập nhật' (lost update) — không AI báo lỗi, chỉ đơn giản MẤT một lần cộng."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 1
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.why-need-locks]
requires: [db.write-amplification]
concepts: [db.why-need-locks]
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
q04 kết thúc bằng một câu hỏi: khi NHIỀU giao dịch cùng đọc/ghi MỘT
kho lưu trữ đồng thời, làm sao đảm bảo chúng không GIẪM lên nhau?
::::

::::explain{#mat-cap-nhat}
Hai giao dịch (`T1`, `T2`) CÙNG đọc số dư TRƯỚC khi ai ghi, cộng
thêm số RIÊNG, rồi ghi lại — giao dịch ghi SAU đè lên kết quả của
giao dịch ghi TRƯỚC:

```python title=readonly
so_du = {'acc1': 100}

gt_T1 = so_du['acc1']
gt_T2 = so_du['acc1']

moi_T1 = gt_T1 + 50
moi_T2 = gt_T2 + 30

so_du['acc1'] = moi_T1
so_du['acc1'] = moi_T2

print(so_du['acc1'])
```

```text title=readonly
130
```

`T1` VÀ `T2` đều đọc số dư GỐC (`100`) TRƯỚC khi bất kỳ ai ghi. `T1`
tính `100+50=150`, `T2` tính `100+30=130`. `T1` ghi TRƯỚC (`150`),
`T2` ghi SAU (`130`) — đè LÊN. Kết quả cuối CÙNG chỉ phản ánh phép
cộng của `T2`, phép cộng `+50` của `T1` biến MẤT hoàn toàn, dù `T1`
"chạy xong" bình thường, không báo lỗi GÌ.
::::

::::example{#tong-dung-phai-la}
Nếu CẢ hai khoản cộng đều được TÍNH đúng (không ai đè LÊN ai), tổng
đúng phải LÀ:

```python title=readonly
print(100 + 50 + 30)
```

```text title=readonly
180
```

`180`, không phải `130`. Chênh lệch ĐÚNG bằng `50` — CHÍNH xác phần
đóng góp của `T1` đã bị `T2` ghi ĐÈ mất. Đây LÀ "mất cập nhật" (lost
update) — MỘT trong những lý do CƠ bản nhất cần khoá khi nhiều giao
dịch cùng động vào MỘT dữ liệu.
::::

::::predict{#doan-mat-cap-nhat-so-khac commitOnce}
Số dư ban đầu LÀ `1000`. `T1` cộng thêm `200`, `T2` cộng thêm `50`
— `T1` ghi TRƯỚC, `T2` ghi SAU:

```python
so_du = {'acc1': 1000}
gt_T1 = so_du['acc1']
gt_T2 = so_du['acc1']
moi_T1 = gt_T1 + 200
moi_T2 = gt_T2 + 50
so_du['acc1'] = moi_T1
so_du['acc1'] = moi_T2
print(so_du['acc1'])
```

Dòng cuối in ra gì?

:::opt{correct}
`1050`
:::

:::opt
`1250` — vì CẢ hai khoản cộng (`200` VÀ `50`) đều được TÍNH vào số
dư, tổng cộng `1000+200+50`
::why
Gần đúng ở việc bạn tính ĐÚNG tổng nếu KHÔNG có mất cập nhật — MỘT
phép cộng chính xác giả định CẢ hai lần ghi đều "cộng dồn" được.

Chỗ lệch: `T2` ghi SAU `T1`, VÀ `T2` tính `moi_T2` TỪ số dư GỐC
(`1000`), không phải TỪ kết quả `T1` vừa ghi — `so_du['acc1'] =
moi_T2` GHI ĐÈ thẳng lên `1150` (kết quả của `T1`), xoá SẠCH đóng
góp `+200`. Kết quả CUỐI chỉ phản ánh `1000+50=1050`.
::
:::

:::opt
`1200` — vì giao dịch ghi TRƯỚC (`T1`) luôn "thắng", giá trị của nó
LÀ giá trị cuối cùng được GIỮ lại
::why
Gần đúng ở việc bạn nghĩ TỚI thứ tự ghi có VAI trò quyết định — một
trực giác ĐÚNG hướng, chỉ SAI chiều.

Chỗ lệch: `T2` ghi SAU `T1` trong đoạn code NÀY — dòng `so_du['acc1']
= moi_T2` chạy SAU CÙNG, nên giá trị của `T2` (`1050`) mới LÀ giá
trị TỒN tại sau cùng, không phải của `T1` (`1200`).
::
:::
::::

::::code{#viet_mo_phong_mat_cap_nhat}
Hoàn thiện `mo_phong_mat_cap_nhat` — sau khi `T1` đã ghi, `T2` ghi
ĐÈ lên bằng kết quả của chính NÓ.

```python title=starter
def mo_phong_mat_cap_nhat(so_du_ban_dau, cong_them_T1, cong_them_T2):
    so_du = {'acc1': so_du_ban_dau}
    gt_T1 = so_du['acc1']
    gt_T2 = so_du['acc1']
    moi_T1 = gt_T1 + cong_them_T1
    moi_T2 = gt_T2 + cong_them_T2
    so_du['acc1'] = moi_T1
    ___
    return so_du['acc1']


print(mo_phong_mat_cap_nhat(100, 50, 30))
```

```python title=solution
def mo_phong_mat_cap_nhat(so_du_ban_dau, cong_them_T1, cong_them_T2):
    so_du = {'acc1': so_du_ban_dau}
    gt_T1 = so_du['acc1']
    gt_T2 = so_du['acc1']
    moi_T1 = gt_T1 + cong_them_T1
    moi_T2 = gt_T2 + cong_them_T2
    so_du['acc1'] = moi_T1
    so_du['acc1'] = moi_T2
    return so_du['acc1']


print(mo_phong_mat_cap_nhat(100, 50, 30))
```

```python title=test
assert mo_phong_mat_cap_nhat(100, 50, 30) == 130, "T2 ghi de len T1, chi con 100+30"
assert mo_phong_mat_cap_nhat(1000, 200, 50) == 1050, "chi con 1000+50, mat dong gop cua T1"
assert mo_phong_mat_cap_nhat(0, 10, 20) == 20, "so du ban dau 0 -- chi con 0+20"
assert mo_phong_mat_cap_nhat(500, 0, 0) == 500, "khong cong gi thi khong doi"
```

:::hints
- kind: attention
  body: "Dong con thieu ghi de so_du['acc1'] bang moi_T2, giong het dong tren nhung dung moi_T2 thay vi moi_T1."
- kind: strategy
  body: "so_du['acc1'] = moi_T2"
- kind: one-line
  body: "so_du['acc1'] = moi_T2"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: static
  onFail: phai gan so_du['acc1'] = moi_T2 sau dong ghi cua T1
  requireAst:
  - kind: uses-name, target: moi_T2, min: 1
  - kind: uses-name, target: so_du, min: 5
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^130\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không khoá, không ai đảm bảo gì — một lần cộng LẶNG lẽ biến mất.
Cách đơn giản nhất để chặn chuyện NÀY là gì?
::::

::::reflect{#nghi-lai}
Không CẦN một lỗi hệ thống, không cần một exception — CHỈ cần hai
giao dịch cùng đọc, cùng tính, cùng ghi mà KHÔNG phối hợp, một lần
cập nhật LẶNG lẽ biến mất. Đây LÀ lý do cơ bản nhất cần một cơ chế
đảm bảo: TẠI một thời điểm, chỉ MỘT giao dịch được phép động vào
một khoá dữ liệu — MỘT bảng khoá (lock table) đơn giản nhất trông
như thế nào?
::::

::::checkpoint{mastery=0.8}
::::
