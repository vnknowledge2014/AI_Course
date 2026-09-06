---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.do-chi-phi-moi-buoc
title: "Đo chi phí mỗi bước — cost/task, KHÁC hẳn completion rate"
summary: "TacVuCoChiPhi{demBuoc; chayMotBuoc(): KetQuaBuocCoChiPhi} mở rộng TacVuMoPhong (q9.4a bài 1) -- mỗi bước trả THÊM một chiPhi cố định (số nguyên mô phỏng bằng bộ đếm, KHÔNG dùng đồng hồ thật). chayVongLapCoCapVaChiPhi(tacVu, soBuocToiDa) mở rộng chayVongLapCoCap (bài 2) để tích luỹ tongChiPhi qua MỌI lần gọi chayMotBuoc(), kể cả lần khiến vòng lặp DỪNG. Trên bộ 4 tác vụ hỗn hợp [hội tụ sau 2 bước chi phí 5/bước, hội tụ sau 5 bước chi phí 1/bước, không bao giờ xong chi phí 4/bước, hội tụ sau 1 bước chi phí 2/bước] với soBuocToiDa=10: trạng thái [thanhCong,thanhCong,hetBuoc,thanhCong], tongChiPhi=[10,5,40,2] -- tỷ lệ thành công 3/4 (in ra 0.75), chi phí trung bình TRÊN MỌI tác vụ 57/4 (in ra 14.25), chi phí trung bình CHỈ tính tác vụ thành công 17/3 (in ra 5.666666666666667). Tác vụ không bao giờ xong (1/4 số tác vụ) một mình đốt 40/57 tổng chi phí -- minh hoạ MASTERPLAN's 'đốt tiền': nó VẪN tính LÀ đúng 1 'tác vụ' như ba tác vụ kia, nhưng tiêu chi phí nhiều hơn CẢ BA tác vụ hội tụ CỘNG LẠI (10+5+2=17 so với 40)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.do-chi-phi-moi-buoc]
requires: [kna.boss-vong-lap-don-gian-vs-day-du]
concepts: [kna.do-chi-phi-moi-buoc]
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
Sáu bài Ở q9.4a đo được "có xong hay không" (completion rate), "hết bước
hay không" (%chạm step cap), và "có kẹt hay không" (no-progress). Nhưng
MASTERPLAN còn nhắc một chế độ hỏng riêng mà SÁU bài đó chưa hề chạm tới:
"đốt tiền" — một tác vụ vẫn tính LÀ đúng MỘT tác vụ trong completion rate,
nhưng có thể tiêu tốn nhiều lần chi phí hơn một tác vụ khác. Bài này dạy
đo đúng con số đó — cost/task — tách BIỆT hoàn toàn khỏi completion rate.
::::

