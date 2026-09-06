---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.ngan-sach-theo-khu-vuc
title: "Ngân sách chia theo khu vực: biết vượt Ở đâu, không chỉ vượt hay không"
summary: "interface NganSachTheoKhuVuc { heThong: number; tomTat: number; ganDay: number; duTruPhanHoi: number } — MỖI khu vực chức năng có ngân sách RIÊNG, không dùng chung MỘT số như vuotNganSach (bài 1, q9.2a). kiemTraNganSachTheoKhuVuc(heThong, tomTat, ganDay, ns): { heThongOk; tomTatOk; ganDayOk; conDuDuTru } kiểm TỪNG khu vực ĐỘC LẬP (tái dùng tinhTongToken cho từng mảng, so <= ngân sách riêng của nó) VÀ conDuDuTru = ns.duTruPhanHoi > 0. Trên một ví dụ cụ thể (actual 13/18/36 token, ngân sách 25/20/30): heThongOk=true, tomTatOk=true, ganDayOk=false — CHỈ khu ganDay vượt. Đối chiếu vuotNganSach (bài 1) trên TỔNG (actual 67, ngân sách cộng lại 75) trả về false (\"không vượt\") — cách nhìn tổng BỎ LỠ đúng vấn đề mà cách nhìn theo khu vực phát hiện được: vượt CỤ THỂ Ở ganDay, không phải chỗ nào khác."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.ngan-sach-theo-khu-vuc]
requires: [kna.tom-tat-thay-vi-xoa]
concepts: [kna.ngan-sach-theo-khu-vuc]
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
Suốt q9.2a VÀ bài trước, `nganSach` LUÔN LÀ MỘT con số DUY NHẤT cho
TOÀN BỘ cửa sổ — `vuotNganSach` chỉ trả lời "tổng có vượt hay không".
Nhưng một cửa sổ THẬT không đồng nhất: chỉ thị hệ thống, các bản tóm
tắt, VÀ các message gần đây phục vụ ba MỤC ĐÍCH khác nhau, VÀ nên có BA
ngân sách khác nhau — không phải chia đều MỘT số. Bài này tách ngân
sách theo KHU VỰC chức năng, VÀ lộ ra một điều cách nhìn "tổng" không
bao giờ thấy được: vượt CỤ THỂ Ở ĐÂU.
::::

