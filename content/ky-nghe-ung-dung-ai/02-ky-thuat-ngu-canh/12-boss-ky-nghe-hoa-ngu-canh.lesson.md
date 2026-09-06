---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.boss-ky-nghe-hoa-ngu-canh
title: "BOSS q9.2b — ráp production-shaped: tóm tắt, ngân sách khu vực, sổ ghi nhớ"
summary: "moPhongPhienDayDu(cacLuot, nganSach, cacTuKhoaCanTrich, chienLuocCuaSo: \"truot\"|\"tom_tat\", nsKhuVuc: NganSachTheoKhuVuc): KetQuaBoss ráp NGUYÊN VĂN capNhatSoGhiNho (bài 9), catCuaSoTruot/catVaTomTat (bài 2 q9.2a/bài 7, chọn qua chienLuocCuaSo), tinhTyLeRecall (bài 10), kiemTraNganSachTheoKhuVuc (bài 8) — qua 7 lượt (CAC_LUOT_HOI_THOAI), ngân sách 130. Đo HAI con số recall TÁCH BIỆT Ở lượt cuối: tyLeRecallSoGhiNho=1.0 (sổ ghi nhớ không mất gì) so với tyLeRecallCuaSo=0.667 (2/3 — cửa sổ hiển thị mất 'XY789') — GIỐNG NHAU Ở CẢ HAI chiến lược cửa sổ (tóm tắt không cứu được đúng cụm từ này, vì nó nằm ngoài 20 ký tự đầu). kiemTraNganSachTheoKhuVuc Ở lượt cuối: ganDayOk=false (khu vực gần đây đã phình to) dù heThongOk/tomTatOk/conDuDuTru đều true. Đóng q9.2b tại 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-ky-nghe-hoa-ngu-canh]
requires: [kna.so-ghi-nho-song-sot-qua-cat-cua-so]
concepts: [kna.boss-ky-nghe-hoa-ngu-canh]
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
Năm bài: tóm tắt thay vì xoá, ngân sách chia theo khu vực, sổ ghi nhớ
trích xuất một lần, tỉ lệ recall đa sự kiện, VÀ cầu nối chứng minh sổ
ghi nhớ sống sót qua cắt cửa sổ. BOSS ráp CẢ NĂM vào MỘT hàm DUY NHẤT,
chạy trên một phiên nhiều lượt, đo ĐỦ những con số một hệ thống
production THẬT cần theo dõi CÙNG lúc: hai loại recall khác nhau, VÀ
ngân sách theo TỪNG khu vực Ở lượt CUỐI CÙNG.
::::

::::explain{#rap_toan_bo_q92b}
`moPhongPhienDayDu` chạy y hệt vòng lặp của bài trước — cập nhật
`soGhiNho` cho MỖI message mới, tích luỹ `lichSu`, cắt khi vượt ngân
sách — nhưng giờ CHỌN chiến lược cắt (`"truot"` gọi `catCuaSoTruot` bài
`2` q9.2a; `"tom_tat"` gọi `catVaTomTat` bài `7`), VÀ Ở CUỐI, tính HAI
con số recall khác nhau CỘNG một phép kiểm ngân sách theo khu vực:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}
interface KetQuaBoss {
  soTurnDaChay: number;
  soGhiNhoCuoiCung: MucGhiNho[];
  cuaSoCuoiCung: ChatMessage[];
  tyLeRecallSoGhiNho: number;
  tyLeRecallCuaSo: number;
  kiemTraNganSachLuotCuoi: { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean };
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}
function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  const phanBiCat = phanCoTheCat.slice(0, batDau);
  const phanConLai = phanCoTheCat.slice(batDau);
  if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai];
  return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(soGhiNhoHienTai: MucGhiNho[], tinMoi: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}
function kiemTraNganSachTheoKhuVuc(
  heThong: ChatMessage[],
  tomTat: ChatMessage[],
  ganDay: ChatMessage[],
  ns: NganSachTheoKhuVuc,
): { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean } {
  const heThongOk = tinhTongToken(heThong) <= ns.heThong;
  const tomTatOk = tinhTongToken(tomTat) <= ns.tomTat;
  const ganDayOk = tinhTongToken(ganDay) <= ns.ganDay;
  const conDuDuTru = ns.duTruPhanHoi > 0;
  return { heThongOk, tomTatOk, ganDayOk, conDuDuTru };
}

