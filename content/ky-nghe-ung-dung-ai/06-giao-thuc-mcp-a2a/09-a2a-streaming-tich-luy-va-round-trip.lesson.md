---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.a2a-streaming-tich-luy-va-round-trip
title: "A2A streaming — tích luỹ message qua nhiều sự kiện, vòng round-trip"
summary: "xuLyLuongSuKien(taskBanDau, cacSuKien) gấp (reduce) một MẢNG sự kiện streaming lên MỘT task duy nhất — khác bài 5 (một lời gọi chuyenTrangThaiTask ứng với một bước rời rạc), ở đây MỘT task tích luỹ history qua NHIỀU sự kiện, và một số sự kiện GIỮ NGUYÊN trạng thái (không gọi chuyenTrangThaiTask) trong khi số khác đổi trạng thái THẬT SỰ. Trên chuỗi 5 sự kiện cụ thể (chunk_agent, chunk_agent, can_them_thong_tin, phan_hoi_user, hoan_tat) xuất phát từ task submitted rỗng lịch sử: hai chunk_agent liên tiếp CHỈ làm history dài thêm (độ dài 1 rồi 2) mà KHÔNG gọi chuyenTrangThaiTask ở bước thứ hai (vì trạng thái đích 'working' trùng trạng thái hiện tại); can_them_thong_tin đưa task sang 'input-required' (một VÒNG ROUND-TRIP thật: cần bước tiếp theo phan_hoi_user mới quay lại 'working'); hoan_tat đưa task về 'completed'. Kết thúc: task.state='completed', task.history.length=5 (đúng số sự kiện, không thiếu không thừa). Gọi trực tiếp apDungSuKien trên MỘT task ở 'input-required' với sự kiện 'hoan_tat' NÉM lỗi (input-required không có đường thẳng tới completed, đúng bảng CHUYEN_HOP_LE bài 5) — chứng minh input-required LUÔN cần một round-trip qua 'working', không có đường tắt."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.a2a-streaming-tich-luy-va-round-trip]
requires: [kna.mcp-resources-list-va-read]
concepts: [kna.a2a-streaming-tich-luy-va-round-trip]
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
Bài `5` (q9.6a) dạy MỘT bước chuyển — một lời gọi `chuyenTrangThaiTask`,
một trạng thái mới. Nhưng một agent A2A hỗ trợ streaming gửi VỀ NHIỀU
mẩu tin nhắn LIÊN TỤC cho CÙNG một task — VÀ đôi khi giữa chừng, nó cần
DỪNG lại hỏi thêm thông tin trước khi tiếp tục. Bài này ráp NHIỀU sự
kiện thành MỘT task, VÀ dựng đúng một vòng round-trip `input-required`.
::::

