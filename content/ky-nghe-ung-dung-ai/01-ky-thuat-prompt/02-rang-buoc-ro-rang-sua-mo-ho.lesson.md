---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.rang-buoc-ro-rang-sua-mo-ho
title: "Ràng buộc rõ ràng sửa mơ hồ: đo lại pass@1 trên cùng bộ test"
summary: "phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]) giu nguyen ba tu khoa cua bai truoc, nhung them mot NHANH THU: neu co system message chua chi thi 'khong duoc tra loi khong_ro', prompt mo ho duoc do THEM theo tu khoa RONG hon (\"y kien\"->tom_tat, \"ngon ngu khac\"->dich) truoc khi roi ve mac dinh tom_tat -- KHONG con duoc tra ve khong_ro nua. Tren CUNG bo 6 prompt cua bai 1: khong rang buoc pass@1=4/6; CO rang buoc (them dung MOT system message vao moi loi goi) pass@1=6/6 -- ca hai cau mo ho deu duoc suy dung, cai thien do duoc bang so, khong noi suong."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.rang-buoc-ro-rang-sua-mo-ho]
requires: [kna.vai-tro-message-va-phan-loai-nhiem-vu]
concepts: [kna.rang-buoc-ro-rang-sua-mo-ho]
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
Bài trước đo được: `pass@1 = 4/6` — hai câu mơ hồ bị phân loại sai
thành `"khong_ro"`. Nhưng `"khong_ro"` không phải một GIỚI HẠN cố định
của harness — nó LÀ hành vi mặc định khi KHÔNG có chỉ thị nào ép model
phải chọn một trong ba loại thật sự. Thêm đúng MỘT ràng buộc, đo lại
TRÊN CÙNG bộ test.
::::

::::explain{#rang-buoc-ep-chon}
Một CHỈ THỊ HỆ THỐNG (message `role="system"`) có thể ép model KHÔNG
được trả lời "tôi không biết" — bắt buộc chọn một trong các lựa chọn đã
liệt kê. `coRangBuocChonMotTrongBa` quét mọi message tìm cụm cố định
`"khong duoc tra loi khong_ro"` Ở bất kỳ message `system` nào.
`phanLoaiNhiemVuCoRangBuoc` giữ NGUYÊN ba từ khoá cứng của bài trước —
"tom tat", "dich", "dem"/"tong" — nhưng khi CÓ ràng buộc VÀ không từ
khoá cứng nào khớp, nó thử thêm hai từ khoá RỘNG hơn ("y kien" gợi Ý
một yêu cầu tóm tắt/nhận xét, "ngon ngu khac" gợi Ý một yêu cầu dịch),
RỒI mới rơi về một lựa chọn mặc định — KHÔNG BAO GIỜ còn trả về
`"khong_ro"` khi ràng buộc có mặt:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}

function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}

function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}

const khongRangBuoc: ChatMessage[] = [{ role: "user", content: "Cho toi y kien ve bai viet nay" }];
console.log("khong co rang buoc:", phanLoaiNhiemVuCoRangBuoc(khongRangBuoc));

