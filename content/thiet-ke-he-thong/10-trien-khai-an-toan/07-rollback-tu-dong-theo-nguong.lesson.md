---
id: thiet-ke-he-thong.trien-khai-an-toan.rollback-tu-dong-theo-nguong
title: "Rollback tự động: điều kiện trigger, tốc độ rollback"
summary: "quyetDinhRollback(trangThai, metricHienTai, nguongCanhBao) trigger rollback NGAY (mot buoc, VE DUNG 0) khi metric > nguong, giu nguyen neu duoi hoac BANG nguong. giamSatLienTuc lap qua nhieu lan doc metric, DUNG doc TIEP ngay khi phat hien vuot (khong doc het danh sach) -- vi du [0.01,0.02,0.015,0.08,0.5] voi nguong 0.05 chi doc 4 lan, dung o 0.08, KHONG doc toi 0.5. Doi lap voi canary (bai 2, tang dan 20% moi buoc, can 5 buoc de len 100%), rollback luon la MOT buoc bat ke dang o ty le nao -- nhanh hon deploy."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.rollback-tu-dong-theo-nguong]
requires: [sd.load-test-tim-diem-gay]
concepts: [sd.rollback-tu-dong-theo-nguong]
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
Load test (bài 6) tìm điểm gãy TRƯỚC khi launch — dựa trên tải mô
phỏng. Nhưng production LUÔN khác kịch bản test Ở MỘT điểm nào đó.
Khi metric THẬT xấu đi NGAY giữa lúc đang rollout, chờ người trực ca
nhận ra VÀ gõ lệnh rollback thủ công LÀ quá chậm — hệ thống cần tự
kéo mình LÙI lại, không chờ ai cả.
::::

