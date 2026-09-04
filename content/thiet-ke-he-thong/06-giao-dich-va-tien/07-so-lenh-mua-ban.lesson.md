---
id: thiet-ke-he-thong.giao-dich-va-tien.so-lenh-mua-ban
title: "Sổ lệnh: mua xếp giá cao trước, bán xếp giá thấp trước"
summary: "themLenh(sl, lenh) them lenh VAO dung mang (lenhMua hoac lenhBan) ROI SAP XEP LAI -- lenh MUA gia 50000, 52000, 48000 sap xep GIAM dan thanh [52000,50000,48000] (gia cao nhat luon o dau); lenh BAN gia 53000, 51000, 54000 sap xep TANG dan thanh [51000,53000,54000] (gia thap nhat luon o dau) -- 'gia tot nhat' cua moi ben LUON nam o vi tri [0], hai mang lenhMua/lenhBan hoan toan doc lap."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.so-lenh-mua-ban]
requires: [sd.vi-dien-tu-chuyen-nguyen-tu]
concepts: [sd.so-lenh-mua-ban]
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
Ba mảnh của "tiền" đã xong — thanh toán, sổ cái, ví. Giờ chuyển sang
một hệ thống khác hẳn: sàn giao dịch chứng khoán. Người mua VÀ người
bán không hề gặp nhau — họ đặt LỆNH, VÀ hệ thống phải biết ngay LẬP
tức lệnh nào TỐT nhất, trong hàng nghìn lệnh đang chờ.
::::

