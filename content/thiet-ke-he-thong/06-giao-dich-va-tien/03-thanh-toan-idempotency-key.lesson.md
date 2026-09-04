---
id: thiet-ke-he-thong.giao-dich-va-tien.thanh-toan-idempotency-key
title: "Idempotency key: gửi lại không nghĩa là trả tiền lại"
summary: "thanhToan(ht, khoaIdempotency, soTien) tra ve KET QUA da LUU neu khoa DA xu ly, chi xu ly THAT khi khoa CHUA tung gap -- lan 1 goi thanhToan(ht,'khoa-abc',150000) xu ly that, tra ve gd-1; lan 2 (mo phong network timeout khien client GUI LAI dung khoa 'khoa-abc') tra ve NGUYEN gd-1, soLanXuLyThat KHONG tang; goi lai 'khoa-abc' voi so tien SAI (999999) van tra ve so tien GOC 150000 da luu -- khong bi tru tien hai lan."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.thanh-toan-idempotency-key]
requires: [sd.dat-phong-huy-va-giu-cho]
concepts: [sd.thanh-toan-idempotency-key]
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
Phòng khách sạn rời khỏi màn hình — giờ tới TIỀN. Mạng KHÔNG đáng tin:
client gửi yêu cầu thanh toán, chờ phản hồi, mạng RỚT trước khi phản
hồi kịp về. Client, không biết server đã xử LÝ hay chưa, làm điều duy
nhất hợp lý: gửi LẠI. Nếu server xử lý y hệt lần trước — khách hàng bị
trừ tiền HAI lần.
::::

