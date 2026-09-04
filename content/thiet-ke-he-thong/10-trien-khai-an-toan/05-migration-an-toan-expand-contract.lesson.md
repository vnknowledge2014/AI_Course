---
id: thiet-ke-he-thong.trien-khai-an-toan.migration-an-toan-expand-contract
title: "Database migration an toàn: expand-contract"
summary: "BanGhi{soDienThoaiCu, soDienThoaiMoi} qua bon buoc: EXPAND (them cot moi, con rong) -> ghiSoDienThoai_DuaGhi ghi CA hai cot -> chuyenDuLieuCu backfill ban ghi cu (moi=undefined) sang cot moi -> docLinhHoat doc UU TIEN cot moi, fallback ve cot cu neu moi con undefined -> xoaCotCu (CONTRACT) chi an toan SAU khi da migrate het. Vi du nguy hiem: xoaCotCu goi SOM (truoc khi migrate) tren ban ghi CHUA tung duoc ghi cot moi -- CA HAI cot deu thanh undefined, du lieu MAT that su, khong phai ly thuyet."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.migration-an-toan-expand-contract]
requires: [sd.health-check-va-readiness-probe]
concepts: [sd.migration-an-toan-expand-contract]
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
Rolling deploy (bài 1) nghĩa LÀ code CŨ và code MỚI chạy CÙNG lúc,
trong SUỐT thời gian rollout. Nếu một migration đổi TÊN cột ngay lập
tức — xoá cột cũ, thêm cột mới, xong trong MỘT bước — thì mọi instance
CŨ vẫn đang chạy sẽ đọc một cột không còn TỒN TẠI. Migration an toàn
phải tôn trọng chính điều rolling deploy đã dạy: có một khoảng thời
gian hai phiên bản code cùng sống.
::::

