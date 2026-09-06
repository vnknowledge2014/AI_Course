---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.a2a-agent-card-phat-hien-ky-nang
title: "A2A — Agent Card, phát hiện khả năng của agent khác"
summary: "kiemAgentCoHoTroKyNang(card, tenKyNang) kiểm một A2aAgentCard có công bố kỹ năng (khớp skill.id) hay không TRƯỚC KHI gửi task. Trên agent-tom-tat (công bố đúng 1 kỹ năng 'tom_tat_van_ban'): kiểm 'tom_tat_van_ban' → true, kiểm 'phan_tich_hinh_anh' (không công bố) → false. guiTaskCoKiemTruoc dùng kết quả đó để CHẶN gửi task cho kỹ năng không được hỗ trợ, trả lỗi nêu rõ tên agent VÀ tên kỹ năng thiếu — đối lập với guiTaskMu (gọi mù, không kiểm gì) LUÔN báo 'đã gửi' bất kể agent có hiểu yêu cầu hay không, che giấu đúng lỗi mà việc kiểm AgentCard trước lẽ ra phải lộ ra ngay từ đầu."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.a2a-agent-card-phat-hien-ky-nang]
requires: [kna.tool-schema-qua-mcp]
concepts: [kna.a2a-agent-card-phat-hien-ky-nang]
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
Ba bài trước LÀ về biên agent↔tool: MCP. Từ bài này, một biên KHÁC —
agent↔agent: A2A. Khi agent A muốn nhờ agent B làm một việc, A không hề
biết B "biết làm gì" trừ khi B TỰ công bố ra. Danh thiếp đó gọi LÀ Agent
Card — VÀ kiểm nó TRƯỚC khi gửi việc LÀ bước đầu tiên, y hệt tinh thần
`"initialize"` Ở MCP.
::::

