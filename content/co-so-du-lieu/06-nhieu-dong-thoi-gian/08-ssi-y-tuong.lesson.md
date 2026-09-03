---
id: co-so-du-lieu.nhieu-dong-thoi-gian.ssi-y-tuong
title: SSI — ý tưởng
summary: "SSI (Serializable Snapshot Isolation) theo dõi KHÔNG chỉ khoá một giao dịch SẼ ghi, mà CẢ khoá nó ĐÃ đọc. Nếu một khoá đã ĐỌC bị giao dịch KHÁC ghi trước khi giao dịch đầu commit, đó là một xung đột đọc-ghi (rw-conflict) — đúng thứ write skew tạo ra, và đúng thứ co_xung_dot_ghi (chỉ nhìn khoá SẼ ghi) đã bỏ sót."
locale: vi
track: co-so-du-lieu
module: nhieu-dong-thoi-gian
order: 8
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.ssi-idea]
requires: [db.write-skew]
concepts: [db.ssi-idea]
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
Write skew (bài TRƯỚC) trốn được kiểm tra ghi-ghi VÌ hai giao dịch
ghi lên khoá KHÁC nhau. Nhưng CẢ hai đều đã ĐỌC khoá của bên KIA
trước khi ghi — theo dõi việc ĐÓ có ích không?
::::

::::explain{#theo-doi-doc}
SSI (Serializable Snapshot Isolation) theo dõi KHÔNG chỉ khoá một
giao dịch SẼ ghi, mà CẢ khoá nó ĐÃ đọc — nếu một khoá ĐÃ đọc bị GHI
bởi giao dịch KHÁC trước khi giao dịch đầu commit, đó LÀ một xung
đột đọc-ghi (rw-conflict):

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
gia_tri_T1_da_doc = kho.doc_snapshot("bacsi2", ts_T1)
print(gia_tri_T1_da_doc)

ts_T2_ghi = kho.timestamp_moi()
kho.ghi("bacsi2", "nghi", ts_T2_ghi)

print(kho.co_xung_dot_ghi("bacsi2", ts_T1))
```

```text title=readonly
truc
True
```

`T1` ĐỌC `"bacsi2"` (thấy `"truc"`) — VIỆC đọc này LÀ MỘT phần của
"đếm số người trực" TRƯỚC khi `T1` quyết định nghỉ. `T2` sau đó GHI
`"bacsi2"` thành `"nghi"`. Dùng LẠI ĐÚNG hàm `co_xung_dot_ghi` —
nhưng lần NÀY áp dụng CHO khoá `T1` đã ĐỌC (không phải khoá nó SẼ
ghi) — phát hiện `True`: CÓ ai đó đã GHI một khoá mà `T1` từng đọc,
SAU khi `T1` bắt đầu.
::::

::::example{#doc-khong-doi-thi-khong-nguy-hiem}
Nếu KHÔNG ai ghi khoá mà `T1` đã đọc, KHÔNG có rw-conflict:

```python title=readonly
print(kho.co_xung_dot_ghi("bacsi1", ts_T1))
```

```text title=readonly
False
```

`T1` (giả sử) CŨNG đọc `"bacsi1"` — NHƯNG chưa AI ghi khoá `"bacsi1"`
sau `ts_T1`. `co_xung_dot_ghi("bacsi1", ts_T1)` trả VỀ `False` —
đọc `"bacsi1"` LÀ AN toàn, dữ liệu vẫn Y hệt lúc `T1` nhìn thấy.
::::

::::predict{#doan-ssi-se-lam-gi commitOnce}
Nếu hệ thống dùng SSI, VÀ phát hiện `T1` CÓ một rw-conflict TRÊN
`"bacsi2"` (như VÍ dụ đầu tiên) — TRƯỚC khi cho phép `T1` commit,
hệ thống SSI sẽ LÀM gì?

:::opt{correct}
Huỷ (abort) `T1`, bắt nó thử LẠI từ đầu
:::

:::opt
Cho `T1` commit BÌNH thường, chỉ ghi LẠI cảnh báo để xem XÉT sau
::why
Gần đúng ở việc bạn nghĩ TỚI một cách xử LÝ "nhẹ nhàng" hơn — MỘT
lựa chọn thiết kế hợp lý cho MỘT số hệ thống ưu tiên khả DỤNG hơn
tính đúng ĐẮN tuyệt đối.

Chỗ lệch: mục ĐÍCH của SSI LÀ đảm bảo tính TUẦN tự hoá (serializ-
ability) — NGHĨA là mọi giao dịch commit thành CÔNG phải cho kết
quả GIỐNG như thể chúng chạy LẦN lượt, không CHỒNG chéo. Để giữ
đúng cam KẾT đó, SSI PHẢI huỷ giao dịch có rw-conflict nguy HIỂM,
không thể chỉ "ghi chú" rồi cho QUA — nếu không, write skew (bài
TRƯỚC) VẪN xảy ra y hệt SNAPSHOT isolation thường.
::
:::

:::opt
Tự động sửa giá trị `T1` định GHI cho khớp với giá trị MỚI nhất của
`"bacsi2"`
::why
Gần đúng ở việc bạn nghĩ TỚI một cách "hoà giải" tự động — một Ý
tưởng thú vị cho một số hệ thống GIẢI quyết xung đột kiểu KHÁC (như
CRDT).

Chỗ lệch: SSI KHÔNG hề biết Ý nghĩa nghiệp vụ của dữ liệu để "sửa"
sao cho ĐÚNG — nó chỉ phát hiện xung ĐỘT VÀ buộc giao dịch NGUY
hiểm phải huỷ, để ỨNG dụng (hoặc người dùng) tự quyết ĐỊNH thử lại
với dữ liệu MỚI.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
SSI theo dõi CẢ đọc lẫn ghi — bắt được write skew mà kiểm tra ghi-
ghi bỏ SÓT. Nhưng lưu vô hạn phiên bản CŨ thì tốn KHÔNG gian mãi —
dọn dẹp thế NÀO?
::::

::::reflect{#nghi-lai}
SSI mở RỘNG Ý tưởng "xung đột" từ CHỈ ghi-ghi sang CẢ đọc-ghi — một
giao dịch phải theo DÕI mọi khoá nó đã ĐỌC, không chỉ khoá nó SẼ
ghi. Nếu bất kỳ khoá ĐÃ đọc nào bị giao dịch KHÁC ghi trước khi nó
commit, đó LÀ dấu hiệu NGUY hiểm — huỷ giao dịch ĐÓ để giữ đúng
tính tuần tự hoá. Đây CHÍNH là cách bắt được write SKEW mà kiểm tra
ghi-ghi đơn thuần bỏ SÓT. Nhưng CÀNG nhiều giao dịch, chuỗi phiên
bản CÀNG dài — phiên bản CŨ không ai cần NỮA phải dọn thế nào?
::::

::::checkpoint{mastery=0.8}
::::