const coRangBuoc: ChatMessage[] = [
  { role: "system", content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro." },
  { role: "user", content: "Cho toi y kien ve bai viet nay" },
];
console.log("co rang buoc:", phanLoaiNhiemVuCoRangBuoc(coRangBuoc));
```

```text title=readonly
khong co rang buoc: khong_ro
co rang buoc: tom_tat
```

CÙNG một prompt — `"Cho toi y kien ve bai viet nay"` — cho hai kết quả
KHÁC nhau, chỉ vì CÓ hay KHÔNG có message `system` đi kèm. Không có
ràng buộc, `buocChon = false`, hai nhánh mới đều bị bỏ qua, hàm rơi
thẳng về `"khong_ro"`. Có ràng buộc, prompt chứa `"y kien"` VÀ
`buocChon = true`, nhánh thứ tư khớp NGAY, trả về `"tom_tat"`.
::::

::::example{#do-lai-tren-bo-test}
Đo lại pass@1 trên ĐÚNG bộ `6` prompt của bài trước — lần NÀY, MỌI lời
gọi đều kèm CÙNG một message hệ thống ép buộc:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}

function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}

function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}

function tinhPassAt1(ketQua: boolean[]): number {
  if (ketQua.length === 0) return 0;
  const soDung = ketQua.filter((x) => x).length;
  return soDung / ketQua.length;
}

const CHI_THI_RANG_BUOC =
  "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro.";

function taoMessagesCoRangBuoc(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: CHI_THI_RANG_BUOC },
    { role: "user", content: prompt },
  ];
}

interface CauKiemThu {
  prompt: string;
  nhanDung: "tom_tat" | "dich" | "dem_so" | "khong_ro";
}

const BO_KIEM_THU: CauKiemThu[] = [
  { prompt: "Hay tom tat doan van ban sau day thanh mot doan ngan", nhanDung: "tom_tat" },
  { prompt: "Dich cau sau sang tieng Anh: Xin chao ban", nhanDung: "dich" },
  { prompt: "Dem so luong don hang thang nay giup toi", nhanDung: "dem_so" },
  { prompt: "Tinh tong doanh thu quy nay", nhanDung: "dem_so" },
  { prompt: "Cho toi y kien ve bai viet nay", nhanDung: "tom_tat" },
  { prompt: "Giup toi chuyen tai lieu nay sang ngon ngu khac", nhanDung: "dich" },
];

const ketQuaCoRangBuoc = BO_KIEM_THU.map(
  (c) => phanLoaiNhiemVuCoRangBuoc(taoMessagesCoRangBuoc(c.prompt)) === c.nhanDung,
);
console.log(BO_KIEM_THU.map((c) => phanLoaiNhiemVuCoRangBuoc(taoMessagesCoRangBuoc(c.prompt))));
console.log(ketQuaCoRangBuoc);
console.log("pass@1 co rang buoc:", tinhPassAt1(ketQuaCoRangBuoc));
```

```text title=readonly
["tom_tat","dich","dem_so","dem_so","tom_tat","dich"]
[true,true,true,true,true,true]
pass@1 co rang buoc: 1
```

`pass@1` tăng từ `4/6` (bài trước, không ràng buộc) lên `6/6` (ràng
buộc trên MỌI lời gọi) — CẢ HAI câu mơ hồ giờ được suy đúng: câu chứa
`"y kien"` suy ra `"tom_tat"`, câu chứa `"ngon ngu khac"` suy ra
`"dich"`. Bốn câu có từ khoá cứng vẫn phân loại y hệt bài trước — ràng
buộc CHỈ mở rộng nhánh xử lý mơ hồ, không đụng vào ba nhánh đã đúng.
::::

::::predict{#doan-khong-khop-tu-khoa-rong commitOnce}
Một prompt mơ hồ, ĐI KÈM ràng buộc, nhưng KHÔNG chứa `"y kien"` LẪN
`"ngon ngu khac"` — ví dụ `"Ban co the giup toi khong"`. Kết quả LÀ gì?

:::opt{correct}
`"tom_tat"` — không từ khoá cứng nào khớp, không từ khoá rộng nào
khớp, nhưng `buocChon` LÀ `true` nên nhánh `if (buocChon) return
"tom_tat";` chạy, trả về một lựa chọn MẶC ĐỊNH — KHÔNG BAO GIỜ còn rơi
xuống `return "khong_ro"` khi có ràng buộc
:::
:::opt
`"khong_ro"` — không khớp bất kỳ từ khoá nào (cứng lẫn rộng) thì vẫn
phải là "không rõ", ràng buộc chỉ áp dụng khi CÓ Ít nhất một tín hiệu
::why
Nhầm "ràng buộc chỉ mở rộng phạm vi từ khoá" VỚI "ràng buộc PHẢI luôn
đi kèm MỘT tín hiệu mới trả lời" — nhưng code có một nhánh MẶC ĐỊNH
`if (buocChon) return "tom_tat";` đứng NGAY TRƯỚC dòng `return
"khong_ro";` cuối cùng.

Chỗ lệch: nhánh đó không kiểm tra bất kỳ từ khoá nào — nó CHỈ kiểm tra
`buocChon`. Chỉ thị hệ thống nói "không được trả lời không rõ" nghĩa LÀ
đúng như vậy: model PHẢI đưa ra một lựa chọn CỤ THỂ, dù không chắc
chắn, thay vì bỏ cuộc.
::
:::
:::opt
Hàm sẽ ném lỗi, vì không còn nhánh nào để rơi vào khi cả ba từ khoá
cứng LẪN hai từ khoá rộng đều không khớp
::why
Gần đúng Ở việc bạn nhận ra prompt NÀY "hết đường" theo MỌI điều kiện
`if` phía trên — quan sát đó đúng.

Chỗ lệch: hết đường Ở CÁC ĐIỀU KIỆN TRƯỚC không có nghĩa là hàm hết
đường HOÀN TOÀN — vẫn còn `if (buocChon) return "tom_tat";` VÀ
`return "khong_ro";` phía sau, không dòng nào trong hàm gọi `throw`.
::
:::
::::

::::code{#viet_phan_loai_co_rang_buoc}
Hoàn thiện `phanLoaiNhiemVuCoRangBuoc` — thêm hai nhánh mở rộng: nếu
CÓ ràng buộc VÀ prompt chứa `"y kien"`, trả về `"tom_tat"`; nếu CÓ ràng
buộc VÀ prompt chứa `"ngon ngu khac"`, trả về `"dich"`. Giữ ĐÚNG thứ
tự: hai nhánh mới này PHẢI đứng SAU ba từ khoá cứng, TRƯỚC nhánh mặc
định cuối cùng.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}

function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}

function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  ___
  ___
  if (buocChon) return "tom_tat";
  return "khong_ro";
}

