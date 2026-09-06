---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.boss-vong-lap-production-grade
title: "BOSS q9.4b — vòng lặp production-grade trên N tác vụ, MỘT tổng ngân sách bước"
summary: "vongLapToiGian(dsTacVu, tongNganSachBuoc) cho MOI tac vu (doc lap) soBuocToiDa = tongNganSachBuoc NHU THE no la tac vu DUY NHAT -- KHONG chia se ngan sach, KHONG no-progress detection, KHONG retry-trong-buoc. vongLapSanXuat(dsTacVu, tongNganSachBuoc) rap NGUYEN VAN ca nam co che: dung-truoc-nhuong-sau (bai 10) CHIA SE mot nganSachConLai xuyen suot 4 tac vu, CONG no-progress detection (q9.4a bai 4) VA retry-trong-buoc (bai 8, qua chayMotTacVuVoiCauHinh bai 11) tren MOI tac vu. Tren bo 4 tac vu hon hop [hoi tu 3 buoc chi phi 2, ket dinh lap lai loi chi phi 2, can retry-trong-buoc de tien bo (n=2) chi phi 3/luot, hoi tu 2 buoc chi phi 1] voi TONG ngan sach = 10: vongLapToiGian hoan thanh 2/4 (in ra 0.5), tong chi phi = 58, TONG SO BUOC THAT SU DUNG = 25 -- VUOT XA tong ngan sach khai bao (10) vi moi tac vu doc lap 'an' ca ngan sach nhu the no mot minh. vongLapSanXuat hoan thanh 3/4 (in ra 0.75), tong chi phi = 26, tong so buoc that su dung = 10 -- KHOP CHINH XAC tong ngan sach khai bao. %cham step cap (hetBuoc rieng biet, KHAC khongTienTrien) giam tu 2/4 (in ra 0.5) xuong 0/4 (in ra 0) -- tac vu ket bi phat hien SOM thay vi dot het ngan sach. Dong track T9.4 tai 12/12: moi co che giai quyet MOT chieu do khac nhau, khong co che nao thay the duoc co che khac."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-vong-lap-production-grade]
requires: [kna.so-sanh-cau-hinh-vong-lap-qua-bang-danh-gia]
concepts: [kna.boss-vong-lap-production-grade]
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

::::byte{trigger=enter mood=happy pose=jump}
Mười một bài: hình dạng vòng lặp, step cap, dừng sớm sai, no-progress
detection, fold trên State, cost/task, retry-trong-bước, cầu nối Kleisli
VÀ Alternative, ngân sách chia sẻ, VÀ một bảng so sánh cấu hình. BOSS
này ráp NGUYÊN VĂN mọi cơ chế đó thành MỘT vòng lặp "production-grade"
DUY NHẤT, chạy trên N tác vụ VỚI MỘT tổng ngân sách bước hữu hạn — so
với một vòng lặp TỐI GIẢN (không cơ chế nào) trên CÙNG bộ tác vụ VÀ
CÙNG tổng ngân sách — để đóng track `T9.4` "Kỹ thuật Vòng lặp" Ở `12/12`.
::::

