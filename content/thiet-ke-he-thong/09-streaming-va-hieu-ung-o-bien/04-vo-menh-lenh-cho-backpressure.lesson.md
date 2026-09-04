---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.vo-menh-lenh-cho-backpressure
title: "Vỏ mệnh lệnh cho backpressure: ai đọc tín hiệu và THỰC SỰ chặn"
summary: "xuLySuKienDen(kho, suKienMoi) la VO (mutate kho.buffer): goi quyetDinhNhanSuKien (loi thuan bai 3) roi HANH DONG theo tin hieu do -- them vao buffer neu 'nhan', tang soLanTuChoi neu 'tu_choi_tam', tang soLanChanLai neu 'chan_lai'; tai hien dung mo hinh loi thuan/vo menh lenh cua quest 'Nhat ky bat bien va fold' bai 6-7, ap dung vao streaming."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.vo-menh-lenh-cho-backpressure]
requires: [sd.fp.backpressure-la-quyet-dinh-thuan]
concepts: [sd.fp.vo-menh-lenh-cho-backpressure]
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
`quyetDinhNhanSuKien` (bài trước) trả về một tín hiệu — nhưng không hề
chạm vào buffer thật nào. Không ai gọi nó, không ai lưu kết quả, không
ai thêm sự kiện vào đâu cả. Cần một lớp NGOÀI đứng ra đọc tín hiệu đó
VÀ biến nó thành hành động: thêm vào buffer, hoặc từ chối thật sự.
::::

::::explain{#vo-doc-tin-hieu-va-hanh-dong}
`xuLySuKienDen` là "vỏ mệnh lệnh": nó gọi `quyetDinhNhanSuKien` (lõi
thuần) với kích thước buffer HIỆN tại, rồi hành động ĐÚNG theo tín
hiệu trả về — thêm `suKienMoi` vào `kho.buffer` nếu `"nhan"`, tăng
`kho.soLanTuChoi` nếu `"tu_choi_tam"`, tăng `kho.soLanChanLai` nếu
`"chan_lai"`. Vỏ không hề tự tính tỉ lệ hay tự so sánh ngưỡng — toàn
bộ logic đó nằm Ở `quyetDinhNhanSuKien`:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}

interface KhoBuffer { buffer: SuKien[]; gioiHanBuffer: number; soLanTuChoi: number; soLanChanLai: number; }
function taoKhoBuffer(gioiHanBuffer: number): KhoBuffer {
  return { buffer: [], gioiHanBuffer, soLanTuChoi: 0, soLanChanLai: 0 };
}

interface KetQuaXuLySuKienDen { tinHieu: TinHieuBackpressure; daNhan: boolean; }

function xuLySuKienDen(kho: KhoBuffer, suKienMoi: SuKien): KetQuaXuLySuKienDen {
  const tinHieu = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer);
  if (tinHieu === "chan_lai") {
    kho.soLanChanLai += 1;
    return { tinHieu, daNhan: false };
  }
  if (tinHieu === "tu_choi_tam") {
    kho.soLanTuChoi += 1;
    return { tinHieu, daNhan: false };
  }
  kho.buffer.push(suKienMoi);
  return { tinHieu, daNhan: true };
}

const suKien = (id: string): SuKien => ({ id, nguon: "edge-1", thoiDiem: 0, soLuong: 1 });

const kho = taoKhoBuffer(5);
for (let i = 1; i <= 6; i++) {
  const kq = xuLySuKienDen(kho, suKien("e" + i));
  console.log("su kien e" + i + ": buffer TRUOC =", kho.buffer.length - (kq.daNhan ? 1 : 0), "-> tinHieu =", kq.tinHieu, ", daNhan =", kq.daNhan, ", buffer SAU =", kho.buffer.length);
}
console.log("tong so lan tu choi tam:", kho.soLanTuChoi);
console.log("tong so lan chan lai:", kho.soLanChanLai);

