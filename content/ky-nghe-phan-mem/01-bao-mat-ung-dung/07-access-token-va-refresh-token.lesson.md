---
id: ky-nghe-phan-mem.bao-mat-ung-dung.access-token-va-refresh-token
title: "Access token ngắn hạn + Refresh token thu hồi được"
summary: "JWT stateless (bài 5-6) không thu hồi được TRƯỚC hạn. Giải pháp: access token SỐNG NGẮN (stateless, nhanh), refresh token SỐNG DÀI nhưng LƯU Ở SERVER (tra cứu/xoá được). Đăng xuất = xoá refresh token — access token cũ tự hết hạn sau đó, không thu hồi tức thời nhưng gần đủ."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [bmud.refresh-token-revocation]
requires: [bmud.token-sign-verify]
concepts: [bmud.refresh-token-revocation]
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
Token stateless (bài 5-6) khó thu hồi TRƯỚC hạn. Chọn `ttlGiay` NGẮN
để giảm rủi ro — nhưng đăng nhập LẠI liên tục thì phiền. Cân bằng?
::::

::::explain{#access-va-refresh}
Giải pháp CHUẨN: **access token** SỐNG NGẮN (vài phút, stateless,
nhanh — GIỐNG bài 5-6), **refresh token** SỐNG DÀI nhưng **LƯU Ở
SERVER** (`Map`, tra cứu được, **XOÁ được**):

```typescript
const refreshConLai = new Map<string, string>(); // refreshToken -> nguoiDung

function capRefreshToken(rt: string, nguoiDung: string): void {
  refreshConLai.set(rt, nguoiDung);
}
function thuHoi(rt: string): void {
  refreshConLai.delete(rt);
}
function conHieuLuc(rt: string): boolean {
  return refreshConLai.has(rt);
}

capRefreshToken("RT-abc", "KH-01");
console.log(conHieuLuc("RT-abc"));
thuHoi("RT-abc"); // đăng xuất
console.log(conHieuLuc("RT-abc"));
```

```text
true
false
```

Đăng xuất = `thuHoi(rt)` — `refreshConLai.delete(...)` XOÁ NGAY, `rt`
đó KHÔNG BAO GIỜ dùng lại được để xin access token MỚI nữa. KHÁC HẲN
access token (bài 5-6, KHÔNG có bảng nào để xoá) — refresh token
**CÓ trạng thái Ở SERVER**, chấp nhận đánh đổi tốc độ (tra cứu Map
MỖI lần cần cấp access token mới, KHÔNG PHẢI mỗi request) để có được
khả năng thu hồi.
::::

::::example{#access-token-cu-van-song-sot}
Đánh đổi QUAN TRỌNG: thu hồi refresh token là **NGAY LẬP TỨC**, NHƯNG
access token **ĐÃ CẤP TRƯỚC ĐÓ** vẫn SỐNG SÓT tới khi TỰ hết hạn:

```typescript title=readonly
type ThongTinToken = { nguoiDung: string; hetHan: number };
function daHetHan(tt: ThongTinToken, gioHienTai: number): boolean {
  return tt.hetHan < gioHienTai;
}

const refreshConLai = new Map<string, string>();
refreshConLai.set("RT-abc", "KH-01");

// Access token TẠO lúc 1000, hết hạn lúc 1300 (5 phút sau)
const accessToken: ThongTinToken = { nguoiDung: "KH-01", hetHan: 1300 };

// User đăng xuất lúc 1100 -- refresh token bị THU HỒI NGAY
refreshConLai.delete("RT-abc");
console.log(refreshConLai.has("RT-abc"));

// NHƯNG access token CŨ, phát hành TRƯỚC đó, VẪN còn hiệu lực tới 1300
console.log(daHetHan(accessToken, 1100));
```

```text title=readonly
false
false
```

`refreshConLai.has("RT-abc")` là `false` (thu hồi NGAY) — NHƯNG
`daHetHan(accessToken, 1100)` VẪN là `false` (access token CHƯA hết
hạn, còn hiệu lực TỚI `1300`) — nghĩa là kẻ tấn công GIỮ được
access token CŨ (đánh cắp TRƯỚC lúc đăng xuất) VẪN dùng được thêm
**TỐI ĐA** thời gian sống của access token (ở đây, 200 giây còn lại)
— đây LÀ lý do access token PHẢI sống **NGẮN**: cửa sổ rủi ro tỉ lệ
THUẬN với `ttlGiay` của access token, KHÔNG PHẢI của refresh token.
::::

::::predict{#doan-thu-hoi-hai-lan commitOnce}
```typescript
const refreshConLai = new Map<string, string>();
function capRefreshToken(rt: string, nguoiDung: string): void {
  refreshConLai.set(rt, nguoiDung);
}
function thuHoi(rt: string): void {
  refreshConLai.delete(rt);
}
function conHieuLuc(rt: string): boolean {
  return refreshConLai.has(rt);
}

capRefreshToken("RT-01", "KH-01");
thuHoi("RT-01");
// GỌI thuHoi LẦN THỨ HAI, trên token ĐÃ bị thu hồi rồi
thuHoi("RT-01");
console.log(conHieuLuc("RT-01"));
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
Chương trình crash — vì `Map.delete` gọi trên MỘT key ĐÃ bị xoá
TRƯỚC ĐÓ (không còn tồn tại) sẽ ném lỗi "key không tồn tại"
::why
Gần đúng ở việc bạn nhớ ĐÚNG `"RT-01"` ĐÃ bị `delete` Ở LẦN GỌI
`thuHoi` ĐẦU TIÊN — quan sát về trạng thái "đã xoá rồi" đó đúng.

Chỗ lệch: `Map.prototype.delete(key)` trong JavaScript **KHÔNG BAO
GIỜ ném lỗi**, dù `key` CÓ tồn tại hay KHÔNG — nó chỉ TRẢ VỀ `boolean`
(`true` nếu XOÁ được thứ gì đó, `false` nếu key VỐN DĨ không tồn
tại), giá trị trả về ĐÓ ở đây bị BỎ QUA (hàm `thuHoi` khai kiểu trả
`void`). Gọi `delete` MỘT key đã xoá LÀ thao tác AN TOÀN, VÔ HẠI —
CHỈ đơn giản "không có gì để xoá thêm".
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `thuHoi("RT-01")` HAI LẦN LIÊN TIẾP với
CÙNG một chuỗi không hợp lệ, TypeScript coi đó là gọi hàm dư thừa
::why
Gần đúng ở việc bạn để ý CÓ hai lời gọi `thuHoi("RT-01")` GIỐNG HỆT
nhau LIÊN TIẾP — một quan sát ĐÚNG về mặt VĂN BẢN code.

Chỗ lệch: TypeScript KHÔNG có khái niệm "gọi hàm dư thừa là lỗi" —
gọi MỘT hàm `void` NHIỀU LẦN với CÙNG đối số hoàn toàn HỢP LỆ (không
có gì đặc biệt về `thuHoi` khiến việc gọi lặp lại bị cấm). Biên dịch
VÀ chạy bình thường.
::
:::
::::

::::code{#viet_refresh_token}
Tự viết `capRefreshToken`, `thuHoi`, `conHieuLuc`.

```typescript title=starter
const refreshConLai = new Map<string, string>();

function capRefreshToken(rt: string, nguoiDung: string): void {
  ___;
}
function thuHoi(rt: string): void {
  ___;
}
function conHieuLuc(rt: string): boolean {
  return ___;
}

capRefreshToken("RT-01", "KH-01");
console.log(conHieuLuc("RT-01"));
```

```typescript title=solution
const refreshConLai = new Map<string, string>();

function capRefreshToken(rt: string, nguoiDung: string): void {
  refreshConLai.set(rt, nguoiDung);
}
function thuHoi(rt: string): void {
  refreshConLai.delete(rt);
}
function conHieuLuc(rt: string): boolean {
  return refreshConLai.has(rt);
}

capRefreshToken("RT-01", "KH-01");
console.log(conHieuLuc("RT-01"));
```

```typescript title=test
if (conHieuLuc("RT-CHUA-TUNG-CAP") !== false) throw new Error("token chưa từng cấp phải KHÔNG còn hiệu lực");

capRefreshToken("RT-99", "KH-99");
if (conHieuLuc("RT-99") !== true) throw new Error("token vừa cấp phải CÒN hiệu lực");

thuHoi("RT-99");
if (conHieuLuc("RT-99") !== false) throw new Error("token đã thu hồi phải KHÔNG còn hiệu lực");

// Thu hồi HAI LẦN không được gây lỗi
thuHoi("RT-99");
if (conHieuLuc("RT-99") !== false) throw new Error("thu hồi lần hai vẫn phải giữ trạng thái KHÔNG còn hiệu lực");

// Token KHÁC không bị ảnh hưởng
capRefreshToken("RT-100", "KH-100");
if (conHieuLuc("RT-100") !== true) throw new Error("token khác không liên quan phải KHÔNG bị ảnh hưởng bởi thu hồi token khác");
```

:::hints
- kind: attention
  body: "capRefreshToken: GHI vào Map bằng .set(rt, nguoiDung). thuHoi: XOÁ khỏi Map bằng .delete(rt). conHieuLuc: kiểm SỰ TỒN TẠI bằng .has(rt)."
- kind: strategy
  body: "refreshConLai.set(rt, nguoiDung) : refreshConLai.delete(rt) : refreshConLai.has(rt) — ba thao tác Map chuẩn."
- kind: one-line
  body: "___ (capRefreshToken) = refreshConLai.set(rt, nguoiDung)\n___ (thuHoi) = refreshConLai.delete(rt)\n___ (conHieuLuc) = refreshConLai.has(rt)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Access token ngắn hạn + refresh token thu hồi được — cân bằng tốc độ
và khả năng kiểm soát. Bước tiếp theo: phân quyền THẬT — vai trò sở
hữu quyền.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Xác thực xong, biết ĐÚNG người dùng là ai — nhưng người dùng đó ĐƯỢC
làm gì cụ thể (đọc, ghi, xoá)? Gán quyền TRỰC TIẾP cho TỪNG người
dùng, hay có cách tổ chức GỌN hơn không?
::::

::::checkpoint{mastery=0.8}
::::
