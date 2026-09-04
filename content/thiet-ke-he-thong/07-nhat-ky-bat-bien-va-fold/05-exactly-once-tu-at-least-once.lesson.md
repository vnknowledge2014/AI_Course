---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.exactly-once-tu-at-least-once
title: "Exactly-once: giao lặp bao nhiêu lần, tính đúng một lần"
summary: "xuLyStreamCoTheLap(banDau, cacSuKien) mo phong he thong giao AT-LEAST-ONCE (cung id gui 2-3 lan) roi fold qua apDungIdempotent (bai 4) -- ket qua CUOI GIONG HET nhu chi nhan dung 1 lan; vi du 3 su kien phan biet giao 6 lan (co lap) van ra dung soDu 85000, soLanApDungThat=3, soLanBoQuaViTrung=3 -- chung minh cong thuc 'exactly-once = at-least-once + idempotency' bang so lieu that."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.exactly-once-tu-at-least-once]
requires: [sd.fp.ap-dung-idempotent]
concepts: [sd.fp.exactly-once-tu-at-least-once]
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
Câu hỏi để ngỏ từ bài trước: `apDungIdempotent` cho ra CÙNG kết quả dù
áp dụng một sự kiện một lần hay nhiều lần. Nhưng đó mới chỉ là tính
chất của MỘT hàm. Câu hỏi lớn hơn: nếu cả một HỆ THỐNG giao sự kiện
kiểu at-least-once (có thể gửi lặp bất kỳ lúc nào, bất kỳ số lần nào),
kết quả CUỐI cùng có còn đáng tin không? Đây LÀ lúc chứng minh, không
chỉ nói suông.
::::

::::explain{#at-least-once-cong-idempotent-bang-exactly-once}
`xuLyStreamCoTheLap` mô phỏng một hệ thống giao at-least-once: nhận
một danh sách sự kiện CÓ THỂ chứa bản sao (cùng `id` xuất hiện nhiều
lần), xử lý từng phần tử qua `apDungIdempotent`, đồng thời đếm số lần
ÁP DỤNG THẬT VÀ số lần BỊ BỎ QUA vì trùng:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }
function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}

interface KetQuaXuLyStream { trangThaiCuoi: TrangThai; soLanApDungThat: number; soLanBoQuaViTrung: number; }
function xuLyStreamCoTheLap(banDau: TrangThai, cacSuKien: SuKien[]): KetQuaXuLyStream {
  let trangThai = banDau;
  let apDungThat = 0;
  let boQua = 0;
  for (const sk of cacSuKien) {
    if (trangThai.idDaApDung.has(sk.id)) {
      boQua += 1;
    } else {
      trangThai = apDungIdempotent(trangThai, sk);
      apDungThat += 1;
    }
  }
  return { trangThaiCuoi: trangThai, soLanApDungThat: apDungThat, soLanBoQuaViTrung: boQua };
}

const suKienGoc: SuKien[] = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 20000 },
  { id: "e3", loai: "da_nap", soTien: 5000 },
];

const giaoThat: SuKien[] = [
  suKienGoc[0]!, suKienGoc[1]!, suKienGoc[0]!, suKienGoc[2]!, suKienGoc[1]!, suKienGoc[1]!,
];

const ketQuaExactlyOnce = xuLyStreamCoTheLap(trangThaiBanDau(), giaoThat);
const ketQuaLyTuong = suKienGoc.reduce(apDungIdempotent, trangThaiBanDau());

