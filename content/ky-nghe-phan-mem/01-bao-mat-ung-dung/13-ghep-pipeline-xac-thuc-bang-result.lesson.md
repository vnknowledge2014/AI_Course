---
id: ky-nghe-phan-mem.bao-mat-ung-dung.ghep-pipeline-xac-thuc-bang-result
title: "Capstone: Ghép pipeline xác thực bằng Result — trích token, xác minh, kiểm quyền"
summary: "Bài chốt cụm 2, code có chấm điểm sống: dùng LẠI Result<T,E>/chainResult (T4.5) áp dụng vào pipeline auth thật. guardAuth ghép BA bước qua chainResult LỒNG: layToken → xacMinh → kiemTraQuyen — một lỗi ở BẤT KỲ bước nào short-circuit NGAY."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [bmud.auth-pipeline-result]
requires: [alg.gate-boss, bmud.oauth2-pkce]
concepts: [bmud.auth-pipeline-result]
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
Bài chốt cụm 2. Bạn có TẤT CẢ mảnh ghép: trích token, xác minh, kiểm
quyền. Ghép BA bước đó thành MỘT pipeline, dùng `Result` (đã học ở
T4.5) — trông thế nào?
::::

::::explain{#guard-auth-pipeline}
Dùng LẠI `Result<T,E>`/`chainResult` (T4.5, ĐÃ dạy — KHÔNG dạy lại lý
thuyết composition) áp dụng vào pipeline auth THẬT. `guardAuth` ghép
BA bước qua `chainResult` **LỒNG**: `layToken` → `xacMinh` →
`kiemTraQuyen`:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (t: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

type LoiXacThuc = "thieu_token" | "token_khong_hop_le" | "het_han" | "khong_du_quyen";
type ThongTin = { nguoiDung: string; vaiTro: string; hetHan: number };

function layToken(header: string | undefined): Result<string, LoiXacThuc> {
  if (!header) return loi("thieu_token");
  if (!header.startsWith("Bearer ")) return loi("token_khong_hop_le");
  return ok(header.slice(7));
}

function xacMinh(token: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  if (token === "HOP_LE") return ok({ nguoiDung: "U1", vaiTro: "admin", hetHan: gioHienTai + 60 });
  if (token === "HET_HAN") return loi("het_han");
  return loi("token_khong_hop_le");
}

function kiemTraQuyen(tt: ThongTin, vaiTroCanCo: string): Result<ThongTin, LoiXacThuc> {
  return tt.vaiTro === vaiTroCanCo || tt.vaiTro === "admin" ? ok(tt) : loi("khong_du_quyen");
}

function guardAuth(header: string | undefined, vaiTroCanCo: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  return chainResult(
    chainResult(layToken(header), (token) => xacMinh(token, gioHienTai)),
    (tt) => kiemTraQuyen(tt, vaiTroCanCo)
  );
}

console.log(JSON.stringify(guardAuth("Bearer HOP_LE", "admin", 1000)));
console.log(JSON.stringify(guardAuth(undefined, "admin", 1000)));
```

```text
{"kind":"ok","giaTri":{"nguoiDung":"U1","vaiTro":"admin","hetHan":1060}}
{"kind":"loi","loi":"thieu_token"}
```

`guardAuth` KHÔNG có `if`/`else` LỒNG NHAU nào — CHỈ hai lời gọi
`chainResult` NỐI ba hàm THUẦN, MỖI hàm test riêng được (nối kỹ thuật
capstone đã học ở track DDD). MỘT lỗi Ở BẤT KỲ bước nào SHORT-CIRCUIT
NGAY: header thiếu → `layToken` lỗi → `xacMinh`/`kiemTraQuyen` KHÔNG
BAO GIỜ được gọi.
::::

::::example{#thu-tu-cac-loai-loi}
BỐN kịch bản lỗi KHÁC NHAU, MỖI kịch bản dừng Ở ĐÚNG bước GÂY LỖI:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (t: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) { case "ok": return f(r.giaTri); case "loi": return loi(r.loi); }
}
type LoiXacThuc = "thieu_token" | "token_khong_hop_le" | "het_han" | "khong_du_quyen";
type ThongTin = { nguoiDung: string; vaiTro: string; hetHan: number };
function layToken(header: string | undefined): Result<string, LoiXacThuc> {
  if (!header) return loi("thieu_token");
  if (!header.startsWith("Bearer ")) return loi("token_khong_hop_le");
  return ok(header.slice(7));
}
function xacMinh(token: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  if (token === "HOP_LE") return ok({ nguoiDung: "U1", vaiTro: "admin", hetHan: gioHienTai + 60 });
  if (token === "HET_HAN") return loi("het_han");
  return loi("token_khong_hop_le");
}
function kiemTraQuyen(tt: ThongTin, vaiTroCanCo: string): Result<ThongTin, LoiXacThuc> {
  return tt.vaiTro === vaiTroCanCo || tt.vaiTro === "admin" ? ok(tt) : loi("khong_du_quyen");
}
function guardAuth(header: string | undefined, vaiTroCanCo: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  return chainResult(chainResult(layToken(header), (token) => xacMinh(token, gioHienTai)), (tt) => kiemTraQuyen(tt, vaiTroCanCo));
}

console.log(JSON.stringify(guardAuth("Bearer HET_HAN", "admin", 1000)));       // lỗi Ở xacMinh
console.log(JSON.stringify(guardAuth("khong-co-bearer", "admin", 1000)));      // lỗi Ở layToken
```

```text title=readonly
{"kind":"loi","loi":"het_han"}
{"kind":"loi","loi":"token_khong_hop_le"}
```

`guardAuth("Bearer HET_HAN", ...)`: `layToken` THÀNH CÔNG (đúng tiền
tố `"Bearer "`), `xacMinh` phát hiện `"HET_HAN"` → lỗi `"het_han"`.
`guardAuth("khong-co-bearer", ...)`: `layToken` phát hiện THIẾU tiền
tố `"Bearer "` → lỗi NGAY `"token_khong_hop_le"`, `xacMinh` KHÔNG BAO
GIỜ được gọi. Mỗi mã lỗi (`LoiXacThuc`) CHỈ RÕ chính xác BƯỚC nào thất
bại — client đọc mã lỗi BIẾT NGAY cần sửa gì (thêm header? đăng nhập
lại? xin thêm quyền?).
::::

::::predict{#doan-vai-tro-khong-du commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (t: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) { case "ok": return f(r.giaTri); case "loi": return loi(r.loi); }
}
type LoiXacThuc = "thieu_token" | "token_khong_hop_le" | "het_han" | "khong_du_quyen";
type ThongTin = { nguoiDung: string; vaiTro: string; hetHan: number };
function layToken(header: string | undefined): Result<string, LoiXacThuc> {
  if (!header) return loi("thieu_token");
  if (!header.startsWith("Bearer ")) return loi("token_khong_hop_le");
  return ok(header.slice(7));
}
function xacMinh(token: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  if (token === "BIEN_TAP") return ok({ nguoiDung: "U2", vaiTro: "bien-tap", hetHan: gioHienTai + 60 });
  return loi("token_khong_hop_le");
}
function kiemTraQuyen(tt: ThongTin, vaiTroCanCo: string): Result<ThongTin, LoiXacThuc> {
  return tt.vaiTro === vaiTroCanCo || tt.vaiTro === "admin" ? ok(tt) : loi("khong_du_quyen");
}
function guardAuth(header: string | undefined, vaiTroCanCo: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  return chainResult(chainResult(layToken(header), (token) => xacMinh(token, gioHienTai)), (tt) => kiemTraQuyen(tt, vaiTroCanCo));
}

// Token HỢP LỆ (vai trò "bien-tap"), NHƯNG endpoint cần vai trò "admin"
console.log(JSON.stringify(guardAuth("Bearer BIEN_TAP", "admin", 1000)));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"loi","loi":"khong_du_quyen"}`
:::

:::opt
`{"kind":"ok","giaTri":{"nguoiDung":"U2","vaiTro":"bien-tap","hetHan":1060}}`
— vì token HOÀN TOÀN hợp lệ (`layToken` VÀ `xacMinh` đều thành công),
nên `guardAuth` trả `ok` với thông tin người dùng đã xác thực được
::why
Gần đúng ở việc bạn nhớ ĐÚNG `layToken` VÀ `xacMinh` đều THÀNH CÔNG
(token đúng định dạng, `"BIEN_TAP"` được `xacMinh` NHẬN RA) — hai
bước ĐẦU thật sự KHÔNG lỗi.

Chỗ lệch: `guardAuth` có **BA** bước, KHÔNG PHẢI hai — bước THỨ BA
(`kiemTraQuyen`) KIỂM `tt.vaiTro === vaiTroCanCo || tt.vaiTro ===
"admin"`. Ở đây `tt.vaiTro = "bien-tap"`, `vaiTroCanCo = "admin"` —
`"bien-tap" === "admin"` là `false`, VÀ `"bien-tap" === "admin"` (vế
sau, kiểm CÓ PHẢI admin không) CŨNG `false` — điều kiện TỔNG là
`false`, `kiemTraQuyen` trả `loi("khong_du_quyen")`. `chainResult`
NGOÀI CÙNG nhận kết quả ĐÓ, TRẢ THẲNG RA — token HỢP LỆ (xác thực
đúng người) KHÔNG đồng nghĩa ĐỦ QUYỀN (phân quyền, bài 1's bài học
"authn thành công không đảm bảo authz thành công", áp dụng LẠI ở
đây).
::
:::

:::opt
Máy báo lỗi biên dịch — `xacMinh` (định nghĩa LẠI trong đoạn code
này, khác định nghĩa Ở PHẦN EXPLAIN) không hợp lệ vì TypeScript
KHÔNG cho phép khai lại MỘT hàm CÙNG TÊN với chữ ký khác
::why
Gần đúng ở việc bạn để ý `xacMinh` Ở ĐÂY có THÂN HÀM khác với
`xacMinh` trong phần explain (kiểm `"BIEN_TAP"` thay vì `"HOP_LE"`) —
một quan sát ĐÚNG về sự khác biệt NỘI DUNG.

Chỗ lệch: MỖI khối code TRONG lesson này chạy TRONG MỘT PHẠM VI RIÊNG
BIỆT, HOÀN TOÀN ĐỘC LẬP (không phải MỘT chương trình DUY NHẤT gộp cả
explain lẫn predict lại) — khai LẠI `xacMinh` với thân hàm KHÁC ở một
khối code KHÁC hoàn toàn HỢP LỆ, không có xung đột "khai lại hàm"
nào cả. Biên dịch sạch.
::
:::
::::

::::code{#viet_laytoken_va_kiemtraquyen}
Tự viết `layToken` và `kiemTraQuyen`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (t: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) { case "ok": return f(r.giaTri); case "loi": return loi(r.loi); }
}

type LoiXacThuc = "thieu_token" | "token_khong_hop_le" | "het_han" | "khong_du_quyen";
type ThongTin = { nguoiDung: string; vaiTro: string; hetHan: number };

function layToken(header: string | undefined): Result<string, LoiXacThuc> {
  if (!header) return ___;
  if (!header.startsWith("Bearer ")) return ___;
  return ok(header.slice(7));
}

function xacMinh(token: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  if (token === "HOP_LE") return ok({ nguoiDung: "U1", vaiTro: "admin", hetHan: gioHienTai + 60 });
  if (token === "HET_HAN") return loi("het_han");
  return loi("token_khong_hop_le");
}

function kiemTraQuyen(tt: ThongTin, vaiTroCanCo: string): Result<ThongTin, LoiXacThuc> {
  return tt.vaiTro === vaiTroCanCo || tt.vaiTro === "admin" ? ___ : ___;
}

function guardAuth(header: string | undefined, vaiTroCanCo: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  return chainResult(chainResult(layToken(header), (token) => xacMinh(token, gioHienTai)), (tt) => kiemTraQuyen(tt, vaiTroCanCo));
}

console.log(JSON.stringify(guardAuth("Bearer HOP_LE", "admin", 1000)));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (t: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) { case "ok": return f(r.giaTri); case "loi": return loi(r.loi); }
}

type LoiXacThuc = "thieu_token" | "token_khong_hop_le" | "het_han" | "khong_du_quyen";
type ThongTin = { nguoiDung: string; vaiTro: string; hetHan: number };

function layToken(header: string | undefined): Result<string, LoiXacThuc> {
  if (!header) return loi("thieu_token");
  if (!header.startsWith("Bearer ")) return loi("token_khong_hop_le");
  return ok(header.slice(7));
}

function xacMinh(token: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  if (token === "HOP_LE") return ok({ nguoiDung: "U1", vaiTro: "admin", hetHan: gioHienTai + 60 });
  if (token === "HET_HAN") return loi("het_han");
  return loi("token_khong_hop_le");
}

function kiemTraQuyen(tt: ThongTin, vaiTroCanCo: string): Result<ThongTin, LoiXacThuc> {
  return tt.vaiTro === vaiTroCanCo || tt.vaiTro === "admin" ? ok(tt) : loi("khong_du_quyen");
}

function guardAuth(header: string | undefined, vaiTroCanCo: string, gioHienTai: number): Result<ThongTin, LoiXacThuc> {
  return chainResult(chainResult(layToken(header), (token) => xacMinh(token, gioHienTai)), (tt) => kiemTraQuyen(tt, vaiTroCanCo));
}

console.log(JSON.stringify(guardAuth("Bearer HOP_LE", "admin", 1000)));
```

```typescript title=test
const thieu = guardAuth(undefined, "admin", 1000);
if (thieu.kind !== "loi" || thieu.loi !== "thieu_token") throw new Error("header thiếu phải ra loi thieu_token");

const saiDinhDang = guardAuth("khong-co-bearer", "admin", 1000);
if (saiDinhDang.kind !== "loi" || saiDinhDang.loi !== "token_khong_hop_le") throw new Error("header không đúng định dạng Bearer phải ra loi token_khong_hop_le");

const hetHan = guardAuth("Bearer HET_HAN", "admin", 1000);
if (hetHan.kind !== "loi" || hetHan.loi !== "het_han") throw new Error("token hết hạn phải ra loi het_han");

const hopLe = guardAuth("Bearer HOP_LE", "admin", 1000);
if (hopLe.kind !== "ok") throw new Error("token hợp lệ, đủ quyền phải ra ok");
if (hopLe.kind === "ok" && hopLe.giaTri.nguoiDung !== "U1") throw new Error("kết quả ok phải giữ đúng nguoiDung");
```

:::hints
- kind: attention
  body: "layToken: header rỗng trả loi(\"thieu_token\"); header không có tiền tố Bearer trả loi(\"token_khong_hop_le\"). kiemTraQuyen: đủ quyền trả ok(tt); không đủ trả loi(\"khong_du_quyen\")."
- kind: strategy
  body: 'loi("thieu_token") : loi("token_khong_hop_le") — layToken. ok(tt) : loi("khong_du_quyen") — kiemTraQuyen.'
- kind: one-line
  body: '___ (layToken, thiếu header) = loi("thieu_token")\n___ (layToken, sai định dạng) = loi("token_khong_hop_le")\n___ (kiemTraQuyen, đủ quyền) = ok(tt)\n___ (kiemTraQuyen, thiếu quyền) = loi("khong_du_quyen")'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "U1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 2 hoàn tất: refresh token thu hồi được, RBAC/ABAC, OAuth2, pipeline
auth ghép bằng Result — mỗi bước short-circuit rõ ràng. Cụm tiếp theo:
lỗ hổng KHÁC hẳn — khi input người dùng trở thành LỆNH thực thi.
::::

::::reflect{#nghi-lai}
Track chuyển hướng: từ "AI được làm gì" (auth/authz) sang "code có LỖ
HỔNG gì". Khi dữ liệu người dùng nhập được nối THẲNG vào một câu lệnh
SQL — chuyện gì có thể xảy ra?
::::

::::checkpoint{mastery=0.8}
::::
