/**
 * Nguồn sự thật PHIÊN BẢN cho MCP (Model Context Protocol, Anthropic) và A2A
 * (Agent2Agent Protocol, Google) — mốc snapshot "2026-06" của dự án này.
 *
 * "2026-06" là một MỐC GIẢ ĐỊNH cho khoá học, không nhất thiết khớp một ngày
 * phát hành thật của hai giao thức — nhưng HÌNH DẠNG field/JSON-RPC dưới đây
 * phản ánh ĐÚNG tinh thần thật của MCP và A2A tại thời điểm biên soạn.
 *
 * Theo `content/curriculum/thu-tu.yaml` (track `giao-thuc-mcp-a2a`, T9.6):
 * phần BẤT BIẾN (vì sao cần chuẩn hoá; JSON-RPC; capability negotiation;
 * vòng đời task; lớp mối đe doạ) nằm TRONG mỗi `.lesson.md`, tự chứa, không
 * import file này. Phần PHIÊN BẢN — tên field, tên endpoint/method, hình
 * dạng chính xác — dồn HẾT vào một chỗ: file này. Test đỏ ở đây → sửa MỘT
 * file, không phải mọi lesson.
 *
 * Các `.lesson.md` ĐỊNH NGHĨA LẠI một bản rút gọn của các type dưới đây
 * ngay trong code block của chính nó (không import chéo — mỗi bài tự chứa
 * hoàn toàn, đúng quy ước xuyên suốt Realm 9). Tên field trong bản rút gọn
 * đó PHẢI khớp CHÍNH XÁC tên field ở đây.
 */

/* ═══════════════ 1. MCP — lớp JSON-RPC 2.0 nền ═══════════════ */

/** Một lời gọi JSON-RPC 2.0. `id` tương quan request↔response — phần mà
 *  `ToolMoPhong` ở T9.3 (harness) KHÔNG có: nó gọi hàm rồi nhận kết quả
 *  ngay tại chỗ, không cần khớp `id` vì không có kênh bất đồng bộ nào ở
 *  giữa. */
export interface JsonRpcRequest<TParams = unknown> {
  readonly jsonrpc: '2.0';
  readonly id: string | number;
  readonly method: string;
  readonly params?: TParams;
}

/** Lỗi Ở TẦNG GIAO THỨC — phân biệt với lỗi NGHIỆP VỤ mà một tool trả về
 *  bên trong `result` (ví dụ `McpToolCallResult.isError`). `code`/`message`
 *  bắt buộc; `data` mang ngữ cảnh thêm, tuỳ chọn. */
export interface JsonRpcError {
  readonly code: number;
  readonly message: string;
  readonly data?: unknown;
}

/** Phản hồi THÀNH CÔNG — có `result`, không có `error`. */
export interface JsonRpcSuccessResponse<TResult = unknown> {
  readonly jsonrpc: '2.0';
  readonly id: string | number;
  readonly result: TResult;
}

/** Phản hồi LỖI GIAO THỨC — có `error`, không có `result`. `id` có thể là
 *  `null` khi lỗi xảy ra TRƯỚC khi server đọc được `id` của request gốc
 *  (ví dụ JSON không parse được). */
export interface JsonRpcErrorResponse {
  readonly jsonrpc: '2.0';
  readonly id: string | number | null;
  readonly error: JsonRpcError;
}

/** Một response JSON-RPC LUÔN là MỘT trong hai hình dạng trên — không bao
 *  giờ có CẢ `result` LẪN `error`, và không bao giờ THIẾU cả hai. */
export type JsonRpcResponse<TResult = unknown> =
  | JsonRpcSuccessResponse<TResult>
  | JsonRpcErrorResponse;

/* ═══════════════ 2. MCP — capability negotiation ═══════════════ */

export interface McpClientInfo {
  readonly name: string;
  readonly version: string;
}

export interface McpServerInfo {
  readonly name: string;
  readonly version: string;
}

/** Mỗi khả năng là MỘT object riêng (không phải `boolean`): rỗng `{}` đã
 *  đủ để báo "có hỗ trợ", và các field bên trong (như `listChanged`) mở
 *  rộng dần mà không phá vỡ phía đã đọc bản cũ. */
