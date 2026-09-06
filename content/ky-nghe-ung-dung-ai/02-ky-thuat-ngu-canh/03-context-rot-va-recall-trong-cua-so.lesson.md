---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.context-rot-va-recall-trong-cua-so
title: "Context rot: khi cửa sổ trượt cắt mất sự kiện quan trọng"
summary: "coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean tìm một cụm từ cụ thể (\"XY789\", mã đơn hàng nhắc TRONG message sớm của LICH_SU_HOI_THOAI, bài 1) trong TOÀN BỘ nội dung các message CÒN LẠI trong cửa sổ. soSanhTruocSauKhiCat ráp coConNhoSuKien với catCuaSoTruot (bài 2, tái dùng nguyên văn): TRƯỚC khi cắt (lịch sử gốc) → true; SAU khi cắt bằng sliding window ngân sách 50 (bài 2) → false — message chứa sự kiện đã bị cắt vì nó nằm giữa lịch sử (cũ), không phải mới nhất. Đây LÀ \"context rot\" đo được bằng boolean cụ thể, không phải mô tả mơ hồ — và nó CHỈ xảy ra khi sự kiện quan trọng nằm Ở message CŨ; nếu nó nằm Ở message MỚI NHẤT, sliding window vẫn giữ được (vì cắt theo tuổi, không theo nội dung)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.context-rot-va-recall-trong-cua-so]
requires: [kna.cua-so-truot]
concepts: [kna.context-rot-va-recall-trong-cua-so]
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
Bài trước cắt ĐÚNG như thiết kế: giữ mới, cắt cũ. Nhưng một trong ba
message bị cắt chứa `"XY789"` — mã đơn hàng mà người dùng nhắc tới Ở
LƯỢT ĐẦU TIÊN. Nếu lượt hội thoại SAU hỏi lại "mã đơn hàng của tôi là
gì", model không còn cách nào biết — không phải vì nó "quên", mà vì
CHÍNH thông tin đó không còn nằm trong cửa sổ nữa. Đây LÀ context rot,
và bài này đo nó bằng MỘT boolean cụ thể.
::::