::::explain{#quyet-dinh-rollback}
`quyetDinhRollback` theo dõi MỘT metric (tỷ lệ lỗi, hoặc độ trễ) SO
với một ngưỡng cảnh báo. Vượt ngưỡng — dù chỉ một lần đo — trigger
rollback NGAY: đưa `tyLeTraffic` VỀ đúng `0`, trong ĐÚNG MỘT bước, chứ
không giảm dần từng chút một:

```typescript title=readonly
interface TrangThaiRollout { tyLeTraffic: number; daRollback: boolean; }
function taoTrangThaiRollout(tyLeBanDau: number): TrangThaiRollout {
  return { tyLeTraffic: tyLeBanDau, daRollback: false };
}

type KetQuaGiamSat = "tiep_tuc" | "rollback_ngay";

function quyetDinhRollback(trangThai: TrangThaiRollout, metricHienTai: number, nguongCanhBao: number): KetQuaGiamSat {
  if (metricHienTai > nguongCanhBao) {
    trangThai.tyLeTraffic = 0;
    trangThai.daRollback = true;
    return "rollback_ngay";
  }
  return "tiep_tuc";
}

const ts = taoTrangThaiRollout(80);
console.log("ty le traffic dang o:", ts.tyLeTraffic);
console.log("metric 0.02, nguong 0.05:", quyetDinhRollback(ts, 0.02, 0.05));
console.log("ty le traffic KHONG doi:", ts.tyLeTraffic);

console.log("metric 0.09, VUOT nguong 0.05:", quyetDinhRollback(ts, 0.09, 0.05));
console.log("ty le traffic sau rollback (VE 0 NGAY, MOT buoc):", ts.tyLeTraffic);
console.log("da rollback:", ts.daRollback);
```

```text title=readonly
ty le traffic dang o: 80
metric 0.02, nguong 0.05: tiep_tuc
ty le traffic KHONG doi: 80
metric 0.09, VUOT nguong 0.05: rollback_ngay
ty le traffic sau rollback (VE 0 NGAY, MOT buoc): 0
da rollback: true
```

Đang Ở `tyLeTraffic=80`, metric `0.09` vượt ngưỡng `0.05` — rollback
đưa THẲNG về `0`, không phải `70`, `60`, rồi `50` giảm dần. TỐC độ LÀ
điểm mấu chốt: rollback không có "quá trình", nó LÀ một hành động
tức thời — trái ngược hoàn toàn với canary vốn CỐ tình tăng CHẬM.
::::

::::example{#giam-sat-lien-tuc}
`soBuocTangDanDenDich` tính số bước canary (bài 2) CẦN để tăng từ một
tỷ lệ lên tỷ lệ đích. `giamSatLienTuc` mô phỏng việc đọc metric LIÊN
TỤC theo thời gian, VÀ dừng đọc NGAY khi phát hiện vượt ngưỡng — không
chờ đọc hết mọi điểm dữ liệu còn lại:

```typescript title=readonly
interface TrangThaiRollout { tyLeTraffic: number; daRollback: boolean; }
function taoTrangThaiRollout(tyLeBanDau: number): TrangThaiRollout {
  return { tyLeTraffic: tyLeBanDau, daRollback: false };
}
type KetQuaGiamSat = "tiep_tuc" | "rollback_ngay";
function quyetDinhRollback(trangThai: TrangThaiRollout, metricHienTai: number, nguongCanhBao: number): KetQuaGiamSat {
  if (metricHienTai > nguongCanhBao) {
    trangThai.tyLeTraffic = 0;
    trangThai.daRollback = true;
    return "rollback_ngay";
  }
  return "tiep_tuc";
}

function soBuocTangDanDenDich(tyLeBatDau: number, tyLeDich: number, buocTang: number): number {
  return Math.ceil((tyLeDich - tyLeBatDau) / buocTang);
}

function giamSatLienTuc(trangThai: TrangThaiRollout, cacMetricTheoThoiGian: number[], nguongCanhBao: number): KetQuaGiamSat[] {
  const cacKetQua: KetQuaGiamSat[] = [];
  for (const metric of cacMetricTheoThoiGian) {
    const kq = quyetDinhRollback(trangThai, metric, nguongCanhBao);
    cacKetQua.push(kq);
    if (kq === "rollback_ngay") break;
  }
  return cacKetQua;
}

console.log("so buoc CAN de tang dan tu 0% len 100% (buoc 20 moi lan):", soBuocTangDanDenDich(0, 100, 20));
console.log("so buoc rollback tu BAT KY ty le nao ve 0% (luon la 1 buoc, khong giam dan):", 1);

const ts = taoTrangThaiRollout(0);
const cacDoc = [0.01, 0.02, 0.015, 0.08, 0.5];
const cacKetQua = giamSatLienTuc(ts, cacDoc, 0.05);
console.log("cac ket qua giam sat (DUNG NGAY khi phat hien vuot, khong doc tiep):", JSON.stringify(cacKetQua));
console.log("so lan doc THAT su da dung (khong phai het 5 lan):", cacKetQua.length);
console.log("ty le traffic cuoi cung:", ts.tyLeTraffic);
```

```text title=readonly
so buoc CAN de tang dan tu 0% len 100% (buoc 20 moi lan): 5
so buoc rollback tu BAT KY ty le nao ve 0% (luon la 1 buoc, khong giam dan): 1
cac ket qua giam sat (DUNG NGAY khi phat hien vuot, khong doc tiep): ["tiep_tuc","tiep_tuc","tiep_tuc","rollback_ngay"]
so lan doc THAT su da dung (khong phai het 5 lan): 4
ty le traffic cuoi cung: 0
```

Tăng dần lên `100%` cần `5` bước — mỗi bước đòi hỏi một chu kỳ quan
sát VÀ chờ đợi. Rollback thì LUÔN LÀ `1` bước, bất kể đang Ở `20%` hay
`80%`. Vòng lặp giám sát chỉ đọc `4` trong số `5` điểm dữ liệu — đọc
tới `0.08` (vượt `0.05`) LÀ dừng NGAY, giá trị `0.5` phía sau — dù tệ
hơn nhiều — không bao giờ cần được đọc TỚI, vì hành động đã XẢY ra.
::::

::::predict{#doan-metric-bang-nguong commitOnce}
Trạng thái đang Ở `tyLeTraffic=60`. Gọi `quyetDinhRollback(ts, 0.05,
0.05)` — metric quan sát được ĐÚNG BẰNG ngưỡng cảnh báo. Kết quả LÀ
gì, VÀ `tyLeTraffic` sau đó bằng bao nhiêu?

:::opt{correct}
`"tiep_tuc"`, `tyLeTraffic` VẪN LÀ `60` — điều kiện trigger dùng `>`
(nghiêm ngặt); `0.05 > 0.05` LÀ `false`, nên metric CHẠM đúng ngưỡng
CHƯA đủ để kích hoạt rollback
:::
:::opt
`"rollback_ngay"`, `tyLeTraffic` thành `0` — một hệ thống rollback AN
TOÀN nên phản ứng NGAY khi metric chạm tới ngưỡng, không đợi vượt HẲN
qua mới hành động
::why
Nhầm "phòng ngừa cẩn thận" VỚI "đúng theo điều kiện đã viết" — bài học
Ở đây không phải LÀ "ngưỡng nào an toàn hơn", mà LÀ đọc ĐÚNG điều kiện
code THẬT sự kiểm tra.

Chỗ lệch: `quyetDinhRollback` viết `if (metricHienTai > nguongCanhBao)`,
dùng `>`. Với `metricHienTai = 0.05` VÀ `nguongCanhBao = 0.05`, biểu
thức LÀ `0.05 > 0.05`, cho `false` — nhánh rollback KHÔNG chạy, hàm
trả về `"tiep_tuc"`, VÀ `tyLeTraffic` giữ nguyên `60`. Muốn trigger
NGAY tại đúng ngưỡng, ngưỡng cấu hình phải được đặt THẤP hơn giá trị
thật sự muốn chặn.
::
:::
::::

::::code{#viet_quyet_dinh_rollback}
Hoàn thiện `quyetDinhRollback` — nếu `metricHienTai` VƯỢT
`nguongCanhBao`, đưa `tyLeTraffic` VỀ `0`, đánh dấu `daRollback`, VÀ
trả về `"rollback_ngay"`; ngược lại trả về `"tiep_tuc"`, KHÔNG đụng gì
tới trạng thái.

```typescript title=starter
interface TrangThaiRollout { tyLeTraffic: number; daRollback: boolean; }
function taoTrangThaiRollout(tyLeBanDau: number): TrangThaiRollout {
  return { tyLeTraffic: tyLeBanDau, daRollback: false };
}
type KetQuaGiamSat = "tiep_tuc" | "rollback_ngay";
function quyetDinhRollback(trangThai: TrangThaiRollout, metricHienTai: number, nguongCanhBao: number): KetQuaGiamSat {
  ___
}

const tsX = taoTrangThaiRollout(50);
console.log(quyetDinhRollback(tsX, 0.2, 0.05), tsX.tyLeTraffic);
```

```typescript title=solution
interface TrangThaiRollout { tyLeTraffic: number; daRollback: boolean; }
function taoTrangThaiRollout(tyLeBanDau: number): TrangThaiRollout {
  return { tyLeTraffic: tyLeBanDau, daRollback: false };
}
type KetQuaGiamSat = "tiep_tuc" | "rollback_ngay";
function quyetDinhRollback(trangThai: TrangThaiRollout, metricHienTai: number, nguongCanhBao: number): KetQuaGiamSat {
  if (metricHienTai > nguongCanhBao) {
    trangThai.tyLeTraffic = 0;
    trangThai.daRollback = true;
    return "rollback_ngay";
  }
  return "tiep_tuc";
}

const tsX = taoTrangThaiRollout(50);
console.log(quyetDinhRollback(tsX, 0.2, 0.05), tsX.tyLeTraffic);
```

```typescript title=test
const tsT = taoTrangThaiRollout(70);
const k1 = quyetDinhRollback(tsT, 0.01, 0.05);
if (k1 !== "tiep_tuc") throw new Error("metric duoi nguong phai tiep tuc");
const tyLeSauK1T = tsT.tyLeTraffic;
if (tyLeSauK1T !== 70) throw new Error("khi tiep tuc, ty le traffic KHONG duoc doi");
const rollbackSauK1T = tsT.daRollback;
if (rollbackSauK1T !== false) throw new Error("khi tiep tuc, daRollback phai van la false");

const k2 = quyetDinhRollback(tsT, 0.2, 0.05);
if (k2 !== "rollback_ngay") throw new Error("metric vuot nguong phai rollback ngay");
const tyLeSauK2T = tsT.tyLeTraffic;
if (tyLeSauK2T !== 0) throw new Error("rollback phai dua ty le traffic VE DUNG 0, MOT buoc, khong giam dan tung chut");
const rollbackSauK2T = tsT.daRollback;
if (rollbackSauK2T !== true) throw new Error("daRollback phai duoc danh dau true");

const tsBienT = taoTrangThaiRollout(40);
const k3 = quyetDinhRollback(tsBienT, 0.05, 0.05);
if (k3 !== "tiep_tuc") throw new Error("metric DUNG BANG nguong (0.05 = 0.05) khong duoc tinh la vuot -- dieu kien phai la >, khong phai >=");
if (tsBienT.tyLeTraffic !== 40) throw new Error("truong hop bien, ty le traffic KHONG duoc doi");
```

:::hints
- kind: attention
  body: "Neu metricHienTai > nguongCanhBao thi: trangThai.tyLeTraffic = 0; trangThai.daRollback = true; return 'rollback_ngay'. Nguoc lai return 'tiep_tuc'."
- kind: strategy
  body: "if (metricHienTai > nguongCanhBao) { trangThai.tyLeTraffic = 0; trangThai.daRollback = true; return 'rollback_ngay'; } return 'tiep_tuc';"
- kind: one-line
  body: "if (metricHienTai > nguongCanhBao) { trangThai.tyLeTraffic = 0; trangThai.daRollback = true; return 'rollback_ngay'; } return 'tiep_tuc';"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "rollback_ngay 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trigger đúng lúc, hành động NGAY LẬP TỨC — rollback tự động đã có
hình dạng cụ thể. Nhưng cho tới GIỜ, canary (bài 2) VÀ rollback (bài
NÀY) vẫn LÀ hai mảnh RIÊNG BIỆT. Ráp chúng LẠI thành MỘT quy trình
duy nhất LÀ bước kế tiếp.
::::

::::reflect{#nghi-lai}
`quyetDinhRollback` cố tình BẤT ĐỐI XỨNG với `quyetDinhTangCanary`
(bài 2): tăng canary LUÔN từng bước nhỏ, có kiểm soát; rollback LUÔN
LÀ một bước, dứt khoát. Sự bất đối xứng NÀY không phải ngẫu nhiên —
tăng traffic sai lầm chỉ ảnh hưởng một phần nhỏ người dùng trong một
khoảng THỜI gian ngắn, nhưng CHẦN CHỪ khi hệ thống đang lỗi lại khiến
CÀNG nhiều người dùng chịu ảnh hưởng theo từng giây trôi qua. Tốc độ
phản ứng không đối xứng LÀ một quyết định thiết kế có chủ đích.
::::

::::checkpoint{mastery=0.81}
::::