::::explain{#idempotency-key-cache-ket-qua}
`thanhToan` nhận một `khoaIdempotency` đi kèm mỗi yêu cầu. Nếu khoá đó
ĐÃ có kết quả lưu sẵn, hàm trả về NGUYÊN kết quả cũ — không chạm vào
`soLanXuLyThat`. Chỉ khi khoá HOÀN TOÀN mới, hàm mới thật sự xử lý VÀ
lưu kết quả LẠI theo khoá đó:

```typescript title=readonly
interface KetQuaThanhToan { idGiaoDich: string; soTien: number; trangThai: "thanh_cong"; }
interface HeThongThanhToan {
  ketQuaTheoKhoa: Map<string, KetQuaThanhToan>;
  soLanXuLyThat: number;
  boDemId: number;
}
function taoHeThongThanhToan(): HeThongThanhToan {
  return { ketQuaTheoKhoa: new Map(), soLanXuLyThat: 0, boDemId: 0 };
}

function thanhToan(ht: HeThongThanhToan, khoaIdempotency: string, soTien: number): KetQuaThanhToan {
  const daCo = ht.ketQuaTheoKhoa.get(khoaIdempotency);
  if (daCo !== undefined) return daCo;
  ht.soLanXuLyThat += 1;
  ht.boDemId += 1;
  const ketQua: KetQuaThanhToan = { idGiaoDich: "gd-" + ht.boDemId, soTien, trangThai: "thanh_cong" };
  ht.ketQuaTheoKhoa.set(khoaIdempotency, ketQua);
  return ketQua;
}

const ht = taoHeThongThanhToan();
const lan1 = thanhToan(ht, "khoa-abc", 150000);
console.log("lan 1 (xu ly that):", JSON.stringify(lan1));
console.log("so lan xu ly THAT sau lan 1:", ht.soLanXuLyThat);

// mo phong network timeout: client GUI LAI dung khoa do
const lan2 = thanhToan(ht, "khoa-abc", 150000);
console.log("lan 2 (gui lai SAU timeout, CUNG khoa):", JSON.stringify(lan2));
console.log("so lan xu ly THAT sau lan 2 (KHONG doi):", ht.soLanXuLyThat);
console.log("hai lan tra ve CUNG mot idGiaoDich?", lan1.idGiaoDich === lan2.idGiaoDich);
```

```text title=readonly
lan 1 (xu ly that): {"idGiaoDich":"gd-1","soTien":150000,"trangThai":"thanh_cong"}
so lan xu ly THAT sau lan 1: 1
lan 2 (gui lai SAU timeout, CUNG khoa): {"idGiaoDich":"gd-1","soTien":150000,"trangThai":"thanh_cong"}
so lan xu ly THAT sau lan 2 (KHONG doi): 1
hai lan tra ve CUNG mot idGiaoDich? true
```

`soLanXuLyThat` dừng lại Ở `1` dù `thanhToan` được gọi HAI lần — lần
gọi thứ hai tìm thấy `"khoa-abc"` ĐÃ có trong `ketQuaTheoKhoa`, trả về
NGUYÊN kết quả cũ (`gd-1`) mà không tăng bộ đếm, không tạo giao dịch
MỚI. Khách hàng chỉ bị tính tiền đúng MỘT lần, dù mạng rớt VÀ client
gửi lại.
::::

::::example{#khoa-la-toi-thuong}
Khoá idempotency LÀ căn cứ DUY nhất — một khoá MỚI luôn được xử lý
thật, còn gửi LẠI một khoá cũ VỚI số tiền khác đi vẫn trả về kết quả
đã lưu theo khoá đó, KHÔNG theo số tiền mới gửi lên:

```typescript title=readonly
interface KetQuaThanhToan { idGiaoDich: string; soTien: number; trangThai: "thanh_cong"; }
interface HeThongThanhToan {
  ketQuaTheoKhoa: Map<string, KetQuaThanhToan>;
  soLanXuLyThat: number;
  boDemId: number;
}
function taoHeThongThanhToan(): HeThongThanhToan {
  return { ketQuaTheoKhoa: new Map(), soLanXuLyThat: 0, boDemId: 0 };
}

function thanhToan(ht: HeThongThanhToan, khoaIdempotency: string, soTien: number): KetQuaThanhToan {
  const daCo = ht.ketQuaTheoKhoa.get(khoaIdempotency);
  if (daCo !== undefined) return daCo;
  ht.soLanXuLyThat += 1;
  ht.boDemId += 1;
  const ketQua: KetQuaThanhToan = { idGiaoDich: "gd-" + ht.boDemId, soTien, trangThai: "thanh_cong" };
  ht.ketQuaTheoKhoa.set(khoaIdempotency, ketQua);
  return ketQua;
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: "khoa-abc" da xu ly
// THAT 1 lan, ket qua gd-1 150000, soLanXuLyThat = 1
const ht = taoHeThongThanhToan();
thanhToan(ht, "khoa-abc", 150000);
thanhToan(ht, "khoa-abc", 150000);

const lanKhac = thanhToan(ht, "khoa-xyz", 90000);
console.log("khoa KHAC (khoa-xyz):", JSON.stringify(lanKhac));
console.log("so lan xu ly that sau khoa-xyz:", ht.soLanXuLyThat);

const lanSai = thanhToan(ht, "khoa-abc", 999999);
console.log("gui lai khoa-abc voi so tien SAI (999999):", JSON.stringify(lanSai));
console.log("so lan xu ly that KHONG doi:", ht.soLanXuLyThat);
```

```text title=readonly
khoa KHAC (khoa-xyz): {"idGiaoDich":"gd-2","soTien":90000,"trangThai":"thanh_cong"}
so lan xu ly that sau khoa-xyz: 2
gui lai khoa-abc voi so tien SAI (999999): {"idGiaoDich":"gd-1","soTien":150000,"trangThai":"thanh_cong"}
so lan xu ly that KHONG doi: 2
```

`"khoa-xyz"` chưa từng gặp — xử lý thật, tạo `gd-2`, tăng bộ đếm lên
`2`. Nhưng gửi LẠI `"khoa-abc"` với `999999` (một con số hoàn toàn
khác `150000` ban đầu) vẫn trả về `soTien: 150000` — giá trị GỐC đã
lưu. Hàm không hề đọc `soTien` mới gửi lên MỘT khi khoá đã có kết quả
— chính KHOÁ, không phải nội dung yêu cầu, LÀ căn cứ để quyết định "đã
xử lý chưa".
::::

::::predict{#doan-goi-lan-ba commitOnce}
Ngay SAU đoạn Ở trên, `ht.soLanXuLyThat` đang LÀ `2`. Gọi thêm
`thanhToan(ht, "khoa-abc", 150000)` một lần NỮA (lần thứ tư tính CẢ từ
đầu, với khoá `"khoa-abc"`) — `ht.soLanXuLyThat` SAU lệnh gọi đó LÀ
bao nhiêu?

:::opt{correct}
`2` — không đổi; `"khoa-abc"` ĐÃ có kết quả trong `ketQuaTheoKhoa` từ
lần xử lý thật ĐẦU tiên, mọi lần gọi SAU với đúng khoá đó chỉ đọc lại
kết quả cũ, không bao giờ chạm `ht.soLanXuLyThat += 1` thêm lần nào
nữa
:::
:::opt
`3` — mỗi lệnh gọi `thanhToan` LÀ một yêu cầu thanh toán, VÀ mỗi yêu
cầu nên được tính LÀ một lần xử lý, dù kết quả trả về giống hệt lần
trước
::why
Nhầm "gọi HÀM" VỚI "xử LÝ thật" — nhưng `thanhToan` chủ động tách hai
việc đó ra bằng nhánh `if (daCo !== undefined) return daCo;` NGAY từ
đầu thân hàm.

Chỗ lệch: dòng `ht.soLanXuLyThat += 1;` nằm SAU nhánh kiểm tra
`daCo`, nên chỉ chạy khi khoá CHƯA từng xử lý. `"khoa-abc"` đã được xử
lý thật Ở lần gọi rất ĐẦU (tạo `gd-1`) — mọi lần gọi SAU, dù thêm bao
nhiêu lần nữa, đều dừng lại NGAY tại `return daCo;`, không bao giờ
chạm dòng tăng bộ đếm.
::
:::
::::

::::code{#viet_thanh_toan}
Hoàn thiện `thanhToan` — nếu `khoaIdempotency` ĐÃ có kết quả lưu sẵn,
trả về NGUYÊN kết quả đó. Ngược lại, xử lý THẬT: tăng `soLanXuLyThat`
VÀ `boDemId`, tạo `KetQuaThanhToan` mới với `idGiaoDich` LÀ `"gd-" +
boDemId`, LƯU kết quả đó vào `ketQuaTheoKhoa` theo khoá, RỒI trả về.

```typescript title=starter
interface KetQuaThanhToan { idGiaoDich: string; soTien: number; trangThai: "thanh_cong"; }
interface HeThongThanhToan {
  ketQuaTheoKhoa: Map<string, KetQuaThanhToan>;
  soLanXuLyThat: number;
  boDemId: number;
}
function taoHeThongThanhToan(): HeThongThanhToan {
  return { ketQuaTheoKhoa: new Map(), soLanXuLyThat: 0, boDemId: 0 };
}

function thanhToan(ht: HeThongThanhToan, khoaIdempotency: string, soTien: number): KetQuaThanhToan {
  ___
}

const htX = taoHeThongThanhToan();
const p1 = thanhToan(htX, "k1", 500);
const p2 = thanhToan(htX, "k1", 500);
console.log(p1.idGiaoDich === p2.idGiaoDich, htX.soLanXuLyThat);
```

```typescript title=solution
interface KetQuaThanhToan { idGiaoDich: string; soTien: number; trangThai: "thanh_cong"; }
interface HeThongThanhToan {
  ketQuaTheoKhoa: Map<string, KetQuaThanhToan>;
  soLanXuLyThat: number;
  boDemId: number;
}
function taoHeThongThanhToan(): HeThongThanhToan {
  return { ketQuaTheoKhoa: new Map(), soLanXuLyThat: 0, boDemId: 0 };
}

function thanhToan(ht: HeThongThanhToan, khoaIdempotency: string, soTien: number): KetQuaThanhToan {
  const daCo = ht.ketQuaTheoKhoa.get(khoaIdempotency);
  if (daCo !== undefined) return daCo;
  ht.soLanXuLyThat += 1;
  ht.boDemId += 1;
  const ketQua: KetQuaThanhToan = { idGiaoDich: "gd-" + ht.boDemId, soTien, trangThai: "thanh_cong" };
  ht.ketQuaTheoKhoa.set(khoaIdempotency, ketQua);
  return ketQua;
}

const htX = taoHeThongThanhToan();
const p1 = thanhToan(htX, "k1", 500);
const p2 = thanhToan(htX, "k1", 500);
console.log(p1.idGiaoDich === p2.idGiaoDich, htX.soLanXuLyThat);
```

```typescript title=test
const htT = taoHeThongThanhToan();
const a1 = thanhToan(htT, "key-1", 200000);
const a2 = thanhToan(htT, "key-1", 200000);
if (a1.idGiaoDich !== a2.idGiaoDich) throw new Error("cung khoa phai tra ve CUNG idGiaoDich");
const soLan1 = htT.soLanXuLyThat;
if (soLan1 !== 1) throw new Error("cung khoa goi 2 lan CHI duoc xu ly THAT 1 lan");

const a3 = thanhToan(htT, "key-2", 300000);
if (a3.idGiaoDich === a1.idGiaoDich) throw new Error("khoa KHAC phai tao giao dich MOI, id khac");
const soLan2 = htT.soLanXuLyThat;
if (soLan2 !== 2) throw new Error("khoa moi phai tang so lan xu ly that len 2");

const a4 = thanhToan(htT, "key-1", 999999999);
if (a4.soTien !== 200000) throw new Error("gui lai key-1 voi so tien KHAC van phai tra ve so tien GOC da luu (200000), khong xu ly lai");
if (a4.idGiaoDich !== a1.idGiaoDich) throw new Error("gui lai key-1 phai tra ve DUNG idGiaoDich cu");
const soLan3 = htT.soLanXuLyThat;
if (soLan3 !== 2) throw new Error("gui lai key-1 KHONG duoc lam tang so lan xu ly that");

if (a1.trangThai !== "thanh_cong") throw new Error("trang thai phai la thanh_cong");
```

:::hints
- kind: attention
  body: "Kiem tra ht.ketQuaTheoKhoa.get(khoaIdempotency) TRUOC. Neu khac undefined, return NGAY ket qua do. Chi khi undefined moi tang soLanXuLyThat, tang boDemId, tao ketQua moi, luu vao map, roi return."
- kind: strategy
  body: "const daCo = ht.ketQuaTheoKhoa.get(khoaIdempotency); if (daCo !== undefined) return daCo; ht.soLanXuLyThat += 1; ht.boDemId += 1; const ketQua = { idGiaoDich: 'gd-' + ht.boDemId, soTien, trangThai: 'thanh_cong' as const }; ht.ketQuaTheoKhoa.set(khoaIdempotency, ketQua); return ketQua;"
- kind: one-line
  body: "const daCo = ht.ketQuaTheoKhoa.get(khoaIdempotency); if (daCo !== undefined) return daCo; ht.soLanXuLyThat += 1; ht.boDemId += 1; const ketQua: KetQuaThanhToan = { idGiaoDich: \"gd-\" + ht.boDemId, soTien, trangThai: \"thanh_cong\" }; ht.ketQuaTheoKhoa.set(khoaIdempotency, ketQua); return ketQua;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gửi lại bao nhiêu lần cũng chỉ xử lý ĐÚNG một lần — miễn khoá không
đổi. Nhưng "xử lý thanh toán" thật ra LÀ đưa tiền từ chỗ này sang chỗ
khác. Làm sao chứng minh không có đồng nào tự SINH ra hay MẤT đi giữa
đường?
::::

::::reflect{#nghi-lai}
`thanhToan` không hề "kiểm tra rồi mới quyết định có xử lý lại hay
không" theo nghĩa so sánh NỘI dung yêu cầu — nó chỉ nhìn vào MỘT thứ
DUY nhất: khoá NÀY đã có kết quả lưu sẵn CHƯA. Idempotency key biến
một thao tác "gửi càng nhiều lần càng nguy hiểm" thành một thao tác
"gửi bao nhiêu lần cũng an toàn NHƯ nhau" — không phải vì server đoán
được ý định của client, mà vì client TỰ mang theo bằng chứng (khoá) về
việc "đây LÀ cùng một yêu cầu".
::::

::::checkpoint{mastery=0.70}
::::