export interface McpToolsCapability {
  readonly listChanged?: boolean;
}
export interface McpResourcesCapability {
  readonly listChanged?: boolean;
  readonly subscribe?: boolean;
}
export interface McpPromptsCapability {
  readonly listChanged?: boolean;
}

/** Ba khả năng CỐT LÕI của MCP. Vắng mặt một field nghĩa là bên kia KHÔNG
 *  hỗ trợ khả năng đó — không phải "hỗ trợ với cấu hình mặc định". */
export interface McpCapabilities {
  readonly tools?: McpToolsCapability;
  readonly resources?: McpResourcesCapability;
  readonly prompts?: McpPromptsCapability;
}

/** Tham số của method `"initialize"` — LỜI GỌI ĐẦU TIÊN bắt buộc trước khi
 *  dùng bất kỳ method nào khác. */
export interface McpInitializeParams {
  readonly protocolVersion: string;
  readonly clientInfo: McpClientInfo;
  readonly capabilities: McpCapabilities;
}

/** Kết quả trả về của `"initialize"` — server công bố NÓ hỗ trợ gì, không
 *  phải công bố lại y hệt những gì client vừa hỏi. */
export interface McpInitializeResult {
  readonly protocolVersion: string;
  readonly serverInfo: McpServerInfo;
  readonly capabilities: McpCapabilities;
}

/* ═══════════════ 3. MCP — tool schema ═══════════════ */

/** JSON Schema TỐI GIẢN — đủ để mô tả tham số một tool, không phải toàn bộ
 *  đặc tả JSON Schema. */
export interface McpJsonSchemaThuocTinh {
  readonly type: 'string' | 'number' | 'boolean' | 'object' | 'array';
  readonly description?: string;
}

export interface McpJsonSchema {
  readonly type: 'object';
  readonly properties: Record<string, McpJsonSchemaThuocTinh>;
  readonly required?: readonly string[];
}

/** Một mục trong kết quả `"tools/list"`. */
export interface McpToolDefinition {
  readonly name: string;
  readonly description: string;
  readonly inputSchema: McpJsonSchema;
}

/** Tham số của method `"tools/call"`. */
export interface McpToolCallParams {
  readonly name: string;
  readonly arguments: Record<string, unknown>;
}

export type McpContentBlock =
  | { readonly type: 'text'; readonly text: string }
  | { readonly type: 'image'; readonly data: string; readonly mimeType: string };

/** Kết quả của `"tools/call"`. `isError: true` nghĩa là tool CHẠY XONG
 *  nhưng bản thân kết quả LÀ một lỗi nghiệp vụ — khác hẳn một lỗi JSON-RPC
 *  ở tầng giao thức (`JsonRpcErrorResponse`), vốn nghĩa là lời gọi KHÔNG
 *  chạy được (sai method, sai tham số ở tầng giao thức, …). */
export interface McpToolCallResult {
  readonly content: readonly McpContentBlock[];
  readonly isError?: boolean;
}

/* ═══════════════ 4. MCP — vòng đời method chuẩn ═══════════════ */

export const MCP_METHODS = {
  initialize: 'initialize',
  toolsList: 'tools/list',
  toolsCall: 'tools/call',
  notificationsInitialized: 'notifications/initialized',
} as const;

export type McpMethodName = (typeof MCP_METHODS)[keyof typeof MCP_METHODS];

/* ═══════════════ 5. A2A — Agent Card ═══════════════ */

export interface A2aAgentSkill {
  readonly id: string;
  readonly name: string;
  readonly description: string;
}

export interface A2aAgentCapabilities {
  readonly streaming?: boolean;
  readonly pushNotifications?: boolean;
}

/** Danh thiếp một agent A2A công bố công khai — CÁCH DUY NHẤT một agent
 *  khác biết được nó có kỹ năng gì TRƯỚC KHI gửi task, không cần gọi thử. */
export interface A2aAgentCard {
  readonly name: string;
  readonly description: string;
  readonly url: string;
  readonly capabilities: A2aAgentCapabilities;
  readonly skills: readonly A2aAgentSkill[];
}

