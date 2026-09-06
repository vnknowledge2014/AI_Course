---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.vong-lap-la-fold-tren-state
title: "Cầu nối FP — agent loop LÀ fold trên State, dừng khi đạt biến thể KetQua cuối"
summary: "Cầu nối FP (dựa trên chapter-27-monads-bind.chapter.md, phần chain/flatMap = 'áp một bước rồi dùng NGUYÊN kết quả bước đó, không bọc thêm lớp'): type Buoc<TrangThai, KetQuaMotBuoc> = (trangThai: TrangThai) => { trangThaiMoi: TrangThai; ketQua: KetQuaMotBuoc } LÀ dạng tối giản của MỘT bước State trong TypeScript -- nhận trạng thái, trả trạng thái MỚI CỘNG một kết quả cục bộ. foldVongLap(buoc, trangThaiBanDau, soBuocToiDa) ÁP DỤNG LẠI cùng một Buoc, GẤP (fold) qua từng bước, tích luỹ trangThai, cho tới khi ketQua đạt biến thể 'xong'/'loi' (ánh xạ sang KetQua<T,E>/andThen đã học ở T9.3 bài 5) hoặc hết soBuocToiDa. Xác nhận bằng số: foldVongLap(taoBuocTuTacVu(taoTacVuHoiTuSauNBuoc(3)), {soBuoc:0}, 5) cho ĐÚNG {trangThai:'thanhCong',giaTri:'hoan_thanh'} -- giống hệt chayVongLapCoCap kiểu mệnh lệnh của bài 2 trên CÙNG tác vụ (đối chiếu qua JSON.stringify bằng nhau). Trên bộ 3 tác vụ [hoiTu(3), hoiTu(1), hoiTu(10)] với cap=5: kết quả [thanhCong,thanhCong,hetBuoc] -- xác nhận foldVongLap tổng quát hoá ĐÚNG mọi vòng lặp bài 1-4, không chỉ MỘT trường hợp riêng."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.vong-lap-la-fold-tren-state]
requires: [kna.phat-hien-khong-tien-trien]
concepts: [kna.vong-lap-la-fold-tren-state]
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
Bốn bài trước đã viết BỐN vòng `while` khác nhau — cơ bản, có step cap,
dừng sớm, phát hiện không tiến triển. Mỗi lần LÀ một hàm mới, nhưng NHÌN KỸ:
cả bốn đều LÀM ĐÚNG một việc — lặp gọi MỘT bước, cập nhật trạng thái, kiểm
tra điều kiện dừng. Lập trình hàm có TÊN cho hình dạng này: **fold trên
State**. Bài này rút hình dạng chung đó ra thành MỘT hàm duy nhất.
::::

