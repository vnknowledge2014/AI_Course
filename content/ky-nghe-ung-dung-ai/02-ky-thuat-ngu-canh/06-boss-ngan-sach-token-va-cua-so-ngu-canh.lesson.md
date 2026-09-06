---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.boss-ngan-sach-token-va-cua-so-ngu-canh
title: "BOSS — mô phỏng phiên nhiều lượt, đo %turn vượt & tỉ lệ recall"
summary: "moPhongPhienHoiThoai(cacLuot: ChatMessage[][], nganSach: number, cumTuCanTim: string, chienLuoc: \"uu_tien\"|\"mu\") ráp NGUYÊN VĂN vuotNganSach (bài 1), catTheoUuTien (bài 4), catCuaSoTruot (bài 2), coConNhoSuKien (bài 3) — chạy qua 7 lượt tích luỹ (CAC_LUOT_HOI_THOAI, dựng từ CHÍNH 6 message của LICH_SU_HOI_THOAI xuyên suốt quest + 1 lượt mới), ngân sách 50. Ở MỖI lượt: nếu vuotNganSach thì áp chiến lược cắt tương ứng. Cả hai chiến lược cho CÙNG %turn vượt ngân sách (4/7 ≈ 57%, vượt hay không KHÔNG phụ thuộc chiến lược cắt) — nhưng coConNhoSuKienCuoiCung khác hẳn: true với 'uu_tien' (ghim động, tìm lại chỉ số chứa sự kiện MỖI lượt), false với 'mu' (catCuaSoTruot không phân biệt được message nào đáng giữ). Đổi ngân sách xuống 40: %turn tăng lên 6/7, nhưng 'uu_tien' vẫn giữ được recall true — pin không bao giờ bị cắt, bất kể ngân sách nhỏ tới đâu. Đóng q9.2a tại 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-ngan-sach-token-va-cua-so-ngu-canh]
requires: [kna.context-assembly-la-monoid-co-rang-buoc-ngan-sach]
concepts: [kna.boss-ngan-sach-token-va-cua-so-ngu-canh]
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
Năm bài: đếm token giả lập, cửa sổ trượt, context rot, lắp theo ưu
tiên, cầu nối Monoid có ràng buộc ngân sách. Nhưng mọi phép đo TỪ NÃY
GIỜ đều LÀ một lát cắt TĨNH — một lịch sử, một lần cắt. Một phiên hội
thoại THẬT không tĩnh: nó tích luỹ qua NHIỀU lượt, và Ở MỖI lượt, hệ
thống phải tự hỏi lại: có vượt ngân sách chưa? Bài này mô phỏng ĐÚNG
điều đó — VÀ đo hai chỉ số cùng lúc: bao nhiêu % lượt phải cắt, VÀ liệu
thông tin quan trọng có sống sót tới lượt cuối hay không.
::::

::::explain{#mo-phong_nhieu_luot}
Một phiên hội thoại LÀ một DÃY các "lượt" — mỗi lượt thêm một hoặc vài
message MỚI vào lịch sử đã có. `moPhongPhienHoiThoai` lặp qua từng
lượt, TÍCH LUỸ lịch sử, VÀ Ở MỖI lượt kiểm `vuotNganSach` (bài `1`,
tái dùng nguyên văn) — nếu vượt, áp chiến lược cắt tương ứng
(`chienLuoc`): `"uu_tien"` gọi `catTheoUuTien` (bài `4`) sau khi TÌM
LẠI chỉ số chứa `cumTuCanTim` bằng `timChiSoChuaCumTu` (MỖI lượt tìm
lại, vì chỉ số dịch chuyển sau mỗi lần cắt); `"mu"` gọi `catCuaSoTruot`
(bài `2`) không ghim gì cả. Cuối cùng, đo `%turn` phải cắt VÀ
`coConNhoSuKien` (bài `3`) trên lịch sử CÒN LẠI Ở LƯỢT CUỐI:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
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
  while (
    batDau < phanCoTheCat.length &&
    tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach
  ) {
    batDau += 1;
  }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}

