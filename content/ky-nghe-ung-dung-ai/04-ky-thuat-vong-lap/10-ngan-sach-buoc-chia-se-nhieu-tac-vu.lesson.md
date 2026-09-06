---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.ngan-sach-buoc-chia-se-nhieu-tac-vu
title: "Ngân sách bước CHIA SẺ giữa nhiều tác vụ — chia đều VS dùng trước nhường sau"
summary: "Hai chiến lược chia MỘT tổng ngân sách bước cố định cho N tác vụ CHẠY TUẦN TỰ (KHÁC hẳn mỗi tác vụ có ngân sách riêng VÔ HẠN như q9.4a-q9.4b bài 1-9). phanBoDeuVaChay(dsTacVu, tongNganSach) cho MỖI tác vụ Math.floor(tongNganSach/dsTacVu.length) bước, BẤT KỂ tác vụ đó cần bao nhiêu. dungTruocNhuongSauVaChay(dsTacVu, tongNganSach) cho tác vụ ĐẦU TIÊN toàn bộ ngân sách CÒN LẠI, trừ đi số bước NÓ THẬT SỰ dùng, rồi nhường phần dư cho tác vụ SAU. Trên bộ 3 tác vụ [cần 2, cần 7, cần 3 bước], tổng ngân sách=9: chia đều (mỗi tác vụ 3 bước) cho [thanhCong,hetBuoc,thanhCong] -- tác vụ cần-7 KHÔNG đủ phần chia; dùng-trước-nhường-sau (thứ tự [2,7,3]) cho [thanhCong,thanhCong,hetBuoc] -- tác vụ cần-7 ĐƯỢC cứu (dùng 7/7 bước còn lại) nhưng tác vụ CUỐI (cần 3) không còn ngân sách nào (cap=0). Đảo thứ tự thành [3,7,2] (CÙNG tổng ngân sách): dùng-trước-nhường-sau CHỈ tác vụ ĐẦU (cần 3) thành công, hai tác vụ sau đều hetBuoc -- chứng minh THỨ TỰ tác vụ quyết định kết cục của chiến lược dùng-trước, không chỉ tổng nhu cầu."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.ngan-sach-buoc-chia-se-nhieu-tac-vu]
requires: [kna.goi-tool-la-kleisli-retry-la-alternative]
concepts: [kna.ngan-sach-buoc-chia-se-nhieu-tac-vu]
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
MỌI tác vụ Ở chín bài trước có `soBuocToiDa` RIÊNG — như thể mỗi tác vụ
sống trong vũ trụ CỦA RIÊNG NÓ, không quan tâm tác vụ khác. Một hệ thống
thật hiếm khi hào phóng như vậy: N tác vụ THƯỜNG chia nhau MỘT tổng ngân
sách bước cố định, chạy TUẦN TỰ. Bài này dạy đúng câu hỏi đó — chia ngân
sách CHUNG đó như thế nào — VÀ cho thấy hai cách chia hợp lý có thể cứu
được HAI tác vụ HOÀN TOÀN khác nhau.
::::

::::explain{#chia_deu_vs_dung_truoc_nhuong_sau}
`phanBoDeuVaChay` chia TỔNG ngân sách ĐỀU cho mọi tác vụ NGAY TỪ ĐẦU —
mỗi tác vụ nhận `Math.floor(tongNganSach / dsTacVu.length)` bước LÀM
`soBuocToiDa` CỦA RIÊNG NÓ, gọi `chayVongLapCoCap` (q9.4a bài `2`) y hệt
như trước — chỉ khác LÀ con số đó KHÔNG còn tuỳ ý, mà bị RÀNG BUỘC bởi
tổng chia số tác vụ:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
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

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

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

function phanBoDeuVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  const nganSachMoiTacVu = Math.floor(tongNganSach / dsTacVu.length);
  return dsTacVu.map((tv) => chayVongLapCoCap(tv, nganSachMoiTacVu));
}