console.log("he thong GIAO (co lap):", giaoThat.length, "lan giao, cho", suKienGoc.length, "su kien PHAN BIET");
console.log("so lan AP DUNG THAT (khong trung):", ketQuaExactlyOnce.soLanApDungThat);
console.log("so lan BI BO QUA (vi trung):", ketQuaExactlyOnce.soLanBoQuaViTrung);
console.log("so du CUOI (tu stream co lap):", ketQuaExactlyOnce.trangThaiCuoi.soDu);
console.log("so du CUOI (neu chi nhan dung 1 lan moi su kien):", ketQuaLyTuong.soDu);
console.log("hai ket qua co GIONG HET nhau khong?", ketQuaExactlyOnce.trangThaiCuoi.soDu === ketQuaLyTuong.soDu);
```

```text title=readonly
he thong GIAO (co lap): 6 lan giao, cho 3 su kien PHAN BIET
so lan AP DUNG THAT (khong trung): 3
so lan BI BO QUA (vi trung): 3
so du CUOI (tu stream co lap): 85000
so du CUOI (neu chi nhan dung 1 lan moi su kien): 85000
hai ket qua co GIONG HET nhau khong? true
```

Hệ thống giao `6` lần cho đúng `3` sự kiện phân biệt (`e1` hai lần,
`e2` ba lần, `e3` một lần). `xuLyStreamCoTheLap` chỉ áp dụng THẬT
đúng `3` lần — mỗi lần còn lại bị nhận ra là trùng và bỏ qua. Kết quả
cuối (`85000`) khớp CHÍNH XÁC với việc giả định hệ thống hoàn hảo,
chỉ giao mỗi sự kiện đúng một lần. Đây chính là công thức: hệ thống
giao at-least-once, cộng một phép áp dụng idempotent, cho hiệu ứng
exactly-once — dù bản thân việc GIAO chưa từng là exactly-once.
::::

::::example{#bat-ky-thu-tu-va-so-lan-lap-nao}
Kết quả không phụ thuộc vào THỨ TỰ hay SỐ LẦN lặp cụ thể — miễn mỗi
sự kiện phân biệt xuất hiện Ít NHẤT một lần, kết quả cuối luôn khớp:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }
function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}
interface KetQuaXuLyStream { trangThaiCuoi: TrangThai; soLanApDungThat: number; soLanBoQuaViTrung: number; }
function xuLyStreamCoTheLap(banDau: TrangThai, cacSuKien: SuKien[]): KetQuaXuLyStream {
  let trangThai = banDau;
  let apDungThat = 0;
  let boQua = 0;
  for (const sk of cacSuKien) {
    if (trangThai.idDaApDung.has(sk.id)) {
      boQua += 1;
    } else {
      trangThai = apDungIdempotent(trangThai, sk);
      apDungThat += 1;
    }
  }
  return { trangThaiCuoi: trangThai, soLanApDungThat: apDungThat, soLanBoQuaViTrung: boQua };
}

const suKienGoc: SuKien[] = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 20000 },
  { id: "e3", loai: "da_nap", soTien: 5000 },
];

const giaoThatKhac: SuKien[] = [
  suKienGoc[2]!, suKienGoc[2]!, suKienGoc[0]!, suKienGoc[1]!, suKienGoc[2]!, suKienGoc[0]!, suKienGoc[1]!, suKienGoc[0]!,
];
const ketQua2 = xuLyStreamCoTheLap(trangThaiBanDau(), giaoThatKhac);
console.log("thu tu VA so lan lap KHAC hoan toan:", giaoThatKhac.length, "lan giao");
console.log("so lan ap dung that:", ketQua2.soLanApDungThat);
console.log("so du cuoi:", ketQua2.trangThaiCuoi.soDu);
console.log("van giong ket qua ly tuong (85000)?", ketQua2.trangThaiCuoi.soDu === 85000);
```

```text title=readonly
thu tu VA so lan lap KHAC hoan toan: 8 lan giao
so lan ap dung that: 3
so du cuoi: 85000
van giong ket qua ly tuong (85000)? true
```

Lần này hệ thống giao tới `8` lần, theo thứ tự HOÀN TOÀN khác (`e3`
trước, `e1` sau) — nhưng `soLanApDungThat` vẫn dừng đúng Ở `3`, và số
dư cuối vẫn là `85000`. Miễn `apDungIdempotent` chặn đúng các bản
trùng, "hệ thống giao bao nhiêu lần theo thứ tự nào" không còn quan
trọng với KẾT QUẢ cuối cùng.
::::