/* ═══════════════ 6. A2A — Task lifecycle ═══════════════ */

/** Sáu trạng thái — KHÔNG có trạng thái thứ bảy nào trong đặc tả A2A. */
export type A2aTaskState =
  | 'submitted'
  | 'working'
  | 'input-required'
  | 'completed'
  | 'canceled'
  | 'failed';

export interface A2aMessage {
  readonly role: 'user' | 'agent';
  readonly text: string;
}

export interface A2aTask {
  readonly id: string;
  readonly state: A2aTaskState;
  readonly history: readonly A2aMessage[];
}

/* ═══════════════ 7. Lớp mối đe doạ MCP — DỮ LIỆU tham chiếu ═══════════════
 *
 * Ba loại này KHÔNG PHẢI một bộ lọc runtime — chúng là TÀI LIỆU, để các
 * lesson trích dẫn khi giải thích VÌ SAO capability negotiation / schema
 * validation / kiểm AgentCard trước khi gửi task lại quan trọng.
 */

export type MoiDeDoaMcp = 'tool_poisoning' | 'confused_deputy' | 'token_passthrough';

export const MO_TA_MOI_DE_DOA: Record<MoiDeDoaMcp, string> = {
  tool_poisoning:
    'Mô tả hoặc tên của một tool bị chèn chỉ dẫn ẩn (nhắm vào model đọc mô tả, ' +
    'không nhắm vào người dùng đọc màn hình) để đánh lừa agent gọi sai tool, ' +
    'gọi đúng tool nhưng sai cách, hoặc lộ dữ liệu ra ngoài — "độc" nằm trong ' +
    'METADATA tool (name/description/inputSchema), không nằm ở tham số của một ' +
    'lần gọi cụ thể.',
  confused_deputy:
    'Agent giữ một quyền hợp lệ (ví dụ token truy cập kho dữ liệu nội bộ) bị ' +
    'lừa DÙNG CHÍNH quyền đó thay mặt một bên không có quyền ấy — agent đóng ' +
    'vai "phó thác" (deputy) nhưng không phân biệt được yêu cầu đang tới từ ' +
    'ai, nên hành động như thể yêu cầu tới từ chủ quyền thật.',
  token_passthrough:
    'Một MCP server nhận token xác thực của client rồi CHUYỂN THẲNG token đó ' +
    'sang một dịch vụ hạ nguồn khác mà không kiểm audience/scope của token — ' +
    'token phát hành để dùng ở nơi A bị dùng sai bối cảnh ở nơi B, vượt khỏi ' +
    'phạm vi nó được cấp.',
};

/* ═══════════════ 8. Type-guard thuần ═══════════════ */

/** `res` có phải một phản hồi LỖI GIAO THỨC (có `error`, không có `result`)? */
export function laJsonRpcError<TResult>(
  res: JsonRpcResponse<TResult>,
): res is JsonRpcErrorResponse {
  return 'error' in res;
}

/** `res` có phải một phản hồi THÀNH CÔNG (có `result`, không có `error`)? */
export function laJsonRpcThanhCong<TResult>(
  res: JsonRpcResponse<TResult>,
): res is JsonRpcSuccessResponse<TResult> {
  return 'result' in res;
}

/** Tool CHẠY XONG nhưng bản thân kết quả LÀ một lỗi nghiệp vụ? */
export function laMcpToolCallLoi(kq: McpToolCallResult): boolean {
  return kq.isError === true;
}

/** Task đã ở một trong ba trạng thái KẾT THÚC (không còn chuyển tiếp được
 *  sang trạng thái khác)? */
export function laA2aTaskKetThuc(state: A2aTaskState): boolean {
  return state === 'completed' || state === 'canceled' || state === 'failed';
}

/** `card` có công bố kỹ năng `tenKyNang` (khớp `id` HOẶC `name`) hay không —
 *  kiểm TRƯỚC KHI gửi task, thay vì gửi mù rồi mới biết agent không hiểu. */
export function agentCoKyNang(card: A2aAgentCard, tenKyNang: string): boolean {
  return card.skills.some((sk) => sk.id === tenKyNang || sk.name === tenKyNang);
}