console.log("--- mo phong buffer da day SAN (vd: burst tu nhieu nguon) ---");
const khoDay: KhoBuffer = { buffer: [suKien("b1"), suKien("b2"), suKien("b3"), suKien("b4"), suKien("b5")], gioiHanBuffer: 5, soLanTuChoi: 0, soLanChanLai: 0 };
const kqChanLai = xuLySuKienDen(khoDay, suKien("b6"));
console.log("buffer da o muc gioi han (5/5), su kien moi den:", JSON.stringify(kqChanLai));
console.log("buffer sau do van la", khoDay.buffer.length, "(khong tang, bi CHAN THAT boi vo)");
```

```text title=readonly
su kien e1: buffer TRUOC = 0 -> tinHieu = nhan , daNhan = true , buffer SAU = 1
su kien e2: buffer TRUOC = 1 -> tinHieu = nhan , daNhan = true , buffer SAU = 2
su kien e3: buffer TRUOC = 2 -> tinHieu = nhan , daNhan = true , buffer SAU = 3
su kien e4: buffer TRUOC = 3 -> tinHieu = nhan , daNhan = true , buffer SAU = 4
su kien e5: buffer TRUOC = 4 -> tinHieu = tu_choi_tam , daNhan = false , buffer SAU = 4
su kien e6: buffer TRUOC = 4 -> tinHieu = tu_choi_tam , daNhan = false , buffer SAU = 4
tong so lan tu choi tam: 2
tong so lan chan lai: 0
--- mo phong buffer da day SAN (vd: burst tu nhieu nguon) ---
buffer da o muc gioi han (5/5), su kien moi den: {"tinHieu":"chan_lai","daNhan":false}
buffer sau do van la 5 (khong tang, bi CHAN THAT boi vo)
```

Bốn sự kiện đầu (`e1`-`e4`) được chấp nhận, đưa buffer lên `4`. Từ đó,
`4/5 = 0.8` khiến `quyetDinhNhanSuKien` LUÔN trả về `"tu_choi_tam"` —
buffer bị "khoá" Ở mức `4`, không bao giờ chạm `5` qua đường tăng dần
bình thường. Đây chính LÀ mục đích của tín hiệu `"tu_choi_tam"`: giữ
buffer dưới ngưỡng đầy, KHÔNG để nó chạm đáy. Tình huống `khoDay` mô
phỏng một burst đã lấp ĐẦY buffer qua đường khác (nhiều nguồn ghi đồng
thời) — lúc đó `xuLySuKienDen` mới thực sự trả về `"chan_lai"`.
::::

::::example{#doan-truoc-tin-hieu-khong-can-qua-vo}
Vì `xuLySuKienDen` không thêm logic quyết định nào của riêng nó, gọi
`quyetDinhNhanSuKien` TRỰC TIẾP với đúng `kho.buffer.length` hiện tại
phải cho ra ĐÚNG tín hiệu mà vỏ sẽ đưa ra Ở lần gọi kế tiếp:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface KhoBuffer { buffer: SuKien[]; gioiHanBuffer: number; soLanTuChoi: number; soLanChanLai: number; }
function taoKhoBuffer(gioiHanBuffer: number): KhoBuffer {
  return { buffer: [], gioiHanBuffer, soLanTuChoi: 0, soLanChanLai: 0 };
}
interface KetQuaXuLySuKienDen { tinHieu: TinHieuBackpressure; daNhan: boolean; }
function xuLySuKienDen(kho: KhoBuffer, suKienMoi: SuKien): KetQuaXuLySuKienDen {
  const tinHieu = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer);
  if (tinHieu === "chan_lai") {
    kho.soLanChanLai += 1;
    return { tinHieu, daNhan: false };
  }
  if (tinHieu === "tu_choi_tam") {
    kho.soLanTuChoi += 1;
    return { tinHieu, daNhan: false };
  }
  kho.buffer.push(suKienMoi);
  return { tinHieu, daNhan: true };
}
const suKien = (id: string): SuKien => ({ id, nguon: "edge-1", thoiDiem: 0, soLuong: 1 });

const kho = taoKhoBuffer(4);
xuLySuKienDen(kho, suKien("p1"));
xuLySuKienDen(kho, suKien("p2"));
xuLySuKienDen(kho, suKien("p3"));
console.log("buffer sau 3 su kien:", kho.buffer.length);

const tinHieuDoanTruoc = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer);
console.log("goi TRUC TIEP quyetDinhNhanSuKien voi kho.buffer.length hien tai:", tinHieuDoanTruoc);

const ketQuaThat = xuLySuKienDen(kho, suKien("p4"));
console.log("goi qua vo xuLySuKienDen:", ketQuaThat.tinHieu);
console.log("hai duong (doan truoc vs qua vo) khop nhau?", tinHieuDoanTruoc === ketQuaThat.tinHieu);

console.log("--- doi NGUONG (0.8) ma khong sua xuLySuKienDen, chi doi ham duoc goi ben trong ---");
function quyetDinhNhanSuKienNghiemNgat(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.5) return "tu_choi_tam";
  return "nhan";
}
function xuLySuKienDenNghiemNgat(kho: KhoBuffer, suKienMoi: SuKien): KetQuaXuLySuKienDen {
  const tinHieu = quyetDinhNhanSuKienNghiemNgat(kho.buffer.length, kho.gioiHanBuffer);
  if (tinHieu === "chan_lai") {
    kho.soLanChanLai += 1;
    return { tinHieu, daNhan: false };
  }
  if (tinHieu === "tu_choi_tam") {
    kho.soLanTuChoi += 1;
    return { tinHieu, daNhan: false };
  }
  kho.buffer.push(suKienMoi);
  return { tinHieu, daNhan: true };
}
const khoNghiemNgat = taoKhoBuffer(10);
for (let i = 1; i <= 6; i++) xuLySuKienDenNghiemNgat(khoNghiemNgat, suKien("n" + i));
console.log("voi nguong 0.5 (thay vi 0.8), buffer dung o muc:", khoNghiemNgat.buffer.length, "/ 10");
```

