---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.a2a-vong-doi-task-sau-trang-thai
title: "A2A — vòng đời task, sáu trạng thái"
summary: "chuyenTrangThaiTask(hienTai, moi) là state machine trên A2aTaskState (6 trạng thái: submitted, working, input-required, completed, canceled, failed) — bảng CHUYEN_HOP_LE giới hạn CHÍNH XÁC những bước chuyển được phép cho MỖI trạng thái xuất phát (working cho phép 4 đích: input-required/completed/failed/canceled; ba trạng thái completed/canceled/failed là NGÕ CỤT, không đích nào). Trên chuỗi hợp lệ submitted→working→input-required→working→completed (4 bước liên tiếp): cả 4 bước đều qua, trạng thái cuối là 'completed'. Gọi chuyenTrangThaiTask('completed','working') NÉM lỗi (Error nêu rõ CẢ hai trạng thái completed và working trong thông điệp) thay vì âm thầm trả về — completed là ngõ cụt, không có đường quay lại working."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.a2a-vong-doi-task-sau-trang-thai]
requires: [kna.a2a-agent-card-phat-hien-ky-nang]
concepts: [kna.a2a-vong-doi-task-sau-trang-thai]
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
Task Ở T9.5 (`ky-thuat-do-thi`) LÀ một node trong đồ thị, trạng thái do
CHÍNH node quyết định lúc runtime. Task Ở A2A CŨNG LÀ một trạng thái
tường minh — nhưng lần này đặc tả CHỈ RA đúng sáu cái tên VÀ đúng những
bước chuyển được phép giữa chúng. Bài này KHÔNG dạy khái niệm mới —
nó dạy đúng SÁU cái tên VÀ tấm bảng giới hạn CÁCH chúng nối với nhau.
::::

