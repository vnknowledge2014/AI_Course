---
id: co-so-du-lieu.nhieu-dong-thoi-gian.write-skew-la-gi
title: Write skew là gì
summary: "Hai giao dịch, mỗi bên ghi lên MỘT khoá KHÁC nhau (không hề đụng khoá của nhau) — co_xung_dot_ghi không báo gì cả trên CẢ hai khoá, vì đúng LÀ không có xung đột ghi-ghi nào. Nhưng kết quả CUỐI cùng vi phạm một bất biến NGẦM liên kết hai khoá đó — đây LÀ write skew: một lỗ hổng của snapshot isolation mà kiểm tra xung đột ghi-ghi hoàn toàn không thấy."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 7
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [db.write-skew]
requires: [db.write-write-conflict]
concepts: [db.write-skew]
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
`co_xung_dot_ghi` (bài TRƯỚC) chỉ nhìn VÀO một khoá TẠI một thời
điểm. Nếu hai giao dịch ghi lên HAI khoá KHÁC nhau, dựa trên CÙNG
một snapshot cũ, nó có bắt được GÌ không?
::::

::::explain{#hai-bac-si-truc}
Hai bác sĩ CÙNG trực, quy tắc LÀ "luôn có ÍT nhất một người trực".
Mỗi bác sĩ kiểm TRA "còn ai khác trực không" TRƯỚC khi xin nghỉ —
CẢ hai đọc CÙNG snapshot, cả hai thấy ĐỦ hai người trực:

```python title=readonly
class KhoLuuMVCC:
    def __init__(self):
        self._phien_ban = {}
        self._dong_ho = 0

    def timestamp_moi(self):
        self._dong_ho += 1
        return self._dong_ho

    def ghi(self, khoa, gia_tri, ts):
        self._phien_ban.setdefault(khoa, []).append((ts, gia_tri))

    def doc_snapshot(self, khoa, ts_snapshot):
        ung_vien = None
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts <= ts_snapshot and (ung_vien is None or ts > ung_vien[0]):
                ung_vien = (ts, gia_tri)
        return ung_vien[1] if ung_vien else None

    def co_xung_dot_ghi(self, khoa, ts_bat_dau):
        for (ts, gia_tri) in self._phien_ban.get(khoa, []):
            if ts > ts_bat_dau:
                return True
        return False


kho = KhoLuuMVCC()
ts0 = kho.timestamp_moi()
kho.ghi("bacsi1", "truc", ts0)
kho.ghi("bacsi2", "truc", ts0)

ts_T1 = ts0
ts_T2 = ts0

so_truc_T1 = sum(1 for bs in ["bacsi1", "bacsi2"] if kho.doc_snapshot(bs, ts_T1) == "truc")
so_truc_T2 = sum(1 for bs in ["bacsi1", "bacsi2"] if kho.doc_snapshot(bs, ts_T2) == "truc")
print(so_truc_T1, so_truc_T2)
```

```text title=readonly
2 2
```

`T1` VÀ `T2` (hai giao dịch, MỖI bên đại diện MỘT bác sĩ) đọc CÙNG
snapshot `ts0` — CẢ hai đếm được `2` người trực. Mỗi bên NGHĨ:
"còn người kia trực, MÌNH nghỉ được".
::::

::::example{#khong-ai-bao-xung-dot}
Cả hai giao dịch GHI lên khoá RIÊNG của mình (`bacsi1` VÀ `bacsi2`)
— kiểm tra xung đột TRƯỚC khi ghi, CẢ hai đều "sạch":

```python title=readonly
print(kho.co_xung_dot_ghi("bacsi1", ts_T1))
ts_T1_ghi = kho.timestamp_moi()
kho.ghi("bacsi1", "nghi", ts_T1_ghi)

print(kho.co_xung_dot_ghi("bacsi2", ts_T2))
ts_T2_ghi = kho.timestamp_moi()
kho.ghi("bacsi2", "nghi", ts_T2_ghi)
```

```text title=readonly
False
False
```

`T1` kiểm TRA xung đột TRÊN `"bacsi1"` — KHÔNG ai ghi khoá NÀY sau
`ts_T1`, `False`. `T2` kiểm tra TRÊN `"bacsi2"` — CŨNG `False`.
ĐÚNG LÀ không có xung đột ghi-GHI nào — mỗi bên chỉ đụng vào khoá
CỦA riêng mình.
::::

::::predict{#doan-ket-qua-cuoi-cung commitOnce}
Cả hai giao dịch đã GHI xong (`"nghi"` cho CẢ `bacsi1` lẫn
`bacsi2`). Đọc trạng THÁI cuối cùng của cả hai:

```python
ts_cuoi = kho.timestamp_moi()
print(kho.doc_snapshot("bacsi1", ts_cuoi), kho.doc_snapshot("bacsi2", ts_cuoi))
```

Dòng cuối in ra gì?

:::opt{correct}
`nghi nghi`
:::

:::opt
`nghi truc` — vì `co_xung_dot_ghi` đã báo `False` CHO cả hai, hệ
thống phải NGĂN được ít nhất MỘT trong hai lần ghi
::why
Gần đúng ở việc bạn tin TƯỞNG kiểm tra xung đột phải BẢO vệ được
tính đúng ĐẮN — một kỳ vọng hợp lý CHO một cơ chế "phát hiện xung
đột".

Chỗ lệch: `co_xung_dot_ghi` CHỈ kiểm tra xung đột GHI-GHI TRÊN
CÙNG một khoá — nó KHÔNG hề biết (VÀ không được thiết kế để biết)
rằng `bacsi1` VÀ `bacsi2` có một RÀNG buộc NGẦM liên kết CHÚNG với
nhau. Cả hai lần ghi ĐỀU hợp lệ THEO đúng luật ghi-ghi, VÀ cả hai
đều thành CÔNG — kết quả LÀ cả hai đều `"nghi"`.
::
:::

:::opt
Máy báo lỗi — vì kết quả CUỐI vi phạm quy tắc "LUÔN có ít nhất một
người trực", hệ thống PHẢI phát hiện và ngăn CHẶN
::why
Gần đúng ở việc bạn nghĩ TỚI quy tắc nghiệp vụ (LUÔN có người trực)
như một điều BẮT buộc hệ thống phải BẢO vệ — một mong ĐỢI hợp lý về
một hệ thống lưu TRỮ "thông minh".

Chỗ lệch: `KhoLuuMVCC` VÀ `co_xung_dot_ghi` KHÔNG hề biết Ý nghĩa
nghiệp vụ của `"bacsi1"`/`"bacsi2"` — chúng chỉ LÀ hai chuỗi KHOÁ
bình thường, không có RÀNG buộc nào được LẬP trình liên kết chúng.
KHÔNG có `raise` nào — write skew CHÍNH LÀ hiện tượng dữ liệu SAI
mà KHÔNG hệ thống nào tự động báo lỗi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không ai vi phạm luật ghi-ghi, nhưng KẾT quả vẫn sai — write skew.
Có cách nào phát hiện ĐƯỢC kiểu xung đột NÀY không?
::::

::::reflect{#nghi-lai}
Write skew xảy RA khi hai giao dịch đọc CÙNG một snapshot, MỖI bên
ghi lên một khoá RIÊNG (không đụng khoá của NHAU), nhưng kết quả
CUỐI cùng phá vỡ một bất BIẾN ngầm liên kết các khoá ĐÓ. Kiểm tra
xung đột ghi-ghi (bài TRƯỚC) hoàn toàn "mù" trước hiện tượng NÀY —
nó chỉ nhìn VÀO MỘT khoá tại một thời điểm. Bắt được write skew CẦN
một Ý tưởng khác — không CHỈ theo dõi ai GHI gì, mà CẢ ai đã ĐỌC
gì.
::::

::::checkpoint{mastery=0.8}
::::