```text title=readonly
buffer sau 3 su kien: 3
goi TRUC TIEP quyetDinhNhanSuKien voi kho.buffer.length hien tai: nhan
goi qua vo xuLySuKienDen: nhan
hai duong (doan truoc vs qua vo) khop nhau? true
--- doi NGUONG (0.8) ma khong sua xuLySuKienDen, chi doi ham duoc goi ben trong ---
voi nguong 0.5 (thay vi 0.8), buffer dung o muc: 5 / 10
```

Gọi `quyetDinhNhanSuKien` trực tiếp với `kho.buffer.length` (KHÔNG qua
`xuLySuKienDen`) cho ra ĐÚNG cùng tín hiệu (`"nhan"`) như gọi qua toàn
bộ vỏ — vì vỏ không thêm hay bớt logic nào. Đổi ngưỡng từ `0.8` xuống
`0.5` (`quyetDinhNhanSuKienNghiemNgat`) khiến buffer "khoá" Ở mức thấp
hơn (`5` thay vì gần `10`) — VÀ `xuLySuKienDenNghiemNgat` không cần
viết LẠI phần điều phối (đọc tín hiệu, thêm vào buffer, tăng bộ đếm),
chỉ đổi ĐÚNG hàm quyết định được gọi bên trong.
::::

