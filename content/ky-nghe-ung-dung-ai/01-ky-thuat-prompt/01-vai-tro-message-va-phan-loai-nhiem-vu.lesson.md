---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.vai-tro-message-va-phan-loai-nhiem-vu
title: "Vai trò message & harness LLM mô phỏng v1: phân loại nhiệm vụ"
summary: "Mở đầu Realm 9 (TypeScript, không Python/Pyodide nữa). interface ChatMessage { role: \"system\"|\"user\"|\"assistant\"; content: string } định nghĩa hình dạng MỘT lượt hội thoại. Harness LLM mô phỏng v1 là hàm RULE-BASED phanLoaiNhiemVu(prompt): tìm từ khoá cố định — \"tom tat\"→tom_tat, \"dich\"→dich, \"dem\"/\"tong\"→dem_so, KHÔNG khớp từ khoá nào→khong_ro (lỗi \"model hiểu SAI nhiệm vụ\" khi prompt mơ hồ). tinhPassAt1(ketQua: boolean[]) = tỉ lệ true trên tổng, viết MỘT lần và tái dùng suốt quest. Trên bộ 6 prompt cố định (4 rõ ràng, 2 mơ hồ), pass@1 = 4/6 — không phải 100%, đo được bằng số, không phải nói suông."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.vai-tro-message-va-phan-loai-nhiem-vu]
requires: []
concepts: [kna.vai-tro-message-va-phan-loai-nhiem-vu]
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
Tám Realm trước dừng ở Python. Realm này — Realm cuối cùng — chuyển hẳn
sang TypeScript, và chủ đề cũng đổi hẳn: không còn huấn luyện mô hình,
mà học cách MỘT chương trình gọi một mô hình ngôn ngữ cho đúng cách.
Không có mạng, không có API thật — mọi "model" ở đây là một hàm
TypeScript tất định, RULE-BASED, mà bạn đọc được toàn bộ logic của nó.
Bắt đầu từ câu hỏi nền tảng nhất: một lượt gọi model trông như thế nào?
::::