::::explain{#dua-ghi-giai-doan-expand}
Giai đoạn EXPAND đã thêm cột MỚI (`soDienThoaiMoi`, còn RỖNG). Code
MỚI, trong giai đoạn NÀY, ghi VÀO cả hai cột CÙNG lúc — dual write —
để cả code cũ LẪN code mới đều đọc ĐÚNG dữ liệu, bất kể instance nào
đang xử lý request:

```typescript title=readonly
interface BanGhi { id: string; soDienThoaiCu: string | undefined; soDienThoaiMoi: string | undefined; }

function taoBanGhi(id: string, soCu: string): BanGhi {
  return { id, soDienThoaiCu: soCu, soDienThoaiMoi: undefined };
}

// buoc EXPAND da xong (cot moi da them, dang la undefined). Code MOI trong giai
// doan nay ghi CA hai cot -- dual write -- de ca code cu VA code moi doc dung
function ghiSoDienThoai_DuaGhi(bg: BanGhi, soMoi: string): void {
  bg.soDienThoaiCu = soMoi;
  bg.soDienThoaiMoi = soMoi;
}

function docSoDienThoai_CodeCu(bg: BanGhi): string | undefined {
  return bg.soDienThoaiCu;
}

function docSoDienThoai_CodeMoi(bg: BanGhi): string | undefined {
  return bg.soDienThoaiMoi;
}

const bg = taoBanGhi("nguoi-1", "0900-000-000");
console.log("truoc khi ghi lai, code cu doc:", docSoDienThoai_CodeCu(bg));
console.log("truoc khi ghi lai, code moi doc (cot moi con RONG):", docSoDienThoai_CodeMoi(bg));

ghiSoDienThoai_DuaGhi(bg, "0911-111-111");
console.log("sau khi DUA GHI, code CU doc:", docSoDienThoai_CodeCu(bg));
console.log("sau khi DUA GHI, code MOI doc:", docSoDienThoai_CodeMoi(bg));
```

```text title=readonly
truoc khi ghi lai, code cu doc: 0900-000-000
truoc khi ghi lai, code moi doc (cot moi con RONG): undefined
sau khi DUA GHI, code CU doc: 0911-111-111
sau khi DUA GHI, code MOI doc: 0911-111-111
```

Trước khi `ghiSoDienThoai_DuaGhi` chạy, cột mới của `bg` còn RỖNG —
đây LÀ trạng thái bình thường ngay SAU expand, không phải LỖI. Sau
dual write, CẢ hai cột đều mang giá trị `"0911-111-111"` — bất kể
instance nào (cũ hay mới) đọc ĐÂU, kết quả đều đúng.
::::

::::example{#migrate-va-contract-som}
`chuyenDuLieuCu` sao chép dữ liệu CŨ (từ trước khi có dual write) sang
cột mới. `xoaCotCu` LÀ bước CONTRACT — chỉ an toàn SAU khi mọi bản ghi
đã có cột mới. Chạy `xoaCotCu` SỚM, trên một bản ghi CHƯA từng được
migrate, xoá luôn dữ liệu THẬT — không chỉ khiến code cũ vỡ:

```typescript title=readonly
interface BanGhi { id: string; soDienThoaiCu: string | undefined; soDienThoaiMoi: string | undefined; }

function taoBanGhi(id: string, soCu: string): BanGhi {
  return { id, soDienThoaiCu: soCu, soDienThoaiMoi: undefined };
}

function docSoDienThoai_CodeCu(bg: BanGhi): string | undefined {
  return bg.soDienThoaiCu;
}
function docSoDienThoai_CodeMoi(bg: BanGhi): string | undefined {
  return bg.soDienThoaiMoi;
}

function chuyenDuLieuCu(dsBanGhi: BanGhi[]): void {
  for (const bg of dsBanGhi) {
    if (bg.soDienThoaiMoi === undefined && bg.soDienThoaiCu !== undefined) {
      bg.soDienThoaiMoi = bg.soDienThoaiCu;
    }
  }
}

function xoaCotCu(bg: BanGhi): void {
  bg.soDienThoaiCu = undefined;
}

// ban ghi CU, tao TU TRUOC khi code dua-ghi duoc trien khai -- cot moi
// dang RONG, chua he duoc ghi
const bgCu = taoBanGhi("nguoi-cu", "0922-222-222");
console.log("ban ghi CU, truoc khi chuyen du lieu, code moi doc:", docSoDienThoai_CodeMoi(bgCu));

chuyenDuLieuCu([bgCu]);
console.log("SAU khi chuyen du lieu (backfill), code moi doc:", docSoDienThoai_CodeMoi(bgCu));
console.log("code cu VAN doc duoc (chua CONTRACT):", docSoDienThoai_CodeCu(bgCu));

// dung luc nay moi CONTRACT: xoa cot cu, vi du lieu da chuyen xong VA
// khong con code nao doc cot cu nua
xoaCotCu(bgCu);
console.log("SAU KHI CONTRACT, code moi VAN doc dung:", docSoDienThoai_CodeMoi(bgCu));
console.log("SAU KHI CONTRACT, code cu (neu VAN con chay) se doc:", docSoDienThoai_CodeCu(bgCu));

// nguy hiem: mot ban ghi CHUA TUNG duoc chuyen du lieu, ma CONTRACT lai chay som
const bgBoQuen = taoBanGhi("nguoi-bo-quen", "0933-333-333");
xoaCotCu(bgBoQuen); // contract truoc khi chuyen du lieu -- SAI THU TU
console.log("ban ghi BI BO QUEN, contract som, code moi doc:", docSoDienThoai_CodeMoi(bgBoQuen));
console.log("ban ghi BI BO QUEN, contract som, code cu doc:", docSoDienThoai_CodeCu(bgBoQuen));
```

```text title=readonly
ban ghi CU, truoc khi chuyen du lieu, code moi doc: undefined
SAU khi chuyen du lieu (backfill), code moi doc: 0922-222-222
code cu VAN doc duoc (chua CONTRACT): 0922-222-222
SAU KHI CONTRACT, code moi VAN doc dung: 0922-222-222
SAU KHI CONTRACT, code cu (neu VAN con chay) se doc: undefined
ban ghi BI BO QUEN, contract som, code moi doc: undefined
ban ghi BI BO QUEN, contract som, code cu doc: undefined
```

`bgCu` an toàn xuyên suốt: migrate TRƯỚC, contract SAU, code mới luôn
đọc đúng. `bgBoQuen` thì KHÔNG — `xoaCotCu` chạy TRƯỚC khi có cơ hội
migrate, VÀ vì cột mới CHƯA từng được ghi, kết quả LÀ cả hai cột đều
`undefined`. Đây không phải "code cũ bị lỗi" — đây LÀ dữ liệu điện
thoại của `"nguoi-bo-quen"` biến MẤT hoàn toàn, không thể phục hồi từ
CHÍNH bản ghi này nữa.
::::

::::predict{#doan-doc-linh-hoat commitOnce}
Một bản ghi `x` vừa qua bước EXPAND — cột mới ĐÃ tồn tại nhưng chưa
từng được ghi hay migrate (`soDienThoaiMoi` vẫn `undefined`), cột cũ
vẫn mang giá trị gốc. Một hàm đọc LINH HOẠT, ưu tiên cột mới VÀ chỉ
fallback về cột cũ khi cột mới `undefined`, được gọi trên `x` — kết
quả LÀ gì?

:::opt{correct}
Giá trị Ở cột CŨ — vì cột mới đang `undefined`, hàm ĐỌC linh hoạt rơi
xuống nhánh fallback VÀ trả về đúng giá trị đã có sẵn TỪ trước expand
:::
:::opt
`undefined` — bản ghi ĐÃ qua expand nghĩa LÀ nó đã sẵn sàng Ở "trạng
thái mới", nên đọc LINH HOẠT nên ưu tiên trả kết quả từ cột mới, dù
đó LÀ rỗng
::why
Nhầm "đã expand" VỚI "đã có dữ liệu Ở cột mới" — nhưng expand chỉ
THÊM cột, KHÔNG tự động điền giá trị VÀO đó.

Chỗ lệch: một hàm đọc linh hoạt kiểm tra `soDienThoaiMoi !== undefined`
TRƯỚC — nếu đúng thì trả cột mới, còn KHÔNG thì fallback về cột cũ.
Ngay sau expand, chưa CÓ dual write LẪN chưa migrate, nên
`soDienThoaiMoi` vẫn LÀ `undefined` — điều kiện SAI, hàm rơi xuống
nhánh fallback VÀ trả về giá trị Ở cột cũ, không phải `undefined`.
::
:::
::::

::::code{#viet_doc_linh_hoat}
Viết `docLinhHoat` — hàm đọc BACKWARD-COMPATIBLE dùng trong giai đoạn
"deploy code đọc được cả cột cũ lẫn mới": ưu tiên `soDienThoaiMoi` nếu
nó CÓ giá trị, ngược lại fallback về `soDienThoaiCu`.

```typescript title=starter
interface BanGhi { id: string; soDienThoaiCu: string | undefined; soDienThoaiMoi: string | undefined; }

function taoBanGhi(id: string, soCu: string): BanGhi {
  return { id, soDienThoaiCu: soCu, soDienThoaiMoi: undefined };
}

function chuyenDuLieuCu(dsBanGhi: BanGhi[]): void {
  for (const bg of dsBanGhi) {
    if (bg.soDienThoaiMoi === undefined && bg.soDienThoaiCu !== undefined) {
      bg.soDienThoaiMoi = bg.soDienThoaiCu;
    }
  }
}

function docLinhHoat(bg: BanGhi): string | undefined {
  ___
}

const bgX = taoBanGhi("x", "0955-555-555");
console.log(docLinhHoat(bgX));
```

```typescript title=solution
interface BanGhi { id: string; soDienThoaiCu: string | undefined; soDienThoaiMoi: string | undefined; }

function taoBanGhi(id: string, soCu: string): BanGhi {
  return { id, soDienThoaiCu: soCu, soDienThoaiMoi: undefined };
}

function chuyenDuLieuCu(dsBanGhi: BanGhi[]): void {
  for (const bg of dsBanGhi) {
    if (bg.soDienThoaiMoi === undefined && bg.soDienThoaiCu !== undefined) {
      bg.soDienThoaiMoi = bg.soDienThoaiCu;
    }
  }
}

function docLinhHoat(bg: BanGhi): string | undefined {
  if (bg.soDienThoaiMoi !== undefined) return bg.soDienThoaiMoi;
  return bg.soDienThoaiCu;
}

const bgX = taoBanGhi("x", "0955-555-555");
console.log(docLinhHoat(bgX));
```

```typescript title=test
const chiCoCuT = taoBanGhi("chi-co-cu", "0900-111-222");
if (docLinhHoat(chiCoCuT) !== "0900-111-222") throw new Error("chi co cot cu, docLinhHoat phai fallback ve gia tri cu");

const daMigrateT = taoBanGhi("da-migrate", "0900-333-444");
chuyenDuLieuCu([daMigrateT]);
if (docLinhHoat(daMigrateT) !== "0900-333-444") throw new Error("sau khi migrate, ca hai cot deu co gia tri, docLinhHoat phai doc dung");

const chiCoMoiT: BanGhi = { id: "chi-co-moi", soDienThoaiCu: undefined, soDienThoaiMoi: "0900-666-777" };
if (docLinhHoat(chiCoMoiT) !== "0900-666-777") throw new Error("sau CONTRACT (cot cu da xoa), docLinhHoat phai UU TIEN doc cot moi");

const khongCoGiT: BanGhi = { id: "rong", soDienThoaiCu: undefined, soDienThoaiMoi: undefined };
if (docLinhHoat(khongCoGiT) !== undefined) throw new Error("khong co cot nao co gia tri thi phai tra ve undefined");

const capNhatMoiT: BanGhi = { id: "cap-nhat", soDienThoaiCu: "0900-CU", soDienThoaiMoi: "0900-MOI" };
if (docLinhHoat(capNhatMoiT) !== "0900-MOI") throw new Error("khi CA HAI cot deu co gia tri (dang dual-write), phai UU TIEN cot moi, khong phai cot cu");
```

:::hints
- kind: attention
  body: "Neu bg.soDienThoaiMoi khac undefined thi return no NGAY. Nguoc lai return bg.soDienThoaiCu -- mot dieu kien, mot fallback."
- kind: strategy
  body: "if (bg.soDienThoaiMoi !== undefined) return bg.soDienThoaiMoi; return bg.soDienThoaiCu;"
- kind: one-line
  body: "if (bg.soDienThoaiMoi !== undefined) return bg.soDienThoaiMoi; return bg.soDienThoaiCu;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "0955-555-555"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn bước, ĐÚNG thứ tự — dữ liệu không mất, code cũ không vỡ. Nhưng
migration an toàn Ở TẦNG dữ liệu chỉ LÀ một nửa câu chuyện. Nửa còn
lại: hệ thống có CHỊU nổi tải THẬT khi tất cả đã lên production chưa?
::::

::::reflect{#nghi-lai}
Bốn hàm — `ghiSoDienThoai_DuaGhi`, `chuyenDuLieuCu`, `docLinhHoat`,
`xoaCotCu` — không hề PHỨC tạp từng cái một. Cái khó nằm Ở THỨ TỰ:
đảo bất kỳ hai bước nào, hoặc bỏ qua một bước, VÀ dữ liệu THẬT bị mất
— như `bgBoQuen` đã cho thấy bằng số liệu, không phải bằng lời cảnh
báo suông. Expand-contract không phải LÀ một quy tắc trừu tượng; nó
LÀ cách duy nhất để giữ đúng bất biến "code cũ và code mới cùng chạy"
mà chính rolling deploy đã tạo ra.
::::

::::checkpoint{mastery=0.78}
::::