function moPhongPhienDayDu(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
  chienLuocCuaSo: "truot" | "tom_tat",
  nsKhuVuc: NganSachTheoKhuVuc,
): KetQuaBoss {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = chienLuocCuaSo === "tom_tat" ? catVaTomTat(lichSu, nganSach) : catCuaSoTruot(lichSu, nganSach);
    }
  }
  const soGhiNhoTuCuaSo = lichSu.flatMap((tin) => trichXuatGhiNho(tin, cacTuKhoaCanTrich));
  const heThong = lichSu.filter((m) => m.role === "system");
  const tomTat = lichSu.filter((m) => m.content.startsWith("[TOM TAT]"));
  const ganDay = lichSu.filter((m) => m.role !== "system" && !m.content.startsWith("[TOM TAT]"));
  return {
    soTurnDaChay: cacLuot.length,
    soGhiNhoCuoiCung: soGhiNho,
    cuaSoCuoiCung: lichSu,
    tyLeRecallSoGhiNho: tinhTyLeRecall(soGhiNho, cacTuKhoaCanTrich),
    tyLeRecallCuaSo: tinhTyLeRecall(soGhiNhoTuCuaSo, cacTuKhoaCanTrich),
    kiemTraNganSachLuotCuoi: kiemTraNganSachTheoKhuVuc(heThong, tomTat, ganDay, nsKhuVuc),
  };
}
```

`tyLeRecallCuaSo` KHÔNG gọi `coConNhoSuKien` (boolean nhị phân, bài
`3` q9.2a) — nó áp `trichXuatGhiNho` (bài `9`) lên TỪNG message CÒN LẠI
trong `cuaSoCuoiCung`, GOM thành một "sổ ghi nhớ tương đương của cửa
sổ", rồi đo `tinhTyLeRecall` TRÊN ĐÓ — CÙNG một cách đo, ÁP lên HAI
nguồn dữ liệu khác nhau (sổ ghi nhớ thật VÀ cửa sổ hiển thị), cho ra
HAI con số so sánh được TRỰC TIẾP với nhau. `heThong`/`tomTat`/`ganDay`
được TÁCH từ `lichSu` cuối cùng bằng `role`/tiền tố `"[TOM TAT]"` —
không cần theo dõi ba mảng riêng suốt vòng lặp.
::::

::::example{#do_ca_hai_chien_luoc}
Bảy lượt (`CAC_LUOT_HOI_THOAI`, bài `6` q9.2a), ngân sách `130`, ba cụm
từ (`"XY789"`, `"30 ngay"`, `"giam gia"`), VÀ ngân sách theo khu vực
`{ heThong: 20, tomTat: 30, ganDay: 40, duTruPhanHoi: 20 }` — so sánh
`"truot"` VÀ `"tom_tat"`:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}
interface KetQuaBoss {
  soTurnDaChay: number;
  soGhiNhoCuoiCung: MucGhiNho[];
  cuaSoCuoiCung: ChatMessage[];
  tyLeRecallSoGhiNho: number;
  tyLeRecallCuaSo: number;
  kiemTraNganSachLuotCuoi: { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean };
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}
function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  const phanBiCat = phanCoTheCat.slice(0, batDau);
  const phanConLai = phanCoTheCat.slice(batDau);
  if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai];
  return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(soGhiNhoHienTai: MucGhiNho[], tinMoi: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}
function kiemTraNganSachTheoKhuVuc(
  heThong: ChatMessage[],
  tomTat: ChatMessage[],
  ganDay: ChatMessage[],
  ns: NganSachTheoKhuVuc,
): { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean } {
  const heThongOk = tinhTongToken(heThong) <= ns.heThong;
  const tomTatOk = tinhTongToken(tomTat) <= ns.tomTat;
  const ganDayOk = tinhTongToken(ganDay) <= ns.ganDay;
  const conDuDuTru = ns.duTruPhanHoi > 0;
  return { heThongOk, tomTatOk, ganDayOk, conDuDuTru };
}
function moPhongPhienDayDu(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
  chienLuocCuaSo: "truot" | "tom_tat",
  nsKhuVuc: NganSachTheoKhuVuc,
): KetQuaBoss {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = chienLuocCuaSo === "tom_tat" ? catVaTomTat(lichSu, nganSach) : catCuaSoTruot(lichSu, nganSach);
    }
  }
  const soGhiNhoTuCuaSo = lichSu.flatMap((tin) => trichXuatGhiNho(tin, cacTuKhoaCanTrich));
  const heThong = lichSu.filter((m) => m.role === "system");
  const tomTat = lichSu.filter((m) => m.content.startsWith("[TOM TAT]"));
  const ganDay = lichSu.filter((m) => m.role !== "system" && !m.content.startsWith("[TOM TAT]"));
  return {
    soTurnDaChay: cacLuot.length,
    soGhiNhoCuoiCung: soGhiNho,
    cuaSoCuoiCung: lichSu,
    tyLeRecallSoGhiNho: tinhTyLeRecall(soGhiNho, cacTuKhoaCanTrich),
    tyLeRecallCuaSo: tinhTyLeRecall(soGhiNhoTuCuaSo, cacTuKhoaCanTrich),
    kiemTraNganSachLuotCuoi: kiemTraNganSachTheoKhuVuc(heThong, tomTat, ganDay, nsKhuVuc),
  };
}

const CAC_LUOT_HOI_THOAI: ChatMessage[][] = [
  [
    { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
    { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ],
  [{ role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." }],
  [{ role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." }],
  [{ role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." }],
  [{ role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." }],
  [{ role: "assistant", content: "Hien tai cua hang dang giam gia 10 phan tram cho don hang tren 500 nghin dong." }],
  [{ role: "user", content: "Toi cung muon hoi them ve thoi gian giao hang trung binh la bao lau." }],
];
const TU_KHOA_BOSS = ["XY789", "30 ngay", "giam gia"];
const NS_KHU_VUC: NganSachTheoKhuVuc = { heThong: 20, tomTat: 30, ganDay: 40, duTruPhanHoi: 20 };

for (const chienLuoc of ["truot", "tom_tat"] as const) {
  const kq = moPhongPhienDayDu(CAC_LUOT_HOI_THOAI, 130, TU_KHOA_BOSS, chienLuoc, NS_KHU_VUC);
  console.log(
    chienLuoc,
    kq.soTurnDaChay,
    kq.tyLeRecallSoGhiNho,
    kq.tyLeRecallCuaSo,
    JSON.stringify(kq.kiemTraNganSachLuotCuoi),
  );
}
```