::::explain{#mua-giam-dan}
`themLenh` đẩy lệnh MỚI vào đúng mảng (`lenhMua` hoặc `lenhBan`) rồi
SẮP xếp lại NGAY. Lệnh MUA sắp theo giá GIẢM dần — giá CAO nhất (người
sẵn sàng trả nhiều nhất) luôn đứng Ở vị trí `[0]`:

```typescript title=readonly
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

const sl = taoSoLenh();
themLenh(sl, { id: "m1", ben: "mua", gia: 50000, soLuong: 10 });
themLenh(sl, { id: "m2", ben: "mua", gia: 52000, soLuong: 5 });
themLenh(sl, { id: "m3", ben: "mua", gia: 48000, soLuong: 8 });

console.log("thu tu gia lenh MUA (giam dan):", JSON.stringify(sl.lenhMua.map((l) => l.gia)));
console.log("lenh mua TOT NHAT (gia cao nhat):", JSON.stringify(sl.lenhMua[0]));
```

```text title=readonly
thu tu gia lenh MUA (giam dan): [52000,50000,48000]
lenh mua TOT NHAT (gia cao nhat): {"id":"m2","ben":"mua","gia":52000,"soLuong":5}
```

Ba lệnh mua được thêm theo thứ TỰ `50000`, `52000`, `48000` — không
hề theo thứ tự giá — nhưng SAU mỗi lần `themLenh`, mảng được sắp lại
NGAY, nên KẾT quả cuối cùng LUÔN đúng thứ tự GIẢM dần. `sl.lenhMua[0]`
LÀ `m2` (`52000`) — giá CAO nhất, bất kể nó được thêm Ở giữa.
::::

::::example{#ban-tang-dan-doc-lap}
Lệnh BÁN sắp theo chiều NGƯỢC lại — giá THẤP nhất đứng đầu — VÀ hai
mảng `lenhMua`/`lenhBan` hoàn toàn không ảnh hưởng lẫn NHAU:

```typescript title=readonly
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: 3 lenh MUA da them,
// sap xep giam dan: [52000 (m2), 50000 (m1), 48000 (m3)]
const sl = taoSoLenh();
themLenh(sl, { id: "m1", ben: "mua", gia: 50000, soLuong: 10 });
themLenh(sl, { id: "m2", ben: "mua", gia: 52000, soLuong: 5 });
themLenh(sl, { id: "m3", ben: "mua", gia: 48000, soLuong: 8 });

themLenh(sl, { id: "b1", ben: "ban", gia: 53000, soLuong: 6 });
themLenh(sl, { id: "b2", ben: "ban", gia: 51000, soLuong: 4 });
themLenh(sl, { id: "b3", ben: "ban", gia: 54000, soLuong: 9 });

console.log("thu tu gia lenh BAN (tang dan):", JSON.stringify(sl.lenhBan.map((l) => l.gia)));
console.log("lenh ban TOT NHAT (gia thap nhat):", JSON.stringify(sl.lenhBan[0]));
console.log("lenh mua VAN giu nguyen thu tu (khong bi anh huong):", JSON.stringify(sl.lenhMua.map((l) => l.gia)));
```

```text title=readonly
thu tu gia lenh BAN (tang dan): [51000,53000,54000]
lenh ban TOT NHAT (gia thap nhat): {"id":"b2","ben":"ban","gia":51000,"soLuong":4}
lenh mua VAN giu nguyen thu tu (khong bi anh huong): [52000,50000,48000]
```

`lenhBan` sắp theo chiều TĂNG dần — `51000` (giá thấp nhất, người sẵn
sàng bán RẺ nhất) đứng Ở `[0]`. Thêm ba lệnh bán hoàn toàn không làm
thay đổi thứ tự của `lenhMua` — hai mảng LÀ hai cấu trúc dữ liệu tách
biệt, `themLenh` chỉ chạm vào ĐÚNG một mảng tương ứng VỚI `lenh.ben`.
::::

::::predict{#doan-trung-gia commitOnce}
Thêm lệnh mua thứ TƯ, `m4`, giá CŨNG LÀ `52000` — TRÙNG giá với `m2`
(lệnh mua tốt nhất hiện tại). Sau khi `themLenh` chạy xong, `m4` đứng
Ở vị trí NÀO so với `m2` trong `sl.lenhMua`?

:::opt{correct}
Đứng NGAY SAU `m2` — `Array.prototype.sort` trong TypeScript hiện đại
LÀ sắp xếp ỔN định (stable): khi hai phần tử có cùng giá (`b.gia -
a.gia` LÀ `0`), thứ tự tương đối giữa chúng TRƯỚC khi sắp xếp được
giữ NGUYÊN; `m4` vừa được `push` vào CUỐI mảng, nên nó đứng SAU `m2`
trong nhóm cùng giá `52000`
:::
:::opt
Đứng NGAY TRƯỚC `m2` — lệnh MỚI hơn nên được ưu tiên đứng đầu trong
nhóm các lệnh cùng mức giá, giống như một hàng đợi LIFO (vào sau ra
trước)
::why
Nhầm "sắp xếp lại theo giá" VỚI "chèn lệnh mới lên đầu nhóm cùng
giá" — nhưng `themLenh` không hề có logic ưu tiên lệnh MỚI; nó chỉ
gọi `.sort()` trên TOÀN bộ mảng.

Chỗ lệch: `sl.lenhMua.push(lenh)` luôn thêm phần tử MỚI vào CUỐI mảng
TRƯỚC khi sắp xếp. Với hai phần tử có `gia` bằng nhau, hàm so sánh
`(a, b) => b.gia - a.gia` trả về `0` — sắp xếp ổn định GIỮ nguyên thứ
tự chúng đang có Ở mảng gốc. Vì `m4` nằm Ở CUỐI mảng (mới push) trong
khi `m2` đã có sẵn Ở đó từ trước, `m4` vẫn đứng SAU `m2` sau khi sắp
xếp — "trùng giá" chỉ dựa vào GIÁ thì chưa đủ để nói ai đứng trước ai;
đây LÀ đúng câu hỏi mà bài SAU của quest sẽ giải quyết bằng cách rõ
ràng thêm thời gian đặt lệnh.
::
:::
::::

::::code{#viet_them_lenh}
Hoàn thiện `themLenh` — nếu `lenh.ben` LÀ `"mua"`, đẩy vào
`sl.lenhMua` RỒI sắp xếp GIẢM dần theo `gia`; nếu LÀ `"ban"`, đẩy vào
`sl.lenhBan` RỒI sắp xếp TĂNG dần theo `gia`.

```typescript title=starter
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  ___
}

const slX = taoSoLenh();
themLenh(slX, { id: "x1", ben: "mua", gia: 10, soLuong: 1 });
themLenh(slX, { id: "x2", ben: "mua", gia: 30, soLuong: 1 });
console.log(JSON.stringify(slX.lenhMua.map((l) => l.gia)));
```

```typescript title=solution
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

const slX = taoSoLenh();
themLenh(slX, { id: "x1", ben: "mua", gia: 10, soLuong: 1 });
themLenh(slX, { id: "x2", ben: "mua", gia: 30, soLuong: 1 });
console.log(JSON.stringify(slX.lenhMua.map((l) => l.gia)));
```

```typescript title=test
const slT = taoSoLenh();
themLenh(slT, { id: "t1", ben: "mua", gia: 100, soLuong: 1 });
themLenh(slT, { id: "t2", ben: "mua", gia: 300, soLuong: 1 });
themLenh(slT, { id: "t3", ben: "mua", gia: 200, soLuong: 1 });
const giaMua = slT.lenhMua.map((l) => l.gia);
if (JSON.stringify(giaMua) !== JSON.stringify([300, 200, 100])) throw new Error("lenh MUA phai sap xep gia GIAM dan");

themLenh(slT, { id: "t4", ben: "ban", gia: 500, soLuong: 1 });
themLenh(slT, { id: "t5", ben: "ban", gia: 150, soLuong: 1 });
themLenh(slT, { id: "t6", ben: "ban", gia: 350, soLuong: 1 });
const giaBan = slT.lenhBan.map((l) => l.gia);
if (JSON.stringify(giaBan) !== JSON.stringify([150, 350, 500])) throw new Error("lenh BAN phai sap xep gia TANG dan");

const soLuongMua = slT.lenhMua.length;
if (soLuongMua !== 3) throw new Error("lenh mua khong duoc lan sang lenh ban");
const soLuongBan = slT.lenhBan.length;
if (soLuongBan !== 3) throw new Error("lenh ban khong duoc lan sang lenh mua");

const lenhMuaTot = slT.lenhMua[0];
if (lenhMuaTot === undefined || lenhMuaTot.id !== "t2") throw new Error("lenh mua TOT NHAT phai la gia cao nhat (t2, 300)");
const lenhBanTot = slT.lenhBan[0];
if (lenhBanTot === undefined || lenhBanTot.id !== "t5") throw new Error("lenh ban TOT NHAT phai la gia thap nhat (t5, 150)");
```

:::hints
- kind: attention
  body: "Re nhanh theo lenh.ben. Neu 'mua': sl.lenhMua.push(lenh) roi sl.lenhMua.sort((a, b) => b.gia - a.gia). Neu 'ban': sl.lenhBan.push(lenh) roi sl.lenhBan.sort((a, b) => a.gia - b.gia)."
- kind: strategy
  body: "if (lenh.ben === 'mua') { sl.lenhMua.push(lenh); sl.lenhMua.sort((a, b) => b.gia - a.gia); } else { sl.lenhBan.push(lenh); sl.lenhBan.sort((a, b) => a.gia - b.gia); }"
- kind: one-line
  body: "if (lenh.ben === \"mua\") { sl.lenhMua.push(lenh); sl.lenhMua.sort((a, b) => b.gia - a.gia); } else { sl.lenhBan.push(lenh); sl.lenhBan.sort((a, b) => a.gia - b.gia); }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[30,10]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giá tốt nhất của mỗi bên luôn Ở đầu mảng — mua giảm dần, bán tăng
dần. Nhưng có sổ lệnh thôi chưa đủ: một lệnh MỚI tới, làm sao biết nó
khớp với lệnh nào?
::::

::::reflect{#nghi-lai}
`themLenh` không hề tối ưu — sắp xếp lại TOÀN bộ mảng sau MỖI lần
thêm chắc chắn không phải cách một sàn giao dịch thật làm (nó dùng
cấu trúc dữ liệu chuyên biệt hơn). Nhưng nó LÀ đúng SỰ THẬT quan
trọng nhất: hai chiều sắp xếp NGƯỢC nhau (mua giảm dần, bán tăng dần)
đều phục vụ CÙNG một mục đích — đưa lệnh TỐT nhất, xét theo GÓC nhìn
của người mới đến muốn khớp, LUÔN về vị trí `[0]`.
::::

::::checkpoint{mastery=0.78}
::::