::::explain{#con-nho-su-kien}
`coConNhoSuKien` không hề "hiểu" nội dung — nó chỉ tìm một CỤM TỪ CỤ
THỂ trong `content` của MỌI message CÒN LẠI. Đây LÀ mô hình đơn giản
nhất cho câu hỏi "thông tin này có còn trong cửa sổ hay không":

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
```

Điểm mấu chốt: `coConNhoSuKien` LUÔN chạy trên MẢNG được truyền vào —
nếu mảng đó LÀ lịch sử GỐC (chưa cắt), nó tìm trên TOÀN BỘ; nếu mảng đó
LÀ kết quả của `catCuaSoTruot` (bài `2`, đã cắt), nó CHỈ tìm trên PHẦN
CÒN LẠI. Cùng một hàm, khác đầu vào, có thể cho ra hai kết quả trái
ngược — VÀ đó CHÍNH LÀ cách đo context rot: so sánh kết quả trên
lịch sử gốc VỚI kết quả trên lịch sử ĐÃ CẮT.
::::

::::example{#do-truoc-sau-khi-cat}
Trên `LICH_SU_HOI_THOAI` (bài `1`), message Ở index `1` chứa
`"XY789"`. So sánh `coConNhoSuKien` TRƯỚC và SAU khi áp `catCuaSoTruot`
(bài `2`) với ngân sách `50`:

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

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

console.log("truoc khi cat:", coConNhoSuKien(LICH_SU_HOI_THOAI, "XY789"));
const sauKhiCat = catCuaSoTruot(LICH_SU_HOI_THOAI, 50);
console.log("sau khi cat:", coConNhoSuKien(sauKhiCat, "XY789"));
```

```text title=readonly
truoc khi cat: true
sau khi cat: false
```

`true` rồi `false` — CÙNG một cụm từ, CÙNG một hàm tìm kiếm, chỉ khác
Ở việc dữ liệu đầu vào đã bị `catCuaSoTruot` cắt bớt hay chưa. Đây
KHÔNG phải model "quên" theo nghĩa nhận thức — đây LÀ THÔNG TIN không
còn tồn tại VẬT LÝ trong cửa sổ được gửi tới model. Context rot đo
được: `true → false`, không phải một cảm giác chung chung "câu trả lời
có vẻ sai".
::::

::::predict{#doan-su-kien-o-cuoi commitOnce}
Giả sử THAY VÌ nằm Ở message SỚM, câu chứa `"XY789"` lại LÀ message
MỚI NHẤT (cuối mảng) — người dùng vừa nhắc mã đơn hàng Ở LƯỢT CUỐI
CÙNG, ngay trước khi gọi `catCuaSoTruot` với CÙNG ngân sách `50`. Context
rot có còn xảy ra không?

:::opt{correct}
KHÔNG — `catCuaSoTruot` LUÔN giữ lại những message MỚI NHẤT (cắt từ
ĐẦU, không phải từ CUỐI); nếu `"XY789"` nằm Ở message cuối, nó nằm
trong PHẦN ĐƯỢC GIỮ LẠI, `coConNhoSuKien` sau khi cắt vẫn trả về `true`
— cửa sổ trượt CHỈ làm mất thông tin CŨ, không làm mất thông tin MỚI
:::
:::opt
CÓ, vẫn xảy ra y hệt — vì `catCuaSoTruot` cắt một SỐ LƯỢNG message cố
định bất kể nội dung nằm Ở đâu
::why
Nhầm "cắt một số lượng cố định" VỚI cách `catCuaSoTruot` THẬT SỰ hoạt
động — nó KHÔNG cắt theo số lượng cố định, nó cắt CHO TỚI KHI VỪA NGÂN
SÁCH, và LUÔN cắt từ PHÍA CŨ (`batDau` tăng dần từ đầu `phanCoTheCat`).

Chỗ lệch: vị trí của thông tin quan trọng (đầu hay cuối lịch sử) hoàn
toàn quyết định nó có bị cắt hay không — cửa sổ trượt thiên vị RÕ RÀNG
cho message MỚI, đó chính LÀ nguyên tắc thiết kế của nó.
::
:::
:::opt
Không xác định được — còn tuỳ vào TÊN của `role` gắn với message chứa
`"XY789"`
::why
Gần đúng Ở việc bạn nhớ `role` CÓ ảnh hưởng tới việc bảo vệ — nhưng chỉ
ĐÚNG cho MỘT trường hợp riêng: `role === "system"` VÀ Ở vị trí ĐẦU
TIÊN (`messages[0]`).

Chỗ lệch: với message KHÔNG Ở vị trí đầu (dù `role` LÀ gì —
`user`/`assistant`), `catCuaSoTruot` chỉ quan tâm THỨ TỰ trong mảng
(cũ/mới), hoàn toàn không đọc `role` của nó.
::
:::
::::

::::code{#viet_con_nho_su_kien}
Hoàn thiện `coConNhoSuKien` — dùng `.some` kiểm tra CÓ message nào
chứa `cumTuCanTim` hay không. Hoàn thiện `soSanhTruocSauKhiCat` — tính
`truocKhiCat` (gọi `coConNhoSuKien` trên lịch sử GỐC) VÀ `sauKhiCat`
(gọi `coConNhoSuKien` trên kết quả `catCuaSoTruot`, bài `2`), trả về cả
hai trong một object.

```typescript title=starter
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

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  ___
}

function soSanhTruocSauKhiCat(
  messages: ChatMessage[],
  nganSach: number,
  cumTuCanTim: string,
): { truocKhiCat: boolean; sauKhiCat: boolean } {
  ___
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaSoSanh = soSanhTruocSauKhiCat(LICH_SU_HOI_THOAI, 50, "XY789");
console.log(ketQuaSoSanh.truocKhiCat, ketQuaSoSanh.sauKhiCat);
```

```typescript title=solution
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

function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}

function soSanhTruocSauKhiCat(
  messages: ChatMessage[],
  nganSach: number,
  cumTuCanTim: string,
): { truocKhiCat: boolean; sauKhiCat: boolean } {
  const truocKhiCat = coConNhoSuKien(messages, cumTuCanTim);
  const sauKhiCat = coConNhoSuKien(catCuaSoTruot(messages, nganSach), cumTuCanTim);
  return { truocKhiCat, sauKhiCat };
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

const ketQuaSoSanh = soSanhTruocSauKhiCat(LICH_SU_HOI_THOAI, 50, "XY789");
console.log(ketQuaSoSanh.truocKhiCat, ketQuaSoSanh.sauKhiCat);
```

```typescript title=test
if (ketQuaSoSanh.truocKhiCat !== true) throw new Error("truoc khi cat phai con nho su kien (con nguyen trong lich su goc)");
if (ketQuaSoSanh.sauKhiCat !== false) throw new Error("sau khi cat bang sliding window ngan sach 50, message chua su kien phai bi mat -- context rot");

const khongCat = soSanhTruocSauKhiCat(LICH_SU_HOI_THOAI, 1000, "XY789");
if (khongCat.truocKhiCat !== true || khongCat.sauKhiCat !== true) throw new Error("ngan sach du lon thi khong cat gi, van con nho ca truoc lan sau");

if (coConNhoSuKien(LICH_SU_HOI_THOAI, "khong ton tai cum tu nay") !== false) throw new Error("cum tu khong xuat hien phai tra ve false");
if (coConNhoSuKien(LICH_SU_HOI_THOAI, "30 ngay") !== true) throw new Error("cum tu xuat hien o message giua/cuoi van phai tim thay");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (coConNhoSuKien): return messages.some voi dieu kien tin.content.includes(cumTuCanTim). Cho hai (soSanhTruocSauKhiCat): tinh truocKhiCat bang coConNhoSuKien(messages, cumTuCanTim), tinh sauKhiCat bang coConNhoSuKien(catCuaSoTruot(messages, nganSach), cumTuCanTim), roi return { truocKhiCat, sauKhiCat }."
- kind: strategy
  body: "Cho dau: return messages.some((tin) => tin.content.includes(cumTuCanTim)); Cho hai: const truocKhiCat = coConNhoSuKien(messages, cumTuCanTim); const sauKhiCat = coConNhoSuKien(catCuaSoTruot(messages, nganSach), cumTuCanTim); return { truocKhiCat, sauKhiCat };"
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
  expect: "true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`true` rồi `false` — context rot đo được bằng đúng một cặp boolean.
Vấn đề KHÔNG nằm Ở việc cắt bớt (bắt buộc, ngân sách hữu hạn) — mà Ở
việc cắt MÙ theo tuổi, không phân biệt được message nào ĐÁNG giữ. Bài
sau sửa đúng lỗ hổng này: cắt theo ƯU TIÊN, không cắt mù theo tuổi.
::::

::::reflect{#nghi-lai}
`coConNhoSuKien` chỉ LÀ một phép tìm kiếm chuỗi ĐƠN GIẢN — sức mạnh của
bài này không nằm Ở bản thân hàm đó, mà Ở cách DÙNG nó để BIẾN một trực
giác mơ hồ ("model có vẻ quên mất mã đơn hàng") thành một PHÉP ĐO cụ
thể: gọi CÙNG một hàm trên HAI đầu vào (trước/sau khi cắt), so sánh kết
quả. `true → false` LÀ context rot. Nếu kết quả LÀ `true → true` (như
khi sự kiện nằm Ở message mới nhất), KHÔNG có context rot — cùng chiến
lược cắt, khác VỊ TRÍ thông tin, khác kết quả hoàn toàn.
::::

::::checkpoint{mastery=0.80}
::::