```text title=readonly
truot 7 1 0.6666666666666666 {"heThongOk":true,"tomTatOk":true,"ganDayOk":false,"conDuDuTru":true}
tom_tat 7 1 0.6666666666666666 {"heThongOk":true,"tomTatOk":true,"ganDayOk":false,"conDuDuTru":true}
```

Cả HAI chiến lược cho ra CÙNG bốn con số: `tyLeRecallSoGhiNho = 1`
(sổ ghi nhớ không mất mục nào), `tyLeRecallCuaSo ≈ 0.667` (`2/3` — CỬA
SỔ chỉ còn giữ được `"30 ngay"` VÀ `"giam gia"`, mất `"XY789"`), VÀ
`ganDayOk: false` (khu vực message gần đây đã phình quá `40` token
ngân sách). `tyLeRecallCuaSo` KHÔNG cao hơn Ở `"tom_tat"` dù bài `7`
từng chứng minh tóm tắt giữ được MỘT PHẦN thông tin — vì `"XY789"` nằm
NGOÀI `20` ký tự đầu của message bị cắt, `tomTatDoan` không giữ được
CHÍNH cụm từ này (dù có thể giữ được cụm từ KHÁC, tình cờ nằm sớm hơn
trong câu). Hai chiến lược VẪN khác nhau Ở CẤU TRÚC cửa sổ (`"tom_tat"`
có thêm một message `"[TOM TAT]"`), chỉ KHÔNG khác Ở bốn con số đo
được lần này.
::::

