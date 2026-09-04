---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.trang-thai-la-fold-cua-su-kien
title: "Trạng thái là fold: số dư không lưu, số dư được tính"
summary: "tinhTrangThai(nhatKy) = nhatKy.reduce(apDung, trangThaiBanDau()) -- so du mot vi KHONG ton tai nhu mot field luu san Ở dau ca, no LA ket qua cua viec fold (gap) toan bo su kien qua ham apDung THUAN; vi du 3 su kien (+100000, -30000, +5000) fold ra 75000, them 1 su kien (-20000) fold LAI tu dau ra 55000."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.fp.trang-thai-la-fold-cua-su-kien]
requires: [sd.fp.su-kien-bat-bien]
concepts: [sd.fp.trang-thai-la-fold-cua-su-kien]
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
Bài trước để ngỏ một câu hỏi: nếu số dư không được lưu Ở đâu, làm sao
biết một ví có bao nhiêu tiền? Câu trả lời không nằm Ở việc TÌM một
chỗ để lưu nó. Câu trả lời là: đừng lưu nó — hãy TÍNH nó, mỗi lần cần,
từ chính nhật ký.
::::

::::explain{#fold-tinh-trang-thai-tu-nhat-ky}
`apDung` là một hàm THUẦN nhận trạng thái hiện tại VÀ một sự kiện, trả
về trạng thái MỚI — nó không sửa `trangThai` truyền vào, nó trả về
một object khác. `tinhTrangThai` gọi `nhatKy.reduce(apDung,
trangThaiBanDau())` — gấp (fold) toàn bộ nhật ký qua `apDung`, bắt đầu
từ trạng thái ban đầu. Không có field `soDu` nào được lưu sẵn Ở bất cứ
đâu — mỗi lần gọi `tinhTrangThai`, số dư được tính LẠI từ đầu:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];

interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }

function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}

function tinhTrangThai(nhatKy: NhatKy): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

const nhatKy: NhatKy = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 30000 },
  { id: "e3", loai: "da_nap", soTien: 5000 },
];

console.log("KHONG co field soDu nao duoc luu san trong nhatKy");
console.log("soDu duoc TINH tu fold:", tinhTrangThai(nhatKy).soDu);

const nhatKy2: NhatKy = [...nhatKy, { id: "e4", loai: "da_tru", soTien: 20000 }];
console.log("sau khi them 1 su kien, tinh LAI tu dau:", tinhTrangThai(nhatKy2).soDu);
```

```text title=readonly
KHONG co field soDu nao duoc luu san trong nhatKy
soDu duoc TINH tu fold: 75000
sau khi them 1 su kien, tinh LAI tu dau: 55000
```

`nhatKy` không hề có field nào tên `soDu`. `tinhTrangThai(nhatKy)`
duyệt qua CẢ ba sự kiện (`+100000`, `-30000`, `+5000`) và cộng dồn ra
`75000` — con số này chỉ tồn tại Ở đầu ra của phép fold, không Ở đâu
khác. Thêm MỘT sự kiện vào nhật ký (`nhatKy2`) rồi gọi lại
`tinhTrangThai` — hàm fold LẠI toàn bộ bốn sự kiện từ đầu, ra `55000`.
Không có "cập nhật số dư" nào xảy ra — chỉ có "tính lại" mỗi lần.
::::

::::example{#trang-thai-tai-tung-thoi-diem}
Vì trạng thái LÀ kết quả của fold, ta có thể lấy trạng thái "tại thời
điểm bất kỳ" trong quá khứ — chỉ cần fold một PHẦN đầu của nhật ký,
không cần lưu riêng một "lịch sử số dư" nào:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function tinhTrangThai(nhatKy: NhatKy): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

const nhatKy: NhatKy = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 30000 },
  { id: "e3", loai: "da_nap", soTien: 5000 },
];

for (let i = 1; i <= nhatKy.length; i++) {
  const phanDau = nhatKy.slice(0, i);
  console.log("sau", i, "su kien dau, so du la", tinhTrangThai(phanDau).soDu);
}
```

```text title=readonly
sau 1 su kien dau, so du la 100000
sau 2 su kien dau, so du la 70000
sau 3 su kien dau, so du la 75000
```

Không có mảng "lịch sử số dư" nào được duy trì song song — mỗi dòng
Ở trên chỉ đơn giản là fold một ĐOẠN đầu khác nhau của CÙNG một nhật
ký. Số dư "tại thời điểm sau sự kiện thứ hai" (`70000`) không phải
một giá trị được lưu sẵn Ở đâu đó chờ tra cứu — nó là kết quả của
việc gọi `tinhTrangThai` trên đúng hai sự kiện đầu.
::::