::::explain{#agent_card_va_ky_nang}
`A2aAgentCard` LÀ danh thiếp một agent công bố công khai: tên, mô tả,
URL, capabilities (streaming, push notification), VÀ QUAN TRỌNG NHẤT —
`skills`, một mảng kỹ năng NÓ tự nhận LÀ làm được, MỖI kỹ năng CÓ `id`
riêng. `kiemAgentCoHoTroKyNang` chỉ hỏi ĐÚNG một câu: card này CÓ một
kỹ năng khớp `id` cần dùng hay không:

```typescript title=readonly
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCapabilities = { streaming?: boolean; pushNotifications?: boolean };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: A2aAgentCapabilities;
  skills: A2aAgentSkill[];
};

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

const agentB: A2aAgentCard = {
  name: "agent-tom-tat",
  description: "Agent tom tat van ban tieng Viet",
  url: "https://vi-du.test/agent-tom-tat",
  capabilities: { streaming: false },
  skills: [
    { id: "tom_tat_van_ban", name: "Tom tat van ban", description: "Tom tat mot doan van ban dai" },
    { id: "dich_thuat", name: "Dich thuat", description: "Dich Anh - Viet" },
  ],
};

console.log(kiemAgentCoHoTroKyNang(agentB, "tom_tat_van_ban"));
console.log(kiemAgentCoHoTroKyNang(agentB, "phan_tich_hinh_anh"));
```

```text title=readonly
true
false
```

`agentB` công bố HAI kỹ năng (`tom_tat_van_ban`, `dich_thuat`). Kiểm một
kỹ năng NÓ có → `true`; kiểm một kỹ năng NÓ không hề nhắc tới
(`phan_tich_hinh_anh`) → `false` — KHÔNG cần gửi bất kỳ task nào, KHÔNG
cần gọi mạng, chỉ đọc đúng dữ liệu card đã có sẵn.
::::

::::example{#goi_mu_vs_kiem_truoc}
So sánh hai cách gửi task: `guiTaskMu` LUÔN báo "đã gửi" bất kể agent CÓ
hiểu yêu cầu hay không; `guiTaskCoKiemTruoc` kiểm `kiemAgentCoHoTroKyNang`
TRƯỚC, chỉ gửi khi CÓ:

```typescript title=readonly
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCapabilities = { streaming?: boolean; pushNotifications?: boolean };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: A2aAgentCapabilities;
  skills: A2aAgentSkill[];
};

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

type KetQuaGuiTask =
  | { thanhCong: true; thongBao: string }
  | { thanhCong: false; loi: string };

function guiTaskMu(card: A2aAgentCard, tenKyNang: string): KetQuaGuiTask {
  return { thanhCong: true, thongBao: `da gui task '${tenKyNang}' cho agent '${card.name}'` };
}

function guiTaskCoKiemTruoc(card: A2aAgentCard, tenKyNang: string): KetQuaGuiTask {
  if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) {
    return { thanhCong: false, loi: `agent '${card.name}' khong cong bo ky nang '${tenKyNang}'` };
  }
  return { thanhCong: true, thongBao: `da gui task '${tenKyNang}' cho agent '${card.name}'` };
}

const agentB: A2aAgentCard = {
  name: "agent-tom-tat",
  description: "Agent tom tat van ban tieng Viet",
  url: "https://vi-du.test/agent-tom-tat",
  capabilities: { streaming: false },
  skills: [{ id: "tom_tat_van_ban", name: "Tom tat van ban", description: "Tom tat mot doan van ban dai" }],
};

console.log(JSON.stringify(guiTaskMu(agentB, "phan_tich_hinh_anh")));
console.log(JSON.stringify(guiTaskCoKiemTruoc(agentB, "phan_tich_hinh_anh")));
```

```text title=readonly
{"thanhCong":true,"thongBao":"da gui task 'phan_tich_hinh_anh' cho agent 'agent-tom-tat'"}
{"thanhCong":false,"loi":"agent 'agent-tom-tat' khong cong bo ky nang 'phan_tich_hinh_anh'"}
```

CÙNG một agent, CÙNG một yêu cầu (`phan_tich_hinh_anh` — kỹ năng
`agentB` KHÔNG hề công bố) — `guiTaskMu` báo `thanhCong: true`, một lời
nói dối tiện lợi che mất đúng chỗ sẽ hỏng; `guiTaskCoKiemTruoc` báo
THẤT BẠI ngay, kèm lý do CỤ THỂ, TRƯỚC KHI bất kỳ task nào thật sự được
gửi đi.
::::

::::predict{#doan-kiem-ten-hay-id commitOnce}
`kiemAgentCoHoTroKyNang(agentB, "Tom tat van ban")` — TRUYỀN VÀO đúng
`name` hiển thị của kỹ năng (`"Tom tat van ban"`, có khoảng trắng VÀ chữ
hoa), KHÔNG PHẢI `id` (`"tom_tat_van_ban"`). Hàm trả về gì?

:::opt{correct}
`false` — `kiemAgentCoHoTroKyNang` chỉ so khớp `sk.id === tenKyNang`;
`"Tom tat van ban"` khác hoàn toàn chuỗi `"tom_tat_van_ban"` (khoảng
trắng, chữ hoa), nên KHÔNG kỹ năng nào khớp
:::
:::opt
`true` — vì đây LÀ tên của kỹ năng "Tom tat van ban" mà agentB đã công
bố, chỉ khác định dạng viết
::why
Nhầm rằng hàm so khớp theo Ý NGHĨA (con người đọc hiểu đây LÀ cùng một
kỹ năng) — nhưng `kiemAgentCoHoTroKyNang` LÀ một phép so sánh CHUỖI
chính xác (`===`), không hề "hiểu" hay chuẩn hoá khoảng trắng/chữ hoa.

Chỗ lệch: `sk.id` của kỹ năng đó LÀ `"tom_tat_van_ban"` — một chuỗi
KHÁC HẲN VỚI `"Tom tat van ban"` khi so bằng `===`.
::
:::
:::opt
Lỗi biên dịch — TypeScript sẽ từ chối truyền một chuỗi không khớp `id`
nào đã khai báo
::why
Nhầm rằng `tenKyNang` có một kiểu literal union bị RÀNG BUỘC theo đúng
các `id` đã khai — nhưng tham số `tenKyNang: string` LÀ kiểu `string`
THUẦN, chấp nhận BẤT KỲ chuỗi nào lúc biên dịch.

Chỗ lệch: hàm CHẠY bình thường VÀ trả `false` LÚC RUNTIME (không khớp
`id` nào) — không có gì ngăn TypeScript chấp nhận lời gọi này lúc biên
dịch.
::
:::
::::

::::code{#viet_gui_task_co_kiem_truoc}
Hoàn thiện `kiemAgentCoHoTroKyNang` — trả `true` NẾU `card.skills` CÓ
một phần tử `sk` mà `sk.id === tenKyNang`. Hoàn thiện
`guiTaskCoKiemTruoc` — NẾU `kiemAgentCoHoTroKyNang` trả `false`, trả về
`{ thanhCong: false; loi }` nêu rõ TÊN agent VÀ TÊN kỹ năng thiếu; NGƯỢC
LẠI trả về `{ thanhCong: true; thongBao }`.

```typescript title=starter
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCapabilities = { streaming?: boolean; pushNotifications?: boolean };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: A2aAgentCapabilities;
  skills: A2aAgentSkill[];
};

type KetQuaGuiTask =
  | { thanhCong: true; thongBao: string }
  | { thanhCong: false; loi: string };

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  ___
}

function guiTaskCoKiemTruoc(card: A2aAgentCard, tenKyNang: string): KetQuaGuiTask {
  ___
}

const agentB: A2aAgentCard = {
  name: "agent-tom-tat",
  description: "Agent tom tat van ban tieng Viet",
  url: "https://vi-du.test/agent-tom-tat",
  capabilities: { streaming: false },
  skills: [{ id: "tom_tat_van_ban", name: "Tom tat van ban", description: "Tom tat mot doan van ban dai" }],
};

const ketQuaHopLe = guiTaskCoKiemTruoc(agentB, "tom_tat_van_ban");
console.log(JSON.stringify(ketQuaHopLe));
```

```typescript title=solution
type A2aAgentSkill = { id: string; name: string; description: string };
type A2aAgentCapabilities = { streaming?: boolean; pushNotifications?: boolean };
type A2aAgentCard = {
  name: string;
  description: string;
  url: string;
  capabilities: A2aAgentCapabilities;
  skills: A2aAgentSkill[];
};

type KetQuaGuiTask =
  | { thanhCong: true; thongBao: string }
  | { thanhCong: false; loi: string };

function kiemAgentCoHoTroKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang);
}

function guiTaskCoKiemTruoc(card: A2aAgentCard, tenKyNang: string): KetQuaGuiTask {
  if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) {
    return { thanhCong: false, loi: `agent '${card.name}' khong cong bo ky nang '${tenKyNang}'` };
  }
  return { thanhCong: true, thongBao: `da gui task '${tenKyNang}' cho agent '${card.name}'` };
}

const agentB: A2aAgentCard = {
  name: "agent-tom-tat",
  description: "Agent tom tat van ban tieng Viet",
  url: "https://vi-du.test/agent-tom-tat",
  capabilities: { streaming: false },
  skills: [{ id: "tom_tat_van_ban", name: "Tom tat van ban", description: "Tom tat mot doan van ban dai" }],
};

const ketQuaHopLe = guiTaskCoKiemTruoc(agentB, "tom_tat_van_ban");
console.log(JSON.stringify(ketQuaHopLe));
```

```typescript title=test
if (!kiemAgentCoHoTroKyNang(agentB, "tom_tat_van_ban")) throw new Error("agentB CO ky nang tom_tat_van_ban, phai tra ve true");
if (kiemAgentCoHoTroKyNang(agentB, "dich_thuat")) throw new Error("agentB KHONG co ky nang dich_thuat, phai tra ve false");

if (!ketQuaHopLe.thanhCong) throw new Error("gui task dung ky nang ma agent co phai THANH CONG");
if (!ketQuaHopLe.thongBao.includes("agent-tom-tat")) throw new Error("thongBao phai nhac ten agent");

const ketQuaSai = guiTaskCoKiemTruoc(agentB, "phan_tich_hinh_anh");
if (ketQuaSai.thanhCong) throw new Error("gui task ky nang agent KHONG co phai THAT BAI, khong duoc bao thanh cong");
if (!ketQuaSai.loi.includes("phan_tich_hinh_anh")) throw new Error("loi phai neu ro ten ky nang khong duoc ho tro");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (kiemAgentCoHoTroKyNang): dung .some() tren card.skills, so sanh sk.id voi tenKyNang. Cho hai (guiTaskCoKiemTruoc): if khong co ky nang thi tra ve thanhCong false kem loi neu ro ten agent va ten ky nang; nguoc lai tra ve thanhCong true."
- kind: strategy
  body: "Cho dau: return card.skills.some((sk) => sk.id === tenKyNang); Cho hai: if (!kiemAgentCoHoTroKyNang(card, tenKyNang)) return { thanhCong: false, loi: `agent '${card.name}' khong cong bo ky nang '${tenKyNang}'` }; return { thanhCong: true, thongBao: `da gui task '${tenKyNang}' cho agent '${card.name}'` };"
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
  expect: "da gui task 'tom_tat_van_ban' cho agent 'agent-tom-tat'"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kiểm Agent Card TRƯỚC khi gửi — một phép so sánh chuỗi đơn giản, nhưng
đủ để biến "gửi rồi mới biết agent không hiểu" thành "biết ngay từ đầu,
kèm lý do". Bài sau: khi task ĐÃ được gửi, nó đi qua ĐÚNG những trạng
thái nào, VÀ trạng thái nào KHÔNG được phép quay lại.
::::

::::reflect{#nghi-lai}
Agent Card VÀ MCP capabilities (bài `2`) giải quyết CÙNG một lớp vấn đề
Ở hai BIÊN khác nhau: MCP hỏi "server này hỗ trợ method gì" TRƯỚC khi
agent gọi tool; A2A hỏi "agent kia công bố kỹ năng gì" TRƯỚC khi agent
gửi task. Cả hai đều LÀ một phép kiểm RẺ (đọc dữ liệu đã có sẵn, không
cần gọi mạng thật) đứng TRƯỚC một hành động ĐẮT (gửi yêu cầu, chờ phản
hồi) — kiểm rẻ trước LÀ cách duy nhất biến một lỗi PHÁT HIỆN MUỘN (sau
khi đã gửi) thành một lỗi PHÁT HIỆN SỚM (trước khi gửi bất cứ thứ gì).
::::

::::checkpoint{mastery=0.85}
::::
