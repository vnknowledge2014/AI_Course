---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.boss-dinh-danh-va-toc-do
title: "BOSS — API Gateway: giới hạn tốc độ + định danh"
summary: "xuLyRequestGateway ráp token bucket (bài 1-2, refill theo thời gian) làm cổng vào, chỉ gọi sinhSnowflakeId (bài 8-9) NEU rate limiter cho qua -- request bị từ chối không hề chạm tới bộ sinh ID (sequence không tăng, id=undefined). Gateway 2 token, tốc độ 1/giây: 2 request đầu được cấp ID tăng dần, 2 request sau bị từ chối (id=undefined); sau 3000ms nạp lại, request thứ 5 được cấp ID mới LỚN HƠN mọi ID trước -- thứ tự thời gian bảo toàn xuyên suốt cả hai hệ thống con."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.boss-dinh-danh-va-toc-do]
requires: [sd.snowflake-sequence-trong-mili-giay]
concepts: [sd.boss-dinh-danh-va-toc-do]
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
Chín bài — bốn thuật toán giới hạn tốc độ (bài 1-7), Snowflake ID
đóng gói bit VÀ quản sequence (bài 8-9). Giờ ráp CẢ hai mảnh VÀO một
cổng API (gateway) nhỏ DUY nhất.
::::

::::explain{#rap_gateway}
Một request đi qua ĐÚNG hai trạm: TRẠM một — token bucket (bài 1-2)
quyết định request CÓ được QUA hay không. Trạm hai — CHỈ khi trạm một
cho QUA, `sinhSnowflakeId` (bài 8-9) mới được GỌI để cấp một định
danh. Request bị TỪ chối Ở trạm một KHÔNG hề chạm tới trạm hai —
sequence CỦA bộ sinh ID không hề tăng LÊN:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}
const SEQUENCE_TOI_DA = 4095;
interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

interface ApiGateway { thung: ThungTokenTG; boSinh: BoSinhSnowflake; }
function taoApiGateway(soTokenToiDa: number, tocDoNapMoiGiay: number, datacenterId: bigint, machineId: bigint, dh: DongHoMoPhong): ApiGateway {
  return { thung: taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh), boSinh: taoBoSinhSnowflake(datacenterId, machineId) };
}
function xuLyRequestGateway(gw: ApiGateway, dh: DongHoMoPhong): { choPhep: boolean; id: bigint | undefined } {
  const choPhep = tieuThuTokenTG(gw.thung, dh);
  if (!choPhep) return { choPhep: false, id: undefined };
  return { choPhep: true, id: sinhSnowflakeId(gw.boSinh, dh) };
}

const dh = taoDongHoMoPhong();
const gw = taoApiGateway(2, 1, 0n, 0n, dh); // toi da 2 token