function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) {
    soLuongDaCat += 1;
  }
  return layKetQua(soLuongDaCat);
}

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}

function timChiSoChuaCumTu(messages: ChatMessage[], cumTuCanTim: string): number[] {
  const ketQua: number[] = [];
  messages.forEach((tin, i) => {
    if (tin.content.includes(cumTuCanTim)) ketQua.push(i);
  });
  return ketQua;
}
```

Chú Ý: `timChiSoChuaCumTu` KHÔNG phải một hàm tái dùng từ bài trước —
nó LÀ phần "keo dán" MỚI, cần thiết vì `catTheoUuTien` cần biết CHỈ SỐ
(không phải nội dung) để ghim, VÀ chỉ số đó phải được tìm LẠI Ở MỖI
lượt (vì sau mỗi lần cắt, vị trí của message quan trọng trong mảng đã
dịch chuyển).
::::

::::example{#do_hai_chien_luoc}
Bảy lượt — dựng TỪ CHÍNH sáu message của `LICH_SU_HOI_THOAI` xuyên
suốt quest (chia nhỏ thành lượt), CỘNG một lượt thứ bảy mới — với ngân
sách `50`, so sánh `"uu_tien"` VÀ `"mu"`:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface KetQuaMoPhongPhien {
  soTurnDaChay: number;
  soTurnVuotNganSach: number;
  tiLePhanTramVuotNganSach: number;
  coConNhoSuKienCuoiCung: boolean;
}
type ChienLuocCat = "uu_tien" | "mu";

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
function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) { soLuongDaCat += 1; }
  return layKetQua(soLuongDaCat);
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function timChiSoChuaCumTu(messages: ChatMessage[], cumTuCanTim: string): number[] {
  const ketQua: number[] = [];
  messages.forEach((tin, i) => { if (tin.content.includes(cumTuCanTim)) ketQua.push(i); });
  return ketQua;
}

function moPhongPhienHoiThoai(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cumTuCanTim: string,
  chienLuoc: ChienLuocCat,
): KetQuaMoPhongPhien {
  let lichSu: ChatMessage[] = [];
  let soTurnVuot = 0;
  for (const luot of cacLuot) {
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      soTurnVuot += 1;
      if (chienLuoc === "uu_tien") {
        const chiSoGhim = timChiSoChuaCumTu(lichSu, cumTuCanTim);
        lichSu = catTheoUuTien(lichSu, nganSach, chiSoGhim);
      } else {
        lichSu = catCuaSoTruot(lichSu, nganSach);
      }
    }
  }
  return {
    soTurnDaChay: cacLuot.length,
    soTurnVuotNganSach: soTurnVuot,
    tiLePhanTramVuotNganSach: soTurnVuot / cacLuot.length,
    coConNhoSuKienCuoiCung: coConNhoSuKien(lichSu, cumTuCanTim),
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

const ketQuaUuTien = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 50, "XY789", "uu_tien");
const ketQuaMu = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 50, "XY789", "mu");
console.log(JSON.stringify(ketQuaUuTien));
console.log(JSON.stringify(ketQuaMu));
```

```text title=readonly
{"soTurnDaChay":7,"soTurnVuotNganSach":4,"tiLePhanTramVuotNganSach":0.5714285714285714,"coConNhoSuKienCuoiCung":true}
{"soTurnDaChay":7,"soTurnVuotNganSach":4,"tiLePhanTramVuotNganSach":0.5714285714285714,"coConNhoSuKienCuoiCung":false}
```

`4/7` lượt (≈`57%`) phải cắt Ở CẢ HAI chiến lược — con số này KHÔNG phụ
thuộc chiến lược cắt, vì `vuotNganSach` chỉ nhìn TỔNG token, không quan
tâm sẽ cắt kiểu gì. Nhưng `coConNhoSuKienCuoiCung` tách hẳn LÀM ĐÔI:
`true` với `"uu_tien"` (mã đơn hàng sống sót tới lượt cuối), `false`
với `"mu"` (context rot Ở đâu đó dọc đường) — CÙNG số liệu vượt ngân
sách, khác hẳn kết quả recall.
::::