::::explain{#sau_trang_thai_va_bang_chuyen}
`A2aTaskState` CÓ đúng sáu giá trị: `"submitted"` (vừa nộp), `"working"`
(đang xử lý), `"input-required"` (cần thêm thông tin từ người gửi),
VÀ ba trạng thái NGÕ CỤT — `"completed"`, `"canceled"`, `"failed"`.
`CHUYEN_HOP_LE` LÀ bảng tra: MỖI trạng thái xuất phát ánh xạ tới một
DANH SÁCH đích được phép — ba trạng thái ngõ cụt ánh xạ tới mảng RỖNG,
nghĩa LÀ không có đích nào hợp lệ từ đó:

```typescript title=readonly
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function laChuyenHopLe(hienTai: A2aTaskState, moi: A2aTaskState): boolean {
  return CHUYEN_HOP_LE[hienTai].includes(moi);
}

console.log(laChuyenHopLe("submitted", "working"));
console.log(laChuyenHopLe("completed", "working"));
console.log(laChuyenHopLe("input-required", "working"));
```

```text title=readonly
true
false
true
```

`working` có ĐÚNG BỐN đích hợp lệ — nhiều nhất trong cả bảng, vì đó LÀ
trạng thái mà MỌI kết cục khác (cần thêm input, xong, lỗi, huỷ) đều CÓ
THỂ xảy ra. `input-required` có thể QUAY LẠI `working` (sau khi nhận đủ
thông tin) — nhưng KHÔNG được nhảy thẳng sang `completed`.
::::

::::example{#chuyen_trang_thai_va_nem_loi}
`chuyenTrangThaiTask` dùng `laChuyenHopLe` LÀM hàng rào: hợp lệ thì trả
về trạng thái mới; KHÔNG hợp lệ thì NÉM lỗi — không âm thầm bỏ qua, VÀ
không âm thầm nhận đại một trạng thái sai:

```typescript title=readonly
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function laChuyenHopLe(hienTai: A2aTaskState, moi: A2aTaskState): boolean {
  return CHUYEN_HOP_LE[hienTai].includes(moi);
}

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!laChuyenHopLe(hienTai, moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

let s: A2aTaskState = "submitted";
s = chuyenTrangThaiTask(s, "working");
s = chuyenTrangThaiTask(s, "input-required");
s = chuyenTrangThaiTask(s, "working");
s = chuyenTrangThaiTask(s, "completed");
console.log(s);

try {
  chuyenTrangThaiTask("completed", "working");
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
completed
chuyen trang thai khong hop le: completed -> working
```

Bốn bước liên tiếp (`submitted→working→input-required→working→
completed`) đều hợp lệ, trạng thái cuối LÀ `"completed"`. Gọi
`chuyenTrangThaiTask("completed", "working")` sau đó NÉM lỗi ngay —
`completed` LÀ ngõ cụt, không có đường quay lại `working`.
::::

::::predict{#doan-chuoi-hai-buoc commitOnce}
Chuỗi `working -> input-required -> completed` (HAI bước liên tiếp,
gọi `chuyenTrangThaiTask` hai lần) — bước NÀO Ở chuỗi này NÉM lỗi, nếu
có?

:::opt{correct}
Bước THỨ HAI (`input-required -> completed`) NÉM lỗi — bước ĐẦU
(`working -> input-required`) hợp lệ (nằm trong danh sách của
`working`), nhưng danh sách của `"input-required"` chỉ có `["working",
"canceled"]`, KHÔNG có `"completed"`
:::
:::opt
Bước ĐẦU (`working -> input-required`) NÉM lỗi — vì `working` đã bắt
đầu xử lý thì không được phép hỏi lại thêm thông tin nữa
::why
Nhầm hướng: `CHUYEN_HOP_LE.working` LÀ `["input-required", "completed",
"failed", "canceled"]` — `"input-required"` LÀ đích ĐẦU TIÊN trong danh
sách đó, nên bước NÀY hợp lệ.

Chỗ lệch: chính bước THỨ HAI mới bị chặn, vì `CHUYEN_HOP_LE["input-
required"]` không hề chứa `"completed"`.
::
:::
:::opt
Không bước nào ném lỗi — vì `completed` LÀ đích CUỐI hợp lệ nói chung
của một task, nên đi tới từ bất kỳ trạng thái nào cũng được
::why
Nhầm rằng "hợp lệ nói chung" nghĩa LÀ "hợp lệ từ MỌI trạng thái xuất
phát" — nhưng `CHUYEN_HOP_LE` quy định RIÊNG cho TỪNG trạng thái xuất
phát, không có một danh sách đích "dùng chung" nào cả.

Chỗ lệch: `CHUYEN_HOP_LE["input-required"]` chỉ liệt kê `["working",
"canceled"]` — `"completed"` không nằm trong đó, nên `laChuyenHopLe(
"input-required", "completed")` trả `false`.
::
:::
::::

::::code{#viet_chuyen_trang_thai_task}
Hoàn thiện `laChuyenHopLe` — tra `CHUYEN_HOP_LE[hienTai]` VÀ kiểm `moi`
CÓ nằm trong danh sách đó không. Hoàn thiện `chuyenTrangThaiTask` — NẾU
`laChuyenHopLe` trả `false`, NÉM `Error` với thông điệp NÊU RÕ cả
`hienTai` LẪN `moi`; NGƯỢC LẠI trả về `moi`.

```typescript title=starter
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function laChuyenHopLe(hienTai: A2aTaskState, moi: A2aTaskState): boolean {
  ___
}

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  ___
}

const sauKhiLamViec = chuyenTrangThaiTask("submitted", "working");
console.log(sauKhiLamViec);
```

```typescript title=solution
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function laChuyenHopLe(hienTai: A2aTaskState, moi: A2aTaskState): boolean {
  return CHUYEN_HOP_LE[hienTai].includes(moi);
}

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!laChuyenHopLe(hienTai, moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

const sauKhiLamViec = chuyenTrangThaiTask("submitted", "working");
console.log(sauKhiLamViec);
```

```typescript title=test
if (sauKhiLamViec !== "working") throw new Error("submitted -> working phai hop le va tra ve 'working'");
if (!laChuyenHopLe("working", "completed")) throw new Error("working -> completed phai hop le");
if (!laChuyenHopLe("input-required", "working")) throw new Error("input-required -> working phai hop le");
if (laChuyenHopLe("completed", "working")) throw new Error("completed -> working KHONG duoc hop le -- completed la trang thai KET THUC");
if (chuyenTrangThaiTask("working", "completed") !== "completed") throw new Error("working -> completed phai tra ve 'completed'");

let daNem = false;
try {
  chuyenTrangThaiTask("completed", "working");
} catch (e) {
  daNem = true;
  if (!(e as Error).message.includes("completed") || !(e as Error).message.includes("working")) {
    throw new Error("thong bao loi phai neu ro CA HAI trang thai (completed va working)");
  }
}
if (!daNem) throw new Error("chuyen completed -> working phai NEM loi, khong duoc tra ve am tham");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (laChuyenHopLe): mot dong tra ve CHUYEN_HOP_LE[hienTai].includes(moi). Cho hai (chuyenTrangThaiTask): if khong hop le thi throw new Error kem ca hai ten trang thai; nguoc lai return moi."
- kind: strategy
  body: "Cho dau: return CHUYEN_HOP_LE[hienTai].includes(moi); Cho hai: if (!laChuyenHopLe(hienTai, moi)) { throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`); } return moi;"
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
  expect: "working"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu trạng thái, một bảng, một hàm ném lỗi đúng lúc — đủ để một task
không bao giờ "lẻn" vào một trạng thái không nghĩa lý gì. BOSS khép
`q9.6a` ráp NGUYÊN VĂN năm bài này thành MỘT kịch bản: agent A kiểm
Agent Card của agent B, gửi task qua vòng đời chuẩn, agent B xử lý bằng
CHÍNH MCP đã học Ở ba bài đầu.
::::

::::reflect{#nghi-lai}
Bảng `CHUYEN_HOP_LE` KHÔNG PHẢI một danh sách tuỳ ý — nó mã hoá đúng một
sự thật về THẾ GIỚI: một task ĐÃ xong (`completed`) không thể "xong lại
lần nữa Ở trạng thái khác", và một task ĐÃ huỷ (`canceled`) không thể
hồi sinh. Ném lỗi Ở `chuyenTrangThaiTask` khi gặp một bước chuyển vi
phạm sự thật đó không phải LÀ một ràng buộc phiền phức — nó LÀ cách DUY
NHẤT một hệ thống theo dõi nhiều task đồng thời tránh được câu hỏi
"trạng thái này thật ra nghĩa LÀ gì" khi có hai phần khác nhau của
chương trình cùng cố sửa MỘT task.
::::

::::checkpoint{mastery=0.85}
::::
