---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.hop-thu-nhieu-nhan
title: "Hộp thư nhiều nhãn: một bản ghi, nhiều cửa"
summary: "ganNhan(ht, idEmail, tenNhan) gan mot email VAO nhieu nhan CUNG luc ma KHONG nhan ban noi dung -- nhan chi la mot Set THAM CHIEU id, ban ghi email that su chi ton tai DUY NHAT trong ht.email. goNhan xoa KHOI DUNG mot nhan (cac nhan khac khong dung), con xoaHan xoa ban ghi goc VA go khoi TAT CA nhan cung luc -- hai muc do xoa khac han nhau."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.hop-thu-nhieu-nhan]
requires: [sd.luu-tru-sao-luu-ben-vung]
concepts: [sd.hop-thu-nhieu-nhan]
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
Bốn mảnh về file đã xong. Mảnh HẠ tầng tiếp theo: một dịch vụ email
phân tán. Một email tới hộp thư của bạn thường nằm Ở NHIỀU chỗ cùng
lúc — "Hộp thư đến" VÀ "Công việc" chẳng hạn. Có phải hệ thống lưu HAI
bản sao của nó không?
::::

::::explain{#nhan-chi-la-tham-chieu}
`ganNhan` không sao chép nội dung email — nó chỉ thêm `idEmail` vào
một `Set` gắn với TỪNG nhãn. Bản ghi THẬT sự của email chỉ tồn tại
đúng MỘT lần, trong `ht.email`; mọi nhãn chỉ giữ THAM chiếu tới cùng
một `id` đó:

```typescript title=readonly
interface Email { id: string; tieuDe: string; }
interface HopThu {
  email: Map<string, Email>;
  nhan: Map<string, Set<string>>;
}
function taoHopThu(): HopThu { return { email: new Map(), nhan: new Map() }; }
function themEmail(ht: HopThu, id: string, tieuDe: string): void { ht.email.set(id, { id, tieuDe }); }
function ganNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  const tap = ht.nhan.get(tenNhan) ?? new Set<string>();
  tap.add(idEmail);
  ht.nhan.set(tenNhan, tap);
}
function goNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  ht.nhan.get(tenNhan)?.delete(idEmail);
}
function xoaHan(ht: HopThu, idEmail: string): void {
  ht.email.delete(idEmail);
  for (const tap of ht.nhan.values()) tap.delete(idEmail);
}
function layEmailTheoNhan(ht: HopThu, tenNhan: string): string[] {
  return Array.from(ht.nhan.get(tenNhan) ?? []);
}

const ht = taoHopThu();
themEmail(ht, "e1", "Bao cao Q3");
ganNhan(ht, "e1", "hop-thu-den");
ganNhan(ht, "e1", "cong-viec");

console.log("nhan hop-thu-den:", layEmailTheoNhan(ht, "hop-thu-den"));
console.log("nhan cong-viec:", layEmailTheoNhan(ht, "cong-viec"));
console.log("so ban ghi email THAT SU luu:", ht.email.size);
console.log("tieu de qua ca hai nhan co giong nhau khong:", ht.email.get("e1")?.tieuDe);
```

```text title=readonly
nhan hop-thu-den: [ 'e1' ]
nhan cong-viec: [ 'e1' ]
so ban ghi email THAT SU luu: 1
tieu de qua ca hai nhan co giong nhau khong: Bao cao Q3
```

`"e1"` xuất hiện Ở CẢ hai nhãn — nhưng `ht.email.size` vẫn LÀ `1`.
`ganNhan` không hề tạo thêm bản ghi email nào MỚI, nó chỉ thêm CÙNG một
`id` vào hai `Set` khác nhau. Đọc email qua nhãn nào cũng RA đúng cùng
một `tieuDe`, vì đó LÀ cùng một bản ghi.
::::

::::example{#hai-muc-do-xoa}
Có HAI cách "xoá" hoàn toàn khác nhau: `goNhan` chỉ gỡ khỏi ĐÚNG một
nhãn, còn `xoaHan` xoá bản ghi GỐC VÀ gỡ khỏi TẤT cả nhãn cùng lúc:

```typescript title=readonly
interface Email { id: string; tieuDe: string; }
interface HopThu {
  email: Map<string, Email>;
  nhan: Map<string, Set<string>>;
}
function taoHopThu(): HopThu { return { email: new Map(), nhan: new Map() }; }
function themEmail(ht: HopThu, id: string, tieuDe: string): void { ht.email.set(id, { id, tieuDe }); }
function ganNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  const tap = ht.nhan.get(tenNhan) ?? new Set<string>();
  tap.add(idEmail);
  ht.nhan.set(tenNhan, tap);
}
function goNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  ht.nhan.get(tenNhan)?.delete(idEmail);
}
function xoaHan(ht: HopThu, idEmail: string): void {
  ht.email.delete(idEmail);
  for (const tap of ht.nhan.values()) tap.delete(idEmail);
}
function layEmailTheoNhan(ht: HopThu, tenNhan: string): string[] {
  return Array.from(ht.nhan.get(tenNhan) ?? []);
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: e1 mang hai nhan
// hop-thu-den VA cong-viec, ban ghi email that su chi co 1
const ht = taoHopThu();
themEmail(ht, "e1", "Bao cao Q3");
ganNhan(ht, "e1", "hop-thu-den");
ganNhan(ht, "e1", "cong-viec");

goNhan(ht, "e1", "hop-thu-den");
console.log("sau khi go khoi hop-thu-den -- con trong hop-thu-den?", layEmailTheoNhan(ht, "hop-thu-den"));
console.log("con trong cong-viec?", layEmailTheoNhan(ht, "cong-viec"));
console.log("ban ghi email con ton tai?", ht.email.has("e1"));

xoaHan(ht, "e1");
console.log("sau khi XOA HAN -- ban ghi email con ton tai?", ht.email.has("e1"));
console.log("con trong cong-viec?", layEmailTheoNhan(ht, "cong-viec"));
```

```text title=readonly
sau khi go khoi hop-thu-den -- con trong hop-thu-den? []
con trong cong-viec? [ 'e1' ]
ban ghi email con ton tai? true
sau khi XOA HAN -- ban ghi email con ton tai? false
con trong cong-viec? []
```

`goNhan(ht, "e1", "hop-thu-den")` chỉ làm rỗng nhãn `"hop-thu-den"` —
`"cong-viec"` VÀ bản ghi email gốc hoàn toàn KHÔNG bị đụng tới.
`xoaHan` LÀ chuyện khác hẳn: nó xoá bản ghi GỐC khỏi `ht.email` VÀ tự
LẶP qua MỌI nhãn để gỡ tham chiếu — `"cong-viec"` cũng trở về rỗng sau
lệnh gọi ĐÓ.
::::

::::predict{#doan-go-mot-nhan-khong-dung-nhan-khac commitOnce}
Email `"e2"` mang BA nhãn: `"hop-thu-den"`, `"cong-viec"`,
`"quan-trong"`. Gọi `goNhan(ht, "e2", "cong-viec")`. NGAY sau đó,
`layEmailTheoNhan(ht, "quan-trong")` trả về gì?

:::opt{correct}
`["e2"]` — `goNhan` chỉ thao tác trên `Set` của nhãn `"cong-viec"`,
KHÔNG hề chạm tới `Set` của `"quan-trong"`; email vẫn còn nguyên Ở
nhãn đó
:::
:::opt
`[]` — gỡ email khỏi MỘT nhãn bất kỳ đồng nghĩa với việc gỡ nó khỏi
TẤT cả nhãn còn lại, vì rốt cuộc đó vẫn LÀ cùng một email
::why
Nhầm `goNhan` VỚI `xoaHan` — `goNhan` chỉ thao tác trên đúng MỘT
`Set` (của `tenNhan` được truyền vào), không hề đụng tới các nhãn KHÁC.

Chỗ lệch: thân hàm `goNhan` LÀ `ht.nhan.get(tenNhan)?.delete(idEmail)`
— nó CHỈ `get` đúng nhãn `"cong-viec"` RỒI xoá `idEmail` khỏi ĐÚNG
`Set` đó. `Set` của `"quan-trong"` LÀ một đối tượng hoàn toàn khác,
`goNhan` không hề có dòng nào chạm tới nó. Chỉ `xoaHan` mới LẶP qua
MỌI nhãn (`for (const tap of ht.nhan.values())`).
::
:::
::::

::::code{#viet_xoa_han}
Viết `xoaHan` — xoá bản ghi email gốc khỏi `ht.email`, VÀ gỡ `idEmail`
khỏi TẤT cả các nhãn (duyệt qua `ht.nhan.values()`).

```typescript title=starter
interface Email { id: string; tieuDe: string; }
interface HopThu {
  email: Map<string, Email>;
  nhan: Map<string, Set<string>>;
}
function taoHopThu(): HopThu { return { email: new Map(), nhan: new Map() }; }
function themEmail(ht: HopThu, id: string, tieuDe: string): void { ht.email.set(id, { id, tieuDe }); }
function ganNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  const tap = ht.nhan.get(tenNhan) ?? new Set<string>();
  tap.add(idEmail);
  ht.nhan.set(tenNhan, tap);
}
function goNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  ht.nhan.get(tenNhan)?.delete(idEmail);
}
function xoaHan(ht: HopThu, idEmail: string): void {
  ___
}
function layEmailTheoNhan(ht: HopThu, tenNhan: string): string[] {
  return Array.from(ht.nhan.get(tenNhan) ?? []);
}

const htX = taoHopThu();
themEmail(htX, "x1", "tieu de");
ganNhan(htX, "x1", "a");
ganNhan(htX, "x1", "b");
xoaHan(htX, "x1");
console.log(htX.email.has("x1"), layEmailTheoNhan(htX, "a"), layEmailTheoNhan(htX, "b"));
```

```typescript title=solution
interface Email { id: string; tieuDe: string; }
interface HopThu {
  email: Map<string, Email>;
  nhan: Map<string, Set<string>>;
}
function taoHopThu(): HopThu { return { email: new Map(), nhan: new Map() }; }
function themEmail(ht: HopThu, id: string, tieuDe: string): void { ht.email.set(id, { id, tieuDe }); }
function ganNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  const tap = ht.nhan.get(tenNhan) ?? new Set<string>();
  tap.add(idEmail);
  ht.nhan.set(tenNhan, tap);
}
function goNhan(ht: HopThu, idEmail: string, tenNhan: string): void {
  ht.nhan.get(tenNhan)?.delete(idEmail);
}
function xoaHan(ht: HopThu, idEmail: string): void {
  ht.email.delete(idEmail);
  for (const tap of ht.nhan.values()) tap.delete(idEmail);
}
function layEmailTheoNhan(ht: HopThu, tenNhan: string): string[] {
  return Array.from(ht.nhan.get(tenNhan) ?? []);
}

const htX = taoHopThu();
themEmail(htX, "x1", "tieu de");
ganNhan(htX, "x1", "a");
ganNhan(htX, "x1", "b");
xoaHan(htX, "x1");
console.log(htX.email.has("x1"), layEmailTheoNhan(htX, "a"), layEmailTheoNhan(htX, "b"));
```

```typescript title=test
const htT = taoHopThu();
themEmail(htT, "e1", "Bao cao Q3");
ganNhan(htT, "e1", "hop-thu-den");
ganNhan(htT, "e1", "cong-viec");
ganNhan(htT, "e1", "quan-trong");

goNhan(htT, "e1", "cong-viec");
if (layEmailTheoNhan(htT, "cong-viec").length !== 0) throw new Error("go khoi 1 nhan phai xoa khoi DUNG nhan do");
if (layEmailTheoNhan(htT, "hop-thu-den").length !== 1) throw new Error("go khoi cong-viec khong duoc dung toi hop-thu-den");
if (layEmailTheoNhan(htT, "quan-trong").length !== 1) throw new Error("go khoi cong-viec khong duoc dung toi quan-trong");
if (htT.email.has("e1") !== true) throw new Error("go khoi mot nhan khong duoc xoa ban ghi email goc");

xoaHan(htT, "e1");
if (htT.email.has("e1") !== false) throw new Error("xoa han phai xoa ban ghi email goc");
if (layEmailTheoNhan(htT, "hop-thu-den").length !== 0) throw new Error("xoa han phai go khoi TAT CA nhan, ke ca hop-thu-den");
if (layEmailTheoNhan(htT, "quan-trong").length !== 0) throw new Error("xoa han phai go khoi TAT CA nhan, ke ca quan-trong");

xoaHan(htT, "khong-ton-tai");
```

:::hints
- kind: attention
  body: "Hai viec: xoa ban ghi email goc bang ht.email.delete(idEmail), roi duyet QUA TAT CA nhan (for...of ht.nhan.values()) va xoa idEmail khoi tung Set do."
- kind: strategy
  body: "ht.email.delete(idEmail); for (const tap of ht.nhan.values()) tap.delete(idEmail);"
- kind: one-line
  body: "ht.email.delete(idEmail); for (const tap of ht.nhan.values()) tap.delete(idEmail);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "false [] []"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhãn giờ chỉ là tham chiếu, không phải bản sao. Nhưng gửi thư đi mới
là lúc mọi thứ có thể trục trặc — hộp thư người nhận đầy, hoặc địa chỉ
không hề tồn tại.
::::

::::reflect{#nghi-lai}
`ht.nhan` không hề "chứa" email — nó chỉ chứa CÁC tập hợp `id`. Tách
danh tính (bản ghi Ở `ht.email`) ra khỏi tổ chức (nhãn Ở `ht.nhan`) LÀ
điều cho phép MỘT email nằm Ở nhiều "chỗ" mà không hề nhân bản dữ
liệu — VÀ cũng LÀ lý do "xoá" cần phân biệt rõ đang xoá tổ chức hay
xoá danh tính.
::::

::::checkpoint{mastery=0.80}
::::
