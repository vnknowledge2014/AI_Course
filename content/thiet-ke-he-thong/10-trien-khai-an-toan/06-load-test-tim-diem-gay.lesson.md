---
id: thiet-ke-he-thong.trien-khai-an-toan.load-test-tim-diem-gay
title: "Load test trước khi launch: tìm điểm gãy"
summary: "moPhongTaiRequest(ht, soRequestDongThoi) tinh doTreMs VA tyLeLoi theo sucChuaToiDa=100: duoi suc chua, doTreMs tang tuyen tinh (100 request -> 250ms, 0% loi); vuot suc chua, doTreMs tang VOT (150 request -> 1250ms) VA tyLeLoi>0 (150 request, 100 xu ly duoc, 50 loi -> 33.3%). chayLoadTest(ht, cacMucTai, {nguongDoTreMs, nguongTyLeLoi}) tang dan tung muc, DUNG LAI NGAY khi vo nguong (khong chay het danh sach) -- vi du [50,100,150,200] voi nguong 250ms/10% chi chay toi 150 (diem gay), khong toi 200."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.load-test-tim-diem-gay]
requires: [sd.migration-an-toan-expand-contract]
concepts: [sd.load-test-tim-diem-gay]
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
Health check (bài 4) đảm bảo traffic chỉ tới instance sẵn sàng. Nhưng
"sẵn sàng" không có nghĩa LÀ "chịu được BAO NHIÊU traffic". Trước khi
launch một thay đổi lớn, câu hỏi cần trả lời TRƯỚC — không phải trong
lúc sự cố — LÀ: hệ thống VỠ Ở đâu, VÀ vỡ như thế NÀO?
::::