::::predict{#doan_tom_tat_co_cuu_duoc_khong commitOnce}
Đổi `chienLuocCuaSo` từ `"truot"` sang `"tom_tat"` (giữ NGUYÊN mọi tham
số khác) — `tyLeRecallCuaSo` có TĂNG lên không, nhờ bài `7` đã chứng
minh tóm tắt giữ lại một phần thông tin thay vì xoá trắng?

:::opt{correct}
KHÔNG — `tyLeRecallCuaSo` VẪN LÀ `0.667` Ở CẢ HAI chiến lược: cụm
`"XY789"` nằm SAU vị trí ký tự thứ `20` của message chứa nó (đã thấy Ở
bài `7`), nên `tomTatDoan` KHÔNG giữ được ĐÚNG cụm từ này dù có tóm tắt
hay không — tóm tắt chỉ giúp khi cụm từ TÌNH CỜ rơi vào `20` ký tự
đầu, không phải một đảm bảo chung cho MỌI cụm từ
:::
:::opt
CÓ, TĂNG lên — bài `7` đã chứng minh tóm tắt giữ được một phần thông
tin thay vì mất trắng, nên `"tom_tat"` LUÔN cho recall cao hơn
`"truot"`
::why
Nhầm "tóm tắt giữ được MỘT PHẦN thông tin nói chung" (đúng, bài `7`) VỚI
"tóm tắt giữ được BẤT KỲ cụm từ CỤ THỂ nào cần kiểm" (sai) —
`tomTatDoan` chỉ giữ `20` ký tự ĐẦU của mỗi message bị cắt; nếu cụm từ
cần nhớ nằm NGOÀI `20` ký tự đó, tóm tắt không giúp được gì cho ĐÚNG
cụm từ đó, dù vẫn giữ được PHẦN KHÁC của message.

Chỗ lệch: `tyLeRecallCuaSo` đo trên `cacTuKhoaCanTrich` CỤ THỂ
(`"XY789"`, `"30 ngay"`, `"giam gia"`) — không đo một khái niệm "giữ
được thông tin nói chung" trừu tượng nào cả; với ĐÚNG ba cụm này, kết
quả trùng nhau giữa hai chiến lược.
::
:::
:::opt
CÓ, nhưng chỉ tăng Ở `kiemTraNganSachLuotCuoi.tomTatOk`, không phải Ở
`tyLeRecallCuaSo`
::why
Gần đúng Ở việc bạn để Ý `"tom_tat"` có ẢNH HƯỞNG tới một trường khác
(`tomTatOk` thật sự chỉ có Ý NGHĨA khi `chienLuocCuaSo` LÀ `"tom_tat"`,
vì chỉ khi đó mới có message `"[TOM TAT]"` để tính) — quan sát Ở
HƯỚNG đó không sai.

Chỗ lệch: trong kịch bản CỤ THỂ này, `tomTatOk` đã LÀ `true` Ở CẢ HAI
chiến lược (`0 <= 30` khi không có tóm tắt, `~8 <= 30` khi có) — không
có gì "tăng lên" Ở đó để so sánh; VÀ như đã giải thích, `tyLeRecallCuaSo`
cũng KHÔNG đổi giữa hai chiến lược cho kịch bản này.
::
:::
::::

::::code{#viet_mo_phong_phien_day_du}
Hoàn thiện `moPhongPhienDayDu` — bên trong nhánh `vuotNganSach`: áp
ĐÚNG chiến lược cắt tương ứng `chienLuocCuaSo`. Sau vòng lặp: tính
`soGhiNhoTuCuaSo` (áp `trichXuatGhiNho` lên TỪNG message còn lại trong
`lichSu`), tách `heThong`/`tomTat`/`ganDay`, rồi trả về đủ sáu trường
của `KetQuaBoss`.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}
interface KetQuaBoss {
  soTurnDaChay: number;
  soGhiNhoCuoiCung: MucGhiNho[];
  cuaSoCuoiCung: ChatMessage[];
  tyLeRecallSoGhiNho: number;
  tyLeRecallCuaSo: number;
  kiemTraNganSachLuotCuoi: { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean };
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}
function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  const phanBiCat = phanCoTheCat.slice(0, batDau);
  const phanConLai = phanCoTheCat.slice(batDau);
  if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai];
  return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(soGhiNhoHienTai: MucGhiNho[], tinMoi: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}
function kiemTraNganSachTheoKhuVuc(
  heThong: ChatMessage[],
  tomTat: ChatMessage[],
  ganDay: ChatMessage[],
  ns: NganSachTheoKhuVuc,
): { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean } {
  const heThongOk = tinhTongToken(heThong) <= ns.heThong;
  const tomTatOk = tinhTongToken(tomTat) <= ns.tomTat;
  const ganDayOk = tinhTongToken(ganDay) <= ns.ganDay;
  const conDuDuTru = ns.duTruPhanHoi > 0;
  return { heThongOk, tomTatOk, ganDayOk, conDuDuTru };
}