::::predict{#doan-goi-lai-tinh-trang-thai commitOnce}
Tiếp tục từ đoạn Ở trên: `nhatKy` có `3` sự kiện, và `tinhTrangThai
(nhatKy).soDu` đã trả về `75000`. Gọi `tinhTrangThai(nhatKy)` một lần
NỮA, với đúng `nhatKy` đó, không thay đổi gì — lần gọi THỨ HAI này có
gì khác so với lần đầu?

:::opt{correct}
Không có gì khác — hàm chạy `nhatKy.reduce(apDung, trangThaiBanDau())`
lại từ đầu, TÍNH LẠI toàn bộ ba sự kiện một lần nữa, và ra đúng
`75000`; không có nơi nào lưu kết quả `75000` từ lần gọi trước để tái
sử dụng
:::
:::opt
Lần gọi thứ hai nhanh hơn — TypeScript tự động nhớ (ghi nhớ) kết quả
`75000` từ lần gọi trước, vì đầu vào `nhatKy` không đổi
::why
Nhầm hành vi của `tinhTrangThai` với một cơ chế ghi nhớ (memoization)
tự động — nhưng TypeScript/JavaScript không tự làm điều đó cho một
hàm thường; `reduce` luôn duyệt lại TOÀN bộ mảng mỗi lần được gọi.

Chỗ lệch: `tinhTrangThai` chỉ có một dòng —
`return nhatKy.reduce(apDung, trangThaiBanDau());`. Không có biến nào
Ở ngoài hàm lưu lại kết quả trước đó, và bản thân `reduce` không hề
biết "lần trước đã tính rồi". Mỗi lệnh gọi là một lần fold HOÀN TOÀN
độc lập, tốn công y hệt lần trước — đây chính là vấn đề bài sau sẽ
giải quyết.
::
:::
::::

::::code{#viet_ap_dung}
Hoàn thiện `apDung` — nếu sự kiện là `"da_nap"`, trả về trạng thái MỚI
với `soDu` được CỘNG thêm `soTien`; nếu là `"da_tru"`, trả về trạng
thái MỚI với `soDu` bị TRỪ đi `soTien`. Không được sửa `trangThai`
truyền vào.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }

function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  ___
}

function tinhTrangThai(nhatKy: NhatKy): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

const nkX: NhatKy = [
  { id: "x1", loai: "da_nap", soTien: 10000 },
  { id: "x2", loai: "da_tru", soTien: 4000 },
];
console.log(tinhTrangThai(nkX).soDu);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }

function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}

function tinhTrangThai(nhatKy: NhatKy): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

const nkX: NhatKy = [
  { id: "x1", loai: "da_nap", soTien: 10000 },
  { id: "x2", loai: "da_tru", soTien: 4000 },
];
console.log(tinhTrangThai(nkX).soDu);
```

```typescript title=test
const t1 = tinhTrangThai([]);
if (t1.soDu !== 0) throw new Error("nhat ky rong phai cho so du 0");

const t2 = tinhTrangThai([{ id: "a", loai: "da_nap", soTien: 500 }]);
if (t2.soDu !== 500) throw new Error("mot su kien da_nap phai CONG dung so tien");

const t3 = tinhTrangThai([
  { id: "a", loai: "da_nap", soTien: 500 },
  { id: "b", loai: "da_tru", soTien: 200 },
]);
if (t3.soDu !== 300) throw new Error("da_tru phai TRU dung so tien");

const tGoc: TrangThai = { soDu: 1000 };
const tKetQua = apDung(tGoc, { id: "c", loai: "da_nap", soTien: 100 });
if (tGoc.soDu !== 1000) throw new Error("apDung KHONG duoc sua doi truc tiep trangThai truyen vao (phai thuan)");
if (tKetQua.soDu !== 1100) throw new Error("ket qua tra ve phai la trang thai MOI voi so du da cong");
```

:::hints
- kind: attention
  body: "Kiem tra suKien.loai. Neu la 'da_nap', tra ve mot OBJECT MOI voi soDu duoc CONG them soTien. Neu la 'da_tru', tra ve mot object moi voi soDu bi TRU. Khong duoc gan lai truc tiep trangThai.soDu = ..."
- kind: strategy
  body: "if (suKien.loai === 'da_nap') return { soDu: trangThai.soDu + suKien.soTien }; return { soDu: trangThai.soDu - suKien.soTien };"
- kind: one-line
  body: "if (suKien.loai === \"da_nap\") return { soDu: trangThai.soDu + suKien.soTien }; return { soDu: trangThai.soDu - suKien.soTien };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "6000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số dư không lưu — số dư được TÍNH, mỗi lần, từ đầu. Nhưng nếu nhật ký
dài hàng nghìn sự kiện, fold lại TOÀN bộ mỗi lần cần biết số dư sẽ
ngày càng chậm đi. Có cách nào tránh làm lại việc đã làm không?
::::

::::reflect{#nghi-lai}
`apDung` không hề biết gì về "toàn bộ lịch sử" — nó chỉ biết CÁCH một
trạng thái biến đổi khi gặp MỘT sự kiện. `tinhTrangThai` mới là nơi
lặp qua nhật ký, gọi `apDung` liên tiếp. Tách hai việc này ra — "một
bước biến đổi trông như thế nào" và "gấp toàn bộ chuỗi bước lại" —
chính là chỗ khác biệt so với `apDungKhongKiemTra` kiểu OOP: Ở đây
không có "cập nhật tại chỗ" nào cả, chỉ có một chuỗi các phép biến đổi
THUẦN, gấp lại thành một giá trị.
::::

::::checkpoint{mastery=0.68}
::::