::::explain{#buoc_la_state_va_fold_la_vong_lap}
`chapter-27-monads-bind.chapter.md` dạy `chain`/`flatMap`: áp một hàm lên
kết quả của bước TRƯỚC, VÀ DÙNG THẲNG kết quả đó — không bọc thêm một lớp
container nào. `andThen` (T9.3 bài `5`) chính LÀ `chain` cho `KetQua<T, E>`.
Một `Buoc<TrangThai, KetQuaMotBuoc>` LÀ dạng tối giản của một bước **State**
trong TypeScript — một hàm nhận trạng thái HIỆN TẠI, trả về trạng thái MỚI
CỘNG một kết quả cục bộ của bước đó:

```typescript title=readonly
type Buoc<TrangThai, KetQuaMotBuoc> = (
  trangThai: TrangThai,
) => { trangThaiMoi: TrangThai; ketQua: KetQuaMotBuoc };

// Fold quen thuoc tren MANG huu han:
const tongMang = [1, 2, 3, 4].reduce((acc, x) => acc + x, 0);
console.log("tong mang:", tongMang);
```

```text title=readonly
tong mang: 10
```

`Array.reduce` LÀ fold trên một danh sách HỮU HẠN, có sẵn TRƯỚC khi fold bắt
đầu chạy. Một agent loop khác Ở chỗ: nó KHÔNG có sẵn một danh sách bước —
mỗi bước được SINH RA từ chính bước trước, VÀ số bước KHÔNG biết trước
(dừng khi nào tuỳ vào chính kết quả). Đó chính LÀ lý do cần một `Buoc<S, R>`
— một "state transition" — thay vì một mảng có sẵn.
::::

::::example{#fold_vong_lap_tuong_duong_bai_2}
`foldVongLap` ÁP DỤNG LẠI CÙNG một `Buoc` nhiều lần, gấp (fold) trạng thái
qua từng bước, cho tới khi `ketQua` báo `"xong"`/`"loi"` (ĐÚNG biến thể cuối
của `KetQua<T, E>`, T9.3 bài `5`) hoặc hết `soBuocToiDa`. `taoBuocTuTacVu`
bọc một `TacVuMoPhong` (bài `1`) thành một `Buoc` — VÀ kết quả PHẢI giống hệt
`chayVongLapCoCap` (bài `2`) trên CÙNG tác vụ:

```typescript title=readonly
type KetQuaBuoc = { trangThai: "xong" | "chua_xong" | "loi"; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };
interface TacVuMoPhong { demBuoc: DemBuoc; chayMotBuoc(): KetQuaBuoc; }

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

type Buoc<TrangThai, KetQuaMotBuoc> = (
  trangThai: TrangThai,
) => { trangThaiMoi: TrangThai; ketQua: KetQuaMotBuoc };

type TrangThaiFold = { soBuoc: number };

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function foldVongLap(
  buoc: Buoc<TrangThaiFold, KetQuaBuoc>,
  trangThaiBanDau: TrangThaiFold,
  soBuocToiDa: number,
): KetQuaVongLapCoCap {
  let trangThai = trangThaiBanDau;
  while (trangThai.soBuoc < soBuocToiDa) {
    const { trangThaiMoi, ketQua } = buoc(trangThai);
    trangThai = trangThaiMoi;
    if (ketQua.trangThai === "xong") return { trangThai: "thanhCong", giaTri: ketQua.giaTri ?? "" };
    if (ketQua.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

function taoBuocTuTacVu(tacVu: TacVuMoPhong): Buoc<TrangThaiFold, KetQuaBuoc> {
  return (trangThai: TrangThaiFold) => {
    const ketQua = tacVu.chayMotBuoc();
    return { trangThaiMoi: { soBuoc: trangThai.soBuoc + 1 }, ketQua };
  };
}

// Doi chieu voi kieu menh lenh cua bai 2:
function chayVongLapCoCap(tacVu: TacVuMoPhong, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "" };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

const tvA = taoTacVuHoiTuSauNBuoc(3);
const ketQuaFold = foldVongLap(taoBuocTuTacVu(tvA), { soBuoc: 0 }, 5);
console.log("fold:", JSON.stringify(ketQuaFold));

const tvB = taoTacVuHoiTuSauNBuoc(3);
const ketQuaCu = chayVongLapCoCap(tvB, 5);
console.log("cu:  ", JSON.stringify(ketQuaCu));

console.log("giong nhau:", JSON.stringify(ketQuaFold) === JSON.stringify(ketQuaCu));
```

```text title=readonly
fold: {"trangThai":"thanhCong","giaTri":"hoan_thanh"}
cu:   {"trangThai":"thanhCong","giaTri":"hoan_thanh"}
giong nhau: true
```

Hai cách viết — MỘT vòng `while` mệnh lệnh trực tiếp (bài `2`), MỘT hàm
`foldVongLap` tổng quát ÁP DỤNG một `Buoc` — cho ra ĐÚNG cùng một kết quả,
từng ký tự. `chayVongLapCoCap` KHÔNG phải một cách viết KHÁC — nó LÀ một
TRƯỜNG HỢP RIÊNG của `foldVongLap`, VỚI `Buoc` cụ thể LÀ "gọi
`tacVu.chayMotBuoc()` VÀ tăng `soBuoc` thêm `1`".
::::

::::predict{#doan-giam-cap-fold commitOnce}
Nếu `soBuocToiDa` truyền vào `foldVongLap` LÀ `2` (thay vì `5`) trên CÙNG
`taoTacVuHoiTuSauNBuoc(3)` (cần THẬT `3` bước để `"xong"`) — kết quả LÀ gì?

:::opt{correct}
`{ trangThai: "hetBuoc" }` — tác vụ cần `3` bước nhưng cap chỉ cho `2`, nên
`trangThai.soBuoc` chạm `2` (điều kiện `while` sai) TRƯỚC KHI `ketQua.trangThai`
kịp LÀ `"xong"`; `foldVongLap` gọi `buoc` ĐÚNG `2` lần rồi dừng, y hệt cách
`chayVongLapCoCap` phản ứng VỚI cùng tình huống Ở bài `2`
:::
:::opt
`{ trangThai: "thanhCong", giaTri: "hoan_thanh" }` — `foldVongLap` LÀ một
hàm TỔNG QUÁT hơn, nên nó "linh hoạt" hơn VÀ vẫn chờ tác vụ xong dù cap nhỏ
hơn
::why
Nhầm rằng khái quát hoá (viết lại thành `foldVongLap`) làm THAY ĐỔI hành vi
— nhưng `foldVongLap` chỉ LÀ một cách viết KHÁC của ĐÚNG cùng logic
`chayVongLapCoCap`, dùng CHUNG điều kiện dừng `trangThai.soBuoc < soBuocToiDa`.

Chỗ lệch: `foldVongLap` không hề "linh hoạt hơn" bài `2` — nó tuân theo
ĐÚNG một quy tắc: vòng lặp dừng khi `soBuoc` (bên trong trạng thái fold)
chạm `soBuocToiDa`, bất kể tác vụ có tiến triển tốt tới đâu.
::
:::
:::opt
Không xác định được — `foldVongLap` VÀ `chayVongLapCoCap` dùng hai kiểu
trạng thái khác nhau (`TrangThaiFold` VÀ một biến `soBuoc` trần), nên không
thể so sánh trực tiếp
::why
Nhầm rằng KHÁC BIỆT VỀ CÁCH BIỂU DIỄN (một bên bọc `soBuoc` trong object
`{ soBuoc }`, một bên dùng biến trần) nghĩa LÀ khác biệt VỀ HÀNH VI — nhưng
`taoBuocTuTacVu` tăng `trangThai.soBuoc` thêm ĐÚNG `1` mỗi bước, y hệt
`soBuoc++` bên phía mệnh lệnh.

Chỗ lệch: hai cách biểu diễn trạng thái (object `{soBuoc}` so VỚI biến trần)
LÀ tương đương ngữ nghĩa hoàn toàn — ví dụ VÀ test Ở bài này đã xác nhận
`JSON.stringify` của hai kết quả BẰNG NHAU trên CÙNG một tác vụ.
::
:::
::::

::::code{#viet_fold_vong_lap}
Hoàn thiện `foldVongLap` — lặp `while` khi `trangThai.soBuoc < soBuocToiDa`,
mỗi lần gọi `buoc(trangThai)` để lấy `{ trangThaiMoi, ketQua }`, cập nhật
`trangThai = trangThaiMoi`; NẾU `ketQua.trangThai` LÀ `"xong"`, trả về
`{ trangThai: "thanhCong", giaTri: ... }` NGAY; NẾU LÀ `"loi"`, trả
`{ trangThai: "loi", loi: "loi_tac_vu" }` NGAY; hết vòng lặp thì trả
`{ trangThai: "hetBuoc" }`. Hoàn thiện `taoBuocTuTacVu` — trả về một hàm
nhận `trangThai`, gọi `tacVu.chayMotBuoc()`, trả về
`{ trangThaiMoi: { soBuoc: trangThai.soBuoc + 1 }, ketQua }`.

```typescript title=starter
type KetQuaBuoc = { trangThai: "xong" | "chua_xong" | "loi"; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };
interface TacVuMoPhong { demBuoc: DemBuoc; chayMotBuoc(): KetQuaBuoc; }

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

type Buoc<TrangThai, KetQuaMotBuoc> = (
  trangThai: TrangThai,
) => { trangThaiMoi: TrangThai; ketQua: KetQuaMotBuoc };

type TrangThaiFold = { soBuoc: number };

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function foldVongLap(
  buoc: Buoc<TrangThaiFold, KetQuaBuoc>,
  trangThaiBanDau: TrangThaiFold,
  soBuocToiDa: number,
): KetQuaVongLapCoCap {
  ___
}

function taoBuocTuTacVu(tacVu: TacVuMoPhong): Buoc<TrangThaiFold, KetQuaBuoc> {
  ___
}

function chayFoldTrenNhieuTacVu(dsTacVu: TacVuMoPhong[], soBuocToiDa: number): KetQuaVongLapCoCap[] {
  const ketQua: KetQuaVongLapCoCap[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(foldVongLap(taoBuocTuTacVu(tv), { soBuoc: 0 }, soBuocToiDa));
  }
  return ketQua;
}

const dsTacVu3: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(1),
  taoTacVuHoiTuSauNBuoc(10),
];
const ketQua3 = chayFoldTrenNhieuTacVu(dsTacVu3, 5);
console.log(JSON.stringify(ketQua3.map((k) => k.trangThai)));
```

```typescript title=solution
type KetQuaBuoc = { trangThai: "xong" | "chua_xong" | "loi"; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };
interface TacVuMoPhong { demBuoc: DemBuoc; chayMotBuoc(): KetQuaBuoc; }

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

type Buoc<TrangThai, KetQuaMotBuoc> = (
  trangThai: TrangThai,
) => { trangThaiMoi: TrangThai; ketQua: KetQuaMotBuoc };

type TrangThaiFold = { soBuoc: number };

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function foldVongLap(
  buoc: Buoc<TrangThaiFold, KetQuaBuoc>,
  trangThaiBanDau: TrangThaiFold,
  soBuocToiDa: number,
): KetQuaVongLapCoCap {
  let trangThai = trangThaiBanDau;
  while (trangThai.soBuoc < soBuocToiDa) {
    const { trangThaiMoi, ketQua } = buoc(trangThai);
    trangThai = trangThaiMoi;
    if (ketQua.trangThai === "xong") return { trangThai: "thanhCong", giaTri: ketQua.giaTri ?? "" };
    if (ketQua.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

function taoBuocTuTacVu(tacVu: TacVuMoPhong): Buoc<TrangThaiFold, KetQuaBuoc> {
  return (trangThai: TrangThaiFold) => {
    const ketQua = tacVu.chayMotBuoc();
    return { trangThaiMoi: { soBuoc: trangThai.soBuoc + 1 }, ketQua };
  };
}

function chayFoldTrenNhieuTacVu(dsTacVu: TacVuMoPhong[], soBuocToiDa: number): KetQuaVongLapCoCap[] {
  const ketQua: KetQuaVongLapCoCap[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(foldVongLap(taoBuocTuTacVu(tv), { soBuoc: 0 }, soBuocToiDa));
  }
  return ketQua;
}

const dsTacVu3: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(1),
  taoTacVuHoiTuSauNBuoc(10),
];
const ketQua3 = chayFoldTrenNhieuTacVu(dsTacVu3, 5);
console.log(JSON.stringify(ketQua3.map((k) => k.trangThai)));
```

```typescript title=test
if (ketQua3.length !== 3) throw new Error("chayFoldTrenNhieuTacVu phai tra ve mang dung 3 phan tu");
if (JSON.stringify(ketQua3.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "thanhCong", "hetBuoc"])) {
  throw new Error("hai tac vu hoi tu trong cap phai la thanhCong, tac vu can 10 buoc voi cap 5 phai la hetBuoc");
}

const tv3 = taoTacVuHoiTuSauNBuoc(3);
const kq3 = foldVongLap(taoBuocTuTacVu(tv3), { soBuoc: 0 }, 5);
if (kq3.trangThai !== "thanhCong") throw new Error("foldVongLap tren tac vu hoi tu sau 3 buoc, cap 5, phai la thanhCong");
if (kq3.trangThai === "thanhCong" && kq3.giaTri !== "hoan_thanh") throw new Error("gia tri tra ve phai la hoan_thanh");
if (tv3.demBuoc.soBuocDaChay !== 3) throw new Error("foldVongLap phai dung lai NGAY khi tac vu bao xong, dung 3 buoc goi that");

const tvCap2 = taoTacVuHoiTuSauNBuoc(3);
const kqCap2 = foldVongLap(taoBuocTuTacVu(tvCap2), { soBuoc: 0 }, 2);
if (kqCap2.trangThai !== "hetBuoc") throw new Error("cap 2 nho hon 3 buoc can thiet phai cho ra hetBuoc, KHONG phai thanhCong");
if (tvCap2.demBuoc.soBuocDaChay !== 2) throw new Error("foldVongLap phai goi DUNG soBuocToiDa lan khi chua xong, khong hon khong kem");

const buocRieng = taoBuocTuTacVu(taoTacVuHoiTuSauNBuoc(1));
const { trangThaiMoi, ketQua: ketQuaBuocRieng } = buocRieng({ soBuoc: 0 });
if (trangThaiMoi.soBuoc !== 1) throw new Error("taoBuocTuTacVu phai tang soBuoc dung 1 moi lan goi buoc");
if (ketQuaBuocRieng.trangThai !== "xong") throw new Error("buoc rieng tren tac vu hoi tu sau 1 buoc phai tra ve trangThai xong ngay lan goi dau");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (foldVongLap): mot vong while dieu kien trangThai.soBuoc < soBuocToiDa, moi lan goi buoc(trangThai) lay ca trangThaiMoi VA ketQua, gan lai trangThai = trangThaiMoi; neu ketQua.trangThai la 'xong' tra thanhCong NGAY, neu la 'loi' tra loi NGAY; het vong lap tra hetBuoc. Cho hai (taoBuocTuTacVu): tra ve MOT HAM nhan trangThai, goi tacVu.chayMotBuoc(), tra ve { trangThaiMoi: { soBuoc: trangThai.soBuoc + 1 }, ketQua }."
- kind: strategy
  body: "Cho dau: let trangThai = trangThaiBanDau; while (trangThai.soBuoc < soBuocToiDa) { const { trangThaiMoi, ketQua } = buoc(trangThai); trangThai = trangThaiMoi; if (ketQua.trangThai === \"xong\") return { trangThai: \"thanhCong\", giaTri: ketQua.giaTri ?? \"\" }; if (ketQua.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\" }; } return { trangThai: \"hetBuoc\" }; Cho hai: return (trangThai: TrangThaiFold) => { const ketQua = tacVu.chayMotBuoc(); return { trangThaiMoi: { soBuoc: trangThai.soBuoc + 1 }, ketQua }; };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[\"thanhCong\",\"thanhCong\",\"hetBuoc\"]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`foldVongLap` không dạy một CƠ CHẾ mới — nó đặt tên CHÍNH THỨC cho hình dạng
mà cả bốn bài trước ĐÃ dùng: lặp một `Buoc`, tích luỹ trạng thái, dừng khi
đạt biến thể `KetQua` cuối. BOSS khép quest ráp NGUYÊN VĂN bốn cơ chế đó lại
— step cap, tránh dừng sớm sai, phát hiện không tiến triển — trên MỘT bộ tác
vụ hỗn hợp, đo `steps-to-success` VÀ `%chạm step cap`.
::::

::::reflect{#nghi-lai}
`Buoc<TrangThai, KetQuaMotBuoc>` VÀ `foldVongLap` không phải khái niệm MỚI —
chúng LÀ cái TÊN chính thức cho đúng thứ bốn bài trước đã làm bằng tay: mỗi
vòng `while` Ở bài `1`-`4` ĐỀU (a) giữ một trạng thái (số bước, đôi khi cộng
thêm `chuKyTruoc`/`soLanLapLaiLienTiep`), (b) áp dụng LẠI cùng một phép biến
đổi mỗi vòng, VÀ (c) dừng khi kết quả cục bộ đạt một trong các biến thể cuối
(`"xong"`, `"loi"`, hay Ở bài `4` LÀ "đã lặp đủ nhiều"). Đó chính LÀ fold:
gấp một chuỗi thao tác LẶP LẠI thành MỘT giá trị cuối cùng, y hệt
`Array.reduce` gấp một mảng — chỉ khác chuỗi Ở đây KHÔNG có sẵn TRƯỚC, mà
được SINH RA từng bước một, VÀ dừng SỚM (không phải luôn duyệt hết) ngay khi
đạt một trạng thái cuối — đúng cách `andThen` (T9.3 bài `5`) "trượt" qua một
lỗi thay vì tiếp tục chạy bước sau nó.
::::

::::checkpoint{mastery=0.9}
::::