const msgsX: ChatMessage[] = [
  { role: "system", content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro." },
  { role: "user", content: "Giup toi chuyen doan hoi thoai nay sang ngon ngu khac" },
];
console.log(phanLoaiNhiemVuCoRangBuoc(msgsX));
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function layNoiDungUserCuoi(messages: ChatMessage[]): string {
  let noiDung = "";
  for (const tin of messages) {
    if (tin.role === "user") noiDung = tin.content;
  }
  return noiDung;
}

function coRangBuocChonMotTrongBa(messages: ChatMessage[]): boolean {
  return messages.some(
    (tin) => tin.role === "system" && tin.content.toLowerCase().includes("khong duoc tra loi khong_ro"),
  );
}

function phanLoaiNhiemVuCoRangBuoc(messages: ChatMessage[]): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const prompt = layNoiDungUserCuoi(messages).toLowerCase();
  const buocChon = coRangBuocChonMotTrongBa(messages);
  if (prompt.includes("tom tat")) return "tom_tat";
  if (prompt.includes("dich")) return "dich";
  if (prompt.includes("dem") || prompt.includes("tong")) return "dem_so";
  if (buocChon && prompt.includes("y kien")) return "tom_tat";
  if (buocChon && prompt.includes("ngon ngu khac")) return "dich";
  if (buocChon) return "tom_tat";
  return "khong_ro";
}