::::predict{#doan-giao-muoi-lan-cho-ba-su-kien commitOnce}
Một hệ thống at-least-once giao tổng cộng `10` lần cho đúng `3` sự
kiện phân biệt (mỗi sự kiện được giao Ít NHẤT một lần, có thể nhiều
hơn, theo bất kỳ thứ tự nào). Sau khi `xuLyStreamCoTheLap` xử lý hết
`10` lần giao đó, `soLanApDungThat` là bao nhiêu?

:::opt{correct}
`3` — luôn bằng đúng số sự kiện PHÂN BIỆT (tính theo `id`), bất kể
tổng số lần hệ thống giao là `10` hay bất kỳ con số nào khác lớn hơn
hoặc bằng `3`
:::
:::opt
`10` — vì hàm xử lý đúng `10` lần gọi trong vòng lặp, nên
`soLanApDungThat` phải phản ánh đúng số lần hàm CHẠY qua
::why
Nhầm "số lần vòng lặp CHẠY QUA một phần tử" với "số lần TRẠNG THÁI
thực sự bị thay đổi" — nhưng `soLanApDungThat` chỉ tăng Ở nhánh
`else`, không tăng Ở nhánh `boQua += 1`.

Chỗ lệch: trong thân vòng lặp, mỗi sự kiện được kiểm tra
`trangThai.idDaApDung.has(sk.id)` trước — nếu ĐÃ có, rơi vào nhánh
`boQua += 1` VÀ không hề gọi `apDungIdempotent`. Chỉ những `id` CHƯA
từng gặp mới rơi vào nhánh `else`, tăng `apDungThat`. Với `3` `id`
phân biệt, đúng `3` lần rơi vào nhánh đó, bất kể tổng số lần giao là
bao nhiêu — phần chênh lệch (`10 - 3 = 7`) toàn bộ rơi vào
`soLanBoQuaViTrung`.
::
:::
::::

::::code{#viet_xu_ly_stream_co_the_lap}
Hoàn thiện `xuLyStreamCoTheLap` — duyệt qua `cacSuKien`, với mỗi sự
kiện: nếu `id` đã có trong `trangThai.idDaApDung` thì tăng `boQua`;
ngược lại gọi `apDungIdempotent` để cập nhật `trangThai` VÀ tăng
`apDungThat`. Trả về `trangThaiCuoi`, `soLanApDungThat`,
`soLanBoQuaViTrung`. PHẢI bắt đầu TỪ `banDau` được truyền vào, không
phải từ một trạng thái rỗng mới.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }
function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}
interface KetQuaXuLyStream { trangThaiCuoi: TrangThai; soLanApDungThat: number; soLanBoQuaViTrung: number; }

function xuLyStreamCoTheLap(banDau: TrangThai, cacSuKien: SuKien[]): KetQuaXuLyStream {
  ___
}

const kqX = xuLyStreamCoTheLap(trangThaiBanDau(), [
  { id: "p", loai: "da_nap", soTien: 300 },
  { id: "p", loai: "da_nap", soTien: 300 },
  { id: "q", loai: "da_tru", soTien: 50 },
]);
console.log(kqX.trangThaiCuoi.soDu, kqX.soLanApDungThat, kqX.soLanBoQuaViTrung);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }
function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}
interface KetQuaXuLyStream { trangThaiCuoi: TrangThai; soLanApDungThat: number; soLanBoQuaViTrung: number; }

function xuLyStreamCoTheLap(banDau: TrangThai, cacSuKien: SuKien[]): KetQuaXuLyStream {
  let trangThai = banDau;
  let apDungThat = 0;
  let boQua = 0;
  for (const sk of cacSuKien) {
    if (trangThai.idDaApDung.has(sk.id)) {
      boQua += 1;
    } else {
      trangThai = apDungIdempotent(trangThai, sk);
      apDungThat += 1;
    }
  }
  return { trangThaiCuoi: trangThai, soLanApDungThat: apDungThat, soLanBoQuaViTrung: boQua };
}

const kqX = xuLyStreamCoTheLap(trangThaiBanDau(), [
  { id: "p", loai: "da_nap", soTien: 300 },
  { id: "p", loai: "da_nap", soTien: 300 },
  { id: "q", loai: "da_tru", soTien: 50 },
]);
console.log(kqX.trangThaiCuoi.soDu, kqX.soLanApDungThat, kqX.soLanBoQuaViTrung);
```

```typescript title=test
const banDau = trangThaiBanDau();
const events: SuKien[] = [
  { id: "a", loai: "da_nap", soTien: 500 },
  { id: "b", loai: "da_tru", soTien: 100 },
  { id: "a", loai: "da_nap", soTien: 500 },
];
const kq = xuLyStreamCoTheLap(banDau, events);
if (kq.trangThaiCuoi.soDu !== 400) throw new Error("so du cuoi phai la 500 - 100 = 400 (su kien a chi tinh 1 lan)");
if (kq.soLanApDungThat !== 2) throw new Error("chi co 2 su kien PHAN BIET (a, b) nen soLanApDungThat phai la 2");
if (kq.soLanBoQuaViTrung !== 1) throw new Error("su kien a xuat hien 2 lan trong stream nen phai co dung 1 lan BI BO QUA");
if (banDau.soDu !== 0 || banDau.idDaApDung.size !== 0) throw new Error("xuLyStreamCoTheLap KHONG duoc mutate trangThai BAN DAU truyen vao");