for (let i = 0; i < 4; i++) {
  const kq = xuLyRequestGateway(gw, dh);
  console.log(`request ${i + 1}: choPhep=${kq.choPhep}, id=${kq.id !== undefined ? String(kq.id) : "undefined"}`);
}
```

```text title=readonly
request 1: choPhep=true, id=0
request 2: choPhep=true, id=1
request 3: choPhep=false, id=undefined
request 4: choPhep=false, id=undefined
```

`xuLyRequestGateway` KHÔNG hề biết chi tiết BÊN trong token bucket
HAY bộ sinh Snowflake — nó chỉ GỌI đúng thứ tự: kiểm rate limit
TRƯỚC (nếu bị từ chối, dừng NGAY, trả `id: undefined`), rồi MỚI cấp
ID. Hai request đầu (còn token) nhận ID `0` VÀ `1` — hai request sau
(hết token) không hề CÓ ID nào cả.
::::

::::example{#kich-ban-day-du}
Gateway `3` token, tốc độ nạp `1` token/giây. Gửi NĂM request đầu
liên tiếp (chỉ `3` token, hai request cuối bị chặn), rồi TIẾN đồng hồ
`2500ms` (nạp thêm) VÀ gửi thêm BA request:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}
const SEQUENCE_TOI_DA = 4095;
interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

interface ApiGateway { thung: ThungTokenTG; boSinh: BoSinhSnowflake; }
function taoApiGateway(soTokenToiDa: number, tocDoNapMoiGiay: number, datacenterId: bigint, machineId: bigint, dh: DongHoMoPhong): ApiGateway {
  return { thung: taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh), boSinh: taoBoSinhSnowflake(datacenterId, machineId) };
}
function xuLyRequestGateway(gw: ApiGateway, dh: DongHoMoPhong): { choPhep: boolean; id: bigint | undefined } {
  const choPhep = tieuThuTokenTG(gw.thung, dh);
  if (!choPhep) return { choPhep: false, id: undefined };
  return { choPhep: true, id: sinhSnowflakeId(gw.boSinh, dh) };
}

const dh = taoDongHoMoPhong();
const gw = taoApiGateway(3, 1, 1n, 1n, dh); // toi da 3 token, nap 1 token/giay

console.log("--- dot 1: 5 request lien tiep tai t=0 (chi 3 token) ---");
const dot1: { choPhep: boolean; id: bigint | undefined }[] = [];
for (let i = 0; i < 5; i++) dot1.push(xuLyRequestGateway(gw, dh));
for (const r of dot1) console.log(`  choPhep=${r.choPhep}, id=${r.id !== undefined ? String(r.id) : "undefined"}`);

tienThoiGian(dh, 2500); // nap them 2.5 token, bi chan o 2 (Math.min voi phan con lai + toi da)
console.log("--- dot 2: sau khi tien 2500ms, gui them 3 request ---");
const dot2: { choPhep: boolean; id: bigint | undefined }[] = [];
for (let i = 0; i < 3; i++) dot2.push(xuLyRequestGateway(gw, dh));
for (const r of dot2) console.log(`  choPhep=${r.choPhep}, id=${r.id !== undefined ? String(r.id) : "undefined"}`);

const tatCaId = [...dot1, ...dot2].map((r) => r.id).filter((id): id is bigint => id !== undefined);
console.log("tat ca ID da cap (theo dung thu tu cap phat):", tatCaId.map(String).join(","));
let idTangDanDung = true;
for (let i = 1; i < tatCaId.length; i++) if (tatCaId[i]! <= tatCaId[i - 1]!) idTangDanDung = false;
console.log("ID luon TANG dan theo dung thu tu cap phat:", idTangDanDung);
console.log("tong so request duoc cap ID:", tatCaId.length, "/ 8");
```

```text title=readonly
--- dot 1: 5 request lien tiep tai t=0 (chi 3 token) ---
  choPhep=true, id=135168
  choPhep=true, id=135169
  choPhep=true, id=135170
  choPhep=false, id=undefined
  choPhep=false, id=undefined
--- dot 2: sau khi tien 2500ms, gui them 3 request ---
  choPhep=true, id=10485895168
  choPhep=true, id=10485895169
  choPhep=false, id=undefined
tat ca ID da cap (theo dung thu tu cap phat): 135168,135169,135170,10485895168,10485895169
ID luon TANG dan theo dung thu tu cap phat: true
tong so request duoc cap ID: 5 / 8
```

Đợt một: ba token ĐẦU cho qua (ID `135168,135169,135170`), hai
request cuối bị CHẶN — hoàn toàn KHÔNG có ID. Sau `2500ms` (nạp thêm
`2.5` token, ĐỦ cho `2` request MỚI), đợt hai: hai request ĐẦU qua
(ID mới, LỚN hơn hẳn ID cũ VÌ timestamp đã tăng), request thứ BA vẫn
bị chặn (chưa đủ token). Tổng CỘNG `5/8` request nhận ID — VÀ toàn bộ
chuỗi ID LUÔN tăng dần đúng theo thứ tự CẤP phát.
::::

