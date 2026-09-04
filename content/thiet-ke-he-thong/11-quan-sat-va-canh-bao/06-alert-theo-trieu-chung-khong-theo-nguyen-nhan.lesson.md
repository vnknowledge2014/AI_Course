---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.alert-theo-trieu-chung-khong-theo-nguyen-nhan
title: "Alert theo TRIỆU CHỨNG (symptom-based), không theo NGUYÊN NHÂN (cause-based)"
summary: "canhBaoTheoNguyenNhan(ts, nguongCpu) chi hoi 'CPU co vuot nguong khong' -- CPU=85% (batch job chay nen, nguoi dung KHONG bi anh huong) van BAO DONG GIA (true). canhBaoTheoTrieuChung(ts, nguongTyLeLoi, nguongDoTreMs) hoi 'nguoi dung co THAT su gap loi khong' (tyLeLoiNguoiDungThay > nguong HOAC doTreP99Ms > nguong) -- CUNG tinh huong CPU cao do, tra ve false (im lang DUNG); nguoc lai, CPU binh thuong (40%) nhung loi nguoi dung 5% thi canh bao theo nguyen nhan BO SOT (false) con canh bao theo trieu chung bat DUNG (true). danhGiaCanhBao dem qua nhieu lan quan sat: bao nhieu lan canh bao nguyen nhan la BAO DONG GIA (khong kem trieu chung that)."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.alert-theo-trieu-chung-khong-theo-nguyen-nhan]
requires: [sd.sli-slo-error-budget]
concepts: [sd.alert-theo-trieu-chung-khong-theo-nguyen-nhan]
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
Bài trước đo được ĐÚNG lúc nào error budget cạn. Nhưng "biết đủ tốt hay
không" chỉ có ích NẾU hệ thống báo động đúng LÚC — không quá sớm (khiến ai
đó luôn phải kiểm tra vô ích), VÀ không quá muộn (khi người dùng đã chịu
ảnh hưởng rồi). Câu hỏi kế tiếp: alert NÊN dựa vào ĐÂU?
::::