function moPhongPhienDayDu(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
  chienLuocCuaSo: "truot" | "tom_tat",
  nsKhuVuc: NganSachTheoKhuVuc,
): KetQuaBoss {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      ___
    }
  }
  ___
}

const CAC_LUOT_HOI_THOAI: ChatMessage[][] = [
  [
    { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
    { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ],
  [{ role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." }],
  [{ role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." }],
  [{ role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." }],
  [{ role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." }],
  [{ role: "assistant", content: "Hien tai cua hang dang giam gia 10 phan tram cho don hang tren 500 nghin dong." }],
  [{ role: "user", content: "Toi cung muon hoi them ve thoi gian giao hang trung binh la bao lau." }],
];
const TU_KHOA_BOSS = ["XY789", "30 ngay", "giam gia"];
const NS_KHU_VUC: NganSachTheoKhuVuc = { heThong: 20, tomTat: 30, ganDay: 40, duTruPhanHoi: 20 };

const ketQuaBoss = moPhongPhienDayDu(CAC_LUOT_HOI_THOAI, 130, TU_KHOA_BOSS, "tom_tat", NS_KHU_VUC);
console.log(
  ketQuaBoss.soTurnDaChay,
  ketQuaBoss.tyLeRecallSoGhiNho,
  ketQuaBoss.tyLeRecallCuaSo,
  JSON.stringify(ketQuaBoss.kiemTraNganSachLuotCuoi),
);
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}
interface KetQuaBoss {
  soTurnDaChay: number;
  soGhiNhoCuoiCung: MucGhiNho[];
  cuaSoCuoiCung: ChatMessage[];
  tyLeRecallSoGhiNho: number;
  tyLeRecallCuaSo: number;
  kiemTraNganSachLuotCuoi: { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean };
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function tomTatDoan(messages: ChatMessage[]): ChatMessage {
  const noiDung = messages.map((tin) => tin.content.slice(0, 20)).join(" | ");
  return { role: "assistant", content: `[TOM TAT] ${noiDung}` };
}
function catVaTomTat(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  const phanBiCat = phanCoTheCat.slice(0, batDau);
  const phanConLai = phanCoTheCat.slice(batDau);
  if (phanBiCat.length === 0) return [...phanGiuCoDinh, ...phanConLai];
  return [...phanGiuCoDinh, tomTatDoan(phanBiCat), ...phanConLai];
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(soGhiNhoHienTai: MucGhiNho[], tinMoi: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}
function kiemTraNganSachTheoKhuVuc(
  heThong: ChatMessage[],
  tomTat: ChatMessage[],
  ganDay: ChatMessage[],
  ns: NganSachTheoKhuVuc,
): { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean } {
  const heThongOk = tinhTongToken(heThong) <= ns.heThong;
  const tomTatOk = tinhTongToken(tomTat) <= ns.tomTat;
  const ganDayOk = tinhTongToken(ganDay) <= ns.ganDay;
  const conDuDuTru = ns.duTruPhanHoi > 0;
  return { heThongOk, tomTatOk, ganDayOk, conDuDuTru };
}

function moPhongPhienDayDu(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
  chienLuocCuaSo: "truot" | "tom_tat",
  nsKhuVuc: NganSachTheoKhuVuc,
): KetQuaBoss {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = chienLuocCuaSo === "tom_tat" ? catVaTomTat(lichSu, nganSach) : catCuaSoTruot(lichSu, nganSach);
    }
  }
  const soGhiNhoTuCuaSo = lichSu.flatMap((tin) => trichXuatGhiNho(tin, cacTuKhoaCanTrich));
  const heThong = lichSu.filter((m) => m.role === "system");
  const tomTat = lichSu.filter((m) => m.content.startsWith("[TOM TAT]"));
  const ganDay = lichSu.filter((m) => m.role !== "system" && !m.content.startsWith("[TOM TAT]"));
  return {
    soTurnDaChay: cacLuot.length,
    soGhiNhoCuoiCung: soGhiNho,
    cuaSoCuoiCung: lichSu,
    tyLeRecallSoGhiNho: tinhTyLeRecall(soGhiNho, cacTuKhoaCanTrich),
    tyLeRecallCuaSo: tinhTyLeRecall(soGhiNhoTuCuaSo, cacTuKhoaCanTrich),
    kiemTraNganSachLuotCuoi: kiemTraNganSachTheoKhuVuc(heThong, tomTat, ganDay, nsKhuVuc),
  };
}

const CAC_LUOT_HOI_THOAI: ChatMessage[][] = [
  [
    { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
    { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ],
  [{ role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." }],
  [{ role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." }],
  [{ role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." }],
  [{ role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." }],
  [{ role: "assistant", content: "Hien tai cua hang dang giam gia 10 phan tram cho don hang tren 500 nghin dong." }],
  [{ role: "user", content: "Toi cung muon hoi them ve thoi gian giao hang trung binh la bao lau." }],
];
const TU_KHOA_BOSS = ["XY789", "30 ngay", "giam gia"];
const NS_KHU_VUC: NganSachTheoKhuVuc = { heThong: 20, tomTat: 30, ganDay: 40, duTruPhanHoi: 20 };

const ketQuaBoss = moPhongPhienDayDu(CAC_LUOT_HOI_THOAI, 130, TU_KHOA_BOSS, "tom_tat", NS_KHU_VUC);
console.log(
  ketQuaBoss.soTurnDaChay,
  ketQuaBoss.tyLeRecallSoGhiNho,
  ketQuaBoss.tyLeRecallCuaSo,
  JSON.stringify(ketQuaBoss.kiemTraNganSachLuotCuoi),
);
```

```typescript title=test
if (ketQuaBoss.soTurnDaChay !== 7) throw new Error("phai chay dung 7 luot");
if (Math.abs(ketQuaBoss.tyLeRecallSoGhiNho - 1) > 1e-9) throw new Error("tom_tat: ty le recall tren SO GHI NHO phai la 1.0");
if (Math.abs(ketQuaBoss.tyLeRecallCuaSo - 2 / 3) > 1e-9) throw new Error("tom_tat: ty le recall tren CUA SO phai la 2/3 (mat XY789)");
if (JSON.stringify(ketQuaBoss.kiemTraNganSachLuotCuoi) !== JSON.stringify({ heThongOk: true, tomTatOk: true, ganDayOk: false, conDuDuTru: true })) {
  throw new Error("kiem tra ngan sach luot cuoi phai la heThongOk=true, tomTatOk=true, ganDayOk=false, conDuDuTru=true");
}
if (!ketQuaBoss.cuaSoCuoiCung.some((m) => m.content.startsWith("[TOM TAT]"))) {
  throw new Error("chien luoc tom_tat phai de lai it nhat mot message bat dau bang [TOM TAT] trong cua so cuoi cung");
}

const ketQuaTruot = moPhongPhienDayDu(CAC_LUOT_HOI_THOAI, 130, TU_KHOA_BOSS, "truot", NS_KHU_VUC);
if (Math.abs(ketQuaTruot.tyLeRecallSoGhiNho - 1) > 1e-9) throw new Error("truot: ty le recall tren SO GHI NHO cung phai la 1.0 (doc lap chien luoc cua so)");
if (Math.abs(ketQuaTruot.tyLeRecallCuaSo - 2 / 3) > 1e-9) throw new Error("truot: ty le recall tren CUA SO cung phai la 2/3");
if (ketQuaTruot.cuaSoCuoiCung.some((m) => m.content.startsWith("[TOM TAT]"))) {
  throw new Error("chien luoc truot KHONG duoc co bat ky message [TOM TAT] nao -- xoa han, khong tom tat");
}

const ganDayTang = moPhongPhienDayDu(CAC_LUOT_HOI_THOAI, 130, TU_KHOA_BOSS, "tom_tat", { ...NS_KHU_VUC, ganDay: 120 });
if (ganDayTang.kiemTraNganSachLuotCuoi.ganDayOk !== true) {
  throw new Error("tang nsKhuVuc.ganDay len 120 phai lam ganDayOk thanh true -- tham so phai that su rang buoc");
}

const khongConDuTru = moPhongPhienDayDu(CAC_LUOT_HOI_THOAI, 130, TU_KHOA_BOSS, "tom_tat", { ...NS_KHU_VUC, duTruPhanHoi: 0 });
if (khongConDuTru.kiemTraNganSachLuotCuoi.conDuDuTru !== false) {
  throw new Error("duTruPhanHoi = 0 phai lam conDuDuTru thanh false");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (ben trong if vuotNganSach): mot dong -- lichSu = chienLuocCuaSo === 'tom_tat' ? catVaTomTat(lichSu, nganSach) : catCuaSoTruot(lichSu, nganSach). Cho hai (sau vong for ngoai): tinh soGhiNhoTuCuaSo bang lichSu.flatMap ap trichXuatGhiNho len tung message; tach heThong/tomTat/ganDay bang filter theo role va tien to '[TOM TAT]'; roi return du 6 truong cua KetQuaBoss (soTurnDaChay, soGhiNhoCuoiCung, cuaSoCuoiCung, tyLeRecallSoGhiNho, tyLeRecallCuaSo, kiemTraNganSachLuotCuoi)."
- kind: strategy
  body: "Cho dau: lichSu = chienLuocCuaSo === \"tom_tat\" ? catVaTomTat(lichSu, nganSach) : catCuaSoTruot(lichSu, nganSach); Cho hai: const soGhiNhoTuCuaSo = lichSu.flatMap((tin) => trichXuatGhiNho(tin, cacTuKhoaCanTrich)); const heThong = lichSu.filter((m) => m.role === \"system\"); const tomTat = lichSu.filter((m) => m.content.startsWith(\"[TOM TAT]\")); const ganDay = lichSu.filter((m) => m.role !== \"system\" && !m.content.startsWith(\"[TOM TAT]\")); return { soTurnDaChay: cacLuot.length, soGhiNhoCuoiCung: soGhiNho, cuaSoCuoiCung: lichSu, tyLeRecallSoGhiNho: tinhTyLeRecall(soGhiNho, cacTuKhoaCanTrich), tyLeRecallCuaSo: tinhTyLeRecall(soGhiNhoTuCuaSo, cacTuKhoaCanTrich), kiemTraNganSachLuotCuoi: kiemTraNganSachTheoKhuVuc(heThong, tomTat, ganDay, nsKhuVuc) };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, DUNG THU TU."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "7 1 0.6666666666666666"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`1.0` VÀ `0.667` — hai con số recall TÁCH BIỆT trên CÙNG một phiên,
CÙNG lúc `ganDayOk: false` chỉ đúng MỘT khu vực cụ thể. Đây LÀ hình
dạng của một hệ thống quản lý ngữ cảnh PRODUCTION thật: không chỉ cắt
bớt khi vượt ngân sách (q9.2a) — mà còn TÓM TẮT thay vì xoá, CHIA ngân
sách theo chức năng, VÀ giữ một SỔ GHI NHỚ bền vững tách biệt khỏi cửa
sổ hiển thị. q9.2b — "Kỹ nghệ hoá ngữ cảnh" — đóng tại `6/6`.
::::

::::reflect{#nghi-lai}
`moPhongPhienDayDu` không thêm bất kỳ Ý TƯỞNG mới nào — nó LÀ đúng năm
mảnh ghép của q9.2b, chạy CÙNG nhau: `capNhatSoGhiNho` (bài `9`) xây
sổ ghi nhớ TỪ TỪNG message, `catCuaSoTruot`/`catVaTomTat` (bài `2` q9.2a/
bài `7`) quản lý cửa sổ hiển thị, `tinhTyLeRecall` (bài `10`) đo được
CẢ HAI phía, VÀ `kiemTraNganSachTheoKhuVuc` (bài `8`) xác nhận từng khu
vực CỦA cửa sổ cuối cùng. Bài học lớn nhất xuyên suốt q9.2b: một hệ
thống ngữ cảnh production KHÔNG chỉ cần "cắt đúng lúc" (q9.2a đã dạy)
— nó cần XỬ LÝ có chủ đích phần bị cắt (tóm tắt, không xoá trắng), CHIA
trách nhiệm ngân sách theo khu vực (không dồn một cục), VÀ tách RIÊNG
bộ nhớ dài hạn khỏi cửa sổ ngắn hạn — ba nguyên lý kỹ nghệ, không phải
một thuật toán cắt thông minh hơn.
::::

::::checkpoint{mastery=0.9}
::::