::::predict{#doan-tu-choi-khong-tang-sequence commitOnce}
Gateway hết TOKEN, MỘT request bị `tieuThuTokenTG` từ chối
(`choPhep = false`). `xuLyRequestGateway` trả VỀ `id: undefined` NGAY
lập tức. Nếu SAU đó gateway được nạp LẠI token VÀ một request MỚI
được chấp NHẬN, `sequenceHienTai` của bộ sinh Snowflake CÓ bị ảnh
hưởng bởi request đã bị từ chối TRƯỚC đó không?

:::opt{correct}
KHÔNG — `xuLyRequestGateway` trả VỀ NGAY sau khi thấy `choPhep ===
false`, KHÔNG hề gọi `sinhSnowflakeId`, nên `sequenceHienTai` VÀ
`msCuoiCung` của bộ sinh hoàn toàn KHÔNG đổi
:::
:::opt
CÓ — MỖI request đi qua gateway, dù bị từ chối HAY không, đều được
"đếm" VÀO sequence để đảm bảo KHÔNG ID nào bị "bỏ SỐ"
::why
Nhầm "MỌI request đi qua gateway" VỚI "MỌI request được CẤP ID" —
nhưng hai khái niệm NÀY hoàn toàn tách biệt trong `xuLyRequestGateway`.

Chỗ lệch: `xuLyRequestGateway` viết `if (!choPhep) return { choPhep:
false, id: undefined };` — dòng NÀY thoát HÀM ngay, câu lệnh gọi
`sinhSnowflakeId(gw.boSinh, dh)` Ở PHÍA dưới không hề được THỰC thi.
Bộ sinh Snowflake hoàn toàn "MÙ" trước sự tồn tại của các request bị
từ chối — nó chỉ biết TỚI những request THỰC SỰ được cấp ID.
::
:::
::::

::::code{#viet_xu_ly_request_gateway}
Hoàn thiện `xuLyRequestGateway` — SAU khi xác nhận `choPhep === true`
(nhánh từ chối đã xử LÝ Ở trên), gọi `sinhSnowflakeId` để cấp ID VÀ
trả về kết quả.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}
const SEQUENCE_TOI_DA = 4095;
interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

interface ApiGateway { thung: ThungTokenTG; boSinh: BoSinhSnowflake; }
function taoApiGateway(soTokenToiDa: number, tocDoNapMoiGiay: number, datacenterId: bigint, machineId: bigint, dh: DongHoMoPhong): ApiGateway {
  return { thung: taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh), boSinh: taoBoSinhSnowflake(datacenterId, machineId) };
}
function xuLyRequestGateway(gw: ApiGateway, dh: DongHoMoPhong): { choPhep: boolean; id: bigint | undefined } {
  const choPhep = tieuThuTokenTG(gw.thung, dh);
  if (!choPhep) return { choPhep: false, id: undefined };
  ___
}

const dh = taoDongHoMoPhong();
const gw = taoApiGateway(1, 1, 0n, 0n, dh);
const r1 = xuLyRequestGateway(gw, dh);
const r2 = xuLyRequestGateway(gw, dh);
console.log(r1.choPhep, String(r1.id), r2.choPhep, String(r2.id));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungTokenTG { soTokenHienTai: number; soTokenToiDa: number; tocDoNapMoiGiay: number; thoiDiemNapCuoi: number; }
function taoThungTokenTG(soTokenToiDa: number, tocDoNapMoiGiay: number, dh: DongHoMoPhong): ThungTokenTG {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa, tocDoNapMoiGiay, thoiDiemNapCuoi: dh.thoiGianHienTai };
}
function napTheoThoiGian(thung: ThungTokenTG, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemNapCuoi;
  if (soMsTroiQua <= 0) return;
  const soTokenNap = (soMsTroiQua / 1000) * thung.tocDoNapMoiGiay;
  thung.soTokenHienTai = Math.min(thung.soTokenToiDa, thung.soTokenHienTai + soTokenNap);
  thung.thoiDiemNapCuoi = dh.thoiGianHienTai;
}
function tieuThuTokenTG(thung: ThungTokenTG, dh: DongHoMoPhong): boolean {
  napTheoThoiGian(thung, dh);
  if (thung.soTokenHienTai < 1) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;
function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}
const SEQUENCE_TOI_DA = 4095;
interface BoSinhSnowflake { datacenterId: bigint; machineId: bigint; msCuoiCung: number; sequenceHienTai: number; }
function taoBoSinhSnowflake(datacenterId: bigint, machineId: bigint): BoSinhSnowflake {
  return { datacenterId, machineId, msCuoiCung: -1, sequenceHienTai: 0 };
}
function sinhSnowflakeId(bo: BoSinhSnowflake, dh: DongHoMoPhong): bigint {
  let msHienTai = dh.thoiGianHienTai;
  if (msHienTai === bo.msCuoiCung) {
    bo.sequenceHienTai++;
    if (bo.sequenceHienTai > SEQUENCE_TOI_DA) {
      while (dh.thoiGianHienTai === bo.msCuoiCung) tienThoiGian(dh, 1);
      msHienTai = dh.thoiGianHienTai;
      bo.sequenceHienTai = 0;
    }
  } else {
    bo.sequenceHienTai = 0;
  }
  bo.msCuoiCung = msHienTai;
  return ghepSnowflakeId(BigInt(msHienTai), bo.datacenterId, bo.machineId, BigInt(bo.sequenceHienTai));
}