::::explain{#nguyen-nhan-vs-trieu-chung}
`canhBaoTheoNguyenNhan` chỉ hỏi MỘT câu: một chỉ số nội bộ (CPU) có vượt
ngưỡng không — dù CPU cao KHÔNG chắc đã ảnh hưởng người dùng, ví dụ khi một
batch job chạy NỀN. `canhBaoTheoTrieuChung` hỏi câu KHÁC hẳn: người dùng có
THẬT sự gặp vấn đề không — tỷ lệ lỗi NGƯỜI DÙNG THẤY, hoặc độ trễ p99, vượt
ngưỡng:

```typescript title=readonly
interface TrangThaiHeThong {
  cpuPhanTram: number;
  tyLeLoiNguoiDungThay: number;
  doTreP99Ms: number;
}
function canhBaoTheoNguyenNhan(ts: TrangThaiHeThong, nguongCpu: number): boolean {
  return ts.cpuPhanTram > nguongCpu;
}
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}

// CPU cao (batch job chay nen) nhung KHONG anh huong nguoi dung
const tsCpuCao: TrangThaiHeThong = { cpuPhanTram: 85, tyLeLoiNguoiDungThay: 0.1, doTreP99Ms: 120 };
console.log("CPU cao (85%), trai nghiem nguoi dung BINH THUONG");
console.log("  canh bao theo NGUYEN NHAN (CPU>80):", canhBaoTheoNguyenNhan(tsCpuCao, 80));
console.log("  canh bao theo TRIEU CHUNG (loi>1% hoac p99>300ms):", canhBaoTheoTrieuChung(tsCpuCao, 1, 300));
```

```text title=readonly
CPU cao (85%), trai nghiem nguoi dung BINH THUONG
  canh bao theo NGUYEN NHAN (CPU>80): true
  canh bao theo TRIEU CHUNG (loi>1% hoac p99>300ms): false
```

`canhBaoTheoNguyenNhan` trả về `true` — MỘT báo động GIẢ, vì CPU cao KHÔNG
hề khiến ai đó không dùng được dịch vụ. `canhBaoTheoTrieuChung` trả về
`false` — ĐÚNG, VÌ tỷ lệ lỗi người dùng thấy (`0.1%`) VÀ độ trễ (`120ms`)
đều nằm dưới ngưỡng. Im lặng ĐÚNG lúc quan trọng KHÔNG kém báo động đúng
lúc — mỗi lần báo động GIẢ LÀ một lần người trực ca mất thời gian VÀ niềm
tin vào hệ thống cảnh báo.
::::

::::example{#bo-sot-va-do-alert-fatigue}
Ngược lại cũng nguy hiểm không kém: khi CPU BÌNH THƯỜNG nhưng người dùng
THẬT sự gặp lỗi (ví dụ một dịch vụ hạ nguồn chậm), `canhBaoTheoNguyenNhan`
hoàn toàn IM LẶNG — bỏ SÓT sự cố thật. `danhGiaCanhBao` đo con số cụ thể
cho hiện tượng "báo động giả" (alert fatigue): bao nhiêu lần canh bao theo
NGUYÊN NHÂN kích hoạt MÀ KHÔNG kèm theo triệu chứng thật nào:

```typescript title=readonly
interface TrangThaiHeThong {
  cpuPhanTram: number;
  tyLeLoiNguoiDungThay: number;
  doTreP99Ms: number;
}
function canhBaoTheoNguyenNhan(ts: TrangThaiHeThong, nguongCpu: number): boolean {
  return ts.cpuPhanTram > nguongCpu;
}
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}

// CPU binh thuong nhung nguoi dung THAT su gap loi (vi du dich vu ha nguon cham)
const tsSuCoThat: TrangThaiHeThong = { cpuPhanTram: 40, tyLeLoiNguoiDungThay: 5, doTreP99Ms: 200 };
console.log("CPU binh thuong (40%), nhung nguoi dung THAT su gap loi (5%)");
console.log("  canh bao theo NGUYEN NHAN (CPU>80):", canhBaoTheoNguyenNhan(tsSuCoThat, 80));
console.log("  canh bao theo TRIEU CHUNG (loi>1% hoac p99>300ms):", canhBaoTheoTrieuChung(tsSuCoThat, 1, 300));

interface KetQuaDanhGia { soLanCanhBaoNguyenNhan: number; soLanCanhBaoTrieuChung: number; soLanBaoDongGia: number; }
function danhGiaCanhBao(
  cacTrangThai: TrangThaiHeThong[],
  nguongCpu: number,
  nguongTyLeLoi: number,
  nguongDoTreMs: number
): KetQuaDanhGia {
  let soLanCanhBaoNguyenNhan = 0;
  let soLanCanhBaoTrieuChung = 0;
  let soLanBaoDongGia = 0;
  for (const ts of cacTrangThai) {
    const canhBaoNN = canhBaoTheoNguyenNhan(ts, nguongCpu);
    const canhBaoTC = canhBaoTheoTrieuChung(ts, nguongTyLeLoi, nguongDoTreMs);
    if (canhBaoNN) soLanCanhBaoNguyenNhan += 1;
    if (canhBaoTC) soLanCanhBaoTrieuChung += 1;
    if (canhBaoNN && !canhBaoTC) soLanBaoDongGia += 1;
  }
  return { soLanCanhBaoNguyenNhan, soLanCanhBaoTrieuChung, soLanBaoDongGia };
}

const cacTrangThaiTuanNay: TrangThaiHeThong[] = [
  { cpuPhanTram: 85, tyLeLoiNguoiDungThay: 0.1, doTreP99Ms: 120 },
  { cpuPhanTram: 90, tyLeLoiNguoiDungThay: 0.2, doTreP99Ms: 150 },
  { cpuPhanTram: 40, tyLeLoiNguoiDungThay: 5, doTreP99Ms: 200 },
  { cpuPhanTram: 95, tyLeLoiNguoiDungThay: 8, doTreP99Ms: 500 },
];
console.log("danh gia canh bao qua 4 lan quan sat:", JSON.stringify(danhGiaCanhBao(cacTrangThaiTuanNay, 80, 1, 300)));
```

```text title=readonly
CPU binh thuong (40%), nhung nguoi dung THAT su gap loi (5%)
  canh bao theo NGUYEN NHAN (CPU>80): false
  canh bao theo TRIEU CHUNG (loi>1% hoac p99>300ms): true
danh gia canh bao qua 4 lan quan sat: {"soLanCanhBaoNguyenNhan":3,"soLanCanhBaoTrieuChung":2,"soLanBaoDongGia":2}
```

`canhBaoTheoNguyenNhan` BỎ SÓT sự cố thật (`false` dù người dùng đang gặp
`5%` lỗi), còn `canhBaoTheoTrieuChung` bắt ĐÚNG (`true`). Qua `4` lần quan
sát, `soLanCanhBaoNguyenNhan=3` — nhưng CHỈ `2` trong số ĐÓ THẬT sự đi kèm
triệu chứng người dùng (`soLanCanhBaoTrieuChung=2`), NGHĨA LÀ `2` lần còn
lại (`soLanBaoDongGia=2`) LÀ báo động GIẢ — đúng con số của hiện tượng
"alert fatigue": người trực ca dần bỏ QUA cảnh báo, vì phần lớn chúng
không hề đi kèm sự cố thật.
::::

::::predict{#doan-bien-trieu-chung commitOnce}
Một trạng thái có `tyLeLoiNguoiDungThay` ĐÚNG BẰNG `nguongTyLeLoi` (ví dụ cả
hai đều LÀ `1`), VÀ `doTreP99Ms` thấp hơn NHIỀU so với `nguongDoTreMs`.
`canhBaoTheoTrieuChung` trả về gì?

:::opt{correct}
`false` — cả hai vế của `||` đều dùng `>` (nghiêm ngặt); `1 > 1` LÀ `false`,
VÀ vế độ trễ cũng `false` (thấp hơn ngưỡng NHIỀU), nên toàn bộ biểu thức LÀ
`false`
:::
:::opt
`true` — tỷ lệ lỗi CHẠM đúng ngưỡng cũng đủ nguy hiểm để coi LÀ một triệu
chứng thật, dù chưa VƯỢT hẳn qua
::why
Nhầm "chạm đúng ngưỡng" VỚI "vượt ngưỡng" — nhưng cả hai điều kiện trong
`canhBaoTheoTrieuChung` đều dùng `>` (nghiêm ngặt), giống HỆT cách các bài
trước (canary, rollback) đã dùng.

Chỗ lệch: `ts.tyLeLoiNguoiDungThay > nguongTyLeLoi` với hai giá trị BẰNG
nhau (`1 > 1`) cho `false`. Vế `doTreP99Ms > nguongDoTreMs` cũng `false`
(vì độ trễ thấp hơn nhiều). `false || false` LÀ `false` — hàm trả về
`false`, không kích hoạt cảnh báo.
::
:::
::::

::::code{#viet_canh_bao_theo_trieu_chung}
Hoàn thiện `canhBaoTheoTrieuChung` — trả về `true` nếu tỷ lệ lỗi người dùng
thấy VƯỢT `nguongTyLeLoi` HOẶC độ trễ p99 VƯỢT `nguongDoTreMs` (một trong
hai LÀ đủ, dùng `||`).

```typescript title=starter
interface TrangThaiHeThong {
  cpuPhanTram: number;
  tyLeLoiNguoiDungThay: number;
  doTreP99Ms: number;
}
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  ___
}

const tsX: TrangThaiHeThong = { cpuPhanTram: 50, tyLeLoiNguoiDungThay: 2, doTreP99Ms: 100 };
console.log(canhBaoTheoTrieuChung(tsX, 1, 300));
```

```typescript title=solution
interface TrangThaiHeThong {
  cpuPhanTram: number;
  tyLeLoiNguoiDungThay: number;
  doTreP99Ms: number;
}
function canhBaoTheoTrieuChung(ts: TrangThaiHeThong, nguongTyLeLoi: number, nguongDoTreMs: number): boolean {
  return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;
}

const tsX: TrangThaiHeThong = { cpuPhanTram: 50, tyLeLoiNguoiDungThay: 2, doTreP99Ms: 100 };
console.log(canhBaoTheoTrieuChung(tsX, 1, 300));
```

```typescript title=test
const caHaiBinhThuongT: TrangThaiHeThong = { cpuPhanTram: 90, tyLeLoiNguoiDungThay: 0.1, doTreP99Ms: 50 };
if (canhBaoTheoTrieuChung(caHaiBinhThuongT, 1, 300) !== false) throw new Error("ca hai chi so deu duoi nguong thi phai la false, du CPU cao");

const chiLoiVuotT: TrangThaiHeThong = { cpuPhanTram: 20, tyLeLoiNguoiDungThay: 5, doTreP99Ms: 50 };
if (canhBaoTheoTrieuChung(chiLoiVuotT, 1, 300) !== true) throw new Error("chi ty le loi vuot nguong cung phai kich hoat (OR)");

const chiDoTreVuotT: TrangThaiHeThong = { cpuPhanTram: 20, tyLeLoiNguoiDungThay: 0.1, doTreP99Ms: 500 };
if (canhBaoTheoTrieuChung(chiDoTreVuotT, 1, 300) !== true) throw new Error("chi do tre vuot nguong cung phai kich hoat (OR)");

const caHaiVuotT: TrangThaiHeThong = { cpuPhanTram: 95, tyLeLoiNguoiDungThay: 10, doTreP99Ms: 800 };
if (canhBaoTheoTrieuChung(caHaiVuotT, 1, 300) !== true) throw new Error("ca hai vuot nguong thi chac chan phai kich hoat");

const bienT: TrangThaiHeThong = { cpuPhanTram: 50, tyLeLoiNguoiDungThay: 1, doTreP99Ms: 100 };
if (canhBaoTheoTrieuChung(bienT, 1, 300) !== false) throw new Error("ty le loi DUNG BANG nguong (1=1) khong duoc tinh la vuot -- dieu kien phai la >, khong phai >=");
```

:::hints
- kind: attention
  body: "Tra ve mot bieu thuc OR: ts.tyLeLoiNguoiDungThay > nguongTyLeLoi HOAC ts.doTreP99Ms > nguongDoTreMs (dung ||, so sanh nghiem ngat)."
- kind: strategy
  body: "return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;"
- kind: one-line
  body: "return ts.tyLeLoiNguoiDungThay > nguongTyLeLoi || ts.doTreP99Ms > nguongDoTreMs;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Alert giờ phản ánh ĐÚNG những gì người dùng thật sự trải qua — không còn
báo động vì một con số nội bộ vô hại. Nhưng biết KHI NÀO cần báo động chỉ
LÀ một nửa câu chuyện — nửa còn lại LÀ: báo cho AI, VÀ người đó cần làm GÌ
khi nhận được nó.
::::

::::reflect{#nghi-lai}
`canhBaoTheoNguyenNhan` không hề SAI về mặt kỹ thuật — CPU THẬT sự vượt
`80%`. Cái sai nằm Ở việc coi một con số NỘI bộ LÀ đại diện cho trải nghiệm
người dùng, trong khi hai thứ ĐÓ có thể hoàn toàn KHÔNG liên quan.
`canhBaoTheoTrieuChung` không hề "thông minh" hơn về mặt thuật toán — nó chỉ
đo ĐÚNG thứ cần đo. `danhGiaCanhBao` biến sự khác biệt ĐÓ thành một con số
cụ thể (`soLanBaoDongGia`), thay vì một lời phàn nàn chung chung VỀ "quá
nhiều cảnh báo giả".
::::

::::checkpoint{mastery=0.80}
::::