::::predict{#doan-tiep-tuc-tu-choi-tam commitOnce}
Tiếp tục từ đoạn `explain`: sau sáu sự kiện, `kho.buffer.length` là
`4` VÀ `kho.soLanTuChoi` là `2`. Gọi thêm
`xuLySuKienDen(kho, suKien("e7"))` một lần NỮA — `kho.buffer.length`
VÀ `kho.soLanTuChoi` SAU lệnh gọi đó là bao nhiêu?

:::opt{correct}
`kho.buffer.length` VẪN là `4`, VÀ `kho.soLanTuChoi` trở thành `3` —
`quyetDinhNhanSuKien(4, 5)` luôn trả về `"tu_choi_tam"` cho ĐẾN khi
`kho.buffer.length` đổi (mà nó không đổi, vì sự kiện bị từ chối); vỏ
chỉ tăng bộ đếm từ chối, không hề "nhượng bộ" sau nhiều lần từ chối
liên tiếp
:::
:::opt
`kho.buffer.length` trở thành `5` — sau NHIỀU lần từ chối liên tiếp
(Ở đây đã là lần thứ ba), hệ thống nên nhượng bộ VÀ chấp nhận sự kiện
để tránh từ chối quá lâu
::why
Nhầm "backpressure có bộ nhớ về số lần đã từ chối trước đó, VÀ tự nới
lỏng theo thời gian" — nhưng `quyetDinhNhanSuKien` không hề nhận
`kho.soLanTuChoi` làm tham số; nó CHỈ nhìn vào `kichThuocBufferHienTai`
VÀ `gioiHanBuffer` tại đúng thời điểm được gọi.

Chỗ lệch: `xuLySuKienDen` gọi `quyetDinhNhanSuKien(kho.buffer.length,
kho.gioiHanBuffer)` — hai tham số này không đổi qua các lần gọi liên
tiếp (buffer luôn Ở mức `4`, giới hạn luôn Ở mức `5`), NÊN kết quả
LUÔN là `"tu_choi_tam"`, bất kể đã từ chối bao nhiêu lần trước đó. Một
hàm thuần không có "trí nhớ" về lịch sử gọi hàm của chính nó — mỗi lần
gọi là một phép tính ĐỘC LẬP dựa hoàn toàn vào tham số truyền vào lần
đó.
::
:::
::::

::::code{#viet_xu_ly_su_kien_den}
Hoàn thiện `xuLySuKienDen` — gọi `quyetDinhNhanSuKien(kho.buffer.length,
kho.gioiHanBuffer)` để lấy `tinHieu`. Nếu là `"chan_lai"`, tăng
`kho.soLanChanLai` rồi trả về `{ tinHieu, daNhan: false }`. Nếu là
`"tu_choi_tam"`, tăng `kho.soLanTuChoi` rồi trả về `{ tinHieu, daNhan:
false }`. Ngược lại (`"nhan"`), thêm `suKienMoi` vào `kho.buffer` rồi
trả về `{ tinHieu, daNhan: true }`.

```typescript title=starter
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface KhoBuffer { buffer: SuKien[]; gioiHanBuffer: number; soLanTuChoi: number; soLanChanLai: number; }
function taoKhoBuffer(gioiHanBuffer: number): KhoBuffer {
  return { buffer: [], gioiHanBuffer, soLanTuChoi: 0, soLanChanLai: 0 };
}
interface KetQuaXuLySuKienDen { tinHieu: TinHieuBackpressure; daNhan: boolean; }

function xuLySuKienDen(kho: KhoBuffer, suKienMoi: SuKien): KetQuaXuLySuKienDen {
  ___
}

const khoX = taoKhoBuffer(10);
xuLySuKienDen(khoX, { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 });
xuLySuKienDen(khoX, { id: "x2", nguon: "e", thoiDiem: 0, soLuong: 1 });
console.log(khoX.buffer.length, khoX.soLanTuChoi, khoX.soLanChanLai);
```

```typescript title=solution
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}
interface KhoBuffer { buffer: SuKien[]; gioiHanBuffer: number; soLanTuChoi: number; soLanChanLai: number; }
function taoKhoBuffer(gioiHanBuffer: number): KhoBuffer {
  return { buffer: [], gioiHanBuffer, soLanTuChoi: 0, soLanChanLai: 0 };
}
interface KetQuaXuLySuKienDen { tinHieu: TinHieuBackpressure; daNhan: boolean; }

function xuLySuKienDen(kho: KhoBuffer, suKienMoi: SuKien): KetQuaXuLySuKienDen {
  const tinHieu = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer);
  if (tinHieu === "chan_lai") {
    kho.soLanChanLai += 1;
    return { tinHieu, daNhan: false };
  }
  if (tinHieu === "tu_choi_tam") {
    kho.soLanTuChoi += 1;
    return { tinHieu, daNhan: false };
  }
  kho.buffer.push(suKienMoi);
  return { tinHieu, daNhan: true };
}

const khoX = taoKhoBuffer(10);
xuLySuKienDen(khoX, { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 });
xuLySuKienDen(khoX, { id: "x2", nguon: "e", thoiDiem: 0, soLuong: 1 });
console.log(khoX.buffer.length, khoX.soLanTuChoi, khoX.soLanChanLai);
```

```typescript title=test
const tSuKien = (id: string): SuKien => ({ id, nguon: "e", thoiDiem: 0, soLuong: 1 });

const tKho1 = taoKhoBuffer(5);
for (let i = 0; i < 4; i++) {
  const r = xuLySuKienDen(tKho1, tSuKien("a" + i));
  if (r.daNhan !== true) throw new Error("bon su kien dau (buffer 0..3, ti le duoi 0.8) phai duoc CHAP NHAN");
}
if (tKho1.buffer.length !== 4) throw new Error("buffer phai co 4 phan tu sau 4 lan chap nhan");

const tR5 = xuLySuKienDen(tKho1, tSuKien("a4"));
if (tR5.daNhan !== false) throw new Error("su kien thu 5 (buffer=4/5=0.8) phai bi TU CHOI TAM, khong duoc them vao buffer");
if (tR5.tinHieu !== "tu_choi_tam") throw new Error("tin hieu phai la tu_choi_tam");
if (tKho1.buffer.length !== 4) throw new Error("buffer KHONG duoc tang khi bi tu choi tam");
if (tKho1.soLanTuChoi !== 1) throw new Error("soLanTuChoi phai tang len 1");

const tKho2: KhoBuffer = { buffer: [tSuKien("b1"), tSuKien("b2"), tSuKien("b3")], gioiHanBuffer: 3, soLanTuChoi: 0, soLanChanLai: 0 };
const tR6 = xuLySuKienDen(tKho2, tSuKien("b4"));
if (tR6.tinHieu !== "chan_lai") throw new Error("buffer da DAY (3/3) phai la chan_lai");
if (tR6.daNhan !== false) throw new Error("su kien bi chan lai KHONG duoc chap nhan");
if (tKho2.buffer.length !== 3) throw new Error("buffer KHONG duoc tang khi bi chan lai");
if (tKho2.soLanChanLai !== 1) throw new Error("soLanChanLai phai tang len 1");

const tKho3 = taoKhoBuffer(100);
const tR7 = xuLySuKienDen(tKho3, tSuKien("c1"));
if (tR7.tinHieu !== "nhan" || tR7.daNhan !== true) throw new Error("buffer con nhieu cho phai la nhan");
if (tKho3.buffer.length !== 1) throw new Error("su kien duoc chap nhan phai duoc THEM vao buffer");
```

:::hints
- kind: attention
  body: "Goi tinHieu = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer). Dung switch/if tren tinHieu: 'chan_lai' -> kho.soLanChanLai += 1, tra ve daNhan:false; 'tu_choi_tam' -> kho.soLanTuChoi += 1, tra ve daNhan:false; con lai ('nhan') -> kho.buffer.push(suKienMoi), tra ve daNhan:true."
- kind: strategy
  body: "const tinHieu = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer); if (tinHieu === 'chan_lai') { kho.soLanChanLai += 1; return { tinHieu, daNhan: false }; } if (tinHieu === 'tu_choi_tam') { kho.soLanTuChoi += 1; return { tinHieu, daNhan: false }; } kho.buffer.push(suKienMoi); return { tinHieu, daNhan: true };"
- kind: one-line
  body: "const tinHieu = quyetDinhNhanSuKien(kho.buffer.length, kho.gioiHanBuffer); if (tinHieu === \"chan_lai\") { kho.soLanChanLai += 1; return { tinHieu, daNhan: false }; } if (tinHieu === \"tu_choi_tam\") { kho.soLanTuChoi += 1; return { tinHieu, daNhan: false }; } kho.buffer.push(suKienMoi); return { tinHieu, daNhan: true };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 0 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lõi thuần quyết định tín hiệu, vỏ đọc tín hiệu VÀ thực sự chặn — mỗi
sự kiện đến đều được xử lý ĐÚNG theo tín hiệu Ở đúng thời điểm đó.
Nhưng buffer là một mảng trong bộ nhớ — làm sao dữ liệu thoát ra khỏi
hệ thống, ra thế giới THẬT bên ngoài?
::::

::::reflect{#nghi-lai}
`xuLySuKienDen` là nơi DUY NHẤT trong bài học này được phép mutate
`kho` — VÀ nó chỉ làm đúng việc đó: hỏi lõi, rồi hành động theo câu
trả lời. Không một dòng nào trong nó tính tỉ lệ hay so sánh ngưỡng —
kiến thức đó nằm TRỌN Ở `quyetDinhNhanSuKien`. Đây là đúng mô hình
"lõi thuần, vỏ mệnh lệnh" đã học Ở quest "Nhật ký bất biến và fold",
tái hiện Ở một bài toán khác hẳn: không phải "chấp nhận hay từ chối
một giao dịch", mà là "chấp nhận hay từ chối một sự kiện đang chảy
vào hệ thống".
::::

::::checkpoint{mastery=0.78}
::::
