---
id: co-so-du-lieu.bon-chu-cai-acid.deadlock-la-gi
title: Deadlock là gì
summary: "Hai giao dịch, cả hai ĐỀU tuân thủ 2PL hoàn hảo — mỗi bên giữ MỘT khoá và chờ khoá của bên KIA. Không ai vi phạm quy tắc nào, nhưng CẢ hai kẹt mãi mãi. Biểu diễn bằng một đồ thị CHỜ (wait-for graph): mỗi giao dịch trỏ TỚI giao dịch nó đang chờ — deadlock LÀ một vòng tròn trong đồ thị đó."
locale: vi
track: co-so-du-lieu
module: bon-chu-cai-acid
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [db.deadlock-idea]
requires: [db.two-phase-locking]
concepts: [db.deadlock-idea]
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
2PL (bài TRƯỚC) đảm bảo một giao dịch không xin THÊM khoá sau khi
đã bắt đầu TRẢ. Nhưng nếu HAI giao dịch, cả hai đều tuân thủ 2PL
hoàn hảo, vẫn KẸT nhau thì sao?
::::

::::explain{#hai-ben-cho-nhau}
`T1` giữ `"A"`, muốn thêm `"B"`. `T2` giữ `"B"`, muốn thêm `"A"` —
CẢ hai đang Ở pha tăng trưởng, KHÔNG hề vi phạm 2PL, nhưng CẢ hai
đều bị TỪ chối:

```python title=readonly
class BangKhoa:
    def __init__(self):
        self._giu_boi = {}

    def xin_khoa(self, khoa, id_giao_dich):
        chu_hien_tai = self._giu_boi.get(khoa)
        if chu_hien_tai is None or chu_hien_tai == id_giao_dich:
            self._giu_boi[khoa] = id_giao_dich
            return True
        return False

    def tra_khoa(self, khoa, id_giao_dich):
        if self._giu_boi.get(khoa) == id_giao_dich:
            del self._giu_boi[khoa]


class GiaoDichHaiPha:
    def __init__(self, bang, id_giao_dich):
        self.bang = bang
        self.id_giao_dich = id_giao_dich
        self.pha = 'tang_truong'
        self.khoa_dang_giu = []

    def xin(self, khoa):
        if self.pha == 'co_lai':
            return False
        thanh_cong = self.bang.xin_khoa(khoa, self.id_giao_dich)
        if thanh_cong:
            self.khoa_dang_giu.append(khoa)
        return thanh_cong

    def tra(self, khoa):
        self.pha = 'co_lai'
        self.bang.tra_khoa(khoa, self.id_giao_dich)
        self.khoa_dang_giu.remove(khoa)


bang = BangKhoa()
gd1 = GiaoDichHaiPha(bang, "T1")
gd2 = GiaoDichHaiPha(bang, "T2")
gd1.xin("A")
gd2.xin("B")
print(gd1.xin("B"), gd2.xin("A"))
```

```text title=readonly
False False
```

`T1` giữ `"A"` (xin THÀNH công lúc đầu), `T2` giữ `"B"`. Rồi `T1`
thử xin `"B"` — thất BẠI, vì `T2` đang giữ. `T2` thử xin `"A"` —
CŨNG thất bại, vì `T1` đang giữ. Cả HAI vẫn Ở pha tăng trưởng (chưa
hề trả gì), nên CẢ hai sẽ CỨ thử lại mãi — VÀ mãi mãi thất bại. Đây
LÀ deadlock: không ai sai luật, nhưng KHÔNG ai tiến lên được.
::::

::::example{#bieu-dien-bang-do-thi}
Biểu diễn tình huống TRÊN bằng một đồ thị CHỜ (wait-for graph) —
mỗi giao dịch trỏ TỚI giao dịch NÓ đang chờ:

```python title=readonly
do_thi_cho = {"T1": {"T2"}, "T2": {"T1"}}
print("T2" in do_thi_cho["T1"], "T1" in do_thi_cho["T2"])
```

```text title=readonly
True True
```

`do_thi_cho["T1"] = {"T2"}` nghĩa LÀ "T1 đang chờ T2" (VÌ khoá T1
muốn ĐANG bị T2 giữ). `do_thi_cho["T2"] = {"T1"}` nghĩa TƯƠNG tự
ngược lại. Hai mũi TÊN trỏ vào NHAU tạo thành một VÒNG tròn — đó
chính LÀ deadlock, dưới dạng đồ THỊ.
::::

::::predict{#doan-khong-phai-deadlock commitOnce}
Một tình huống KHÁC: `T1` đang chờ `T2`, NHƯNG `T2` không hề chờ AI
cả (nó ĐÃ có đủ mọi khoá cần, sắp xong VIỆC):

```python
do_thi_khong_deadlock = {"T1": {"T2"}}
print("T2" in do_thi_khong_deadlock)
```

Dòng cuối in ra gì?

:::opt{correct}
`False`
:::

:::opt
`True` — vì `T2` XUẤT hiện Ở đâu đó trong đồ thị (LÀ giá trị mà
`T1` đang chờ), nên NÓ phải LÀ một khoá của `do_thi_khong_deadlock`
::why
Gần đúng ở việc bạn để Ý ĐÚNG `"T2"` CÓ xuất hiện trong đồ thị —
một quan sát chính XÁC, chỉ SAI vị trí.

Chỗ lệch: `"T2"` xuất hiện BÊN TRONG tập GIÁ TRỊ của `"T1"`
(`do_thi_khong_deadlock["T1"] = {"T2"}`), KHÔNG phải LÀ một khoá
riêng của chính `do_thi_khong_deadlock`. Kiểm tra `"T2" in
do_thi_khong_deadlock` hỏi "T2 có phải MỘT khoá cấp cao KHÔNG" —
CÂU trả lời LÀ không, vì `T2` không hề CHỜ ai, nên nó KHÔNG xuất
hiện làm khoá.
::
:::

:::opt
Không xác định được — CẦN biết `T2` đang giữ những khoá NÀO mới
kết luận được nó có chờ AI hay không
::why
Gần đúng ở việc bạn nghĩ TỚI việc cần THÊM thông tin để hiểu TOÀN
cảnh — một sự thận trọng hợp LÝ khi phân tích một hệ thống thật.

Chỗ lệch: Ở đây, đồ thị `do_thi_khong_deadlock` LÀ toàn BỘ thông
tin đã cho — nó CHỈ ghi "T1 chờ T2", KHÔNG hề có mục nào CHO "T2"
cả. Việc `T2` không CÓ mặt như một khoá TRONG dict đã đủ để BIẾT
`T2` không đang chờ AI trong đồ thị này, theo đúng ĐỊNH nghĩa dict
Python — `in` kiểm tra tường minh, không cần suy đoán THÊM.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không ai sai luật, nhưng cả hai kẹt MÃI mãi — một vòng tròn trong
đồ thị CHỜ. Máy tính phát hiện điều NÀY tự động bằng cách nào?
::::

::::reflect{#nghi-lai}
Deadlock KHÔNG cần một lỗi LOGIC nào — chỉ cần hai (hay NHIỀU hơn)
giao dịch, MỖI bên giữ một phần tài NGUYÊN mà bên khác cần, VÀ mỗi
bên chờ đúng phần bên KIA đang giữ. Biểu diễn bằng đồ thị CHỜ, một
deadlock LÀ một vòng tròn — TỪ một giao dịch, đi theo các mũi TÊN
"đang chờ", CUỐI cùng quay lại chính NÓ. Nhưng con người vẽ đồ thị
BẰNG mắt dễ, máy tính phải PHÁT hiện vòng tròn đó tự ĐỘNG — làm sao?
::::

::::checkpoint{mastery=0.8}
::::