const ketQuaDeu = phanBoDeuVaChay([taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7)], 9);
console.log(JSON.stringify(ketQuaDeu.map((k) => k.trangThai)));
```

```text title=readonly
["thanhCong","hetBuoc"]
```

Tổng ngân sách `9` chia đều cho `2` tác vụ = `4` bước MỖI tác vụ. Tác vụ
cần `2` bước NẰM TRONG phần chia (`2 < 4`), thành công. Tác vụ cần `7`
bước VƯỢT XA phần chia (`7 > 4`), `"hetBuoc"` — dù NẾU được cấp toàn bộ
`9`, nó ĐÃ đủ để xong.
::::

::::example{#dung_truoc_nhuong_sau_va_thu_tu_quan_trong}
`dungTruocNhuongSauVaChay` KHÔNG chia trước — nó cho tác vụ ĐẦU TIÊN
`soBuocToiDa` bằng TOÀN BỘ ngân sách CÒN LẠI, chạy `chayVongLapCoCap`,
rồi TRỪ đi số bước tác vụ đó THẬT SỰ dùng (`tv.demBuoc.soBuocDaChay`)
khỏi phần còn lại — TRƯỚC KHI chuyển sang tác vụ kế tiếp:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
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

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

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

function phanBoDeuVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  const nganSachMoiTacVu = Math.floor(tongNganSach / dsTacVu.length);
  return dsTacVu.map((tv) => chayVongLapCoCap(tv, nganSachMoiTacVu));
}

function dungTruocNhuongSauVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  let nganSachConLai = tongNganSach;
  const ketQua: KetQuaVongLapCoCap[] = [];
  for (const tv of dsTacVu) {
    const kq = chayVongLapCoCap(tv, nganSachConLai);
    ketQua.push(kq);
    nganSachConLai -= tv.demBuoc.soBuocDaChay;
  }
  return ketQua;
}

// Bo 3 tac vu [can 2, can 7, can 3 buoc], tong ngan sach = 9.
const dsDeu = [taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(3)];
const ketQuaDeu = phanBoDeuVaChay(dsDeu, 9);
console.log("chia deu:  ", JSON.stringify(ketQuaDeu.map((k) => k.trangThai)), JSON.stringify(dsDeu.map((t) => t.demBuoc.soBuocDaChay)));

const dsTruoc = [taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(3)];
const ketQuaTruoc = dungTruocNhuongSauVaChay(dsTruoc, 9);
console.log("dung truoc:", JSON.stringify(ketQuaTruoc.map((k) => k.trangThai)), JSON.stringify(dsTruoc.map((t) => t.demBuoc.soBuocDaChay)));
```

```text title=readonly
chia deu:   ["thanhCong","hetBuoc","thanhCong"] [2,3,3]
dung truoc: ["thanhCong","thanhCong","hetBuoc"] [2,7,0]
```

CÙNG `9` ngân sách, CÙNG `3` tác vụ — nhưng hai chiến lược cứu HAI tác
vụ HOÀN TOÀN khác nhau. Chia đều (mỗi tác vụ `3` bước): tác vụ cần `7`
KHÔNG đủ phần chia, `"hetBuoc"` — nhưng tác vụ CUỐI (cần `3`) vừa đủ,
thành công. Dùng trước nhường sau: tác vụ cần `7` ĐƯỢC CỨU (dùng đúng
`7` trong toàn bộ ngân sách CÒN LẠI LÀ `9`, dư `2` cho tác vụ SAU) —
nhưng tác vụ CUỐI (cần `3`) không CÒN MỘT bước ngân sách nào (`9 - 2 -
7 = 0`), `"hetBuoc"` NGAY LẬP TỨC, không được thử lấy MỘT lần.
::::

