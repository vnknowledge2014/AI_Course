---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.capability-negotiation-truoc-khi-dung
title: "Capability negotiation — \"initialize\" trước khi dùng"
summary: "moPhongInitialize(clientParams, serverCapabilities) trả về McpInitializeResult qua layGiaoCapabilities — giao ĐÚNG những khoá (tools/resources/prompts) mà CẢ client lẫn server cùng khai, không phải mọi khoá client hỏi. Trên kịch bản client hỏi 3 khoá {tools,resources,prompts}, server chỉ khai 2 khoá {tools,prompts}: kết quả capabilities CHỈ có đúng 2 khoá tools+prompts, KHÔNG có resources dù client có hỏi. Trên kịch bản server capabilities rỗng {}: giao luôn là object rỗng {} dù client hỏi gì. protocolVersion và serverInfo trong kết quả không qua giao — protocolVersion lấy nguyên từ clientParams, serverInfo là một hằng số tĩnh của server. Đối chiếu: một agent bỏ qua bước 'initialize', cứ giả định server hỗ trợ đúng những gì mình hỏi, có thể gọi nhầm một method (ví dụ 'resources/list') mà server KHÔNG hề công bố hỗ trợ."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.capability-negotiation-truoc-khi-dung]
requires: [kna.json-rpc-la-gi-va-vi-sao-can-chuan-hoa]
concepts: [kna.capability-negotiation-truoc-khi-dung]
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
Bài trước, `xuLyYeuCauJsonRpc` hiểu ĐÚNG MỘT method — "cong". Một server
MCP thật có THỂ hỗ trợ tools, resources, prompts — hoặc chỉ MỘT trong ba.
Trước khi gọi bất kỳ method nào, MCP bắt CẢ hai bên nói chuyện MỘT lần:
client gửi `"initialize"` kèm khả năng NÓ muốn dùng, server trả lời kèm
khả năng NÓ thật sự hỗ trợ. Bỏ qua bước này LÀ đoán mù.
::::