::::explain{#ngan-sach-theo-khu-vuc}
`NganSachTheoKhuVuc` có BỐN trường — mỗi trường LÀ ngân sách RIÊNG cho
MỘT khu vực: `heThong` (chỉ thị hệ thống), `tomTat` (các bản tóm tắt,
bài trước), `ganDay` (các message gần đây, còn nguyên văn), VÀ
`duTruPhanHoi` (số token DÀNH RIÊNG cho câu trả lời model sắp tạo ra —
không phải ngân sách cho DỮ LIỆU ĐẦU VÀO như ba trường kia).
`kiemTraNganSachTheoKhuVuc` kiểm TỪNG khu vực ĐỘC LẬP, tái dùng
`tinhTongToken` (bài `1`, q9.2a) cho MỖI mảng:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
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
```

Mỗi trường `*Ok` so sánh bằng `<=` (ĐÚNG BẰNG ngân sách VẪN LÀ ok, khác
`vuotNganSach` bài `1` dùng `>` cho chiều "vượt") — VÀ MỖI trường CHỈ
đọc mảng VÀ ngân sách CỦA CHÍNH khu vực đó, không hề đọc chéo sang khu
vực khác. `conDuDuTru` không kiểm một MẢNG nào cả — nó chỉ kiểm
`ns.duTruPhanHoi` có LỚN HƠN `0` hay không (còn ngân sách dành cho câu
trả lời sắp tới, hay đã cạn/âm).
::::

::::example{#mot-khu-vuc-vuot-cac-khu-khac-on}
Ba khu vực với nội dung THẬT (số token khác nhau: `13`, `18`, `36`) VÀ
ba ngân sách khác nhau (`25`, `20`, `30`) — so sánh kết quả theo khu
vực VỚI cách nhìn "tổng" của `vuotNganSach` (bài `1`, q9.2a):

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
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

const KHU_HE_THONG: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
];
const KHU_TOM_TAT: ChatMessage[] = [
  { role: "assistant", content: "[TOM TAT] khach hang XY789 hoi giao hang, doi tra, va khuyen mai them." },
];
const KHU_GAN_DAY: ChatMessage[] = [
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];
const NS: NganSachTheoKhuVuc = { heThong: 25, tomTat: 20, ganDay: 30, duTruPhanHoi: 20 };

console.log("token tung khu vuc:", tinhTongToken(KHU_HE_THONG), tinhTongToken(KHU_TOM_TAT), tinhTongToken(KHU_GAN_DAY));
console.log(JSON.stringify(kiemTraNganSachTheoKhuVuc(KHU_HE_THONG, KHU_TOM_TAT, KHU_GAN_DAY, NS)));

const TAT_CA = [...KHU_HE_THONG, ...KHU_TOM_TAT, ...KHU_GAN_DAY];
const TONG_NGAN_SACH = NS.heThong + NS.tomTat + NS.ganDay;
console.log("tong token:", tinhTongToken(TAT_CA), "tong ngan sach:", TONG_NGAN_SACH, "vuot(tong)?", vuotNganSach(TAT_CA, TONG_NGAN_SACH));
```

```text title=readonly
token tung khu vuc: 13 18 36
{"heThongOk":true,"tomTatOk":true,"ganDayOk":false,"conDuDuTru":true}
tong token: 67 tong ngan sach: 75 vuot(tong)? false
```

`ganDayOk: false` — CHỈ khu `ganDay` vượt (`36 > 30`), hai khu còn lại
ổn. Nhưng nhìn theo TỔNG (`vuotNganSach` VỚI tổng ngân sách cộng dồn
`75`): `67 <= 75`, `vuot(tong)` LÀ `false` — cách nhìn TỔNG nói "ổn cả",
trong khi CHÍNH XÁC MỘT khu vực đang vượt. Đây LÀ giá trị của việc chia
ngân sách theo khu vực: nó phát hiện được vấn đề CỤ THỂ Ở ĐÂU, điều mà
một con số tổng duy nhất không bao giờ lộ ra.
::::

::::predict{#doan-tang-ngan-sach-gan-day commitOnce}
Tăng `ns.ganDay` từ `30` lên `40` (giữ nguyên MỌI thứ khác — cùng nội
dung ba khu vực, cùng `heThong`/`tomTat`/`duTruPhanHoi`). `ganDayOk` sẽ
đổi thành gì, VÀ ba trường còn lại (`heThongOk`, `tomTatOk`,
`conDuDuTru`) có bị ảnh hưởng theo không?

:::opt{correct}
`ganDayOk` đổi thành `true` (`36 <= 40`); BA trường còn lại KHÔNG đổi —
mỗi trường được tính từ MỘT phép so sánh ĐỘC LẬP, chỉ đọc đúng mảng VÀ
ngân sách CỦA CHÍNH nó, không có phép tính nào trong `kiemTraNganSachTheoKhuVuc`
đọc chéo sang trường khác
:::
:::opt
CẢ BỐN trường đều đổi, vì tăng MỘT ngân sách LÀM tổng ngân sách chung
tăng theo, ảnh hưởng tới TOÀN BỘ kết quả
::why
Nhầm cách `vuotNganSach` (bài `1`) nhìn MỘT TỔNG DUY NHẤT VỚI cách
`kiemTraNganSachTheoKhuVuc` hoạt động — hàm này KHÔNG bao giờ cộng dồn
ba ngân sách lại thành MỘT con số để so sánh, nó tính BA phép so sánh
TÁCH BIỆT HOÀN TOÀN.

Chỗ lệch: `heThongOk` chỉ đọc `heThong`/`ns.heThong`, `tomTatOk` chỉ đọc
`tomTat`/`ns.tomTat` — không có biến nào trong hai phép tính đó phụ
thuộc `ns.ganDay`, nên tăng `ns.ganDay` không thể làm chúng đổi.
::
:::
:::opt
Không đổi gì cả — `ganDayOk` đã LÀ `false` nên tăng ngân sách không đủ
LÀM nó bật lại thành `true`, phải tăng ngân sách CỦA TẤT CẢ khu vực mới
có tác dụng
::why
Gần đúng Ở việc bạn nhận ra có một PHÉP SO SÁNH ngưỡng đang diễn ra —
quan sát đó đúng hướng.

Chỗ lệch: `ganDayOk` CHỈ phụ thuộc `tinhTongToken(ganDay)` (`36`, cố
định) VÀ `ns.ganDay` — tăng riêng `ns.ganDay` từ `30` lên `40` LÀM
`36 <= 40` chuyển từ `false` sang `true` NGAY LẬP TỨC, không cần đụng
tới bất kỳ ngân sách nào khác.
::
:::
::::

::::code{#viet_kiem_tra_ngan_sach_theo_khu_vuc}
Hoàn thiện `kiemTraNganSachTheoKhuVuc` — phần ĐẦU: tính `heThongOk`
(so `tinhTongToken(heThong)` VỚI `ns.heThong`, dùng `<=`). Phần CUỐI
(sau `tomTatOk`/`ganDayOk` đã cho sẵn): tính `conDuDuTru`, rồi trả về
đủ bốn trường.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}

function kiemTraNganSachTheoKhuVuc(
  heThong: ChatMessage[],
  tomTat: ChatMessage[],
  ganDay: ChatMessage[],
  ns: NganSachTheoKhuVuc,
): { heThongOk: boolean; tomTatOk: boolean; ganDayOk: boolean; conDuDuTru: boolean } {
  const heThongOk = ___;
  const tomTatOk = tinhTongToken(tomTat) <= ns.tomTat;
  const ganDayOk = tinhTongToken(ganDay) <= ns.ganDay;
  ___
}

const KHU_HE_THONG: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
];
const KHU_TOM_TAT: ChatMessage[] = [
  { role: "assistant", content: "[TOM TAT] khach hang XY789 hoi giao hang, doi tra, va khuyen mai them." },
];
const KHU_GAN_DAY: ChatMessage[] = [
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];
const NS: NganSachTheoKhuVuc = { heThong: 25, tomTat: 20, ganDay: 30, duTruPhanHoi: 20 };

const ketQuaKhuVuc = kiemTraNganSachTheoKhuVuc(KHU_HE_THONG, KHU_TOM_TAT, KHU_GAN_DAY, NS);
console.log(JSON.stringify(ketQuaKhuVuc));
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface NganSachTheoKhuVuc {
  heThong: number;
  tomTat: number;
  ganDay: number;
  duTruPhanHoi: number;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
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

const KHU_HE_THONG: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
];
const KHU_TOM_TAT: ChatMessage[] = [
  { role: "assistant", content: "[TOM TAT] khach hang XY789 hoi giao hang, doi tra, va khuyen mai them." },
];
const KHU_GAN_DAY: ChatMessage[] = [
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];
const NS: NganSachTheoKhuVuc = { heThong: 25, tomTat: 20, ganDay: 30, duTruPhanHoi: 20 };

const ketQuaKhuVuc = kiemTraNganSachTheoKhuVuc(KHU_HE_THONG, KHU_TOM_TAT, KHU_GAN_DAY, NS);
console.log(JSON.stringify(ketQuaKhuVuc));
```

```typescript title=test
if (JSON.stringify(ketQuaKhuVuc) !== JSON.stringify({ heThongOk: true, tomTatOk: true, ganDayOk: false, conDuDuTru: true })) {
  throw new Error("ket qua phai la heThongOk=true, tomTatOk=true, ganDayOk=false, conDuDuTru=true");
}

const ganDayTang = kiemTraNganSachTheoKhuVuc(KHU_HE_THONG, KHU_TOM_TAT, KHU_GAN_DAY, { ...NS, ganDay: 40 });
if (ganDayTang.ganDayOk !== true) throw new Error("tang ns.ganDay len 40 phai lam ganDayOk thanh true");
if (ganDayTang.heThongOk !== true || ganDayTang.tomTatOk !== true || ganDayTang.conDuDuTru !== true) {
  throw new Error("doi rieng ganDay khong duoc lam doi ba truong con lai");
}

const khongConDuTru = kiemTraNganSachTheoKhuVuc(KHU_HE_THONG, KHU_TOM_TAT, KHU_GAN_DAY, { ...NS, duTruPhanHoi: 0 });
if (khongConDuTru.conDuDuTru !== false) throw new Error("duTruPhanHoi = 0 thi conDuDuTru phai la false, khong con du tru");
if (khongConDuTru.heThongOk !== true || khongConDuTru.ganDayOk !== false) {
  throw new Error("doi duTruPhanHoi khong duoc anh huong toi heThongOk/ganDayOk");
}

const heThongDungBang: ChatMessage[] = [{ role: "system", content: "a".repeat(100) }];
if (kiemTraNganSachTheoKhuVuc(heThongDungBang, [], [], { heThong: 25, tomTat: 0, ganDay: 0, duTruPhanHoi: 1 }).heThongOk !== true) {
  throw new Error("dung bang ngan sach (100 ky tu = 25 token, ngan sach 25) van phai la Ok = true, dung <=");
}
const heThongVuotMotToken: ChatMessage[] = [{ role: "system", content: "a".repeat(101) }];
if (kiemTraNganSachTheoKhuVuc(heThongVuotMotToken, [], [], { heThong: 25, tomTat: 0, ganDay: 0, duTruPhanHoi: 1 }).heThongOk !== false) {
  throw new Error("vuot dung 1 token (101 ky tu = 26 token, ngan sach 25) phai la Ok = false");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau: const heThongOk = tinhTongToken(heThong) <= ns.heThong (dung <=, khong phai <). Cho hai (sau ganDayOk): const conDuDuTru = ns.duTruPhanHoi > 0; roi return { heThongOk, tomTatOk, ganDayOk, conDuDuTru };"
- kind: strategy
  body: "Cho dau: const heThongOk = tinhTongToken(heThong) <= ns.heThong; Cho hai: const conDuDuTru = ns.duTruPhanHoi > 0; return { heThongOk, tomTatOk, ganDayOk, conDuDuTru };"
- kind: one-line
  body: "Sao chep dung hai dong o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"heThongOk\":true,\"tomTatOk\":true,\"ganDayOk\":false,\"conDuDuTru\":true}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`ganDayOk: false` trong khi TỔNG (`67 <= 75`) nói "ổn cả" — chia ngân
sách theo khu vực tìm ra đúng vấn đề mà cách nhìn tổng bỏ lỡ. Nhưng
CẢ hai bài vừa qua đều xử lý một cửa sổ TĨNH. Bài sau chuyển hướng: một
SỔ GHI NHỚ tách biệt hoàn toàn khỏi cửa sổ, tích luỹ qua nhiều lượt,
không quan tâm cửa sổ có bị cắt bao nhiêu lần.
::::

::::reflect{#nghi-lai}
`kiemTraNganSachTheoKhuVuc` không hề "thông minh" hơn `vuotNganSach`
(bài `1`, q9.2a) Ở khả năng ĐẾM — cả hai đều chỉ so sánh MỘT tổng VỚI
MỘT ngưỡng. Khác biệt duy nhất, nhưng quan trọng: `vuotNganSach` đếm
TRÊN MỘT MẢNG GỘP CHUNG, còn `kiemTraNganSachTheoKhuVuc` đếm TRÊN BA
MẢNG TÁCH RIÊNG rồi so từng cặp ĐỘC LẬP. Chính sự tách biệt đơn giản đó
— không phải một thuật toán phức tạp hơn — LÀ thứ biến "có vượt hay
không" thành "vượt Ở ĐÂU". Một hệ thống production cần biết CHÍNH XÁC
khu vực nào đang phình to để sửa ĐÚNG chỗ, không phải cắt bừa MỘT bên
nào đó chỉ vì tổng chung báo động.
::::

::::checkpoint{mastery=0.8}
::::