const msgsX: ChatMessage[] = [
  { role: "system", content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro." },
  { role: "user", content: "Giup toi chuyen doan hoi thoai nay sang ngon ngu khac" },
];
console.log(phanLoaiNhiemVuCoRangBuoc(msgsX));
```

```typescript title=test
const buocChonFalse: ChatMessage[] = [{ role: "user", content: "Cho toi y kien ve bai viet nay" }];
if (phanLoaiNhiemVuCoRangBuoc(buocChonFalse) !== "khong_ro") throw new Error("khong co rang buoc, prompt mo ho van phai la khong_ro (hanh vi bai truoc khong doi)");

function taoMsgRangBuoc(prompt: string): ChatMessage[] {
  return [
    { role: "system", content: "Chi duoc chon mot trong ba loai: tom_tat, dich, hoac dem_so. Khong duoc tra loi khong_ro." },
    { role: "user", content: prompt },
  ];
}

if (phanLoaiNhiemVuCoRangBuoc(taoMsgRangBuoc("Hay tom tat doan van nay")) !== "tom_tat") throw new Error("co rang buoc nhung tu khoa ro rang van phai duoc uu tien nhu cu");
if (phanLoaiNhiemVuCoRangBuoc(taoMsgRangBuoc("Dem so don hang thang nay")) !== "dem_so") throw new Error("co rang buoc nhung tu khoa 'dem' van phai duoc nhan dung nhu cu");
if (phanLoaiNhiemVuCoRangBuoc(taoMsgRangBuoc("Cho toi y kien ve bai viet nay")) !== "tom_tat") throw new Error("co rang buoc, prompt mo ho co tu 'y kien' phai duoc suy ra la tom_tat");
if (phanLoaiNhiemVuCoRangBuoc(taoMsgRangBuoc("Giup toi chuyen tai lieu nay sang ngon ngu khac")) !== "dich") throw new Error("co rang buoc, prompt mo ho co tu 'ngon ngu khac' phai duoc suy ra la dich");
if (phanLoaiNhiemVuCoRangBuoc(taoMsgRangBuoc("Ban co the giup toi khong")) !== "tom_tat") throw new Error("co rang buoc nhung khong khop tu khoa rong nao, phai roi ve mac dinh tom_tat, KHONG con duoc tra ve khong_ro");
if (phanLoaiNhiemVuCoRangBuoc(taoMsgRangBuoc("Cho toi y kien ve tai lieu nay, co the chuyen sang ngon ngu khac khong")) !== "tom_tat") throw new Error("prompt co CA HAI tu khoa rong phai uu tien tom_tat (kiem tra 'y kien' truoc trong thu tu if)");
```

:::hints
- kind: attention
  body: "Hai cho trong, ca hai deu la mot dong if theo dung mau: neu (buocChon VA prompt chua tu khoa rong) thi tra ve nhan tuong ung. Cho dau kiem 'y kien' tra ve tom_tat; cho hai kiem 'ngon ngu khac' tra ve dich. Thu tu hai dong nay QUAN TRONG: 'y kien' phai kiem TRUOC 'ngon ngu khac'."
- kind: strategy
  body: "Cho dau: if (buocChon && prompt.includes(\"y kien\")) return \"tom_tat\"; Cho hai: if (buocChon && prompt.includes(\"ngon ngu khac\")) return \"dich\";"
- kind: one-line
  body: "Sao chep dung hai dong o phan Strategy, DUNG THU TU: 'y kien' truoc, 'ngon ngu khac' sau."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "dich"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`pass@1` từ `4/6` lên `6/6` — MỘT system message, đo được TRÊN CÙNG bộ
test, không phải nói suông. Nhưng phân loại ĐÚNG loại nhiệm vụ chỉ LÀ
một nửa câu chuyện: một hệ thống production cần ĐỌC ĐƯỢC câu trả lời
BẰNG CODE — VÀ đó LÀ một trục hoàn toàn khác: định dạng.
::::

::::reflect{#nghi-lai}
Ràng buộc rõ ràng không hề "làm cho model thông minh hơn" — nó THÊM
MỘT NHÁNH XỬ LÝ vào ĐÚNG chỗ code đã chừa sẵn: khi ba từ khoá cứng
không khớp, thử từ khoá RỘNG hơn, RỒI mới rơi về một lựa chọn MẶC ĐỊNH
thay vì bỏ cuộc. `phanLoaiNhiemVuCoRangBuoc` không đoán ĐÚNG mọi
trường hợp mơ hồ — nó chỉ đảm bảo KHÔNG BAO GIỜ trả lời "không biết"
khi bị cấm làm vậy. Đó LÀ đúng những gì một chỉ thị hệ thống như
"không được trả lời không rõ" THẬT SỰ làm được: ép ra một lựa chọn,
không đảm bảo lựa chọn đó luôn đúng.
::::

::::checkpoint{mastery=0.78}
::::