interface ApiGateway { thung: ThungTokenTG; boSinh: BoSinhSnowflake; }
function taoApiGateway(soTokenToiDa: number, tocDoNapMoiGiay: number, datacenterId: bigint, machineId: bigint, dh: DongHoMoPhong): ApiGateway {
  return { thung: taoThungTokenTG(soTokenToiDa, tocDoNapMoiGiay, dh), boSinh: taoBoSinhSnowflake(datacenterId, machineId) };
}
function xuLyRequestGateway(gw: ApiGateway, dh: DongHoMoPhong): { choPhep: boolean; id: bigint | undefined } {
  const choPhep = tieuThuTokenTG(gw.thung, dh);
  if (!choPhep) return { choPhep: false, id: undefined };
  return { choPhep: true, id: sinhSnowflakeId(gw.boSinh, dh) };
}

const dh = taoDongHoMoPhong();
const gw = taoApiGateway(1, 1, 0n, 0n, dh);
const r1 = xuLyRequestGateway(gw, dh);
const r2 = xuLyRequestGateway(gw, dh);
console.log(r1.choPhep, String(r1.id), r2.choPhep, String(r2.id));
```

```typescript title=test
function laySequenceGateway(gw: ApiGateway): number { return gw.boSinh.sequenceHienTai; }
function laySoTokenGateway(gw: ApiGateway): number { return gw.thung.soTokenHienTai; }

const dhT = taoDongHoMoPhong();
const gwT = taoApiGateway(2, 1, 0n, 0n, dhT);

const kq1 = xuLyRequestGateway(gwT, dhT);
const kq2 = xuLyRequestGateway(gwT, dhT);
const kq3 = xuLyRequestGateway(gwT, dhT);
const kq4 = xuLyRequestGateway(gwT, dhT);

if (kq1.choPhep !== true || kq1.id === undefined) throw new Error("request 1 (con token) phai duoc phep VA co ID");
if (kq2.choPhep !== true || kq2.id === undefined) throw new Error("request 2 (con token) phai duoc phep VA co ID");
if (kq3.choPhep !== false || kq3.id !== undefined) throw new Error("request 3 (het token) phai BI TU CHOI VA khong co ID (undefined)");
if (kq4.choPhep !== false || kq4.id !== undefined) throw new Error("request 4 (het token) phai BI TU CHOI VA khong co ID (undefined)");
if (kq2.id! <= kq1.id!) throw new Error("id request 2 phai LON hon id request 1");
if (laySequenceGateway(gwT) !== 1) throw new Error("request bi tu choi KHONG duoc lam sequence cua bo sinh ID tang them (van la 1, tu 2 lan cap ID thanh cong)");
if (laySoTokenGateway(gwT) !== 0) throw new Error("sau 2 lan cap token thanh cong (toi da 2), token con lai phai la 0");

tienThoiGian(dhT, 3000);
const kq5 = xuLyRequestGateway(gwT, dhT);
if (kq5.choPhep !== true || kq5.id === undefined) throw new Error("sau 3000ms (nap lai token), request 5 phai duoc phep VA co ID");
if (kq5.id! <= kq2.id!) throw new Error("id request 5 (t=3000) phai LON hon id request 2 (t=0) -- ID luon tang theo thoi gian");
```

:::hints
- kind: attention
  body: "Da xac nhan choPhep=true o tren -- gio cap ID that va tra ve, mot dong."
- kind: strategy
  body: "return { choPhep: true, id: sinhSnowflakeId(gw.boSinh, dh) };"
- kind: one-line
  body: "return { choPhep: true, id: sinhSnowflakeId(gw.boSinh, dh) };"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "undefined"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Rate limiter chặn đúng, Snowflake ID chỉ cấp cho request được phép,
thứ tự thời gian được bảo toàn xuyên suốt. Quest "Định danh và tốc
độ" đã xong — tiếp theo LÀ ráp thêm bộ định tuyến (load balancer, quest
trước) VÀO một dịch vụ rút gọn URL và thu thập web hoàn chỉnh.
::::

::::reflect{#nghi-lai}
`xuLyRequestGateway` không PHÁT minh gì mới — nó chỉ GỌI đúng thứ tự
hai hệ thống con ĐÃ xây riêng lẻ (`tieuThuTokenTG`, `sinhSnowflakeId`),
VÀ dừng NGAY khi trạm đầu từ chối. Đây LÀ bài học lặp lại xuyên suốt
Realm 7: một hệ thống LỚN thường không CẦN thuật toán mới, chỉ CẦN
ghép ĐÚNG thứ tự VÀ đúng ranh giới trách nhiệm các mảnh ĐÃ đúng sẵn.
::::

::::checkpoint{mastery=0.85}
::::