::::predict{#doan_giam_ngan_sach commitOnce}
Giảm ngân sách xuống RẤT THẤP (ví dụ `10` — hầu như MỌI lượt đều phải
cắt). Với chiến lược `"uu_tien"`, `coConNhoSuKienCuoiCung` có bị ảnh
hưởng không?

:::opt{correct}
KHÔNG — `timChiSoChuaCumTu` tìm LẠI chỉ số chứa `"XY789"` Ở MỖI lượt
(một khi nó CÒN tồn tại trong lịch sử), VÀ `catTheoUuTien` KHÔNG BAO
GIỜ cắt một chỉ số đã được ghim, bất kể ngân sách nhỏ tới đâu — mã đơn
hàng được BẢO VỆ ngay từ lượt đầu tiên nó xuất hiện, VÀ sự bảo vệ đó
được LÀM MỚI Ở MỌI lượt sau, nên vẫn LÀ `true` dù ngân sách cực thấp
:::
:::opt
CÓ — ngân sách càng thấp thì càng nhiều message bị cắt, sớm muộn gì
message chứa sự kiện cũng sẽ bị cắt theo
::why
Nhầm "càng thấp càng cắt nhiều" (đúng, về TỔNG SỐ message bị cắt) VỚI
"sớm muộn MESSAGE GHIM cũng bị cắt" (sai — `catTheoUuTien` không hề
CÓ khái niệm "cắt bớt Ở message đã ghim khi ngân sách quá thấp", nó chỉ
cắt trong tập `chiSoCoTheCat`, tập KHÔNG BAO GIỜ chứa chỉ số đã ghim).

Chỗ lệch: ngân sách thấp làm TĂNG số message KHÔNG-ghim bị cắt (VÀ tăng
`%turn` vượt ngân sách) — nhưng KHÔNG đụng được tới message ĐÃ ghim,
dù cửa sổ cuối cùng có thể chỉ còn lại `system` + message ghim đó, gần
như trống trơn phần còn lại.
::
:::
:::opt
Không xác định được — phụ thuộc thứ tự các lượt được xử lý trong vòng
lặp `for`
::why
Gần đúng Ở việc bạn nghĩ tới THỨ TỰ xử lý — quan sát đó ĐÚNG hướng
nhưng thứ tự Ở đây đã CỐ ĐỊNH (đúng thứ tự của `cacLuot`, không ngẫu
nhiên).

Chỗ lệch: với thứ tự CỐ ĐỊNH VÀ ngân sách CỐ ĐỊNH, kết quả LÀ tất
định hoàn toàn — không có yếu tố "không đoán trước được" nào trong
`moPhongPhienHoiThoai`, mọi bước đều LÀ vòng lặp `for` tuần tự.
::
:::
::::

::::code{#viet_mo_phong_phien_hoi_thoai}
Hoàn thiện `moPhongPhienHoiThoai` — bên trong nhánh `vuotNganSach`:
tăng `soTurnVuot`, rồi áp ĐÚNG chiến lược cắt tương ứng với `chienLuoc`
(`"uu_tien"` dùng `catTheoUuTien` sau khi tìm chỉ số ghim;
`"mu"` dùng `catCuaSoTruot`). Sau vòng lặp: trả về đủ bốn trường của
`KetQuaMoPhongPhien`.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface KetQuaMoPhongPhien {
  soTurnDaChay: number;
  soTurnVuotNganSach: number;
  tiLePhanTramVuotNganSach: number;
  coConNhoSuKienCuoiCung: boolean;
}
type ChienLuocCat = "uu_tien" | "mu";

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
function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) { soLuongDaCat += 1; }
  return layKetQua(soLuongDaCat);
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function timChiSoChuaCumTu(messages: ChatMessage[], cumTuCanTim: string): number[] {
  const ketQua: number[] = [];
  messages.forEach((tin, i) => { if (tin.content.includes(cumTuCanTim)) ketQua.push(i); });
  return ketQua;
}

function moPhongPhienHoiThoai(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cumTuCanTim: string,
  chienLuoc: ChienLuocCat,
): KetQuaMoPhongPhien {
  let lichSu: ChatMessage[] = [];
  let soTurnVuot = 0;
  for (const luot of cacLuot) {
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

const ketQuaUuTien = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 50, "XY789", "uu_tien");
const ketQuaMu = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 50, "XY789", "mu");
console.log(JSON.stringify(ketQuaUuTien), JSON.stringify(ketQuaMu));
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface KetQuaMoPhongPhien {
  soTurnDaChay: number;
  soTurnVuotNganSach: number;
  tiLePhanTramVuotNganSach: number;
  coConNhoSuKienCuoiCung: boolean;
}
type ChienLuocCat = "uu_tien" | "mu";

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
function catTheoUuTien(messages: ChatMessage[], nganSach: number, cacChiSoGhim: number[]): ChatMessage[] {
  const ghim = new Set<number>(cacChiSoGhim);
  if (messages.length > 0 && messages[0]!.role === "system") ghim.add(0);
  const chiSoCoTheCat = messages.map((_, i) => i).filter((i) => !ghim.has(i));
  let soLuongDaCat = 0;
  const layKetQua = (soCat: number): ChatMessage[] => {
    const biCat = new Set(chiSoCoTheCat.slice(0, soCat));
    return messages.filter((_, i) => !biCat.has(i));
  };
  while (soLuongDaCat < chiSoCoTheCat.length && tinhTongToken(layKetQua(soLuongDaCat)) > nganSach) { soLuongDaCat += 1; }
  return layKetQua(soLuongDaCat);
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function timChiSoChuaCumTu(messages: ChatMessage[], cumTuCanTim: string): number[] {
  const ketQua: number[] = [];
  messages.forEach((tin, i) => { if (tin.content.includes(cumTuCanTim)) ketQua.push(i); });
  return ketQua;
}

function moPhongPhienHoiThoai(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cumTuCanTim: string,
  chienLuoc: ChienLuocCat,
): KetQuaMoPhongPhien {
  let lichSu: ChatMessage[] = [];
  let soTurnVuot = 0;
  for (const luot of cacLuot) {
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      soTurnVuot += 1;
      if (chienLuoc === "uu_tien") {
        const chiSoGhim = timChiSoChuaCumTu(lichSu, cumTuCanTim);
        lichSu = catTheoUuTien(lichSu, nganSach, chiSoGhim);
      } else {
        lichSu = catCuaSoTruot(lichSu, nganSach);
      }
    }
  }
  return {
    soTurnDaChay: cacLuot.length,
    soTurnVuotNganSach: soTurnVuot,
    tiLePhanTramVuotNganSach: soTurnVuot / cacLuot.length,
    coConNhoSuKienCuoiCung: coConNhoSuKien(lichSu, cumTuCanTim),
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

const ketQuaUuTien = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 50, "XY789", "uu_tien");
const ketQuaMu = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 50, "XY789", "mu");
console.log(JSON.stringify(ketQuaUuTien), JSON.stringify(ketQuaMu));
```

```typescript title=test
if (ketQuaUuTien.soTurnDaChay !== 7) throw new Error("phai chay dung 7 luot");
if (ketQuaUuTien.soTurnVuotNganSach !== 4) throw new Error("uu tien: so luot vuot ngan sach phai la 4");
if (Math.abs(ketQuaUuTien.tiLePhanTramVuotNganSach - 4 / 7) > 1e-9) throw new Error("uu tien: ti le vuot phai la 4/7");
if (ketQuaUuTien.coConNhoSuKienCuoiCung !== true) throw new Error("uu tien: phai con nho su kien XY789 o luot cuoi");

if (ketQuaMu.soTurnVuotNganSach !== 4) throw new Error("mu: so luot vuot ngan sach cung phai la 4 (giong het uu tien, vi vuot hay khong KHONG phu thuoc chien luoc cat)");
if (Math.abs(ketQuaMu.tiLePhanTramVuotNganSach - 4 / 7) > 1e-9) throw new Error("mu: ti le vuot phai la 4/7");
if (ketQuaMu.coConNhoSuKienCuoiCung !== false) throw new Error("mu: KHONG con nho su kien XY789 o luot cuoi -- context rot xay ra voi cat mu");

const ketQuaBudget40 = moPhongPhienHoiThoai(CAC_LUOT_HOI_THOAI, 40, "XY789", "uu_tien");
if (ketQuaBudget40.soTurnVuotNganSach !== 6) throw new Error("doi ngan sach xuong 40 phai lam so luot vuot tang len 6 -- tham so nganSach phai that su rang buoc");
if (ketQuaBudget40.coConNhoSuKienCuoiCung !== true) throw new Error("ngan sach thap hon van khong duoc lam mat message da ghim voi chien luoc uu_tien");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (ben trong if vuotNganSach): tang soTurnVuot len 1; roi if (chienLuoc === 'uu_tien') tim chiSoGhim bang timChiSoChuaCumTu(lichSu, cumTuCanTim) va gan lai lichSu = catTheoUuTien(lichSu, nganSach, chiSoGhim); else gan lai lichSu = catCuaSoTruot(lichSu, nganSach). Cho hai (sau vong for): return mot KetQuaMoPhongPhien voi bon truong: soTurnDaChay = cacLuot.length, soTurnVuotNganSach = soTurnVuot, tiLePhanTramVuotNganSach = soTurnVuot / cacLuot.length, coConNhoSuKienCuoiCung = coConNhoSuKien(lichSu, cumTuCanTim)."
- kind: strategy
  body: "Cho dau: soTurnVuot += 1; if (chienLuoc === \"uu_tien\") { const chiSoGhim = timChiSoChuaCumTu(lichSu, cumTuCanTim); lichSu = catTheoUuTien(lichSu, nganSach, chiSoGhim); } else { lichSu = catCuaSoTruot(lichSu, nganSach); } Cho hai: return { soTurnDaChay: cacLuot.length, soTurnVuotNganSach: soTurnVuot, tiLePhanTramVuotNganSach: soTurnVuot / cacLuot.length, coConNhoSuKienCuoiCung: coConNhoSuKien(lichSu, cumTuCanTim) };"
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
  expect: "\"tiLePhanTramVuotNganSach\":0.5714285714285714,\"coConNhoSuKienCuoiCung\":true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`4/7` lượt phải cắt Ở CẢ HAI chiến lược — CÙNG con số vượt ngân sách.
Nhưng `coConNhoSuKienCuoiCung`: `true` với ghim động, `false` với cắt
mù. Ngân sách càng nhỏ, %turn càng tăng, nhưng chiến lược ưu tiên VẪN
giữ được recall — vì cái BỊ bảo vệ không phải LÀ "message cũ ít hơn",
mà LÀ "message chứa thông tin đã biết LÀ quan trọng". Đóng `q9.2a` —
"Ngân sách token & cửa sổ ngữ cảnh" — tại `6/6`.
::::

::::reflect{#nghi-lai}
`moPhongPhienHoiThoai` không phát minh thêm một kỹ thuật cắt nào MỚI —
nó ráp ĐÚNG bốn hàm đã xây riêng lẻ Ở bốn bài trước, chạy CHÚNG lặp đi
lặp lại qua nhiều lượt: `vuotNganSach` (bài `1`) trả lời "có cần cắt
không", `catTheoUuTien`/`catCuaSoTruot` (bài `4`/`2`) trả lời "cắt cái
gì", `coConNhoSuKien` (bài `3`) trả lời "còn nhớ được không". Bài học
quan trọng nhất của CẢ track: `%turn` vượt ngân sách LÀ một thuộc tính
của DỮ LIỆU (bao nhiêu nội dung tích luỹ so với ngân sách) — hoàn toàn
ĐỘC LẬP với chiến lược cắt. Nhưng RECALL LÀ một thuộc tính của CHIẾN
LƯỢC — cùng phải cắt y hệt nhau, một chiến lược nhớ được, một chiến
lược quên mất. Đo cả hai CÙNG lúc, trên CÙNG kịch bản, LÀ cách duy nhất
biết được một hệ thống quản lý ngữ cảnh có thật sự tốt hay không.
::::

::::checkpoint{mastery=0.9}
::::
