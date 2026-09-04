---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.tombstone-tich-luy-mai
title: "Tombstone tích luỹ mãi"
summary: "demSoBiaMo đếm số bản ghi giaTri===null trong một kho. Xoá 3 khoá làm số bia mộ tăng lên 3 -- nhưng dù đã qua gc_grace (đủ điều kiện coTheXoaBiaMo), số bia mộ THẬT trong kho vẫn giữ nguyên 3 nếu KHÔNG có compaction nào chạy. 'Đủ điều kiện xoá' và 'đã bị xoá' là hai chuyện khác nhau."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.tombstone-tich-luy-mai]
requires: [db.gc-grace-thoi-gian-an-han]
concepts: [db.tombstone-tich-luy-mai]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
`coTheXoaBiaMo` (bài trước) chỉ TRẢ lời "đủ điều kiện CHƯA" — không
tự XOÁ gì. Nếu KHÔNG ai thật sự dọn, bia mộ tích luỹ tới MỨC nào?
::::

::::explain{#dem-so-bia-mo}
`demSoBiaMo` đếm số bản ghi `giaTri===null` (bia mộ) trong MỘT kho —
đơn giản LÀ một phép đếm, cho THẤY quy mô của vấn ĐỀ:

```typescript title=readonly
interface BanGhi { giaTri: number | null; thoiGian: number; }

function demSoBiaMo(kho: Map<string, BanGhi>): number {
  let dem = 0;
  for (const [, bg] of kho) if (bg.giaTri === null) dem++;
  return dem;
}

const kho = new Map<string, BanGhi>();
for (let i = 0; i < 10; i++) kho.set("khoa" + i, { giaTri: i * 10, thoiGian: i });
console.log("truoc xoa gi ca:", demSoBiaMo(kho));

kho.set("khoa2", { giaTri: null, thoiGian: 100 });
kho.set("khoa5", { giaTri: null, thoiGian: 200 });
kho.set("khoa8", { giaTri: null, thoiGian: 300 });
console.log("sau xoa 3 khoa:", demSoBiaMo(kho));
```

```text title=readonly
truoc xoa gi ca: 0
sau xoa 3 khoa: 3
```

`10` khoá BAN đầu đều LÀ giá trị THẬT — `0` bia mộ. Xoá `3` trong SỐ
đó (`khoa2, khoa5, khoa8`) — mỗi lần xoá LÀ một lần GHI (bài 1), NÊN
`demSoBiaMo` tăng ĐÚNG theo số lần xoá, không hề GIẢM số khoá tổng
(giống hệt q04 bài 11: bia mộ LÀ một bản ghi, không phải một lỗ
trống).
::::

::::example{#du-dieu-kien-nhung-chua-mat}
Dù CẢ `3` bia mộ Ở TRÊN đã qua đủ `gcGraceMs` (`coTheXoaBiaMo` trả về
`true` cho CẢ ba), gọi LẠI `demSoBiaMo` VẪN cho ĐÚNG `3` — không hề
tự giảm. "Đủ điều kiện xoá" (bài 3) VÀ "đã THẬT sự bị xoá khỏi kho"
LÀ hai chuyện HOÀN toàn khác nhau: chỉ CÓ compaction (bài SAU) mới
THẬT sự dọn chúng khỏi bộ nhớ/đĩa.
::::

::::predict{#doan-xoa-lai-khoa-da-xoa commitOnce}
Kho đang CÓ bia mộ Ở `"khoa2"` (`giaTri:null`). Xoá `"khoa2"` MỘT lần
NỮA (ghi thêm một bia mộ MỚI, `thoiGian` lớn hơn). `demSoBiaMo` SAU
đó thay đổi thế NÀO?

:::opt{correct}
KHÔNG đổi — `"khoa2"` VẪN chỉ LÀ MỘT entry trong `Map` (khoá TRÙNG
thì ghi ĐÈ), nên đếm bia mộ VẪN y hệt trước đó
:::

:::opt
Tăng thêm `1` — xoá MỘT khoá đã bị xoá RỒI vẫn LÀ một thao TÁC xoá
mới, phải TẠO thêm một bia mộ MỚI riêng biệt
::why
Trực giác NÀY hợp lý nếu bia mộ được LƯU dạng danh sách LỊCH sử (mỗi
lần xoá một bản ghi MỚI, giữ lại TẤT cả) — một số hệ THỐNG khác thật
sự làm VẬY.

Chỗ lệch: `kho` Ở đây LÀ một `Map<string, BanGhi>` — MỖI khoá chỉ có
ĐÚNG một entry, ghi ĐÈ khi trùng tên (giống HỆT `chenMemtable` q04
bài 11). Xoá một khoá ĐÃ là bia mộ chỉ cập nhật LẠI `thoiGian` của
entry SẴN có, không hề tạo THÊM entry nào — `demSoBiaMo` đếm SỐ
`entry` có `giaTri===null`, không đếm SỐ lần gọi xoá.
::
:::
::::

::::code{#viet_dem_so_bia_mo}
Hoàn thiện `demSoBiaMo` — đếm số bản ghi CÓ `giaTri` LÀ `null`.

```typescript title=starter
interface BanGhi { giaTri: number | null; thoiGian: number; }

function demSoBiaMo(kho: Map<string, BanGhi>): number {
  let dem = 0;
  for (const [, bg] of kho) {
    ___
  }
  return dem;
}

const kho = new Map<string, BanGhi>();
kho.set("a", { giaTri: 1, thoiGian: 0 });
kho.set("b", { giaTri: null, thoiGian: 1 });
console.log(demSoBiaMo(kho));
```

```typescript title=solution
interface BanGhi { giaTri: number | null; thoiGian: number; }

function demSoBiaMo(kho: Map<string, BanGhi>): number {
  let dem = 0;
  for (const [, bg] of kho) {
    if (bg.giaTri === null) dem++;
  }
  return dem;
}

const kho = new Map<string, BanGhi>();
kho.set("a", { giaTri: 1, thoiGian: 0 });
kho.set("b", { giaTri: null, thoiGian: 1 });
console.log(demSoBiaMo(kho));
```

```typescript title=test
const kho2 = new Map<string, BanGhi>();
if (demSoBiaMo(kho2) !== 0) throw new Error("kho rong phai co 0 bia mo");

for (let i = 0; i < 10; i++) kho2.set("khoa" + i, { giaTri: i * 10, thoiGian: i });
if (demSoBiaMo(kho2) !== 0) throw new Error("10 gia tri that, chua xoa gi -- phai la 0 bia mo");

kho2.set("khoa2", { giaTri: null, thoiGian: 100 });
kho2.set("khoa5", { giaTri: null, thoiGian: 200 });
kho2.set("khoa8", { giaTri: null, thoiGian: 300 });
if (demSoBiaMo(kho2) !== 3) throw new Error("xoa 3 khoa -- phai co dung 3 bia mo");
if (kho2.size !== 10) throw new Error("xoa khong lam giam so entry trong kho -- bia mo la mot ban ghi");

kho2.set("khoa2", { giaTri: null, thoiGian: 999 });
if (demSoBiaMo(kho2) !== 3) throw new Error("xoa lai mot khoa DA la bia mo khong tao them entry moi -- van phai la 3");

kho2.set("khoa0", { giaTri: 0, thoiGian: 500 });
if (demSoBiaMo(kho2) !== 3) throw new Error("giaTri=0 la gia tri THAT (khac null) -- khong tinh la bia mo");
```

:::hints
- kind: attention
  body: "Neu gia tri cua ban ghi la null thi tang bien dem -- mot dong."
- kind: strategy
  body: "if (bg.giaTri === null) dem++;"
- kind: one-line
  body: "if (bg.giaTri === null) dem++;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bia mộ tích luỹ, không tự dọn. Cần một tiến trình chủ ĐỘNG gộp dữ
liệu VÀ lọc bỏ bia mộ đủ tuổi — chọn dữ liệu NÀO để gộp trước LÀ cả
một chiến lược.
::::

::::reflect{#nghi-lai}
`demSoBiaMo` LÀ một phép đếm rất nhỏ, nhưng nó LÀM rõ MỘT sự thật:
xoá trong hệ LSM (q04) hay hệ phân TÁN (q11-q14) đều KHÔNG bao giờ
"miễn phí không gian" — mỗi lần xoá LÀ một lần GHI THÊM, không phải
một lần giải phóng. Không CÓ compaction định kỳ, số bia mộ CHỈ có thể
tăng (hoặc giữ NGUYÊN khi xoá lại chính nó), KHÔNG BAO GIỜ tự giảm.
Compaction (bài SAU) LÀ nơi DUY nhất bia mộ THẬT sự biến mất.
::::

::::checkpoint{mastery=0.8}
::::