::::explain{#tac_vu_co_chi_phi}
`TacVuCoChiPhi` giữ NGUYÊN hình dạng `TacVuMoPhong` (q9.4a bài `1`) —
`chayMotBuoc()` vẫn trả `trangThai` — nhưng THÊM một trường `chiPhi`:
một số nguyên CỐ ĐỊNH đại diện chi phí của ĐÚNG bước đó (mô phỏng bằng
tham số truyền vào lúc tạo tác vụ, KHÔNG dùng `Date.now()` hay bất kỳ
đồng hồ thật nào). Hai tác vụ CÙNG hội tụ sau 3 bước vẫn có thể tiêu chi
phí khác hẳn nhau, nếu `chiPhiMoiBuoc` của chúng khác nhau:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocCoChiPhi = { trangThai: TrangThaiBuoc; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuCoChiPhi {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChiPhi;
}

function taoTacVuHoiTuSauNBuocCoChiPhi(n: number, chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

const tv = taoTacVuHoiTuSauNBuocCoChiPhi(3, 2);
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
```

```text title=readonly
{"trangThai":"chua_xong","chiPhi":2}
{"trangThai":"chua_xong","chiPhi":2}
{"trangThai":"xong","chiPhi":2,"giaTri":"hoan_thanh"}
```

Mỗi bước — dù `trangThai` LÀ `"chua_xong"` hay `"xong"` — đều mang CÙNG
`chiPhi: 2`, đúng tham số `chiPhiMoiBuoc` đã truyền vào lúc tạo tác vụ.
Chi phí LÀ một thuộc tính CỦA BƯỚC, không phải thuộc tính CỦA KẾT QUẢ
cuối cùng — nó phát sinh NGAY CẢ Ở những bước chưa xong.
::::

::::example{#bon_tac_vu_hon_hop_va_hai_cach_do_chi_phi}
`chayVongLapCoCapVaChiPhi` mở rộng `chayVongLapCoCap` (bài `2`): vẫn
step cap, vẫn ba biến thể `"thanhCong"`/`"loi"`/`"hetBuoc"`, nhưng THÊM
`tongChiPhi` — cộng dồn `kq.chiPhi` Ở MỌI vòng lặp, kể cả vòng khiến hàm
`return` (thành công, lỗi, HAY hết bước). Chạy trên `4` tác vụ hỗn hợp,
`soBuocToiDa = 10`:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocCoChiPhi = { trangThai: TrangThaiBuoc; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuCoChiPhi {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChiPhi;
}

function taoTacVuHoiTuSauNBuocCoChiPhi(n: number, chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKhongBaoGioXongCoChiPhi(chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

type KetQuaVongLapCoChiPhi =
  | { trangThai: "thanhCong"; giaTri: string; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; tongChiPhi: number }
  | { trangThai: "hetBuoc"; tongChiPhi: number };

function chayVongLapCoCapVaChiPhi(tacVu: TacVuCoChiPhi, soBuocToiDa: number): KetQuaVongLapCoChiPhi {
  let soBuoc = 0;
  let tongChiPhi = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", tongChiPhi };
  }
  return { trangThai: "hetBuoc", tongChiPhi };
}

function chayTrenNhieuTacVuCoChiPhi(dsTacVu: TacVuCoChiPhi[], soBuocToiDa: number): KetQuaVongLapCoChiPhi[] {
  const ketQua: KetQuaVongLapCoChiPhi[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(chayVongLapCoCapVaChiPhi(tv, soBuocToiDa));
  }
  return ketQua;
}

function tinhTyLeThanhCong(ketQua: KetQuaVongLapCoChiPhi[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhChiPhiTrungBinh(ketQua: KetQuaVongLapCoChiPhi[]): number {
  if (ketQua.length === 0) return 0;
  const tong = ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0);
  return tong / ketQua.length;
}

function tinhChiPhiTrungBinhKhiThanhCong(ketQua: KetQuaVongLapCoChiPhi[]): number {
  const thanhCong = ketQua.filter((k) => k.trangThai === "thanhCong");
  if (thanhCong.length === 0) return 0;
  const tong = thanhCong.reduce((acc, k) => acc + k.tongChiPhi, 0);
  return tong / thanhCong.length;
}

const dsTacVu4: TacVuCoChiPhi[] = [
  taoTacVuHoiTuSauNBuocCoChiPhi(2, 5),
  taoTacVuHoiTuSauNBuocCoChiPhi(5, 1),
  taoTacVuKhongBaoGioXongCoChiPhi(4),
  taoTacVuHoiTuSauNBuocCoChiPhi(1, 2),
];
const ketQua4 = chayTrenNhieuTacVuCoChiPhi(dsTacVu4, 10);
console.log(JSON.stringify(ketQua4.map((k) => k.trangThai)));
console.log(JSON.stringify(ketQua4.map((k) => k.tongChiPhi)));
console.log("ty le thanh cong:", tinhTyLeThanhCong(ketQua4));
console.log("chi phi trung binh (tat ca):", tinhChiPhiTrungBinh(ketQua4));
console.log("chi phi trung binh (chi thanh cong):", tinhChiPhiTrungBinhKhiThanhCong(ketQua4));
```

```text title=readonly
["thanhCong","thanhCong","hetBuoc","thanhCong"]
[10,5,40,2]
ty le thanh cong: 0.75
chi phi trung binh (tat ca): 14.25
chi phi trung binh (chi thanh cong): 5.666666666666667
```

Ba tác vụ hội tụ dùng chi phí `10`, `5`, `2` — hợp lý, tỷ lệ THUẬN với
số bước THẬT SỰ cần. Tác vụ không bao giờ xong CHẠM `soBuocToiDa = 10`
VỚI chi phí `4` mỗi bước — tổng `40`, MỘT MÌNH nó ĐÃ vượt xa tổng của
BA tác vụ hội tụ cộng lại (`10 + 5 + 2 = 17`). Completion rate (`0.75`)
KHÔNG hề lộ ra điều đó — nó chỉ đếm "xong hay không", không đếm "tốn
bao nhiêu". Hai cách đo chi phí trung bình cũng kể hai câu chuyện khác
nhau: tính TRÊN MỌI tác vụ (`14.25`) bị tác vụ kẹt KÉO LÊN rất nhiều so
với chỉ tính trên tác vụ THẬT SỰ thành công (`5.666666666666667`).
::::

::::predict{#doan-tang-chi-phi-tac-vu-ket commitOnce}
Nếu tăng `chiPhiMoiBuoc` của tác vụ KHÔNG BAO GIỜ xong (tác vụ thứ `3`)
từ `4` lên `10` (giữ NGUYÊN `soBuocToiDa = 10` VÀ ba tác vụ còn lại) —
`trangThai` VÀ `tongChiPhi` của NÓ đổi ra sao?

:::opt{correct}
`trangThai` VẪN LÀ `"hetBuoc"` (tác vụ này không có cơ chế nào để tự
chuyển sang `"xong"`, bất kể chi phí mỗi bước LÀ bao nhiêu) — NHƯNG
`tongChiPhi` của riêng nó tăng từ `40` lên `100` (`10` bước × chi phí
mới `10`/bước), kéo chi phí trung bình TRÊN MỌI tác vụ tăng theo, TRONG
KHI completion rate (`0.75`) hoàn toàn KHÔNG đổi
:::
:::opt
Tác vụ này sẽ chuyển sang `"loi"`, vì chi phí quá cao khiến vòng lặp bỏ
cuộc SỚM hơn
::why
Nhầm rằng `chiPhi` ảnh hưởng tới ĐIỀU KIỆN dừng của vòng lặp — nhưng
`chayVongLapCoCapVaChiPhi` chỉ đọc `kq.trangThai` để quyết định
`"thanhCong"`/`"loi"`/tiếp tục lặp; `tongChiPhi` CHỈ được CỘNG DỒN, KHÔNG
hề được SO SÁNH với bất kỳ ngưỡng nào để quyết định dừng sớm.

Chỗ lệch: dòng `tongChiPhi += kq.chiPhi;` không nằm trong bất kỳ điều
kiện `if` nào kiểm tra giá trị của `tongChiPhi` — chi phí LÀ một con số
được GHI LẠI, không phải một RÀO CẢN.
::
:::
:::opt
Completion rate của CẢ bộ `4` tác vụ sẽ giảm xuống, vì tác vụ chi phí
cao "kéo" các tác vụ khác thất bại theo
::why
Nhầm rằng chi phí của MỘT tác vụ ảnh hưởng tới KẾT QUẢ của tác vụ KHÁC
— nhưng mỗi tác vụ trong `dsTacVu4` được chạy qua `chayVongLapCoCapVaChiPhi`
HOÀN TOÀN ĐỘC LẬP (một lời gọi hàm riêng, một `tacVu` riêng), không hề
chia sẻ trạng thái hay ngân sách nào với nhau Ở BÀI NÀY.

Chỗ lệch: `chayTrenNhieuTacVuCoChiPhi` gọi `chayVongLapCoCapVaChiPhi`
LẦN LƯỢT cho TỪNG tác vụ, mỗi lần nhận một `tacVu` và MỘT `soBuocToiDa`
riêng — kết quả của tác vụ NÀY không đọc VÀO, cũng không ghi ĐÈ, kết
quả của tác vụ khác.
::
:::
::::

::::code{#viet_chay_vong_lap_co_chi_phi}
Hoàn thiện `chayVongLapCoCapVaChiPhi` — lặp `while` tối đa `soBuocToiDa`
lần, mỗi lần gọi `tacVu.chayMotBuoc()` VÀ CỘNG DỒN `kq.chiPhi` VÀO
`tongChiPhi` NGAY (kể cả Ở vòng khiến hàm `return`); NẾU `trangThai` LÀ
`"xong"`, trả `{ trangThai: "thanhCong", giaTri: ..., tongChiPhi }` NGAY;
NẾU LÀ `"loi"`, trả `{ trangThai: "loi", loi: "loi_tac_vu", tongChiPhi }`
NGAY; hết vòng lặp thì trả `{ trangThai: "hetBuoc", tongChiPhi }`. Hoàn
thiện `chayTrenNhieuTacVuCoChiPhi` — với MỖI tác vụ, gọi
`chayVongLapCoCapVaChiPhi(tv, soBuocToiDa)`, đẩy kết quả vào mảng trả về.

```typescript title=starter
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocCoChiPhi = { trangThai: TrangThaiBuoc; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuCoChiPhi {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChiPhi;
}

function taoTacVuHoiTuSauNBuocCoChiPhi(n: number, chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKhongBaoGioXongCoChiPhi(chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuLoiNgayLapTucCoChiPhi(chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      return { trangThai: "loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

type KetQuaVongLapCoChiPhi =
  | { trangThai: "thanhCong"; giaTri: string; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; tongChiPhi: number }
  | { trangThai: "hetBuoc"; tongChiPhi: number };

function tinhTyLeThanhCong(ketQua: KetQuaVongLapCoChiPhi[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhChiPhiTrungBinh(ketQua: KetQuaVongLapCoChiPhi[]): number {
  if (ketQua.length === 0) return 0;
  const tong = ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0);
  return tong / ketQua.length;
}

function tinhChiPhiTrungBinhKhiThanhCong(ketQua: KetQuaVongLapCoChiPhi[]): number {
  const thanhCong = ketQua.filter((k) => k.trangThai === "thanhCong");
  if (thanhCong.length === 0) return 0;
  const tong = thanhCong.reduce((acc, k) => acc + k.tongChiPhi, 0);
  return tong / thanhCong.length;
}

function chayVongLapCoCapVaChiPhi(tacVu: TacVuCoChiPhi, soBuocToiDa: number): KetQuaVongLapCoChiPhi {
  ___
}

function chayTrenNhieuTacVuCoChiPhi(dsTacVu: TacVuCoChiPhi[], soBuocToiDa: number): KetQuaVongLapCoChiPhi[] {
  ___
}

const dsTacVu4: TacVuCoChiPhi[] = [
  taoTacVuHoiTuSauNBuocCoChiPhi(2, 5),
  taoTacVuHoiTuSauNBuocCoChiPhi(5, 1),
  taoTacVuKhongBaoGioXongCoChiPhi(4),
  taoTacVuHoiTuSauNBuocCoChiPhi(1, 2),
];
const ketQua4 = chayTrenNhieuTacVuCoChiPhi(dsTacVu4, 10);
console.log(
  JSON.stringify(ketQua4.map((k) => k.trangThai)),
  JSON.stringify(ketQua4.map((k) => k.tongChiPhi)),
  tinhTyLeThanhCong(ketQua4),
  tinhChiPhiTrungBinh(ketQua4),
  tinhChiPhiTrungBinhKhiThanhCong(ketQua4),
);
```

```typescript title=solution
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocCoChiPhi = { trangThai: TrangThaiBuoc; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuCoChiPhi {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChiPhi;
}

function taoTacVuHoiTuSauNBuocCoChiPhi(n: number, chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKhongBaoGioXongCoChiPhi(chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuLoiNgayLapTucCoChiPhi(chiPhiMoiBuoc: number): TacVuCoChiPhi {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChiPhi {
      demBuoc.soBuocDaChay++;
      return { trangThai: "loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

type KetQuaVongLapCoChiPhi =
  | { trangThai: "thanhCong"; giaTri: string; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; tongChiPhi: number }
  | { trangThai: "hetBuoc"; tongChiPhi: number };

function tinhTyLeThanhCong(ketQua: KetQuaVongLapCoChiPhi[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhChiPhiTrungBinh(ketQua: KetQuaVongLapCoChiPhi[]): number {
  if (ketQua.length === 0) return 0;
  const tong = ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0);
  return tong / ketQua.length;
}

function tinhChiPhiTrungBinhKhiThanhCong(ketQua: KetQuaVongLapCoChiPhi[]): number {
  const thanhCong = ketQua.filter((k) => k.trangThai === "thanhCong");
  if (thanhCong.length === 0) return 0;
  const tong = thanhCong.reduce((acc, k) => acc + k.tongChiPhi, 0);
  return tong / thanhCong.length;
}

function chayVongLapCoCapVaChiPhi(tacVu: TacVuCoChiPhi, soBuocToiDa: number): KetQuaVongLapCoChiPhi {
  let soBuoc = 0;
  let tongChiPhi = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", tongChiPhi };
  }
  return { trangThai: "hetBuoc", tongChiPhi };
}

function chayTrenNhieuTacVuCoChiPhi(dsTacVu: TacVuCoChiPhi[], soBuocToiDa: number): KetQuaVongLapCoChiPhi[] {
  const ketQua: KetQuaVongLapCoChiPhi[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(chayVongLapCoCapVaChiPhi(tv, soBuocToiDa));
  }
  return ketQua;
}

const dsTacVu4: TacVuCoChiPhi[] = [
  taoTacVuHoiTuSauNBuocCoChiPhi(2, 5),
  taoTacVuHoiTuSauNBuocCoChiPhi(5, 1),
  taoTacVuKhongBaoGioXongCoChiPhi(4),
  taoTacVuHoiTuSauNBuocCoChiPhi(1, 2),
];
const ketQua4 = chayTrenNhieuTacVuCoChiPhi(dsTacVu4, 10);
console.log(
  JSON.stringify(ketQua4.map((k) => k.trangThai)),
  JSON.stringify(ketQua4.map((k) => k.tongChiPhi)),
  tinhTyLeThanhCong(ketQua4),
  tinhChiPhiTrungBinh(ketQua4),
  tinhChiPhiTrungBinhKhiThanhCong(ketQua4),
);
```

```typescript title=test
if (ketQua4.length !== 4) throw new Error("chayTrenNhieuTacVuCoChiPhi phai tra ve mang dung 4 phan tu");
if (JSON.stringify(ketQua4.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "thanhCong", "hetBuoc", "thanhCong"])) {
  throw new Error("trang thai tren 4 tac vu phai la [thanhCong,thanhCong,hetBuoc,thanhCong]");
}
if (JSON.stringify(ketQua4.map((k) => k.tongChiPhi)) !== JSON.stringify([10, 5, 40, 2])) {
  throw new Error("tongChiPhi tren 4 tac vu phai la [10,5,40,2]");
}
if (Math.abs(tinhTyLeThanhCong(ketQua4) - 0.75) > 1e-9) throw new Error("ty le thanh cong phai la 0.75");
if (Math.abs(tinhChiPhiTrungBinh(ketQua4) - 14.25) > 1e-9) throw new Error("chi phi trung binh (tat ca) phai la 14.25");
if (Math.abs(tinhChiPhiTrungBinhKhiThanhCong(ketQua4) - 17 / 3) > 1e-9) {
  throw new Error("chi phi trung binh (chi thanh cong) phai la 17/3");
}

const tvRieng = taoTacVuHoiTuSauNBuocCoChiPhi(3, 7);
const kqRieng = chayVongLapCoCapVaChiPhi(tvRieng, 10);
if (kqRieng.trangThai !== "thanhCong") throw new Error("tac vu hoi tu sau 3 buoc phai THANH CONG");
if (kqRieng.trangThai === "thanhCong" && kqRieng.tongChiPhi !== 21) {
  throw new Error("3 buoc x chi phi 7 moi buoc phai cho tongChiPhi = 21");
}

const tvLoi = taoTacVuLoiNgayLapTucCoChiPhi(9);
const kqLoi = chayVongLapCoCapVaChiPhi(tvLoi, 10);
if (kqLoi.trangThai !== "loi") throw new Error("tac vu bao loi ngay phai tra trangThai loi");
if (kqLoi.trangThai === "loi" && kqLoi.tongChiPhi !== 9) {
  throw new Error("BUOC THAT BAI van phai duoc tinh chi phi -- tongChiPhi phai la 9, khong phai 0");
}
if (tvLoi.demBuoc.soBuocDaChay !== 1) throw new Error("tac vu bao loi ngay phai dung lai o LAN GOI DAU");

const ketQua1 = chayTrenNhieuTacVuCoChiPhi([taoTacVuHoiTuSauNBuocCoChiPhi(2, 3)], 10);
if (ketQua1.length !== 1) throw new Error("doi so danh sach tac vu phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayVongLapCoCapVaChiPhi): giong het chayVongLapCoCap bai 2, nhung THEM mot bien tongChiPhi = 0 truoc vong while, va CONG kq.chiPhi vao tongChiPhi NGAY sau moi lan goi chayMotBuoc() (truoc khi kiem trangThai) -- ca ba nhanh return (thanhCong/loi/hetBuoc) deu phai mang theo tongChiPhi. Cho hai (chayTrenNhieuTacVuCoChiPhi): mot vong for-of qua dsTacVu, goi chayVongLapCoCapVaChiPhi(tv, soBuocToiDa) tren moi phan tu, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: let soBuoc = 0; let tongChiPhi = 0; while (soBuoc < soBuocToiDa) { const kq = tacVu.chayMotBuoc(); soBuoc++; tongChiPhi += kq.chiPhi; if (kq.trangThai === \"xong\") return { trangThai: \"thanhCong\", giaTri: kq.giaTri ?? \"\", tongChiPhi }; if (kq.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\", tongChiPhi }; } return { trangThai: \"hetBuoc\", tongChiPhi }; Cho hai: const ketQua: KetQuaVongLapCoChiPhi[] = []; for (const tv of dsTacVu) { ketQua.push(chayVongLapCoCapVaChiPhi(tv, soBuocToiDa)); } return ketQua;"
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
  expect: "0.75 14.25 5.666666666666667"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Completion rate `0.75` không hề đổi, nhưng `tongChiPhi` của tác vụ kẹt
(`40`) một mình vượt xa tổng CỦA BA tác vụ hội tụ (`17`) — đúng "đốt
tiền" MASTERPLAN nhắc tới. Bài sau đưa chi phí đó vào một tình huống
THẬT hơn: mỗi bước của vòng lặp giờ THẬT SỰ gọi một tool mô phỏng
(T9.3), qua một lớp retry-trong-bước — VÀ lộ ra một ngân sách THỨ HAI,
tách biệt khỏi ngân sách bước của vòng lặp ngoài.
::::

::::reflect{#nghi-lai}
`chiPhi` không phải một cơ chế DỪNG vòng lặp — nó không hề tham gia vào
điều kiện `while`, không so sánh với ngưỡng nào, không khiến vòng lặp
dừng sớm hơn hay muộn hơn. Nó CHỈ LÀ một con số được GHI LẠI song song
với `trangThai`. Sự tách biệt đó chính LÀ điểm mấu chốt: MASTERPLAN định
nghĩa LOOP bằng BA trục đo — steps-to-success, %chạm step cap, VÀ
cost/task — vì một hệ thống có thể HOÀN TOÀN ổn Ở hai trục đầu (completion
rate cao, ít tác vụ chạm cap) NHƯNG vẫn "đốt tiền" Ở trục thứ ba, nếu một
số tác vụ kẹt tiêu tốn chi phí gấp bội trước khi bị coi LÀ thất bại. Đo
đúng đòi hỏi giữ BA trục đó tách RIÊNG, không gộp thành một con số.
::::

::::checkpoint{mastery=0.85}
::::