::::explain{#giao_capabilities}
`McpCapabilities` LÀ một object CÓ tối đa ba khoá — `tools`, `resources`,
`prompts` — MỖI khoá LÀ optional: CÓ khoá (dù giá trị rỗng `{}`) nghĩa
LÀ "có hỗ trợ", VẮNG khoá nghĩa LÀ "không hỗ trợ". `layGiaoCapabilities`
tính GIAO của hai bộ khả năng: một khoá CHỈ xuất hiện Ở kết quả khi CẢ
client LẪN server cùng khai khoá đó — client hỏi mà server không hỗ trợ
thì khoá đó BIẾN MẤT, không hề có mặt "một nửa":

```typescript title=readonly
type McpCapabilities = {
  tools?: Record<string, never>;
  resources?: Record<string, never>;
  prompts?: Record<string, never>;
};

function layGiaoCapabilities(client: McpCapabilities, server: McpCapabilities): McpCapabilities {
  const ketQua: McpCapabilities = {};
  if (client.tools !== undefined && server.tools !== undefined) ketQua.tools = {};
  if (client.resources !== undefined && server.resources !== undefined) ketQua.resources = {};
  if (client.prompts !== undefined && server.prompts !== undefined) ketQua.prompts = {};
  return ketQua;
}

const giao = layGiaoCapabilities({ tools: {}, resources: {} }, { tools: {} });
console.log(JSON.stringify(giao));
```

```text title=readonly
{"tools":{}}
```

Client hỏi HAI khoá (`tools`, `resources`); server chỉ khai MỘT
(`tools`). Kết quả giao chỉ CÓ `tools` — `resources` không xuất hiện dù
client CÓ hỏi, vì server KHÔNG khai nó.
::::

::::example{#mo_phong_initialize}
`moPhongInitialize` LÀ vòng bắt tay đầy đủ: nhận `McpInitializeParams`
CỦA client (kèm `protocolVersion` VÀ `capabilities` NÓ muốn), trả về
`McpInitializeResult` CỦA server — `protocolVersion` giữ nguyên từ
client, `serverInfo` LÀ một hằng số tĩnh, VÀ `capabilities` LÀ GIAO vừa
tính Ở trên, KHÔNG PHẢI y hệt những gì client đã hỏi:

```typescript title=readonly
type McpCapabilities = {
  tools?: Record<string, never>;
  resources?: Record<string, never>;
  prompts?: Record<string, never>;
};

type McpClientInfo = { name: string; version: string };
type McpServerInfo = { name: string; version: string };

type McpInitializeParams = {
  protocolVersion: string;
  clientInfo: McpClientInfo;
  capabilities: McpCapabilities;
};

type McpInitializeResult = {
  protocolVersion: string;
  serverInfo: McpServerInfo;
  capabilities: McpCapabilities;
};

function layGiaoCapabilities(client: McpCapabilities, server: McpCapabilities): McpCapabilities {
  const ketQua: McpCapabilities = {};
  if (client.tools !== undefined && server.tools !== undefined) ketQua.tools = {};
  if (client.resources !== undefined && server.resources !== undefined) ketQua.resources = {};
  if (client.prompts !== undefined && server.prompts !== undefined) ketQua.prompts = {};
  return ketQua;
}

function moPhongInitialize(
  clientParams: McpInitializeParams,
  serverCapabilities: McpCapabilities,
): McpInitializeResult {
  return {
    protocolVersion: clientParams.protocolVersion,
    serverInfo: { name: "byte-mcp-server", version: "1.0.0" },
    capabilities: layGiaoCapabilities(clientParams.capabilities, serverCapabilities),
  };
}

const clientParams: McpInitializeParams = {
  protocolVersion: "2026-06",
  clientInfo: { name: "byte-agent", version: "0.1.0" },
  capabilities: { tools: {}, resources: {} },
};
const serverCapabilities: McpCapabilities = { tools: {} };
const ketQua = moPhongInitialize(clientParams, serverCapabilities);
console.log(JSON.stringify(ketQua));
```

```text title=readonly
{"protocolVersion":"2026-06","serverInfo":{"name":"byte-mcp-server","version":"1.0.0"},"capabilities":{"tools":{}}}
```

`clientParams.capabilities` hỏi CẢ `tools` LẪN `resources`, nhưng
`serverCapabilities` chỉ khai `tools` — kết quả CHỈ có `tools`. Một agent
BỎ QUA bước `"initialize"` này VÀ cứ đinh ninh server hỗ trợ đúng những
gì mình hỏi CÓ THỂ gọi thẳng `"resources/list"` — một method mà server
này KHÔNG HỀ công bố hỗ trợ.
::::

::::predict{#doan-giao-ba-khoa commitOnce}
Client hỏi ĐỦ BA khoá `{tools:{}, resources:{}, prompts:{}}`. Server chỉ
khai HAI khoá `{tools:{}, prompts:{}}` (không có `resources`).
`moPhongInitialize` trả về `capabilities` có BAO NHIÊU khoá, VÀ khoá
nào?

:::opt{correct}
Đúng HAI khoá — `tools` VÀ `prompts`; `resources` KHÔNG xuất hiện, vì
`layGiaoCapabilities` chỉ giữ khoá khi CẢ client LẪN server cùng khai,
VÀ server Ở đây không khai `resources`
:::
:::opt
Đúng BA khoá — `tools`, `resources`, VÀ `prompts` — vì client đã hỏi
đủ ba nên server "nên" trả đủ ba để không làm client thất vọng
::why
Nhầm rằng phía hỏi nhiều hơn thì phía kia PHẢI đáp ứng đủ — nhưng
`layGiaoCapabilities` không hề đọc "client muốn gì" để QUYẾT ĐỊNH kết
quả, nó chỉ kiểm CẢ HAI phía có cùng khai một khoá hay không.

Chỗ lệch: server không khai `resources` (`server.resources` LÀ
`undefined`), nên điều kiện `client.resources !== undefined && server.
resources !== undefined` LÀ `false` — `resources` không bao giờ được
gán vào `ketQua`.
::
:::
:::opt
Đúng MỘT khoá — object rỗng `{}`, vì client VÀ server không khớp
HOÀN TOÀN (client hỏi 3, server chỉ có 2) nên coi như không ai hỗ trợ
gì cả
::why
Nhầm rằng giao chỉ có nghĩa khi hai bộ khả năng khớp NHAU HOÀN TOÀN —
nhưng `layGiaoCapabilities` xét TỪNG khoá riêng lẻ, không đòi hỏi cả bộ
phải giống hệt nhau.

Chỗ lệch: `tools` VÀ `prompts` đều được CẢ hai bên khai, nên CẢ hai khoá
đó đều lọt vào `ketQua` — chỉ `resources` (khoá DUY NHẤT server không
khai) mới bị loại.
::
:::
::::

::::code{#viet_mo_phong_initialize}
Hoàn thiện `layGiaoCapabilities` — với MỖI khoá (`tools`, `resources`,
`prompts`), CHỈ gán khoá đó vào `ketQua` (giá trị `{}`) khi CẢ `client`
LẪN `server` cùng khai khoá đó (khác `undefined`). Hoàn thiện
`moPhongInitialize` — trả về `protocolVersion` LẤY từ `clientParams`,
`serverInfo` LÀ hằng số `{ name: "byte-mcp-server", version: "1.0.0" }`,
VÀ `capabilities` LÀ kết quả của `layGiaoCapabilities` gọi trên
`clientParams.capabilities` VÀ `serverCapabilities`.

```typescript title=starter
type McpCapabilities = {
  tools?: Record<string, never>;
  resources?: Record<string, never>;
  prompts?: Record<string, never>;
};

type McpClientInfo = { name: string; version: string };
type McpServerInfo = { name: string; version: string };

type McpInitializeParams = {
  protocolVersion: string;
  clientInfo: McpClientInfo;
  capabilities: McpCapabilities;
};

type McpInitializeResult = {
  protocolVersion: string;
  serverInfo: McpServerInfo;
  capabilities: McpCapabilities;
};

function layGiaoCapabilities(client: McpCapabilities, server: McpCapabilities): McpCapabilities {
  ___
}

function moPhongInitialize(
  clientParams: McpInitializeParams,
  serverCapabilities: McpCapabilities,
): McpInitializeResult {
  ___
}

const ketQuaBaKhaNang = moPhongInitialize(
  {
    protocolVersion: "2026-06",
    clientInfo: { name: "byte-agent", version: "0.1.0" },
    capabilities: { tools: {}, resources: {}, prompts: {} },
  },
  { tools: {}, prompts: {} },
);
console.log(JSON.stringify(ketQuaBaKhaNang));
```

```typescript title=solution
type McpCapabilities = {
  tools?: Record<string, never>;
  resources?: Record<string, never>;
  prompts?: Record<string, never>;
};

type McpClientInfo = { name: string; version: string };
type McpServerInfo = { name: string; version: string };

type McpInitializeParams = {
  protocolVersion: string;
  clientInfo: McpClientInfo;
  capabilities: McpCapabilities;
};

type McpInitializeResult = {
  protocolVersion: string;
  serverInfo: McpServerInfo;
  capabilities: McpCapabilities;
};

function layGiaoCapabilities(client: McpCapabilities, server: McpCapabilities): McpCapabilities {
  const ketQua: McpCapabilities = {};
  if (client.tools !== undefined && server.tools !== undefined) ketQua.tools = {};
  if (client.resources !== undefined && server.resources !== undefined) ketQua.resources = {};
  if (client.prompts !== undefined && server.prompts !== undefined) ketQua.prompts = {};
  return ketQua;
}

function moPhongInitialize(
  clientParams: McpInitializeParams,
  serverCapabilities: McpCapabilities,
): McpInitializeResult {
  return {
    protocolVersion: clientParams.protocolVersion,
    serverInfo: { name: "byte-mcp-server", version: "1.0.0" },
    capabilities: layGiaoCapabilities(clientParams.capabilities, serverCapabilities),
  };
}

const ketQuaBaKhaNang = moPhongInitialize(
  {
    protocolVersion: "2026-06",
    clientInfo: { name: "byte-agent", version: "0.1.0" },
    capabilities: { tools: {}, resources: {}, prompts: {} },
  },
  { tools: {}, prompts: {} },
);
console.log(JSON.stringify(ketQuaBaKhaNang));
```

```typescript title=test
if (!("tools" in ketQuaBaKhaNang.capabilities)) throw new Error("server co tools, client cung hoi tools -> phai co tools trong giao");
if (!("prompts" in ketQuaBaKhaNang.capabilities)) throw new Error("server co prompts, client cung hoi prompts -> phai co prompts trong giao");
if ("resources" in ketQuaBaKhaNang.capabilities) throw new Error("server KHONG co resources -> giao KHONG duoc co resources, du client co hoi");
if (ketQuaBaKhaNang.protocolVersion !== "2026-06") throw new Error("protocolVersion phai lay tu clientParams");
if (ketQuaBaKhaNang.serverInfo.name !== "byte-mcp-server") throw new Error("serverInfo.name phai la 'byte-mcp-server'");

const giaoRong = layGiaoCapabilities({ tools: {} }, {});
if ("tools" in giaoRong) throw new Error("server rong -> giao KHONG duoc co tools du client co hoi");
if (Object.keys(giaoRong).length !== 0) throw new Error("giao voi server rong phai la object rong");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (layGiaoCapabilities): ba dong if giong het nhau, moi dong kiem MOT khoa co mat o CA HAI phia (client va server) khac undefined chua. Cho hai (moPhongInitialize): tra ve mot object voi ba truong -- protocolVersion tu clientParams, serverInfo la hang so, capabilities goi layGiaoCapabilities."
- kind: strategy
  body: "Cho dau: const ketQua: McpCapabilities = {}; if (client.tools !== undefined && server.tools !== undefined) ketQua.tools = {}; if (client.resources !== undefined && server.resources !== undefined) ketQua.resources = {}; if (client.prompts !== undefined && server.prompts !== undefined) ketQua.prompts = {}; return ketQua; Cho hai: return { protocolVersion: clientParams.protocolVersion, serverInfo: { name: 'byte-mcp-server', version: '1.0.0' }, capabilities: layGiaoCapabilities(clientParams.capabilities, serverCapabilities) };"
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
  expect: "\"capabilities\":{\"tools\":{},\"prompts\":{}}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Client hỏi ba, server khai hai, giao ra đúng hai — không hơn, không kém.
Từ bài sau, mọi lời gọi tool đi qua ĐÚNG capabilities đã thương lượng Ở
đây — bỏ qua bước này LÀ gọi vào một khả năng chưa ai xác nhận LÀ CÓ.
::::

::::reflect{#nghi-lai}
Điều dễ hiểu lầm nhất về capability negotiation: nó KHÔNG PHẢI một cuộc
"thoả hiệp" nơi hai bên nhượng bộ nhau — nó LÀ một phép GIAO tập hợp đơn
giản, và kết quả CHỈ phụ thuộc vào những gì MỖI BÊN thật sự khai, không
phụ thuộc vào bên nào "muốn" nhiều hơn. Client hỏi bao nhiêu không quan
trọng bằng server THẬT SỰ hỗ trợ bao nhiêu — VÀ đó chính LÀ lý do bước
`"initialize"` phải đứng TRƯỚC mọi method khác: nó biến một giả định
("chắc server hỗ trợ cái này") thành một sự thật đã xác nhận.
::::

::::checkpoint{mastery=0.85}
::::