const kqRong = xuLyStreamCoTheLap(trangThaiBanDau(), []);
if (kqRong.soLanApDungThat !== 0 || kqRong.soLanBoQuaViTrung !== 0) throw new Error("stream rong khong duoc ap dung hay bo qua gi ca");

const suKienLap: SuKien = { id: "z", loai: "da_nap", soTien: 50 };
const kqLap = xuLyStreamCoTheLap(trangThaiBanDau(), [suKienLap, suKienLap, suKienLap, suKienLap]);
if (kqLap.soLanApDungThat !== 1) throw new Error("4 lan giao CUNG 1 id phai chi ap dung THAT 1 lan");
if (kqLap.soLanBoQuaViTrung !== 3) throw new Error("4 lan giao CUNG 1 id phai bo qua DUNG 3 lan con lai");
if (kqLap.trangThaiCuoi.soDu !== 50) throw new Error("so du cuoi phai la 50 (chi cong 1 lan)");

const banDauGia: TrangThai = { soDu: 100000, idDaApDung: new Set(["already"]) };
const kqGia = xuLyStreamCoTheLap(banDauGia, [{ id: "new1", loai: "da_nap", soTien: 10 }]);
if (kqGia.trangThaiCuoi.soDu !== 100010) throw new Error("phai CONG DON len tren trangThai BAN DAU duoc truyen vao, khong bat dau lai tu 0");
if (!kqGia.trangThaiCuoi.idDaApDung.has("already")) throw new Error("idDaApDung cua ket qua phai GIU LAI cac id da co san trong trangThai ban dau");
if (banDauGia.soDu !== 100000) throw new Error("xuLyStreamCoTheLap KHONG duoc mutate tham so banDau truyen vao");
```

:::hints
- kind: attention
  body: "Bat dau 'let trangThai = banDau;' (KHONG phai trangThaiBanDau()). Dung mot vong lap for...of qua cacSuKien: neu id da co trong trangThai.idDaApDung thi boQua += 1; nguoc lai goi apDungIdempotent de cap nhat trangThai VA apDungThat += 1."
- kind: strategy
  body: "let trangThai = banDau; let apDungThat = 0; let boQua = 0; for (const sk of cacSuKien) { if (trangThai.idDaApDung.has(sk.id)) { boQua += 1; } else { trangThai = apDungIdempotent(trangThai, sk); apDungThat += 1; } } return { trangThaiCuoi: trangThai, soLanApDungThat: apDungThat, soLanBoQuaViTrung: boQua };"
- kind: one-line
  body: "let trangThai = banDau; let apDungThat = 0; let boQua = 0; for (const sk of cacSuKien) { if (trangThai.idDaApDung.has(sk.id)) { boQua += 1; } else { trangThai = apDungIdempotent(trangThai, sk); apDungThat += 1; } } return { trangThaiCuoi: trangThai, soLanApDungThat: apDungThat, soLanBoQuaViTrung: boQua };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "250 2 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Exactly-once xây LẠI hoàn toàn từ at-least-once cộng idempotency —
không cần một cơ chế "chỉ giao đúng một lần" tốn kém nào cả. Nhưng cả
`apDung` lẫn `apDungIdempotent` mới chỉ trả lời "trạng thái biến đổi
thế nào". Ai quyết định MỘT sự kiện có được PHÉP xảy ra hay không?
::::

::::reflect{#nghi-lai}
"Exactly-once = at-least-once + idempotency" không phải một khẩu hiệu
— `xuLyStreamCoTheLap` chứng minh nó bằng số liệu cụ thể: `6` lần
giao, `3` lần áp dụng thật, VÀ kết quả cuối khớp tuyệt đối với kịch
bản lý tưởng. Hệ thống giao KHÔNG cần hoàn hảo — nó chỉ cần đảm bảo
"giao Ít NHẤT một lần"; phần còn lại — lọc trùng — nằm trọn trong
phép fold, không cần một tầng hạ tầng riêng nào để "đảm bảo đúng một
lần" Ở khâu truyền tải.
::::

::::checkpoint{mastery=0.74}
::::