::::predict{#doan-doi-thu-tu-tac-vu commitOnce}
Nếu ĐỔI THỨ TỰ danh sách tác vụ trong `dungTruocNhuongSauVaChay` từ
`[cần 2, cần 7, cần 3]` thành `[cần 3, cần 7, cần 2]` (GIỮ NGUYÊN tổng
ngân sách `= 9`) — kết quả đổi ra sao?

:::opt{correct}
CHỈ tác vụ ĐẦU TIÊN (cần `3` bước) thành công — HAI tác vụ SAU (cần `7`,
cần `2`) đều `"hetBuoc"`: tác vụ cần-`7` dùng HẾT `6` bước còn lại
(`9 - 3 = 6`, chưa đủ `7`), để lại `0` ngân sách cho tác vụ cần-`2` cuối
cùng — nó không được thử lấy MỘT bước nào
:::
:::opt
Kết quả GIỐNG HỆT lần trước (`[thanhCong,thanhCong,hetBuoc]`) vì tổng
ngân sách VÀ tập hợp NHU CẦU của ba tác vụ không đổi, chỉ đổi thứ tự
liệt kê
::why
Nhầm rằng chiến lược dùng-trước-nhường-sau chỉ phụ thuộc TẬP HỢP nhu cầu
(`{2, 7, 3}`), không phụ thuộc THỨ TỰ chạy — nhưng `dungTruocNhuongSauVaChay`
LUÔN cho tác vụ ĐANG XÉT toàn bộ phần CÒN LẠI, VÀ tác vụ nào được xét
TRƯỚC sẽ "ăn" phần ngân sách lớn hơn, bất kể nhu cầu THẬT của nó LÀ bao
nhiêu.

Chỗ lệch: đổi thứ tự LÀ đổi tác vụ nào nhận được PHẦN LỚN của ngân sách
CÒN LẠI TRƯỚC — tác vụ cần-`7` giờ đứng THỨ HAI (sau khi tác vụ cần-`3`
đã lấy đi `3`), nên nó chỉ còn `6` ngân sách, KHÔNG đủ `7` như lần
trước (khi nó đứng THỨ HAI nhưng SAU tác vụ cần-`2`, còn nguyên `7`).
::
:::
:::opt
CẢ BA tác vụ đều thất bại (`hetBuoc`) vì tổng NHU CẦU (`3 + 7 + 2 = 12`)
vượt quá tổng ngân sách (`9`)
::why
Nhầm rằng tổng NHU CẦU vượt tổng ngân sách nghĩa LÀ KHÔNG tác vụ nào có
thể thành công — nhưng chiến lược dùng-trước cho tác vụ ĐẦU TIÊN toàn
bộ ngân sách nó CẦN mà KHÔNG bị chia nhỏ, nên nó VẪN có thể xong TRỌN
VẸN; chỉ những tác vụ Ở PHÍA SAU mới chịu thiệt khi tổng nhu cầu vượt
ngân sách.

Chỗ lệch: `nganSachConLai` Ở tác vụ ĐẦU TIÊN LUÔN LÀ `tongNganSach`
(chưa ai dùng trước nó) — nó chỉ cần `soBuocToiDa >= 3` để thành công,
VÀ `9 >= 3` LÀ đúng.
::
:::
::::

::::code{#viet_hai_chien_luoc_phan_bo}
Hoàn thiện `phanBoDeuVaChay` — tính `nganSachMoiTacVu = Math.floor(tongNganSach
/ dsTacVu.length)`, rồi gọi `chayVongLapCoCap(tv, nganSachMoiTacVu)`
cho TỪNG tác vụ, trả về mảng kết quả (dùng `.map`). Hoàn thiện
`dungTruocNhuongSauVaChay` — giữ một biến `nganSachConLai` khởi tạo
bằng `tongNganSach`; VỚI MỖI tác vụ: gọi `chayVongLapCoCap(tv,
nganSachConLai)`, đẩy kết quả vào mảng, rồi TRỪ `tv.demBuoc.soBuocDaChay`
khỏi `nganSachConLai` TRƯỚC KHI xét tác vụ kế tiếp.

```typescript title=starter
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
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

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

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

function phanBoDeuVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  ___
}

function dungTruocNhuongSauVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  ___
}

const dsDeu = [taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(3)];
const ketQuaDeu = phanBoDeuVaChay(dsDeu, 9);
console.log("chia deu:  ", JSON.stringify(ketQuaDeu.map((k) => k.trangThai)), JSON.stringify(dsDeu.map((t) => t.demBuoc.soBuocDaChay)));

const dsTruoc = [taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(3)];
const ketQuaTruoc = dungTruocNhuongSauVaChay(dsTruoc, 9);
console.log("dung truoc:", JSON.stringify(ketQuaTruoc.map((k) => k.trangThai)), JSON.stringify(dsTruoc.map((t) => t.demBuoc.soBuocDaChay)));
```

```typescript title=solution
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
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

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

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

function phanBoDeuVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  const nganSachMoiTacVu = Math.floor(tongNganSach / dsTacVu.length);
  return dsTacVu.map((tv) => chayVongLapCoCap(tv, nganSachMoiTacVu));
}

function dungTruocNhuongSauVaChay(dsTacVu: TacVuMoPhong[], tongNganSach: number): KetQuaVongLapCoCap[] {
  let nganSachConLai = tongNganSach;
  const ketQua: KetQuaVongLapCoCap[] = [];
  for (const tv of dsTacVu) {
    const kq = chayVongLapCoCap(tv, nganSachConLai);
    ketQua.push(kq);
    nganSachConLai -= tv.demBuoc.soBuocDaChay;
  }
  return ketQua;
}

const dsDeu = [taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(3)];
const ketQuaDeu = phanBoDeuVaChay(dsDeu, 9);
console.log("chia deu:  ", JSON.stringify(ketQuaDeu.map((k) => k.trangThai)), JSON.stringify(dsDeu.map((t) => t.demBuoc.soBuocDaChay)));

const dsTruoc = [taoTacVuHoiTuSauNBuoc(2), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(3)];
const ketQuaTruoc = dungTruocNhuongSauVaChay(dsTruoc, 9);
console.log("dung truoc:", JSON.stringify(ketQuaTruoc.map((k) => k.trangThai)), JSON.stringify(dsTruoc.map((t) => t.demBuoc.soBuocDaChay)));
```

```typescript title=test
if (JSON.stringify(ketQuaDeu.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "hetBuoc", "thanhCong"])) {
  throw new Error("chia deu tren [2,7,3] voi tong 9 phai la [thanhCong,hetBuoc,thanhCong] -- tac vu can 7 buoc chi duoc chia 3");
}
if (JSON.stringify(dsDeu.map((t) => t.demBuoc.soBuocDaChay)) !== JSON.stringify([2, 3, 3])) {
  throw new Error("so buoc THAT SU dung o chia deu phai la [2,3,3]");
}

if (JSON.stringify(ketQuaTruoc.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "thanhCong", "hetBuoc"])) {
  throw new Error("dung truoc nhuong sau tren [2,7,3] voi tong 9 phai la [thanhCong,thanhCong,hetBuoc] -- tac vu cuoi khong con ngan sach");
}
if (JSON.stringify(dsTruoc.map((t) => t.demBuoc.soBuocDaChay)) !== JSON.stringify([2, 7, 0])) {
  throw new Error("so buoc THAT SU dung o dung truoc phai la [2,7,0] -- tac vu cuoi dung 0 buoc vi ngan sach da het");
}

const dsTruocDoiThuTu2 = [taoTacVuHoiTuSauNBuoc(3), taoTacVuHoiTuSauNBuoc(7), taoTacVuHoiTuSauNBuoc(2)];
const ketQuaDoiThuTu2 = dungTruocNhuongSauVaChay(dsTruocDoiThuTu2, 9);
if (JSON.stringify(ketQuaDoiThuTu2.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "hetBuoc", "hetBuoc"])) {
  throw new Error("doi thu tu tac vu (giu nguyen tong ngan sach) phai doi CA ket qua -- thu tu THAT SU anh huong chien luoc dung truoc");
}

const ketQuaMotTacVu = phanBoDeuVaChay([taoTacVuHoiTuSauNBuoc(4)], 10);
if (ketQuaMotTacVu.length !== 1) throw new Error("phanBoDeuVaChay phai tra ve mang cung do dai voi dsTacVu");
if (ketQuaMotTacVu[0]!.trangThai !== "thanhCong") throw new Error("mot tac vu can 4 buoc voi ngan sach 10 (chia deu cho 1) phai THANH CONG");

const ketQuaMotTacVuTruoc = dungTruocNhuongSauVaChay([taoTacVuHoiTuSauNBuoc(4)], 10);
if (ketQuaMotTacVuTruoc.length !== 1) throw new Error("dungTruocNhuongSauVaChay phai tra ve mang cung do dai voi dsTacVu");
if (ketQuaMotTacVuTruoc[0]!.trangThai !== "thanhCong") throw new Error("mot tac vu can 4 buoc voi ngan sach 10 phai THANH CONG");

const nganSachNho = phanBoDeuVaChay([taoTacVuHoiTuSauNBuoc(3), taoTacVuHoiTuSauNBuoc(3)], 4);
if (JSON.stringify(nganSachNho.map((k) => k.trangThai)) !== JSON.stringify(["hetBuoc", "hetBuoc"])) {
  throw new Error("chia deu 4 ngan sach cho 2 tac vu can 3 buoc moi tac vu (moi tac vu chi duoc 2) phai la [hetBuoc,hetBuoc]");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (phanBoDeuVaChay): tinh nganSachMoiTacVu = Math.floor(tongNganSach / dsTacVu.length) MOT LAN o dau ham, roi dung dsTacVu.map((tv) => chayVongLapCoCap(tv, nganSachMoiTacVu)) va return ket qua do. Cho hai (dungTruocNhuongSauVaChay): khoi tao let nganSachConLai = tongNganSach; mot vong for-of qua dsTacVu, MOI lan goi chayVongLapCoCap(tv, nganSachConLai), day ket qua vao mang, roi TRU tv.demBuoc.soBuocDaChay khoi nganSachConLai (SAU KHI da goi, dung so buoc THAT SU tac vu do dung, khong phai ngan sach da cap)."
- kind: strategy
  body: "Cho dau: const nganSachMoiTacVu = Math.floor(tongNganSach / dsTacVu.length); return dsTacVu.map((tv) => chayVongLapCoCap(tv, nganSachMoiTacVu)); Cho hai: let nganSachConLai = tongNganSach; const ketQua: KetQuaVongLapCoCap[] = []; for (const tv of dsTacVu) { const kq = chayVongLapCoCap(tv, nganSachConLai); ketQua.push(kq); nganSachConLai -= tv.demBuoc.soBuocDaChay; } return ketQua;"
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
  expect: "dung truoc: [\"thanhCong\",\"thanhCong\",\"hetBuoc\"] [2,7,0]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CÙNG `9` ngân sách, CÙNG `3` tác vụ — chia đều cứu tác vụ CUỐI (cần `3`
bước), dùng trước nhường sau cứu tác vụ GIỮA (cần `7` bước). KHÔNG chiến
lược nào "ĐÚNG hơn" MỘT cách tuyệt đối — mỗi chiến lược đánh đổi khác
nhau. Bài sau đóng gói MỌI biến thể đã học (step cap, no-progress, cost,
retry-trong-bước) thành MỘT hàm so sánh, xếp thành bảng.
::::

::::reflect{#nghi-lai}
Bài này KHÔNG hề dạy một mẹo lập trình mới — `Math.floor` VÀ một vòng
`for` trừ dần LÀ những thao tác quen thuộc từ những bài đầu tiên của
khoá học. Điều đáng học nằm Ở tầng QUYẾT ĐỊNH: khi NHIỀU tác vụ chia
nhau MỘT ngân sách hữu hạn, "công bằng" (chia đều) VÀ "ưu tiên ai tới
trước" (dùng trước nhường sau) LÀ hai triết lý khác nhau, VÀ chúng cứu
được hai tập hợp tác vụ HOÀN TOÀN không trùng nhau. Không có chiến lược
nào miễn phí: chia đều BỎ LỠ một tác vụ cần NHIỀU bước hơn phần chia dù
nó CÓ THỂ đã xong nếu được ưu tiên; dùng trước nhường sau có thể để một
tác vụ ở CUỐI hàng đợi không còn MỘT bước ngân sách nào, dù nhu cầu của
NÓ nhỏ hơn tác vụ đã "ăn" hết phần trước nó rất nhiều.
::::

::::checkpoint{mastery=0.86}
::::
