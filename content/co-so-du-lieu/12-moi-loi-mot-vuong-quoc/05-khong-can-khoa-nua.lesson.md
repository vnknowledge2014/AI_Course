---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.khong-can-khoa-nua
title: "Không cần khoá nữa"
summary: "ghiVaoLoiDungCua ghép routing (chonLoiChoKhoa, bài 4) với kho riêng từng lõi (taoKhoChoMoiLoi, bài 3): tìm đúng chỉ số lõi rồi ghi thẳng vào Map của lõi đó — không khoá nào cả. Hai khoá khác nhau CÓ THỂ route tới cùng một lõi (nguoidung0 và nguoidung2 cùng vào lõi 2, bài 4), nhưng đó không phải tranh chấp: một lõi luôn xử lý công việc của chính nó tuần tự, từng việc một — 'lost update' của bài 1 không còn cách nào xảy ra."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.khong-can-khoa-nua]
requires: [db.khoa-thuoc-loi-nao]
concepts: [db.khong-can-khoa-nua]
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
Kho RIÊNG mỗi lõi (bài 3) VÀ routing khoá→lõi (bài 4) — ghép LẠI, còn
CẦN khoá GÌ nữa không?
::::

::::explain{#ghi-vao-loi-dung-cua}
`ghiVaoLoiDungCua` tìm chỉ số LÕI đúng CHO khoá (`chonLoiChoKhoa`,
bài 4), RỒI ghi THẲNG vào `Map` của CHÍNH lõi đó — KHÔNG có khoá nào
CẢ:

```typescript title=readonly
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) {
    khoCacLoi.push(new Map());
  }
  return khoCacLoi;
}

function ghiVaoLoiDungCua(khoCacLoi: Map<string, number>[], khoa: string, giaTri: number): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, khoCacLoi.length);
  khoCacLoi[chiSoLoi]!.set(khoa, giaTri);
}

const kho = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho, "nguoidung0", 100);
ghiVaoLoiDungCua(kho, "nguoidung1", 200);
console.log(kho[2]!.get("nguoidung0"));
console.log(kho[0]!.get("nguoidung1"));
```

```text title=readonly
100
200
```

`nguoidung0` route VỀ lõi `2` (q11's công thức, bài 4) — VÀ giá trị
`100` NẰM đúng TRONG `kho[2]`. `nguoidung1` route VỀ lõi `0`, giá trị
`200` NẰM trong `kho[0]`. MỖI lần ghi CHỈ chạm ĐÚNG MỘT `Map` — không
CÓ bước "xin khoá" nào TRƯỚC khi ghi.
::::

::::example{#hai-khoa-cung-mot-loi-khong-sao}
`nguoidung0` VÀ `nguoidung2` (q11's ví dụ, bài 4) CÙNG route VỀ lõi
`2` — ghi CẢ hai VÀO cùng một `Map`:

```typescript title=readonly
ghiVaoLoiDungCua(kho, "nguoidung2", 300);
console.log(kho[2]!.get("nguoidung0"));
console.log(kho[2]!.get("nguoidung2"));
```

```text title=readonly
100
300
```

CẢ hai giá trị đều ĐÚNG, KHÔNG cái nào bị GHI đè hay MẤT. Hai khoá
KHÁC nhau CÙNG một `Map` không phải vấn đề — CHÚNG chỉ LÀ hai entry
KHÁC nhau TRONG cùng một `Map`, VÀ chỉ MỘT luồng thực thi (chính lõi
`2`) từng đụng và `Map` NÀY, xử lý TỪNG việc một, tuần TỰ — không CÓ
hai thao TÁC nào cùng "xen KẼ" đọc/ghi trên CÙNG một entry NHƯ kịch
bản gây LỖI Ở bài 1.
::::

::::predict{#doan-hai-yeu-cau-cung-khoa commitOnce}
HAI yêu cầu KHÁC nhau, CÙNG tăng giá trị CỦA khoá `"nguoidung0"` LÊN
`1`, đến "gần NHƯ đồng thời". CẢ hai yêu CẦU này được xử LÝ Ở lõi
nào, VÀ CÓ xảy ra "lost update" (bài 1) không?

:::opt{correct}
CẢ hai đều được xử LÝ Ở lõi `2` (VÌ `chonLoiChoKhoa` LUÔN routing
CÙNG một khoá VỀ cùng một lõi) — VÀ lõi `2` xử LÝ từng việc MỘT, tuần
tự, nên KHÔNG có "lost update" NÀO
:::

:::opt
CÓ THỂ xảy ra "lost update" — hai YÊU cầu tới "gần như đồng thời"
nghĩa LÀ chúng CÓ thể được xử lý Ở HAI lõi khác nhau CÙNG lúc, giống
kịch bản bài 1
::why
Gần đúng ở việc bạn nhớ ĐÚNG kịch bản "lost update" (bài 1) CẦN hai
lõi KHÁC nhau cùng đụng và MỘT dữ liệu — một mối lo HỢP lý nếu hai
yêu cầu THẬT sự có thể rơi VÀO hai lõi khác nhau.

Chỗ lệch: `chonLoiChoKhoa("nguoidung0", soLoi)` LUÔN cho ra ĐÚNG cùng
một kết quả (tất định, bài 4) — KHÔNG có khái niệm "gần đồng thời"
làm THAY đổi lõi nào xử lý khoá NÀO. CẢ hai yêu cầu, dù ĐẾN "đồng
thời" hay CÁCH nhau bao lâu, đều PHẢI đi qua ĐÚNG lõi `2` — VÀ một
lõi (một LUỒNG thực thi DUY nhất) không thể tự "xen kẽ" với chính
NÓ, nó xử lý đúng MỘT việc tại một THỜI điểm. "Lost update" cần HAI
luồng độc lập CÙNG đụng một dữ liệu — Ở ĐÂY chỉ có đúng MỘT luồng
(lõi `2`) từng chạm và khoá `"nguoidung0"`.
::
:::
::::

::::code{#viet_ghi_vao_loi_dung_cua}
Hoàn thiện `ghiVaoLoiDungCua` — TÌM chỉ số lõi ĐÚNG cho khoá, RỒI ghi
`giaTri` vào `Map` CỦA đúng lõi đó.

```typescript title=starter
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) {
    khoCacLoi.push(new Map());
  }
  return khoCacLoi;
}

function ghiVaoLoiDungCua(khoCacLoi: Map<string, number>[], khoa: string, giaTri: number): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, khoCacLoi.length);
  ___
}

const kho = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho, "nguoidung0", 100);
console.log(kho[2]!.get("nguoidung0"));
```

```typescript title=solution
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) {
    khoCacLoi.push(new Map());
  }
  return khoCacLoi;
}

function ghiVaoLoiDungCua(khoCacLoi: Map<string, number>[], khoa: string, giaTri: number): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, khoCacLoi.length);
  khoCacLoi[chiSoLoi]!.set(khoa, giaTri);
}

const kho = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho, "nguoidung0", 100);
console.log(kho[2]!.get("nguoidung0"));
```

```typescript title=test
const kho2 = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho2, "nguoidung0", 100);
console.log(kho2[2]!.get("nguoidung0"));
if (kho2[2]!.get("nguoidung0") !== 100) throw new Error("nguoidung0 phai duoc ghi VAO dung loi 2, gia tri 100");
if (kho2[0]!.get("nguoidung0") !== undefined) throw new Error("nguoidung0 KHONG duoc xuat hien o loi 0");

ghiVaoLoiDungCua(kho2, "nguoidung1", 200);
if (kho2[0]!.get("nguoidung1") !== 200) throw new Error("nguoidung1 phai duoc ghi vao dung loi 0, gia tri 200");

ghiVaoLoiDungCua(kho2, "nguoidung2", 300);
if (kho2[2]!.get("nguoidung2") !== 300) throw new Error("nguoidung2 phai duoc ghi vao dung loi 2 (cung loi voi nguoidung0)");
if (kho2[2]!.get("nguoidung0") !== 100) throw new Error("ghi nguoidung2 KHONG duoc lam mat gia tri nguoidung0 da ghi truoc do");
```

:::hints
- kind: attention
  body: "Ghi giaTri vao Map cua dung chiSoLoi, voi khoa lam key -- mot dong."
- kind: strategy
  body: "khoCacLoi[chiSoLoi]!.set(khoa, giaTri);"
- kind: one-line
  body: "khoCacLoi[chiSoLoi]!.set(khoa, giaTri);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "100"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không CÒN khoá — nhưng NẾU một yêu cầu Ở lõi NÀY cần dữ liệu Ở lõi
KHÁC thì SAO? Nó KHÔNG được phép đọc trực TIẾP.
::::

::::reflect{#nghi-lai}
`ghiVaoLoiDungCua` KHÔNG hề "quản LÝ" tranh chấp giỏi HƠN — nó LOẠI
bỏ HẲN tình huống có THỂ tranh chấp: routing tất ĐỊNH (bài 4) đảm bảo
MỘT khoá CHỈ từng thuộc VỀ đúng một lõi, VÀ mỗi lõi LÀ một luồng THỰC
thi duy nhất xử LÝ việc của chính NÓ tuần tự — hai điều kiện NÀY cộng
lại LÀM cho khái niệm "hai luồng CÙNG đụng một dữ liệu" không CÒN
tồn tại. Đây LÀ "share-nothing" ĐẦY đủ: không chỉ dữ liệu KHÔNG chia
sẻ (bài 3), mà CẢ quyền quyết ĐỊNH "ai xử lý khoá NÀO" cũng tất định
VÀ không cần thương LƯỢNG. Nhưng một hệ THẬT hiếm khi mọi thứ gọn
gàng NHƯ vậy — khi lõi `0` cần ĐỌC dữ liệu đang nằm Ở lõi `2`, nó
PHẢI làm gì?
::::

::::checkpoint{mastery=0.85}
::::
</content>