::::explain{#mo-phong-tai-request}
`moPhongTaiRequest` mô phỏng phản ứng của hệ thống trước một số lượng
request đồng thời — dưới `sucChuaToiDa`, độ trễ tăng TUYẾN tính VÀ
không có lỗi; vượt quá, độ trễ tăng VỌT VÀ một phần request bắt đầu
LỖI (phần vượt quá khả năng xử lý):

```typescript title=readonly
interface HeThongMoPhong { sucChuaToiDa: number; }

interface KetQuaTaiThuNghiem { soRequestDongThoi: number; doTreMs: number; tyLeLoi: number; }

function moPhongTaiRequest(ht: HeThongMoPhong, soRequestDongThoi: number): KetQuaTaiThuNghiem {
  const doTreMs =
    soRequestDongThoi <= ht.sucChuaToiDa
      ? 50 + soRequestDongThoi * 2
      : 50 + ht.sucChuaToiDa * 2 + (soRequestDongThoi - ht.sucChuaToiDa) * 20;
  const soVuotQua = Math.max(0, soRequestDongThoi - ht.sucChuaToiDa);
  const tyLeLoi = soRequestDongThoi === 0 ? 0 : soVuotQua / soRequestDongThoi;
  return { soRequestDongThoi, doTreMs, tyLeLoi };
}

const ht: HeThongMoPhong = { sucChuaToiDa: 100 };

console.log("50 request dong thoi (duoi suc chua):", JSON.stringify(moPhongTaiRequest(ht, 50)));
console.log("100 request dong thoi (dung suc chua):", JSON.stringify(moPhongTaiRequest(ht, 100)));
console.log("150 request dong thoi (VUOT suc chua 50):", JSON.stringify(moPhongTaiRequest(ht, 150)));
```

```text title=readonly
50 request dong thoi (duoi suc chua): {"soRequestDongThoi":50,"doTreMs":150,"tyLeLoi":0}
100 request dong thoi (dung suc chua): {"soRequestDongThoi":100,"doTreMs":250,"tyLeLoi":0}
150 request dong thoi (VUOT suc chua 50): {"soRequestDongThoi":150,"doTreMs":1250,"tyLeLoi":0.3333333333333333}
```

Từ `50` lên `100` request, độ trễ tăng ĐỀU (`150ms` lên `250ms`, cộng
thêm `2ms` cho mỗi request). Nhưng từ `100` lên `150` — chỉ thêm `50`
request VƯỢT khả năng — độ trễ NHẢY vọt lên `1250ms`, VÀ `1/3` số
request bắt đầu LỖI. Đây LÀ đúng hình dạng của một "điểm gãy": KHÔNG
suy giảm từ từ, mà sụp Ở một ngưỡng cụ thể.
::::

::::example{#chay-load-test}
`chayLoadTest` tăng dần số request đồng thời theo danh sách mốc, VÀ
DỪNG LẠI ngay khi phát hiện một mốc vượt ngưỡng cho phép (độ trễ hoặc
tỷ lệ lỗi) — không cần chạy hết mọi mốc còn lại Ở phía SAU:

```typescript title=readonly
interface HeThongMoPhong { sucChuaToiDa: number; }
interface KetQuaTaiThuNghiem { soRequestDongThoi: number; doTreMs: number; tyLeLoi: number; }

function moPhongTaiRequest(ht: HeThongMoPhong, soRequestDongThoi: number): KetQuaTaiThuNghiem {
  const doTreMs =
    soRequestDongThoi <= ht.sucChuaToiDa
      ? 50 + soRequestDongThoi * 2
      : 50 + ht.sucChuaToiDa * 2 + (soRequestDongThoi - ht.sucChuaToiDa) * 20;
  const soVuotQua = Math.max(0, soRequestDongThoi - ht.sucChuaToiDa);
  const tyLeLoi = soRequestDongThoi === 0 ? 0 : soVuotQua / soRequestDongThoi;
  return { soRequestDongThoi, doTreMs, tyLeLoi };
}

interface CauHinhNguong { nguongDoTreMs: number; nguongTyLeLoi: number; }
interface KetQuaLoadTest { diemGay: number | undefined; cacKetQua: KetQuaTaiThuNghiem[]; }

function chayLoadTest(ht: HeThongMoPhong, cacMucTai: number[], cauHinh: CauHinhNguong): KetQuaLoadTest {
  const cacKetQua: KetQuaTaiThuNghiem[] = [];
  let diemGay: number | undefined = undefined;
  for (const muc of cacMucTai) {
    const kq = moPhongTaiRequest(ht, muc);
    cacKetQua.push(kq);
    if (kq.doTreMs > cauHinh.nguongDoTreMs || kq.tyLeLoi > cauHinh.nguongTyLeLoi) {
      diemGay = muc;
      break;
    }
  }
  return { diemGay, cacKetQua };
}

const ht: HeThongMoPhong = { sucChuaToiDa: 100 };
const cauHinh: CauHinhNguong = { nguongDoTreMs: 250, nguongTyLeLoi: 0.1 };
const ketQua = chayLoadTest(ht, [50, 100, 150, 200], cauHinh);

console.log("cac muc tai da thu (dung LAI khi vo, khong chay het):", JSON.stringify(ketQua.cacKetQua.map((k) => k.soRequestDongThoi)));
console.log("diem gay (capacity can co la DUOI muc nay):", ketQua.diemGay);
for (const kq of ketQua.cacKetQua) {
  console.log(`  ${kq.soRequestDongThoi} request: do tre ${kq.doTreMs}ms, ty le loi ${(kq.tyLeLoi * 100).toFixed(1)}%`);
}
```

```text title=readonly
cac muc tai da thu (dung LAI khi vo, khong chay het): [50,100,150]
diem gay (capacity can co la DUOI muc nay): 150
  50 request: do tre 150ms, ty le loi 0.0%
  100 request: do tre 250ms, ty le loi 0.0%
  150 request: do tre 1250ms, ty le loi 33.3%
```

`200` KHÔNG hề xuất hiện trong `cacKetQua` — vòng lặp DỪNG NGAY khi
`150` cho thấy đã vượt cả hai ngưỡng (`1250ms > 250ms` VÀ `33.3% >
10%`). `diemGay=150` LÀ tín hiệu rõ ràng: capacity thật CẦN có phải
NẰM dưới mức này, chứ không cần thử tiếp những mức còn tệ hơn.
::::

::::predict{#doan-diem-gay-khac commitOnce}
Một hệ thống khác có `sucChuaToiDa=80`. Chạy `chayLoadTest` với mốc
`[60, 90]` VÀ ngưỡng `{ nguongDoTreMs: 300, nguongTyLeLoi: 0.05 }` —
`diemGay` LÀ bao nhiêu, VÀ `cacKetQua` có BAO NHIÊU phần tử?

:::opt{correct}
`diemGay = 90`, `cacKetQua` có `2` phần tử — Ở `60` (dưới `80`), độ
trễ `170ms` VÀ không lỗi, DƯỚI cả hai ngưỡng nên tiếp tục; Ở `90` (vượt
`80` đúng `10`), độ trễ nhảy lên `410ms` VÀ tỷ lệ lỗi `11.1%`, VƯỢT cả
hai ngưỡng, VÒNG lặp đẩy kết quả VÀO mảng RỒI mới dừng
:::
:::opt
`diemGay = 90`, `cacKetQua` có `1` phần tử — vì `90` LÀ mốc gây vỡ,
chỉ mốc ĐÓ mới đáng được ghi lại, mốc `60` (an toàn) không cần lưu
::why
Nhầm "chỉ lưu mốc gây vỡ" VỚI "lưu MỌI mốc đã thử qua, kể cả mốc an
toàn" — nhưng `chayLoadTest` đẩy KẾT quả vào `cacKetQua` Ở MỌI vòng
lặp, kể cả những vòng KHÔNG vỡ, trước khi kiểm tra điều kiện dừng.

Chỗ lệch: dòng `cacKetQua.push(kq);` nằm TRƯỚC dòng kiểm tra ngưỡng,
VÀ chạy Ở MỌI lần lặp, không riêng lần gây vỡ. Mốc `60` (an toàn) VẪN
được đẩy vào mảng trước khi vòng lặp tới `90` VÀ dừng — kết quả cuối
LÀ mảng có ĐÚNG `2` phần tử: `60` VÀ `90`, không phải chỉ `90`.
::
:::
::::

::::code{#viet_chay_load_test}
Hoàn thiện `chayLoadTest` — với MỖI mức tải, mô phỏng kết quả, đẩy vào
`cacKetQua`, RỒI kiểm tra xem đã vượt ngưỡng độ trễ HAY tỷ lệ lỗi
chưa; nếu vượt, ghi lại `diemGay` VÀ dừng vòng lặp NGAY.

```typescript title=starter
interface HeThongMoPhong { sucChuaToiDa: number; }
interface KetQuaTaiThuNghiem { soRequestDongThoi: number; doTreMs: number; tyLeLoi: number; }

function moPhongTaiRequest(ht: HeThongMoPhong, soRequestDongThoi: number): KetQuaTaiThuNghiem {
  const doTreMs =
    soRequestDongThoi <= ht.sucChuaToiDa
      ? 50 + soRequestDongThoi * 2
      : 50 + ht.sucChuaToiDa * 2 + (soRequestDongThoi - ht.sucChuaToiDa) * 20;
  const soVuotQua = Math.max(0, soRequestDongThoi - ht.sucChuaToiDa);
  const tyLeLoi = soRequestDongThoi === 0 ? 0 : soVuotQua / soRequestDongThoi;
  return { soRequestDongThoi, doTreMs, tyLeLoi };
}

interface CauHinhNguong { nguongDoTreMs: number; nguongTyLeLoi: number; }
interface KetQuaLoadTest { diemGay: number | undefined; cacKetQua: KetQuaTaiThuNghiem[]; }

function chayLoadTest(ht: HeThongMoPhong, cacMucTai: number[], cauHinh: CauHinhNguong): KetQuaLoadTest {
  const cacKetQua: KetQuaTaiThuNghiem[] = [];
  let diemGay: number | undefined = undefined;
  for (const muc of cacMucTai) {
    ___
  }
  return { diemGay, cacKetQua };
}

const htX: HeThongMoPhong = { sucChuaToiDa: 100 };
const kqX = chayLoadTest(htX, [50, 100, 150, 200], { nguongDoTreMs: 250, nguongTyLeLoi: 0.1 });
console.log(kqX.diemGay, kqX.cacKetQua.length);
```

```typescript title=solution
interface HeThongMoPhong { sucChuaToiDa: number; }
interface KetQuaTaiThuNghiem { soRequestDongThoi: number; doTreMs: number; tyLeLoi: number; }

function moPhongTaiRequest(ht: HeThongMoPhong, soRequestDongThoi: number): KetQuaTaiThuNghiem {
  const doTreMs =
    soRequestDongThoi <= ht.sucChuaToiDa
      ? 50 + soRequestDongThoi * 2
      : 50 + ht.sucChuaToiDa * 2 + (soRequestDongThoi - ht.sucChuaToiDa) * 20;
  const soVuotQua = Math.max(0, soRequestDongThoi - ht.sucChuaToiDa);
  const tyLeLoi = soRequestDongThoi === 0 ? 0 : soVuotQua / soRequestDongThoi;
  return { soRequestDongThoi, doTreMs, tyLeLoi };
}

interface CauHinhNguong { nguongDoTreMs: number; nguongTyLeLoi: number; }
interface KetQuaLoadTest { diemGay: number | undefined; cacKetQua: KetQuaTaiThuNghiem[]; }

function chayLoadTest(ht: HeThongMoPhong, cacMucTai: number[], cauHinh: CauHinhNguong): KetQuaLoadTest {
  const cacKetQua: KetQuaTaiThuNghiem[] = [];
  let diemGay: number | undefined = undefined;
  for (const muc of cacMucTai) {
    const kq = moPhongTaiRequest(ht, muc);
    cacKetQua.push(kq);
    if (kq.doTreMs > cauHinh.nguongDoTreMs || kq.tyLeLoi > cauHinh.nguongTyLeLoi) {
      diemGay = muc;
      break;
    }
  }
  return { diemGay, cacKetQua };
}

const htX: HeThongMoPhong = { sucChuaToiDa: 100 };
const kqX = chayLoadTest(htX, [50, 100, 150, 200], { nguongDoTreMs: 250, nguongTyLeLoi: 0.1 });
console.log(kqX.diemGay, kqX.cacKetQua.length);
```

```typescript title=test
const htT: HeThongMoPhong = { sucChuaToiDa: 100 };
const kqKhongVoT = chayLoadTest(htT, [20, 40, 60], { nguongDoTreMs: 1000, nguongTyLeLoi: 0.5 });
if (kqKhongVoT.diemGay !== undefined) throw new Error("khong muc tai nao vuot nguong thi diemGay phai la undefined");
if (kqKhongVoT.cacKetQua.length !== 3) throw new Error("khong vo thi phai thu HET ca 3 muc tai");

const kqVoT = chayLoadTest(htT, [50, 100, 150, 200], { nguongDoTreMs: 250, nguongTyLeLoi: 0.1 });
if (kqVoT.diemGay !== 150) throw new Error("diem gay dau tien phai la 150 (do tre 1250ms vuot nguong 250ms)");
if (kqVoT.cacKetQua.length !== 3) throw new Error("phai DUNG LAI ngay khi vo, khong chay tiep muc 200 (chi con 3 phan tu: 50,100,150)");
const buocCuoiVoT = kqVoT.cacKetQua[kqVoT.cacKetQua.length - 1];
if (buocCuoiVoT === undefined || buocCuoiVoT.soRequestDongThoi !== 150) throw new Error("phan tu cuoi cung trong ket qua phai la muc gay vo (150)");

const kqBienT = chayLoadTest(htT, [100], { nguongDoTreMs: 250, nguongTyLeLoi: 0.1 });
if (kqBienT.diemGay !== undefined) throw new Error("do tre DUNG BANG nguong (250 = 250) khong duoc tinh la vo -- dieu kien phai la >, khong phai >=");
```

:::hints
- kind: attention
  body: "Trong than for, can dung 4 dong: const kq = moPhongTaiRequest(ht, muc); cacKetQua.push(kq); neu (kq.doTreMs > cauHinh.nguongDoTreMs || kq.tyLeLoi > cauHinh.nguongTyLeLoi) thi { diemGay = muc; break; }"
- kind: strategy
  body: "const kq = moPhongTaiRequest(ht, muc); cacKetQua.push(kq); if (kq.doTreMs > cauHinh.nguongDoTreMs || kq.tyLeLoi > cauHinh.nguongTyLeLoi) { diemGay = muc; break; }"
- kind: one-line
  body: "const kq = moPhongTaiRequest(ht, muc); cacKetQua.push(kq); if (kq.doTreMs > cauHinh.nguongDoTreMs || kq.tyLeLoi > cauHinh.nguongTyLeLoi) { diemGay = muc; break; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "150 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Điểm gãy đã có SỐ LIỆU cụ thể — không còn LÀ phỏng đoán. Nhưng biết
TRƯỚC hệ thống vỡ Ở đâu chỉ giúp CHUẨN bị, không giúp PHẢN ỨNG khi sự
cố THẬT sự xảy ra giữa lúc đang rollout. Bước tiếp theo: khi metric
xấu đi NGAY trong lúc triển khai, ai — hoặc CÁI gì — kéo hệ thống lùi
lại?
::::

::::reflect{#nghi-lai}
`chayLoadTest` không hề đoán TRƯỚC điểm gãy nằm Ở đâu — nó THỬ dần,
quan sát, VÀ chỉ dừng khi có bằng chứng cụ thể. `moPhongTaiRequest` mô
hình hoá đúng hình DẠNG của một hệ thống thật: ổn định trong VÙNG an
toàn, rồi suy giảm ĐỘT ngột — không tuyến tính — ngay khi vượt quá khả
năng xử lý. Load test trước khi launch biến "hy vọng hệ thống chịu
được" thành một con số: `diemGay`.
::::

::::checkpoint{mastery=0.80}
::::