::::explain{#toi_gian_khong_chia_se_ngan_sach}
`vongLapToiGian` — KHÔNG cơ chế nào — chạy TỪNG tác vụ ĐỘC LẬP, cho MỖI
tác vụ `soBuocToiDa` bằng ĐÚNG `tongNganSachBuoc`, NHƯ THỂ nó LÀ tác vụ
DUY NHẤT tồn tại. Đây LÀ bản năng "tự nhiên" của một kỹ sư CHƯA học bài
`10`: không hề chia sẻ ngân sách, mỗi tác vụ cứ "xài" bao nhiêu cũng
được, miễn KHÔNG vượt quá TRẦN CỦA RIÊNG NÓ:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

function vongLapToiGian(dsTacVu: TacVuVongLapDayDu[], tongNganSachBuoc: number): KetQuaVongLapDayDu[] {
  return dsTacVu.map((tv) =>
    chayMotTacVuVoiCauHinh(tv, {
      soBuocToiDa: tongNganSachBuoc,
      coPhatHienKhongTienTrien: false,
      soLanLapLaiToiDa: 3,
      soLanThuLaiToiDaTrongBuoc: 0,
    }),
  );
}

const ketQuaHai = vongLapToiGian([taoTacVuKetDinhDayDu(2), taoTacVuKetDinhDayDu(2)], 10);
console.log(JSON.stringify(ketQuaHai.map((k) => k.trangThai)), ketQuaHai.reduce((acc, k) => acc + k.tongChiPhi, 0));
```

```text title=readonly
["hetBuoc","hetBuoc"] 40
```

HAI tác vụ kẹt, `tongNganSachBuoc = 10` — NHƯNG `vongLapToiGian` cho MỖI
tác vụ ĐỦ `10` bước RIÊNG, nên tổng chi phí THẬT LÀ `2 × (10 × 2) = 40`
— GẤP `4` lần con số `10` mà cái tên "tổng ngân sách" ngụ ý. Đây chính
LÀ vấn đề bài `10` đã dạy: một vòng lặp KHÔNG chia sẻ ngân sách không hề
tôn trọng RÀNG BUỘC toàn cục, dù MỖI tác vụ riêng lẻ vẫn "tuân thủ" cap
CỦA RIÊNG NÓ.
::::

::::example{#san_xuat_rap_nam_co_che}
`vongLapSanXuat` ráp NĂM cơ chế: dùng-trước-nhường-sau (bài `10` — MỘT
`nganSachConLai` CHIA SẺ, trừ dần sau MỖI tác vụ), no-progress detection
(q9.4a bài `4`), retry-trong-bước (bài `8`, qua `chayMotTacVuVoiCauHinh`
bài `11`), VÀ cost tracking (bài `7`) — TRÊN bộ `4` tác vụ hỗn hợp,
`tongNganSachBuoc = 10`:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuCanRetryTrongBuoc(n: number, chiPhiMoiLuotGoi: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  const demTienBo = { soTienBo: 0 };
  return {
    demBuoc,
    chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      if (soLanThuLaiToiDaTrongBuoc < 1) {
        return { trangThai: "chua_xong", chuKy: "can_retry_trong_buoc", chiPhi: chiPhiMoiLuotGoi };
      }
      demTienBo.soTienBo++;
      const xong = demTienBo.soTienBo >= n;
      const chiPhiBuocNay = chiPhiMoiLuotGoi * 2;
      if (xong) {
        return { trangThai: "xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay, giaTri: "hoan_thanh_qua_retry" };
      }
      return { trangThai: "chua_xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

function vongLapSanXuat(dsTacVu: TacVuVongLapDayDu[], tongNganSachBuoc: number): KetQuaVongLapDayDu[] {
  let nganSachConLai = tongNganSachBuoc;
  const ketQua: KetQuaVongLapDayDu[] = [];
  for (const tv of dsTacVu) {
    const kq = chayMotTacVuVoiCauHinh(tv, {
      soBuocToiDa: nganSachConLai,
      coPhatHienKhongTienTrien: true,
      soLanLapLaiToiDa: 3,
      soLanThuLaiToiDaTrongBuoc: 1,
    });
    ketQua.push(kq);
    nganSachConLai -= tv.demBuoc.soBuocDaChay;
    if (nganSachConLai < 0) nganSachConLai = 0;
  }
  return ketQua;
}

function tinhTyLeThanhCong(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}
function tinhTyLeChamCap(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}
function tinhTongChiPhi(ketQua: KetQuaVongLapDayDu[]): number {
  return ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0);
}
function tinhTongSoBuocThatSuDung(dsTacVu: TacVuVongLapDayDu[]): number {
  return dsTacVu.reduce((acc, tv) => acc + tv.demBuoc.soBuocDaChay, 0);
}

function taoBoTacVuHonHopBoss(): TacVuVongLapDayDu[] {
  return [
    taoTacVuHoiTuDayDu(3, 2),
    taoTacVuKetDinhDayDu(2),
    taoTacVuCanRetryTrongBuoc(2, 3),
    taoTacVuHoiTuDayDu(2, 1),
  ];
}

const TONG_NGAN_SACH = 10;

const dsToiGian = taoBoTacVuHonHopBoss();
const ketQuaToiGian = vongLapToiGian(dsToiGian, TONG_NGAN_SACH);

const dsSanXuat = taoBoTacVuHonHopBoss();
const ketQuaSanXuat = vongLapSanXuat(dsSanXuat, TONG_NGAN_SACH);

console.log("toi gian:", JSON.stringify(ketQuaToiGian.map((k) => k.trangThai)));
console.log("san xuat:", JSON.stringify(ketQuaSanXuat.map((k) => k.trangThai)));
console.log(
  "so lieu:",
  tinhTyLeThanhCong(ketQuaToiGian),
  tinhTyLeThanhCong(ketQuaSanXuat),
  tinhTyLeChamCap(ketQuaToiGian),
  tinhTyLeChamCap(ketQuaSanXuat),
  tinhTongChiPhi(ketQuaToiGian),
  tinhTongChiPhi(ketQuaSanXuat),
  tinhTongSoBuocThatSuDung(dsToiGian),
  tinhTongSoBuocThatSuDung(dsSanXuat),
);
```

```text title=readonly
toi gian: ["thanhCong","hetBuoc","hetBuoc","thanhCong"]
san xuat: ["thanhCong","khongTienTrien","thanhCong","thanhCong"]
so lieu: 0.5 0.75 0.5 0 58 26 25 10
```

`vongLapToiGian`: hoàn thành `2`/`4` (`0.5`), NHƯNG dùng `25` bước THẬT
SỰ — GẤP `2.5` lần `tongNganSachBuoc = 10` khai báo, vì MỖI tác vụ độc
lập "ăn" tới `10` bước RIÊNG. `vongLapSanXuat`: hoàn thành `3`/`4`
(`0.75` — tác vụ retry-trong-bước giờ THÀNH CÔNG), tổng chi phí THẤP hơn
NHIỀU (`26` so VỚI `58`), VÀ tổng số bước THẬT SỰ dùng KHỚP CHÍNH XÁC
`10` — ĐÚNG bằng `tongNganSachBuoc` khai báo. `%chạm step cap` (đếm
RIÊNG `"hetBuoc"`, KHÁC `"khongTienTrien"`) giảm từ `0.5` xuống `0` —
tác vụ kẹt bị PHÁT HIỆN sớm (`"khongTienTrien"`) thay vì đốt hết ngân
sách rồi mới `"hetBuoc"`.
::::

::::predict{#doan-tang-tong-ngan-sach commitOnce}
Nếu tăng `TONG_NGAN_SACH` từ `10` lên `20` (giữ NGUYÊN `4` tác vụ) —
`vongLapSanXuat` có đổi `tyLeThanhCong`, `tongChiPhi`, VÀ tổng số bước
THẬT SỰ dùng không? CÒN `vongLapToiGian` thì sao?

:::opt{correct}
`vongLapSanXuat` KHÔNG đổi GÌ CẢ (`tyLeThanhCong = 0.75`, `tongChiPhi =
26`, tổng bước dùng `= 10` — GIỐNG HỆT lúc ngân sách `= 10`): cả bốn tác
vụ ĐÃ được giải quyết (thành công hoặc phát hiện kẹt) TRONG PHẠM VI `10`
bước ĐẦU, phần ngân sách DÔI RA hoàn toàn KHÔNG được dùng tới.
`vongLapToiGian` VẪN `tyLeThanhCong = 0.5` (retry-trong-bước KHÔNG được
bật, hai tác vụ kẹt VẪN không bao giờ xong bất kể cap LỚN cỡ nào), NHƯNG
`tongChiPhi` TĂNG lên `108` (gấp đôi `58`) — MỖI tác vụ độc lập giờ
"ăn" tới `20` bước RIÊNG thay vì `10`, đốt THÊM tiền MÀ KHÔNG cứu thêm
được tác vụ nào
:::
:::opt
CẢ HAI vòng lặp đều TĂNG `tyLeThanhCong`, vì ngân sách lớn hơn LUÔN cho
nhiều tác vụ cơ hội thành công hơn
::why
Nhầm rằng TĂNG ngân sách LUÔN cải thiện completion rate — nhưng tác vụ
`taoTacVuKetDinhDayDu` (kẹt VĨNH VIỄN, CÙNG `chuKy` mọi bước) không CÓ
cơ chế nào để tự chuyển sang `"xong"`, bất kể được cấp bao nhiêu bước;
VÀ Ở `vongLapToiGian`, retry-trong-bước KHÔNG hề được bật (`soLanThuLaiToiDaTrongBuoc:
0` cứng trong cấu hình), nên tác vụ retry-trong-bước CŨNG không được
cứu dù ngân sách lớn hơn.

Chỗ lệch: `tyLeThanhCong` chỉ tăng khi có tác vụ THẬT SỰ cần THÊM bước
để hội tụ (chưa từng xảy ra Ở batch NÀY VỚI CẢ hai mức ngân sách `10`
VÀ `20` — mọi tác vụ đã "chốt" số phận trong `10` bước đầu).
::
:::
:::opt
`vongLapSanXuat` sẽ TĂNG `tyLeThanhCong` lên `1` (`4`/`4`), vì ngân sách
`20` đủ RỘNG để phát hiện no-progress "cứu" luôn CẢ tác vụ kẹt
::why
Nhầm rằng no-progress detection LÀ một cơ chế PHỤC HỒI (như retry) —
nhưng nó CHỈ dừng SỚM một tác vụ đã CHẮC CHẮN không hội tụ (q9.4a bài
`4`, VÀ BOSS bài `6`), KHÔNG hề làm tác vụ đó THÀNH CÔNG. Tác vụ kẹt VẪN
LÀ `"khongTienTrien"`, không phải `"thanhCong"`, bất kể ngân sách lớn
cỡ nào.

Chỗ lệch: `tyLeThanhCong` chỉ đếm biến thể `"thanhCong"` — VÀ tác vụ
kẹt Ở batch NÀY (`taoTacVuKetDinhDayDu`) KHÔNG BAO GIỜ trả về `"xong"`,
dù `chayMotBuoc` được gọi bao nhiêu lần.
::
:::
::::

::::code{#viet_hai_vong_lap_boss}
Hoàn thiện `vongLapToiGian` — với MỖI tác vụ trong `dsTacVu` (ĐỘC LẬP),
gọi `chayMotTacVuVoiCauHinh` VỚI `soBuocToiDa: tongNganSachBuoc`,
`coPhatHienKhongTienTrien: false`, `soLanThuLaiToiDaTrongBuoc: 0` (dùng
`.map`). Hoàn thiện `vongLapSanXuat` — giữ MỘT `nganSachConLai` khởi tạo
bằng `tongNganSachBuoc`; VỚI MỖI tác vụ (tuần tự): gọi
`chayMotTacVuVoiCauHinh` VỚI `soBuocToiDa: nganSachConLai`,
`coPhatHienKhongTienTrien: true`, `soLanLapLaiToiDa: 3`,
`soLanThuLaiToiDaTrongBuoc: 1`; đẩy kết quả vào mảng; TRỪ
`tv.demBuoc.soBuocDaChay` khỏi `nganSachConLai` (không để ÂM).

```typescript title=starter
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuCanRetryTrongBuoc(n: number, chiPhiMoiLuotGoi: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  const demTienBo = { soTienBo: 0 };
  return {
    demBuoc,
    chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      if (soLanThuLaiToiDaTrongBuoc < 1) {
        return { trangThai: "chua_xong", chuKy: "can_retry_trong_buoc", chiPhi: chiPhiMoiLuotGoi };
      }
      demTienBo.soTienBo++;
      const xong = demTienBo.soTienBo >= n;
      const chiPhiBuocNay = chiPhiMoiLuotGoi * 2;
      if (xong) {
        return { trangThai: "xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay, giaTri: "hoan_thanh_qua_retry" };
      }
      return { trangThai: "chua_xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

function tinhTyLeThanhCong(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}
function tinhTyLeChamCap(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}
function tinhTongChiPhi(ketQua: KetQuaVongLapDayDu[]): number {
  return ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0);
}
function tinhTongSoBuocThatSuDung(dsTacVu: TacVuVongLapDayDu[]): number {
  return dsTacVu.reduce((acc, tv) => acc + tv.demBuoc.soBuocDaChay, 0);
}

function vongLapToiGian(dsTacVu: TacVuVongLapDayDu[], tongNganSachBuoc: number): KetQuaVongLapDayDu[] {
  ___
}

function vongLapSanXuat(dsTacVu: TacVuVongLapDayDu[], tongNganSachBuoc: number): KetQuaVongLapDayDu[] {
  ___
}

function taoBoTacVuHonHopBoss(): TacVuVongLapDayDu[] {
  return [
    taoTacVuHoiTuDayDu(3, 2),
    taoTacVuKetDinhDayDu(2),
    taoTacVuCanRetryTrongBuoc(2, 3),
    taoTacVuHoiTuDayDu(2, 1),
  ];
}

const TONG_NGAN_SACH = 10;

const dsToiGian = taoBoTacVuHonHopBoss();
const ketQuaToiGian = vongLapToiGian(dsToiGian, TONG_NGAN_SACH);

const dsSanXuat = taoBoTacVuHonHopBoss();
const ketQuaSanXuat = vongLapSanXuat(dsSanXuat, TONG_NGAN_SACH);

console.log(
  JSON.stringify(ketQuaToiGian.map((k) => k.trangThai)),
  JSON.stringify(ketQuaSanXuat.map((k) => k.trangThai)),
  tinhTyLeThanhCong(ketQuaToiGian),
  tinhTyLeThanhCong(ketQuaSanXuat),
  tinhTyLeChamCap(ketQuaToiGian),
  tinhTyLeChamCap(ketQuaSanXuat),
  tinhTongChiPhi(ketQuaToiGian),
  tinhTongChiPhi(ketQuaSanXuat),
  tinhTongSoBuocThatSuDung(dsToiGian),
  tinhTongSoBuocThatSuDung(dsSanXuat),
);
```

```typescript title=solution
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuCanRetryTrongBuoc(n: number, chiPhiMoiLuotGoi: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  const demTienBo = { soTienBo: 0 };
  return {
    demBuoc,
    chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      if (soLanThuLaiToiDaTrongBuoc < 1) {
        return { trangThai: "chua_xong", chuKy: "can_retry_trong_buoc", chiPhi: chiPhiMoiLuotGoi };
      }
      demTienBo.soTienBo++;
      const xong = demTienBo.soTienBo >= n;
      const chiPhiBuocNay = chiPhiMoiLuotGoi * 2;
      if (xong) {
        return { trangThai: "xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay, giaTri: "hoan_thanh_qua_retry" };
      }
      return { trangThai: "chua_xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

function tinhTyLeThanhCong(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}
function tinhTyLeChamCap(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}
function tinhTongChiPhi(ketQua: KetQuaVongLapDayDu[]): number {
  return ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0);
}
function tinhTongSoBuocThatSuDung(dsTacVu: TacVuVongLapDayDu[]): number {
  return dsTacVu.reduce((acc, tv) => acc + tv.demBuoc.soBuocDaChay, 0);
}

function vongLapToiGian(dsTacVu: TacVuVongLapDayDu[], tongNganSachBuoc: number): KetQuaVongLapDayDu[] {
  return dsTacVu.map((tv) =>
    chayMotTacVuVoiCauHinh(tv, {
      soBuocToiDa: tongNganSachBuoc,
      coPhatHienKhongTienTrien: false,
      soLanLapLaiToiDa: 3,
      soLanThuLaiToiDaTrongBuoc: 0,
    }),
  );
}

function vongLapSanXuat(dsTacVu: TacVuVongLapDayDu[], tongNganSachBuoc: number): KetQuaVongLapDayDu[] {
  let nganSachConLai = tongNganSachBuoc;
  const ketQua: KetQuaVongLapDayDu[] = [];
  for (const tv of dsTacVu) {
    const kq = chayMotTacVuVoiCauHinh(tv, {
      soBuocToiDa: nganSachConLai,
      coPhatHienKhongTienTrien: true,
      soLanLapLaiToiDa: 3,
      soLanThuLaiToiDaTrongBuoc: 1,
    });
    ketQua.push(kq);
    nganSachConLai -= tv.demBuoc.soBuocDaChay;
    if (nganSachConLai < 0) nganSachConLai = 0;
  }
  return ketQua;
}

function taoBoTacVuHonHopBoss(): TacVuVongLapDayDu[] {
  return [
    taoTacVuHoiTuDayDu(3, 2),
    taoTacVuKetDinhDayDu(2),
    taoTacVuCanRetryTrongBuoc(2, 3),
    taoTacVuHoiTuDayDu(2, 1),
  ];
}

const TONG_NGAN_SACH = 10;

const dsToiGian = taoBoTacVuHonHopBoss();
const ketQuaToiGian = vongLapToiGian(dsToiGian, TONG_NGAN_SACH);

const dsSanXuat = taoBoTacVuHonHopBoss();
const ketQuaSanXuat = vongLapSanXuat(dsSanXuat, TONG_NGAN_SACH);

console.log(
  JSON.stringify(ketQuaToiGian.map((k) => k.trangThai)),
  JSON.stringify(ketQuaSanXuat.map((k) => k.trangThai)),
  tinhTyLeThanhCong(ketQuaToiGian),
  tinhTyLeThanhCong(ketQuaSanXuat),
  tinhTyLeChamCap(ketQuaToiGian),
  tinhTyLeChamCap(ketQuaSanXuat),
  tinhTongChiPhi(ketQuaToiGian),
  tinhTongChiPhi(ketQuaSanXuat),
  tinhTongSoBuocThatSuDung(dsToiGian),
  tinhTongSoBuocThatSuDung(dsSanXuat),
);
```

```typescript title=test
if (ketQuaToiGian.length !== 4) throw new Error("vongLapToiGian phai duoc goi tren dung 4 tac vu");
if (ketQuaSanXuat.length !== 4) throw new Error("vongLapSanXuat phai duoc goi tren dung 4 tac vu");

if (JSON.stringify(ketQuaToiGian.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "hetBuoc", "hetBuoc", "thanhCong"])) {
  throw new Error("vongLapToiGian tren bo hon hop (ngan sach 10) phai la [thanhCong,hetBuoc,hetBuoc,thanhCong]");
}
if (JSON.stringify(ketQuaSanXuat.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "khongTienTrien", "thanhCong", "thanhCong"])) {
  throw new Error("vongLapSanXuat tren bo hon hop (ngan sach 10) phai la [thanhCong,khongTienTrien,thanhCong,thanhCong]");
}

if (Math.abs(tinhTyLeThanhCong(ketQuaToiGian) - 0.5) > 1e-9) throw new Error("vongLapToiGian: tyLeThanhCong phai la 0.5");
if (Math.abs(tinhTyLeThanhCong(ketQuaSanXuat) - 0.75) > 1e-9) throw new Error("vongLapSanXuat: tyLeThanhCong phai la 0.75 -- CAO HON toi-gian nho retry-trong-buoc");

if (Math.abs(tinhTyLeChamCap(ketQuaToiGian) - 0.5) > 1e-9) throw new Error("vongLapToiGian: ty le cham step cap (hetBuoc) phai la 0.5");
if (tinhTyLeChamCap(ketQuaSanXuat) !== 0) throw new Error("vongLapSanXuat: ty le cham step cap (hetBuoc) phai la 0 -- tac vu ket duoc bat qua khongTienTrien, KHONG phai hetBuoc");

if (tinhTongChiPhi(ketQuaToiGian) !== 58) throw new Error("vongLapToiGian: tong chi phi phai la 58");
if (tinhTongChiPhi(ketQuaSanXuat) !== 26) throw new Error("vongLapSanXuat: tong chi phi phai la 26 -- THAP HON NHIEU nho no-progress detection VA chia se ngan sach");

if (tinhTongSoBuocThatSuDung(dsToiGian) !== 25) throw new Error("vongLapToiGian: tong so buoc THAT SU dung phai la 25 -- VUOT XA tong ngan sach khai bao (10)");
if (tinhTongSoBuocThatSuDung(dsSanXuat) !== 10) throw new Error("vongLapSanXuat: tong so buoc THAT SU dung phai la 10 -- KHOP CHINH XAC tong ngan sach khai bao");

const ketQuaMotTacVu = vongLapSanXuat([taoTacVuHoiTuDayDu(2, 1)], 5);
if (ketQuaMotTacVu.length !== 1) throw new Error("doi so danh sach tac vu phai doi do dai mang tra ve -- tham so phai duoc dung that");
if (ketQuaMotTacVu[0]!.trangThai !== "thanhCong") throw new Error("mot tac vu hoi tu sau 2 buoc, ngan sach 5, phai THANH CONG qua vongLapSanXuat");
```

:::hints
- kind: attention
  body: "Hai cho trong. vongLapToiGian: dung .map tren dsTacVu, moi tac vu goi chayMotTacVuVoiCauHinh VOI soBuocToiDa BANG tongNganSachBuoc (KHONG chia se, KHONG doi giua cac tac vu), coPhatHienKhongTienTrien: false, soLanThuLaiToiDaTrongBuoc: 0. vongLapSanXuat: mot bien nganSachConLai = tongNganSachBuoc, mot vong for-of qua dsTacVu, MOI lan goi chayMotTacVuVoiCauHinh VOI soBuocToiDa: nganSachConLai (CAP NHAT lien tuc), coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 -- SAU MOI tac vu, TRU tv.demBuoc.soBuocDaChay khoi nganSachConLai (khong de am)."
- kind: strategy
  body: "vongLapToiGian: return dsTacVu.map((tv) => chayMotTacVuVoiCauHinh(tv, { soBuocToiDa: tongNganSachBuoc, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 })); vongLapSanXuat: let nganSachConLai = tongNganSachBuoc; const ketQua: KetQuaVongLapDayDu[] = []; for (const tv of dsTacVu) { const kq = chayMotTacVuVoiCauHinh(tv, { soBuocToiDa: nganSachConLai, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 }); ketQua.push(kq); nganSachConLai -= tv.demBuoc.soBuocDaChay; if (nganSachConLai < 0) nganSachConLai = 0; } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "0.5 0.75 0.5 0 58 26 25 10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`58` so VỚI `26` chi phí, `25` so VỚI `10` bước THẬT SỰ dùng — trên
CÙNG bộ `4` tác vụ, CÙNG "tổng ngân sách" khai báo. Track `T9.4` "Kỹ
thuật Vòng lặp" đóng lại Ở `12/12`: q9.4a dựng nền (hình dạng, step cap,
dừng sớm, no-progress, fold trên State); q9.4b mở rộng SÂU hơn (cost,
retry-trong-bước xuyên hai track, cầu nối Kleisli/Alternative, ngân
sách chia sẻ, đo lường có hệ thống) VÀ ráp TẤT CẢ lại thành MỘT BOSS.
::::

::::reflect{#nghi-lai}
Con số `25` so VỚI `10` LÀ phát hiện quan trọng nhất của bài này — QUAN
TRỌNG HƠN cả `tyLeThanhCong` tăng từ `0.5` lên `0.75`. Một vòng lặp
KHÔNG chia sẻ ngân sách không chỉ "kém tối ưu" — nó làm cho chính CON SỐ
"tổng ngân sách" MẤT Ý NGHĨA: khai báo `10` NHƯNG thực chi `25` nghĩa LÀ
không ai có thể LẬP KẾ HOẠCH dựa trên con số đó. `vongLapSanXuat` không
chỉ tốt hơn Ở HAI trục đo quen thuộc (completion rate cao hơn, chi phí
thấp hơn) — nó LÀ vòng lặp DUY NHẤT trong hai cái mà con số "tổng ngân
sách" THẬT SỰ LÀ một RÀNG BUỘC, không phải một cái tên suông. Đây chính
LÀ bài học xuyên suốt q9.4b: MỖI cơ chế (cost tracking, retry-trong-
bước, ngân sách chia sẻ, no-progress detection) giải quyết đúng MỘT
lớp vấn đề riêng — cost tracking đo được "đốt tiền" nhưng không NGĂN
được nó; retry-trong-bước cứu được lỗi tạm thời NHƯNG không quản được
tổng ngân sách; CHỈ khi ngân sách chia sẻ ĐƯỢC tôn trọng (dùng-trước-
nhường-sau, bài `10`), những cơ chế còn lại mới vận hành Ở ĐÚNG QUY MÔ
mà một hệ thống nhiều-tác-vụ THẬT SỰ cần.
::::

::::checkpoint{mastery=0.95}
::::