::::explain{#tich_luy_va_giu_nguyen_trang_thai}
Một `SuKienLuong` LÀ một mẩu tin ĐẾN theo thời gian — bốn loại: mẩu
tiến trình từ agent (`chunk_agent`, task VẪN Ở `working`), yêu cầu thêm
thông tin (`can_them_thong_tin`, task chuyển `input-required`), phản hồi
từ người dùng (`phan_hoi_user`, task VỀ lại `working`), VÀ hoàn tất
(`hoan_tat`, task chuyển `completed`). Điều khác biệt Ở streaming: NHIỀU
sự kiện liên tiếp có THỂ cùng ứng với MỘT trạng thái — VÀ khi đó, KHÔNG
có "bước chuyển" nào cả, chỉ có `history` dài thêm:

```typescript title=readonly
type A2aMessage = { role: "user" | "agent"; text: string };
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTaskCoLichSu = { id: string; state: A2aTaskState; history: A2aMessage[] };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

const taskVaSau1Chunk: A2aTaskCoLichSu = {
  id: "task-1",
  state: chuyenTrangThaiTask("submitted", "working"),
  history: [{ role: "agent", text: "Dang xu ly yeu cau..." }],
};
console.log(taskVaSau1Chunk.state, taskVaSau1Chunk.history.length);
```

```text title=readonly
working 1
```

`submitted → working` LÀ một bước chuyển THẬT (hai trạng thái KHÁC
nhau) — gọi `chuyenTrangThaiTask` LÀ đúng. Nhưng một `chunk_agent` THỨ
HAI, khi task ĐÃ Ở `working`, sẽ đưa `trạng thái đích` VỀ LẠI `working`
— TRÙNG với trạng thái hiện tại. Gọi `chuyenTrangThaiTask("working",
"working")` sẽ NÉM lỗi (`CHUYEN_HOP_LE.working` không hề chứa
`"working"` — không có "bước chuyển" nào TỚI chính nó). `apDungSuKien`
phải PHÂN BIỆT hai trường hợp NÀY.
::::

::::example{#ap_dung_su_kien_va_luong}
`apDungSuKien` tính TRƯỚC trạng thái đích của sự kiện; NẾU trạng thái
đích TRÙNG trạng thái hiện tại, chỉ nối `history` (không gọi
`chuyenTrangThaiTask`); NGƯỢC LẠI, gọi `chuyenTrangThaiTask` — hàng rào
`chuyenTrangThaiTask` VẪN đứng gác đúng những bước chuyển THẬT SỰ khác
trạng thái:

```typescript title=readonly
type A2aMessage = { role: "user" | "agent"; text: string };
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTaskCoLichSu = { id: string; state: A2aTaskState; history: A2aMessage[] };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

type SuKienLuong =
  | { loai: "chunk_agent"; text: string }
  | { loai: "can_them_thong_tin"; text: string }
  | { loai: "phan_hoi_user"; text: string }
  | { loai: "hoan_tat"; text: string };

function apDungSuKien(task: A2aTaskCoLichSu, suKien: SuKienLuong): A2aTaskCoLichSu {
  const trangThaiMoi: A2aTaskState =
    suKien.loai === "can_them_thong_tin" ? "input-required" : suKien.loai === "hoan_tat" ? "completed" : "working";
  const nguoiGui: "user" | "agent" = suKien.loai === "phan_hoi_user" ? "user" : "agent";
  const lichSuMoi: A2aMessage[] = [...task.history, { role: nguoiGui, text: suKien.text }];
  if (trangThaiMoi === task.state) {
    return { id: task.id, state: task.state, history: lichSuMoi };
  }
  return { id: task.id, state: chuyenTrangThaiTask(task.state, trangThaiMoi), history: lichSuMoi };
}

function xuLyLuongSuKien(taskBanDau: A2aTaskCoLichSu, cacSuKien: SuKienLuong[]): A2aTaskCoLichSu {
  return cacSuKien.reduce(apDungSuKien, taskBanDau);
}

const taskBanDau: A2aTaskCoLichSu = { id: "task-1", state: "submitted", history: [] };
const cacSuKien: SuKienLuong[] = [
  { loai: "chunk_agent", text: "Dang xu ly yeu cau..." },
  { loai: "chunk_agent", text: "Da doc xong du lieu dau vao." },
  { loai: "can_them_thong_tin", text: "Ban muon xuat ra dinh dang JSON hay CSV?" },
  { loai: "phan_hoi_user", text: "JSON" },
  { loai: "hoan_tat", text: "Hoan tat, ket qua dang JSON." },
];
const ketQuaLuong = xuLyLuongSuKien(taskBanDau, cacSuKien);
console.log(ketQuaLuong.state, ketQuaLuong.history.length);
console.log(ketQuaLuong.history.map((m) => m.role).join(","));
```

```text title=readonly
completed 5
agent,agent,agent,user,agent
```

Năm sự kiện, history dài đúng `5` — VÀ trạng thái đi ĐÚNG
`submitted → working → input-required → working → completed`. Đọc kỹ
CHUỖI vai trò (`agent,agent,agent,user,agent`): hai `chunk_agent` liên
tiếp KHÔNG hề gọi `chuyenTrangThaiTask` lần hai (cùng LÀ `working`), VÀ
`input-required` (sự kiện thứ ba, cũng LÀ `agent` — agent hỏi thêm)
BẮT BUỘC một `phan_hoi_user` (thứ tư) TRƯỚC KHI có thể `hoan_tat` — đó
LÀ vòng round-trip: không có đường TẮT nào từ `input-required` thẳng
tới `completed`.
::::

::::predict{#doan-hai-can-them-thong-tin-lien-tiep commitOnce}
Thêm MỘT sự kiện `{ loai: "can_them_thong_tin", text: "..." }` NGAY SAU
sự kiện thứ ba (cũng LÀ `can_them_thong_tin`) — hai sự kiện
`can_them_thong_tin` LIÊN TIẾP, TRƯỚC KHI có `phan_hoi_user` nào. Điều
gì xảy ra Ở sự kiện THỨ HAI đó?

:::opt{correct}
KHÔNG NÉM lỗi — trạng thái đích của sự kiện thứ hai (`"input-required"`)
TRÙNG trạng thái hiện tại của task (ĐÃ Ở `"input-required"` từ sự kiện
đầu), nên `apDungSuKien` chỉ nối THÊM một tin nhắn agent vào `history`,
KHÔNG gọi `chuyenTrangThaiTask`
:::
:::opt
NÉM lỗi — vì `CHUYEN_HOP_LE["input-required"]` LÀ `["working",
"canceled"]`, không hề chứa `"input-required"`, nên chuyển VỀ chính nó
phải bị coi LÀ không hợp lệ
::why
Nhầm rằng MỌI bước Ở `xuLyLuongSuKien` đều gọi `chuyenTrangThaiTask` —
nhưng `apDungSuKien` CHỈ gọi hàm đó khi `trangThaiMoi !== task.state`.
Khi trạng thái đích TRÙNG trạng thái hiện tại, nhánh `if` sớm trả về
NGAY, KHÔNG hề chạm `chuyenTrangThaiTask` — nên `CHUYEN_HOP_LE["input-
required"]` không chứa `"input-required"` KHÔNG hề gây ra lỗi Ở đây.

Chỗ lệch: điều kiện `trangThaiMoi === task.state` chặn TRƯỚC khi hàm
kiểm bảng chuyển được gọi tới.
::
:::
:::opt
Task tự động chuyển sang `"working"` — vì HAI lần hỏi thêm thông tin
liên tiếp nghĩa LÀ agent đã "tự trả lời được", nên coi như xong việc
hỏi
::why
Nhầm rằng `apDungSuKien` có LOGIC suy luận ý nghĩa của việc lặp lại một
loại sự kiện — nhưng hàm chỉ ánh xạ MÁY MÓC `suKien.loai` sang MỘT
trạng thái đích cố định (`"can_them_thong_tin"` LUÔN ánh xạ tới
`"input-required"`, không có ngoại lệ "lần thứ hai thì khác").

Chỗ lệch: `trangThaiMoi` của sự kiện thứ hai VẪN LÀ `"input-required"`
Y HỆT sự kiện đầu — không có nhánh nào trong `apDungSuKien` đọc "đây LÀ
lần thứ mấy" để đổi kết quả.
::
:::
::::

::::code{#viet_xu_ly_luong_su_kien}
Hoàn thiện `apDungSuKien` — tính `trangThaiMoi` (`"input-required"` cho
`can_them_thong_tin`, `"completed"` cho `hoan_tat`, `"working"` cho HAI
loại còn lại), `nguoiGui` (`"user"` cho `phan_hoi_user`, `"agent"` cho
BA loại còn lại), nối `suKien.text` VÀO `task.history`; NẾU
`trangThaiMoi === task.state` chỉ trả `history` mới (KHÔNG gọi
`chuyenTrangThaiTask`); NGƯỢC LẠI gọi `chuyenTrangThaiTask` để lấy
trạng thái mới. Hoàn thiện `xuLyLuongSuKien` — `cacSuKien.reduce(...)`
bắt đầu từ `taskBanDau`.

```typescript title=starter
type A2aMessage = { role: "user" | "agent"; text: string };
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTaskCoLichSu = { id: string; state: A2aTaskState; history: A2aMessage[] };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

type SuKienLuong =
  | { loai: "chunk_agent"; text: string }
  | { loai: "can_them_thong_tin"; text: string }
  | { loai: "phan_hoi_user"; text: string }
  | { loai: "hoan_tat"; text: string };

function apDungSuKien(task: A2aTaskCoLichSu, suKien: SuKienLuong): A2aTaskCoLichSu {
  ___
}

function xuLyLuongSuKien(taskBanDau: A2aTaskCoLichSu, cacSuKien: SuKienLuong[]): A2aTaskCoLichSu {
  ___
}

const taskBanDau: A2aTaskCoLichSu = { id: "task-1", state: "submitted", history: [] };
const cacSuKien: SuKienLuong[] = [
  { loai: "chunk_agent", text: "Dang xu ly yeu cau..." },
  { loai: "chunk_agent", text: "Da doc xong du lieu dau vao." },
  { loai: "can_them_thong_tin", text: "Ban muon xuat ra dinh dang JSON hay CSV?" },
  { loai: "phan_hoi_user", text: "JSON" },
  { loai: "hoan_tat", text: "Hoan tat, ket qua dang JSON." },
];
const ketQuaLuong = xuLyLuongSuKien(taskBanDau, cacSuKien);
console.log(ketQuaLuong.state, ketQuaLuong.history.length);
```

```typescript title=solution
type A2aMessage = { role: "user" | "agent"; text: string };
type A2aTaskState = "submitted" | "working" | "input-required" | "completed" | "canceled" | "failed";
type A2aTaskCoLichSu = { id: string; state: A2aTaskState; history: A2aMessage[] };

const CHUYEN_HOP_LE: Record<A2aTaskState, A2aTaskState[]> = {
  submitted: ["working", "canceled"],
  working: ["input-required", "completed", "failed", "canceled"],
  "input-required": ["working", "canceled"],
  completed: [],
  canceled: [],
  failed: [],
};

function chuyenTrangThaiTask(hienTai: A2aTaskState, moi: A2aTaskState): A2aTaskState {
  if (!CHUYEN_HOP_LE[hienTai].includes(moi)) {
    throw new Error(`chuyen trang thai khong hop le: ${hienTai} -> ${moi}`);
  }
  return moi;
}

type SuKienLuong =
  | { loai: "chunk_agent"; text: string }
  | { loai: "can_them_thong_tin"; text: string }
  | { loai: "phan_hoi_user"; text: string }
  | { loai: "hoan_tat"; text: string };

function apDungSuKien(task: A2aTaskCoLichSu, suKien: SuKienLuong): A2aTaskCoLichSu {
  const trangThaiMoi: A2aTaskState =
    suKien.loai === "can_them_thong_tin" ? "input-required" : suKien.loai === "hoan_tat" ? "completed" : "working";
  const nguoiGui: "user" | "agent" = suKien.loai === "phan_hoi_user" ? "user" : "agent";
  const lichSuMoi: A2aMessage[] = [...task.history, { role: nguoiGui, text: suKien.text }];
  if (trangThaiMoi === task.state) {
    return { id: task.id, state: task.state, history: lichSuMoi };
  }
  return { id: task.id, state: chuyenTrangThaiTask(task.state, trangThaiMoi), history: lichSuMoi };
}

function xuLyLuongSuKien(taskBanDau: A2aTaskCoLichSu, cacSuKien: SuKienLuong[]): A2aTaskCoLichSu {
  return cacSuKien.reduce(apDungSuKien, taskBanDau);
}

const taskBanDau: A2aTaskCoLichSu = { id: "task-1", state: "submitted", history: [] };
const cacSuKien: SuKienLuong[] = [
  { loai: "chunk_agent", text: "Dang xu ly yeu cau..." },
  { loai: "chunk_agent", text: "Da doc xong du lieu dau vao." },
  { loai: "can_them_thong_tin", text: "Ban muon xuat ra dinh dang JSON hay CSV?" },
  { loai: "phan_hoi_user", text: "JSON" },
  { loai: "hoan_tat", text: "Hoan tat, ket qua dang JSON." },
];
const ketQuaLuong = xuLyLuongSuKien(taskBanDau, cacSuKien);
console.log(ketQuaLuong.state, ketQuaLuong.history.length);
```

```typescript title=test
if (ketQuaLuong.state !== "completed") throw new Error("sau du 5 su kien, task phai o trang thai 'completed'");
if (ketQuaLuong.history.length !== 5) throw new Error("history phai co dung 5 phan tu, dung bang so su kien");
if (ketQuaLuong.history[2]?.role !== "agent") throw new Error("su kien thu ba (can_them_thong_tin) phai la tin nhan tu 'agent'");
if (ketQuaLuong.history[3]?.role !== "user") throw new Error("su kien thu tu (phan_hoi_user) phai la tin nhan tu 'user'");

const taskHaiChunk = xuLyLuongSuKien(
  { id: "task-2", state: "submitted", history: [] },
  [
    { loai: "chunk_agent", text: "a" },
    { loai: "chunk_agent", text: "b" },
  ],
);
if (taskHaiChunk.state !== "working") throw new Error("hai chunk_agent lien tiep phai giu task o 'working'");
if (taskHaiChunk.history.length !== 2) throw new Error("hai chunk_agent lien tiep phai tao ra 2 phan tu history");

let daNem = false;
try {
  apDungSuKien({ id: "task-3", state: "input-required", history: [] }, { loai: "hoan_tat", text: "xong" });
} catch (e) {
  daNem = true;
  if (!(e as Error).message.includes("input-required") || !(e as Error).message.includes("completed")) {
    throw new Error("thong bao loi phai neu ro CA HAI trang thai (input-required va completed)");
  }
}
if (!daNem) throw new Error("tu input-required nhay thang toi completed (bo qua working) phai NEM loi");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (apDungSuKien): tinh trangThaiMoi va nguoiGui bang ternary theo suKien.loai, noi history, roi if trangThaiMoi bang task.state thi KHONG goi chuyenTrangThaiTask, nguoc lai moi goi. Cho hai (xuLyLuongSuKien): mot dong cacSuKien.reduce(apDungSuKien, taskBanDau)."
- kind: strategy
  body: "Cho dau: const trangThaiMoi: A2aTaskState = suKien.loai === 'can_them_thong_tin' ? 'input-required' : suKien.loai === 'hoan_tat' ? 'completed' : 'working'; const nguoiGui: 'user' | 'agent' = suKien.loai === 'phan_hoi_user' ? 'user' : 'agent'; const lichSuMoi: A2aMessage[] = [...task.history, { role: nguoiGui, text: suKien.text }]; if (trangThaiMoi === task.state) return { id: task.id, state: task.state, history: lichSuMoi }; return { id: task.id, state: chuyenTrangThaiTask(task.state, trangThaiMoi), history: lichSuMoi }; Cho hai: return cacSuKien.reduce(apDungSuKien, taskBanDau);"
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
  expect: "completed 5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm sự kiện, một `reduce`, một vòng round-trip bắt buộc qua
`input-required`. Bài sau chuyển hẳn sang một góc KHÁC: KHÔNG PHẢI hình
dạng dữ liệu sai, MÀ LÀ nội dung được CỐ Ý viết để đánh lừa — tool
poisoning.
::::

::::reflect{#nghi-lai}
Bài `5` dạy rằng một bước chuyển LÀ hợp lệ hay không PHỤ THUỘC bảng
`CHUYEN_HOP_LE`. Bài này thêm đúng MỘT ý: không phải MỌI sự kiện đến
đều LÀ một "bước chuyển" — một số chỉ LÀ dữ liệu tích luỹ (`history`)
trong khi trạng thái đứng YÊN. Nhầm lẫn hai điều này (coi MỌI sự kiện
streaming LÀ một bước chuyển trạng thái) sẽ khiến `chuyenTrangThaiTask`
ném lỗi SAI ngay giữa một luồng hoàn toàn bình thường — VÀ nhầm theo
chiều ngược lại (không bao giờ gọi `chuyenTrangThaiTask`) sẽ để lọt
đúng NHỮNG bước chuyển cần bị chặn, như nhảy thẳng từ `input-required`
sang `completed`.
::::

::::checkpoint{mastery=0.87}
::::