::::explain{#chat-message-va-harness}
Một lời gọi LLM — thật hay mô phỏng — không nhận một đoạn văn bản liền
mạch. Nó nhận một DANH SÁCH các lượt, mỗi lượt có một VAI TRÒ
(`role`) và một nội dung (`content`). TypeScript cho phép ghi CHÍNH XÁC
ba vai trò hợp lệ bằng một union kiểu chuỗi, không phải một `string`
chung chung:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

const vd: ChatMessage = { role: "user", content: "Xin chao" };
// const sai: ChatMessage = { role: "khach", content: "x" };
// ^ TypeScript CHAN dong nay tai bien dich -- "khach" khong nam trong
// union "system" | "user" | "assistant"
console.log(vd.role, vd.content);
```

```text title=readonly
user Xin chao
```

Đây LÀ điểm khác biệt với Python (Realm 8): ở đó, `role` là một `str`
bất kỳ, sai chính tả một chữ ("syste") vẫn chạy được, chỉ lặng lẽ không
khớp branch nào. Ở TypeScript, gõ sai `role` là LỖI BIÊN DỊCH — bị chặn
TRƯỚC khi mã kịp chạy, không phải một lỗi runtime âm thầm.

Harness LLM mô phỏng của quest này LÀ một hàm nhận `prompt` (câu hỏi
của người dùng) VÀ trả về một PHÂN LOẠI nhiệm vụ — bước đầu tiên của
bất kỳ hệ thống production nào dùng LLM: biết người dùng đang muốn LÀM
GÌ trước khi làm bất cứ điều gì khác. `phanLoaiNhiemVu` RULE-BASED —
tìm từ khoá CỐ ĐỊNH trong prompt, theo đúng thứ tự ưu tiên:
::::

::::example{#phan-loai-va-pass-at1}
`phanLoaiNhiemVu` kiểm tra ba từ khoá theo thứ tự — "tom tat" trước,
"dich" kế tiếp, rồi "dem"/"tong" — và CHỈ khi KHÔNG khớp từ khoá nào
mới trả về `"khong_ro"`. Đây CHÍNH LÀ mô phỏng lỗi "model hiểu SAI
nhiệm vụ" khi prompt viết mơ hồ, không chứa tín hiệu rõ ràng nào.
`tinhPassAt1` đo tỉ lệ phân loại ĐÚNG trên một lượt thử DUY NHẤT mỗi
câu — định nghĩa chuẩn của `pass@1`, sẽ được TÁI DÙNG nguyên vẹn xuyên
suốt cả quest:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function phanLoaiNhiemVu(prompt: string): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const p = prompt.toLowerCase();
  if (p.includes("tom tat")) return "tom_tat";
  if (p.includes("dich")) return "dich";
  if (p.includes("dem") || p.includes("tong")) return "dem_so";
  return "khong_ro";
}

function tinhPassAt1(ketQua: boolean[]): number {
  if (ketQua.length === 0) return 0;
  const soDung = ketQua.filter((x) => x).length;
  return soDung / ketQua.length;
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

const ketQua = BO_KIEM_THU.map((c) => phanLoaiNhiemVu(c.prompt) === c.nhanDung);
console.log(BO_KIEM_THU.map((c) => phanLoaiNhiemVu(c.prompt)));
console.log(ketQua);
console.log("pass@1:", tinhPassAt1(ketQua));
```

```text title=readonly
["tom_tat","dich","dem_so","dem_so","khong_ro","khong_ro"]
[true,true,true,true,false,false]
pass@1: 0.6666666666666666
```

Bốn câu ĐẦU chứa từ khoá tường minh — phân loại ĐÚNG cả bốn. Hai câu
CUỐI hoàn toàn không chứa "tom tat", "dich", "dem" hay "tong" nào —
`"Cho toi y kien ve bai viet nay"` thật ra LÀ một yêu cầu tóm tắt/nhận
xét, và `"Giup toi chuyen tai lieu nay sang ngon ngu khac"` thật ra LÀ
một yêu cầu dịch — nhưng vì không viết đúng từ khoá, harness trả về
`"khong_ro"` cho CẢ HAI, sai NHÃN mong đợi. `pass@1` chỉ đạt
`4/6`, KHÔNG phải `100%` — con số đo được, không phải một nhận xét
chung chung.
::::

::::predict{#doan-uu-tien-tu-khoa commitOnce}
Gọi `phanLoaiNhiemVu("Hay tom tat va dich doan van nay")` — prompt NÀY
chứa CẢ HAI từ khoá "tom tat" LẪN "dich" cùng lúc. Kết quả LÀ gì?

:::opt{correct}
`"tom_tat"` — ba điều kiện `if` được kiểm theo ĐÚNG thứ tự viết trong
code: "tom tat" được kiểm TRƯỚC "dich"; khi điều kiện đầu tiên khớp,
hàm `return` NGAY, hai điều kiện sau không hề được xét tới
:::
:::opt
`"dich"` — "dich" LÀ từ khoá ngắn hơn VÀ cụ thể hơn "tom tat" (hai từ),
nên nó nên được ưu tiên khớp trước
::why
Nhầm "ngắn hơn nên ưu tiên hơn" VỚI thứ tự THẬT SỰ quyết định kết quả —
nhưng `phanLoaiNhiemVu` không hề so sánh ĐỘ DÀI của từ khoá.

Chỗ lệch: ưu tiên trong hàm này LÀ thứ tự VIẾT của các dòng `if`, đọc
từ trên xuống. Dòng `if (p.includes("tom tat")) return "tom_tat";`
đứng TRƯỚC dòng kiểm "dich" — khi cả hai đều khớp, `return` ở dòng ĐẦU
tiên chạy VÀ hàm thoát ngay, không bao giờ chạm tới dòng thứ hai.
::
:::
:::opt
Hàm sẽ NÉM lỗi vì phát hiện prompt "mâu thuẫn" — chứa hai loại nhiệm vụ
cùng lúc
::why
Gần đúng Ở việc bạn nhận ra prompt NÀY khác thường (chứa hai tín hiệu
cùng lúc) — quan sát đó đúng, nhưng suy luận tiếp theo thì không.

Chỗ lệch: `phanLoaiNhiemVu` không hề kiểm tra "có bao nhiêu từ khoá
khớp" hay ném lỗi khi phát hiện nhiều hơn một. Nó CHỈ đơn giản trả về
NGAY tại điều kiện `if` đầu tiên khớp — không có nhánh nào trong code
gọi `throw`.
::
:::
::::

::::code{#viet_phan_loai_va_pass_at1}
Hoàn thiện `phanLoaiNhiemVu` — thêm nhánh thứ ba: nếu prompt chứa
`"dem"` HOẶC `"tong"`, trả về `"dem_so"`. Hoàn thiện `tinhPassAt1` —
mảng RỖNG trả về `0` (không được `NaN`), ngược lại trả về tỉ lệ phần tử
`true` trên tổng số phần tử.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function phanLoaiNhiemVu(prompt: string): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const p = prompt.toLowerCase();
  if (p.includes("tom tat")) return "tom_tat";
  if (p.includes("dich")) return "dich";
  ___
  return "khong_ro";
}

function tinhPassAt1(ketQua: boolean[]): number {
  ___
}

const ketQuaX = [phanLoaiNhiemVu("Hay dem tong so don hang"), phanLoaiNhiemVu("Ban khoe khong")];
console.log(ketQuaX[0], ketQuaX[1], tinhPassAt1([true, false, true, true]));
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

function phanLoaiNhiemVu(prompt: string): "tom_tat" | "dich" | "dem_so" | "khong_ro" {
  const p = prompt.toLowerCase();
  if (p.includes("tom tat")) return "tom_tat";
  if (p.includes("dich")) return "dich";
  if (p.includes("dem") || p.includes("tong")) return "dem_so";
  return "khong_ro";
}

function tinhPassAt1(ketQua: boolean[]): number {
  if (ketQua.length === 0) return 0;
  const soDung = ketQua.filter((x) => x).length;
  return soDung / ketQua.length;
}

const ketQuaX = [phanLoaiNhiemVu("Hay dem tong so don hang"), phanLoaiNhiemVu("Ban khoe khong")];
console.log(ketQuaX[0], ketQuaX[1], tinhPassAt1([true, false, true, true]));
```

```typescript title=test
if (phanLoaiNhiemVu("Hay tom tat bai bao nay") !== "tom_tat") throw new Error("prompt co tu khoa 'tom tat' phai tra ve tom_tat");
if (phanLoaiNhiemVu("Dich giup toi cau nay") !== "dich") throw new Error("prompt co tu khoa 'dich' phai tra ve dich");
if (phanLoaiNhiemVu("Dem so luong san pham") !== "dem_so") throw new Error("prompt co tu khoa 'dem' phai tra ve dem_so");
if (phanLoaiNhiemVu("Tinh tong doanh so") !== "dem_so") throw new Error("prompt co tu khoa 'tong' cung phai tra ve dem_so");
if (phanLoaiNhiemVu("Ban co khoe khong") !== "khong_ro") throw new Error("prompt KHONG co tu khoa nao phai tra ve khong_ro");
if (phanLoaiNhiemVu("Hay tom tat va dich doan van nay") !== "tom_tat") throw new Error("prompt co CA HAI tu khoa phai uu tien tom_tat theo dung thu tu kiem tra");

if (tinhPassAt1([]) !== 0) throw new Error("mang rong phai tra ve 0, khong duoc NaN");
if (tinhPassAt1([true, true, true, true]) !== 1) throw new Error("toan bo dung phai la 1");
if (tinhPassAt1([false, false]) !== 0) throw new Error("toan bo sai phai la 0");
if (tinhPassAt1([true, false, true, false]) !== 0.5) throw new Error("nua dung nua sai phai la 0.5");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau trong phanLoaiNhiemVu: mot dong if kiem tra p.includes('dem') HOAC p.includes('tong'), tra ve dem_so. Cho hai trong tinhPassAt1: neu ketQua.length la 0 tra ve 0 NGAY, nguoc lai tinh soDung (so phan tu true) chia cho ketQua.length."
- kind: strategy
  body: "Cho dau: if (p.includes(\"dem\") || p.includes(\"tong\")) return \"dem_so\"; Cho hai: if (ketQua.length === 0) return 0; const soDung = ketQua.filter((x) => x).length; return soDung / ketQua.length;"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "dem_so khong_ro 0.75"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm rule-based, một con số `pass@1` đo được: `4/6`. Không cần một
LLM thật để thấy lỗi "hiểu sai nhiệm vụ" — chỉ cần một prompt viết mơ
hồ. Bài sau sửa CHÍNH lỗi này bằng kỹ thuật đầu tiên: một ràng buộc
tường minh, đo lại pass@1 TRÊN CÙNG bộ test.
::::

::::reflect{#nghi-lai}
`phanLoaiNhiemVu` không hề "hiểu" ngôn ngữ tự nhiên — nó CHỈ tra cứu từ
khoá cố định, giống hệt cách một LLM thật, khi thiếu tín hiệu rõ ràng
trong prompt, có thể đoán sai Ý ĐỊNH của người dùng. `tinhPassAt1` biến
"model có vẻ hiểu prompt" thành một con số cụ thể: `4/6`, không phải
một cảm giác. Hai công cụ NÀY — hàm phân loại RULE-BASED và phép đo
pass@1 — sẽ được TÁI DÙNG, không viết lại, xuyên suốt toàn bộ quest.
::::

::::checkpoint{mastery=0.75}
::::
