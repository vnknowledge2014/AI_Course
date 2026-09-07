---
id: ky-nghe-ung-dung-ai.boss-ky-nghe-ung-dung-ai.dat-bai-toan-tro-ly-nghien-cuu-qua-mcp
title: "T9.7 bài 1 — trợ lý nghiên cứu gọi tool qua MCP, đo end-to-end"
summary: "traLoiCauHoi(catalog, cauHoi): KetQuaTraLoi ráp PROMPT (chonToolCoCauTruc — structured tool-call output, phân biệt da_chon/khong_xac_dinh_nhiem_vu/thieu_tham_so) VỚI PROTOCOL (goiMcpTool — validate tham số theo McpJsonSchema TRƯỚC khi chạm hai tool thật tra_cuu_ho_so/tra_cuu_gia_co_phieu) ở mức đơn giản nhất. Trên BO_CAU_HOI_L1 (4 câu — 2 hợp lệ, 1 không xác định nhiệm vụ, 1 hỏi mã công ty không tồn tại): tyLeHoanThanh=0.5 (2/4), tongChiPhi=5, tongSoBuoc=3, soLanChanAnToan=0 (chưa có mối đe doạ nào ở bài này). Định nghĩa BỐN chỉ số 'đo end-to-end' mà toàn bộ T9.7 sẽ dùng lại tới BOSS cuối cùng."
locale: vi
track: ky-nghe-ung-dung-ai
module: boss-ky-nghe-ung-dung-ai
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.dat-bai-toan-tro-ly-nghien-cuu-qua-mcp]
requires: [kna.boss-t9-6-day-du-nam-cong-vs-khong-chuan-hoa]
concepts: [kna.dat-bai-toan-tro-ly-nghien-cuu-qua-mcp]
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
Sáu track đã đóng: prompt, ngữ cảnh, harness, vòng lặp, đồ thị, giao thức
MCP/A2A. `T9.7` ráp TẤT CẢ thành MỘT agent giải MỘT nhiệm vụ thật — một
"trợ lý nghiên cứu" trả lời câu hỏi bằng cách gọi tool qua MCP. Bài này mở
màn: PROMPT (đầu ra tool-call có cấu trúc) cộng PROTOCOL (gọi tool qua MCP
có validate) ở mức đơn giản nhất — MỘT câu hỏi, MỘT lượt gọi tool.
::::

::::explain{#tro_ly_nghien_cuu_va_bon_chi_so}
Nhiệm vụ thật: người dùng hỏi về MỘT công ty (hồ sơ hay giá cổ phiếu); trợ
lý phải QUYẾT ĐỊNH tool nào cần gọi, GỌI nó qua một giao thức chuẩn hoá
(giống MCP đã học ở `T9.6`), rồi trả về kết quả. Hai lớp ráp lại:

> **PROMPT** (structured tool-call output, `q9.1a`) — `chonToolCoCauTruc`
> không đoán mò; nó trả về MỘT trong ba kết cục tường minh: `da_chon` (đủ
> tool + tham số), `khong_xac_dinh_nhiem_vu` (câu hỏi không khớp tool nào),
> hoặc `thieu_tham_so` (khớp tool nhưng thiếu mã công ty).
>
> **PROTOCOL** (MCP tool schema, `T9.6` bài `tool-schema-qua-mcp`) —
> `goiMcpTool` tra `catalog` theo tên, validate tham số ĐÚNG
> `McpJsonSchema` (`kiemTraThamSoTheoSchema`, tái dùng nguyên vẹn) TRƯỚC KHI
> chạm tool thật.

`T9.7` đo "end-to-end" bằng ĐÚNG bốn chỉ số — mọi bài sau, tới BOSS cuối,
đều dùng lại bốn chỉ số này:

1. **`tyLeHoanThanh`** — tỉ lệ câu hỏi được trả lời thành công.
2. **`tongChiPhi`** — chi phí giả lập, tính theo BẢNG giá riêng từng tool
   (`tra_cuu_ho_so` giá `1`, `tra_cuu_gia_co_phieu` giá `2` — hai tool
   KHÔNG đồng giá, để chi phí và số bước có thể lệch nhau ngay từ bài đầu).
3. **`tongSoBuoc`** — số câu hỏi THẬT SỰ đi tới bước gọi tool (không đếm
   câu bị từ chối ngay ở PROMPT vì không xác định được nhiệm vụ).
4. **`soLanChanAnToan`** — số lần một cơ chế an toàn phải can thiệp. Ở
   bài này LUÔN LÀ `0`, vì CHƯA có mối đe doạ nào được dạy — con số này
   chỉ có ý nghĩa thật từ bài `phong-thu-injection-va-loc-tool-poisoning`.

```typescript title=readonly
type KetQua<T, E> = { thanhCong: true; giaTri: T } | { thanhCong: false; loi: E };

type McpJsonSchema = { type: "object"; properties: Record<string, { type: "string" | "number" | "boolean" }>; required?: string[] };
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };

type HoSoCongTy = { maCongTy: string; nganh: string; namThanhLap: number };
const CSDL_HO_SO: Record<string, HoSoCongTy> = {
  ACME: { maCongTy: "ACME", nganh: "ban le", namThanhLap: 1998 },
  GLOB: { maCongTy: "GLOB", nganh: "cong nghe", namThanhLap: 2005 },
};
const CSDL_GIA: Record<string, number> = { ACME: 42, GLOB: 108 };

const demGoiToolThat = { soLan: 0 };

function thucThiTraCuuHoSo(thamSo: Record<string, unknown>): KetQua<HoSoCongTy, string> {
  demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const hoSo = CSDL_HO_SO[maCongTy];
  if (!hoSo) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: hoSo };
}
function thucThiTraCuuGiaCoPhieu(thamSo: Record<string, unknown>): KetQua<number, string> {
  demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const gia = CSDL_GIA[maCongTy];
  if (gia === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: gia };
}

const CATALOG_TOOL_NGHIEN_CUU: McpToolDefinition[] = [
  { name: "tra_cuu_ho_so", description: "Tra cuu ho so cong ty theo ma", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_gia_co_phieu", description: "Tra cuu gia co phieu hien tai theo ma cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
];

function trichMaCongTy(cauHoi: string): string | null {
  const khop = /\b[A-Z]{4}\b/.exec(cauHoi);
  return khop ? khop[0] : null;
}

type LoiGoiTool = { tenTool: string; thamSo: Record<string, unknown> };
type LoiGoiToolTho = LoiGoiTool | null;

function llmMoPhongChonTool(cauHoi: string): LoiGoiToolTho {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  const thamSo: Record<string, unknown> = maCongTy !== null ? { maCongTy } : {};
  if (c.includes("gia") || c.includes("co phieu")) return { tenTool: "tra_cuu_gia_co_phieu", thamSo };
  if (c.includes("ho so") || c.includes("thong tin")) return { tenTool: "tra_cuu_ho_so", thamSo };
  return null;
}

type KetQuaChonTool =
  | { loai: "da_chon"; loiGoi: LoiGoiTool }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string };

function chonToolCoCauTruc(cauHoi: string): KetQuaChonTool {
  const tho = llmMoPhongChonTool(cauHoi);
  if (tho === null) return { loai: "khong_xac_dinh_nhiem_vu" };
  if (!("maCongTy" in tho.thamSo)) return { loai: "thieu_tham_so", tenTool: tho.tenTool };
  return { loai: "da_chon", loiGoi: tho };
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

type KetQuaGoiTool = KetQua<unknown, string>;

function thucThiTool(tenTool: string, thamSo: Record<string, unknown>): KetQuaGoiTool {
  if (tenTool === "tra_cuu_ho_so") return thucThiTraCuuHoSo(thamSo);
  if (tenTool === "tra_cuu_gia_co_phieu") return thucThiTraCuuGiaCoPhieu(thamSo);
  return { thanhCong: false, loi: `tool_chua_cai_dat:${tenTool}` };
}

function goiMcpTool(catalog: McpToolDefinition[], loiGoi: LoiGoiTool): KetQuaGoiTool {
  const dinhNghia = catalog.find((t) => t.name === loiGoi.tenTool);
  if (dinhNghia === undefined) return { thanhCong: false, loi: `tool_khong_ton_tai:${loiGoi.tenTool}` };
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, loiGoi.thamSo)) {
    return { thanhCong: false, loi: `tham_so_khong_hop_le:${loiGoi.tenTool}` };
  }
  return thucThiTool(loiGoi.tenTool, loiGoi.thamSo);
}
```

`chonToolCoCauTruc` KHÔNG bao giờ tự "đoán" mã công ty sai định dạng: nó
CHỈ đọc token bốn chữ hoa liên tiếp (`trichMaCongTy`), không hiểu nghĩa
tiếng Việt của câu — đúng tinh thần mô phỏng LLM tất định đã dùng suốt
`T9.1`-`T9.6`.
::::

::::example{#do_bon_chi_so_tren_bo_cau_hoi}
```typescript title=readonly
function traLoiCauHoi(catalog: McpToolDefinition[], cauHoi: string): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = goiMcpTool(catalog, chon.loiGoi);
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return { loai: "thanh_cong", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };
}

const CHI_PHI_THEO_TOOL: Record<string, number> = { tra_cuu_ho_so: 1, tra_cuu_gia_co_phieu: 2 };

function tinhChiPhi(ketQua: KetQuaTraLoi[]): number {
  return ketQua.reduce((tong, k) => {
    if (k.loai === "khong_xac_dinh_nhiem_vu") return tong;
    return tong + (CHI_PHI_THEO_TOOL[k.tenTool] ?? 0);
  }, 0);
}
function tinhSoBuoc(ketQua: KetQuaTraLoi[]): number {
  return ketQua.filter((k) => k.loai !== "khong_xac_dinh_nhiem_vu").length;
}
function tinhTyLeHoanThanh(ketQua: KetQuaTraLoi[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.loai === "thanh_cong").length / ketQua.length;
}

function doEndToEnd(catalog: McpToolDefinition[], cacCauHoi: string[]): BaoCaoDoEndToEnd {
  const ketQua = cacCauHoi.map((c) => traLoiCauHoi(catalog, c));
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

const BO_CAU_HOI_L1 = [
  "Gia co phieu cua ACME hien tai la bao nhieu",
  "Cho toi thong tin ho so cua GLOB",
  "Thoi tiet hom nay the nao",
  "Gia co phieu cua ZETA la bao nhieu",
];

const baoCao = doEndToEnd(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L1);
console.log(JSON.stringify(baoCao), demGoiToolThat.soLan);
```

```text title=readonly
{"tyLeHoanThanh":0.5,"tongChiPhi":5,"tongSoBuoc":3,"soLanChanAnToan":0} 3
```

Bốn câu: `2` hợp lệ (giá `ACME` → `42`, hồ sơ `GLOB`), `1` không xác định
nhiệm vụ (hỏi thời tiết — không khớp `gia`/`co phieu`/`ho so`/`thong tin`),
`1` hỏi giá `ZETA` — mã BỐN CHỮ HOA hợp lệ về HÌNH THỨC nhưng KHÔNG có
trong `CSDL_GIA`, nên `PROMPT` vẫn chọn được tool (`da_chon`), CHỈ thất
bại Ở tầng thực thi (`khong_tim_thay_cong_ty`). `tongSoBuoc = 3` (câu hỏi
thời tiết không tính, vì nó chưa từng chạm `goiMcpTool`); `tongChiPhi = 5`
(giá `2` + `1` + `2`, câu `ZETA` VẪN tính phí dù thất bại — nó đã THỬ gọi
tool); `tyLeHoanThanh = 2/4` (chỉ `2` câu thật sự có kết quả).
::::

::::predict{#doan_hai_ma_trong_cung_cau commitOnce}
Đổi câu hỏi cuối (`"Gia co phieu cua ZETA la bao nhieu"`) thành
`"Gia co phieu cua ACME va GLOB la bao nhieu"` — câu này chứa CẢ HAI mã
hợp lệ. `trichMaCongTy` dùng `/\b[A-Z]{4}\b/.exec(...)`, chỉ trả về MỘT
kết quả. `baoCao` mới có gì thay đổi so với bản gốc?

:::opt{correct}
`tyLeHoanThanh` tăng từ `0.5` lên `0.75` (`ACME` là mã KHỚP ĐẦU TIÊN mà
`.exec()` tìm thấy, tra được giá thật `42`, câu hỏi thành công) — nhưng
`tongChiPhi` GIỮ NGUYÊN LÀ `5`: câu này vẫn LÀ đúng MỘT lượt gọi
`tra_cuu_gia_co_phieu` (giá `2`), y hệt câu `ZETA` cũ, chỉ khác Ở việc
thành công hay không
:::
:::opt
`tongChiPhi` tăng lên `7`, vì giờ hệ thống phải "thử" tra cả `ACME` LẪN
`GLOB`
::why
Nhầm rằng `chonToolCoCauTruc` có cơ chế thử NHIỀU mã trong CÙNG một câu
hỏi — nhưng `trichMaCongTy` dùng `.exec()` của regex, CHỈ trả về kết quả
khớp ĐẦU TIÊN rồi dừng lại; `GLOB` không bao giờ được "nhìn" tới.

Chỗ lệch: `traLoiCauHoi` luôn gọi `goiMcpTool` ĐÚNG MỘT lần cho MỘT câu
hỏi (không có vòng lặp thử nhiều mã) — chi phí chỉ tính theo TÊN TOOL
được gọi, không theo số mã xuất hiện trong văn bản.
::
:::
:::opt
`tyLeHoanThanh` VẪN LÀ `0.5`, vì regex không khớp được khi có HAI mã
BỐN-CHỮ-HOA trong cùng một câu
::why
Gần đúng Ở việc bạn nghi ngờ hai mã "gây nhiễu" nhau — quan sát Ề rằng có
gì đó phức tạp hơn khi có nhiều mã là hợp lý để cân nhắc.

Chỗ lệch: `/\\b[A-Z]{4}\\b/` khớp BẤT KỲ token nào đúng bốn chữ hoa LIÊN
TIẾP, không quan tâm có mã KHÁC ở gần đó hay không — `"ACME va GLOB"` vẫn
khớp `"ACME"` bình thường Ở LẦN GỌI `.exec()` đầu tiên.
::
:::
::::

::::code{#viet_tra_loi_cau_hoi}
Hoàn thiện `traLoiCauHoi`: sau khi `chonToolCoCauTruc` trả về `da_chon`,
gọi `goiMcpTool` (PROTOCOL) rồi ráp kết quả THÀNH CÔNG.

```typescript title=starter
type KetQua<T, E> = { thanhCong: true; giaTri: T } | { thanhCong: false; loi: E };

type McpJsonSchema = { type: "object"; properties: Record<string, { type: "string" | "number" | "boolean" }>; required?: string[] };
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };

type HoSoCongTy = { maCongTy: string; nganh: string; namThanhLap: number };
const CSDL_HO_SO: Record<string, HoSoCongTy> = {
  ACME: { maCongTy: "ACME", nganh: "ban le", namThanhLap: 1998 },
  GLOB: { maCongTy: "GLOB", nganh: "cong nghe", namThanhLap: 2005 },
};
const CSDL_GIA: Record<string, number> = { ACME: 42, GLOB: 108 };

const demGoiToolThat = { soLan: 0 };

function thucThiTraCuuHoSo(thamSo: Record<string, unknown>): KetQua<HoSoCongTy, string> {
  demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const hoSo = CSDL_HO_SO[maCongTy];
  if (!hoSo) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: hoSo };
}
function thucThiTraCuuGiaCoPhieu(thamSo: Record<string, unknown>): KetQua<number, string> {
  demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const gia = CSDL_GIA[maCongTy];
  if (gia === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: gia };
}

const CATALOG_TOOL_NGHIEN_CUU: McpToolDefinition[] = [
  { name: "tra_cuu_ho_so", description: "Tra cuu ho so cong ty theo ma", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_gia_co_phieu", description: "Tra cuu gia co phieu hien tai theo ma cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
];

function trichMaCongTy(cauHoi: string): string | null {
  const khop = /\b[A-Z]{4}\b/.exec(cauHoi);
  return khop ? khop[0] : null;
}

type LoiGoiTool = { tenTool: string; thamSo: Record<string, unknown> };
type LoiGoiToolTho = LoiGoiTool | null;

function llmMoPhongChonTool(cauHoi: string): LoiGoiToolTho {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  const thamSo: Record<string, unknown> = maCongTy !== null ? { maCongTy } : {};
  if (c.includes("gia") || c.includes("co phieu")) return { tenTool: "tra_cuu_gia_co_phieu", thamSo };
  if (c.includes("ho so") || c.includes("thong tin")) return { tenTool: "tra_cuu_ho_so", thamSo };
  return null;
}

type KetQuaChonTool =
  | { loai: "da_chon"; loiGoi: LoiGoiTool }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string };

function chonToolCoCauTruc(cauHoi: string): KetQuaChonTool {
  const tho = llmMoPhongChonTool(cauHoi);
  if (tho === null) return { loai: "khong_xac_dinh_nhiem_vu" };
  if (!("maCongTy" in tho.thamSo)) return { loai: "thieu_tham_so", tenTool: tho.tenTool };
  return { loai: "da_chon", loiGoi: tho };
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

type KetQuaGoiTool = KetQua<unknown, string>;

function thucThiTool(tenTool: string, thamSo: Record<string, unknown>): KetQuaGoiTool {
  if (tenTool === "tra_cuu_ho_so") return thucThiTraCuuHoSo(thamSo);
  if (tenTool === "tra_cuu_gia_co_phieu") return thucThiTraCuuGiaCoPhieu(thamSo);
  return { thanhCong: false, loi: `tool_chua_cai_dat:${tenTool}` };
}

function goiMcpTool(catalog: McpToolDefinition[], loiGoi: LoiGoiTool): KetQuaGoiTool {
  const dinhNghia = catalog.find((t) => t.name === loiGoi.tenTool);
  if (dinhNghia === undefined) return { thanhCong: false, loi: `tool_khong_ton_tai:${loiGoi.tenTool}` };
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, loiGoi.thamSo)) {
    return { thanhCong: false, loi: `tham_so_khong_hop_le:${loiGoi.tenTool}` };
  }
  return thucThiTool(loiGoi.tenTool, loiGoi.thamSo);
}

type KetQuaTraLoi =
  | { loai: "thanh_cong"; tenTool: string; giaTri: unknown }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string }
  | { loai: "loi_thuc_thi"; tenTool: string; loi: string };

const CHI_PHI_THEO_TOOL: Record<string, number> = { tra_cuu_ho_so: 1, tra_cuu_gia_co_phieu: 2 };

function tinhChiPhi(ketQua: KetQuaTraLoi[]): number {
  return ketQua.reduce((tong, k) => {
    if (k.loai === "khong_xac_dinh_nhiem_vu") return tong;
    return tong + (CHI_PHI_THEO_TOOL[k.tenTool] ?? 0);
  }, 0);
}
function tinhSoBuoc(ketQua: KetQuaTraLoi[]): number {
  return ketQua.filter((k) => k.loai !== "khong_xac_dinh_nhiem_vu").length;
}
function tinhTyLeHoanThanh(ketQua: KetQuaTraLoi[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.loai === "thanh_cong").length / ketQua.length;
}

type BaoCaoDoEndToEnd = {
  tyLeHoanThanh: number;
  tongChiPhi: number;
  tongSoBuoc: number;
  soLanChanAnToan: number;
};

function traLoiCauHoi(catalog: McpToolDefinition[], cauHoi: string): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = ___;
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return ___;
}

function doEndToEnd(catalog: McpToolDefinition[], cacCauHoi: string[]): BaoCaoDoEndToEnd {
  const ketQua = cacCauHoi.map((c) => traLoiCauHoi(catalog, c));
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

const BO_CAU_HOI_L1 = [
  "Gia co phieu cua ACME hien tai la bao nhieu",
  "Cho toi thong tin ho so cua GLOB",
  "Thoi tiet hom nay the nao",
  "Gia co phieu cua ZETA la bao nhieu",
];

const baoCao = doEndToEnd(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L1);
console.log(JSON.stringify(baoCao), demGoiToolThat.soLan);
```

```typescript title=solution
type KetQua<T, E> = { thanhCong: true; giaTri: T } | { thanhCong: false; loi: E };

type McpJsonSchema = { type: "object"; properties: Record<string, { type: "string" | "number" | "boolean" }>; required?: string[] };
type McpToolDefinition = { name: string; description: string; inputSchema: McpJsonSchema };

type HoSoCongTy = { maCongTy: string; nganh: string; namThanhLap: number };
const CSDL_HO_SO: Record<string, HoSoCongTy> = {
  ACME: { maCongTy: "ACME", nganh: "ban le", namThanhLap: 1998 },
  GLOB: { maCongTy: "GLOB", nganh: "cong nghe", namThanhLap: 2005 },
};
const CSDL_GIA: Record<string, number> = { ACME: 42, GLOB: 108 };

const demGoiToolThat = { soLan: 0 };

function thucThiTraCuuHoSo(thamSo: Record<string, unknown>): KetQua<HoSoCongTy, string> {
  demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const hoSo = CSDL_HO_SO[maCongTy];
  if (!hoSo) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: hoSo };
}
function thucThiTraCuuGiaCoPhieu(thamSo: Record<string, unknown>): KetQua<number, string> {
  demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const gia = CSDL_GIA[maCongTy];
  if (gia === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: gia };
}

const CATALOG_TOOL_NGHIEN_CUU: McpToolDefinition[] = [
  { name: "tra_cuu_ho_so", description: "Tra cuu ho so cong ty theo ma", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_gia_co_phieu", description: "Tra cuu gia co phieu hien tai theo ma cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
];

function trichMaCongTy(cauHoi: string): string | null {
  const khop = /\b[A-Z]{4}\b/.exec(cauHoi);
  return khop ? khop[0] : null;
}

type LoiGoiTool = { tenTool: string; thamSo: Record<string, unknown> };
type LoiGoiToolTho = LoiGoiTool | null;

function llmMoPhongChonTool(cauHoi: string): LoiGoiToolTho {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  const thamSo: Record<string, unknown> = maCongTy !== null ? { maCongTy } : {};
  if (c.includes("gia") || c.includes("co phieu")) return { tenTool: "tra_cuu_gia_co_phieu", thamSo };
  if (c.includes("ho so") || c.includes("thong tin")) return { tenTool: "tra_cuu_ho_so", thamSo };
  return null;
}

type KetQuaChonTool =
  | { loai: "da_chon"; loiGoi: LoiGoiTool }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string };

function chonToolCoCauTruc(cauHoi: string): KetQuaChonTool {
  const tho = llmMoPhongChonTool(cauHoi);
  if (tho === null) return { loai: "khong_xac_dinh_nhiem_vu" };
  if (!("maCongTy" in tho.thamSo)) return { loai: "thieu_tham_so", tenTool: tho.tenTool };
  return { loai: "da_chon", loiGoi: tho };
}

function kiemTraThamSoTheoSchema(schema: McpJsonSchema, thamSo: Record<string, unknown>): boolean {
  for (const truong of schema.required ?? []) {
    if (!(truong in thamSo)) return false;
  }
  for (const ten of Object.keys(schema.properties)) {
    const dinhNghia = schema.properties[ten];
    if (dinhNghia === undefined) continue;
    if (ten in thamSo && typeof thamSo[ten] !== dinhNghia.type) return false;
  }
  return true;
}

type KetQuaGoiTool = KetQua<unknown, string>;

function thucThiTool(tenTool: string, thamSo: Record<string, unknown>): KetQuaGoiTool {
  if (tenTool === "tra_cuu_ho_so") return thucThiTraCuuHoSo(thamSo);
  if (tenTool === "tra_cuu_gia_co_phieu") return thucThiTraCuuGiaCoPhieu(thamSo);
  return { thanhCong: false, loi: `tool_chua_cai_dat:${tenTool}` };
}

function goiMcpTool(catalog: McpToolDefinition[], loiGoi: LoiGoiTool): KetQuaGoiTool {
  const dinhNghia = catalog.find((t) => t.name === loiGoi.tenTool);
  if (dinhNghia === undefined) return { thanhCong: false, loi: `tool_khong_ton_tai:${loiGoi.tenTool}` };
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, loiGoi.thamSo)) {
    return { thanhCong: false, loi: `tham_so_khong_hop_le:${loiGoi.tenTool}` };
  }
  return thucThiTool(loiGoi.tenTool, loiGoi.thamSo);
}

type KetQuaTraLoi =
  | { loai: "thanh_cong"; tenTool: string; giaTri: unknown }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string }
  | { loai: "loi_thuc_thi"; tenTool: string; loi: string };

const CHI_PHI_THEO_TOOL: Record<string, number> = { tra_cuu_ho_so: 1, tra_cuu_gia_co_phieu: 2 };

function tinhChiPhi(ketQua: KetQuaTraLoi[]): number {
  return ketQua.reduce((tong, k) => {
    if (k.loai === "khong_xac_dinh_nhiem_vu") return tong;
    return tong + (CHI_PHI_THEO_TOOL[k.tenTool] ?? 0);
  }, 0);
}
function tinhSoBuoc(ketQua: KetQuaTraLoi[]): number {
  return ketQua.filter((k) => k.loai !== "khong_xac_dinh_nhiem_vu").length;
}
function tinhTyLeHoanThanh(ketQua: KetQuaTraLoi[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.loai === "thanh_cong").length / ketQua.length;
}

type BaoCaoDoEndToEnd = {
  tyLeHoanThanh: number;
  tongChiPhi: number;
  tongSoBuoc: number;
  soLanChanAnToan: number;
};

function traLoiCauHoi(catalog: McpToolDefinition[], cauHoi: string): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = goiMcpTool(catalog, chon.loiGoi);
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return { loai: "thanh_cong", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };
}

function doEndToEnd(catalog: McpToolDefinition[], cacCauHoi: string[]): BaoCaoDoEndToEnd {
  const ketQua = cacCauHoi.map((c) => traLoiCauHoi(catalog, c));
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

const BO_CAU_HOI_L1 = [
  "Gia co phieu cua ACME hien tai la bao nhieu",
  "Cho toi thong tin ho so cua GLOB",
  "Thoi tiet hom nay the nao",
  "Gia co phieu cua ZETA la bao nhieu",
];

const baoCao = doEndToEnd(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L1);
console.log(JSON.stringify(baoCao), demGoiToolThat.soLan);
```

```typescript title=test
if (baoCao.tyLeHoanThanh !== 0.5) throw new Error("tyLeHoanThanh tren BO_CAU_HOI_L1 phai la 0.5 (2/4)");
if (baoCao.tongChiPhi !== 5) throw new Error("tongChiPhi phai la 5 (2 + 1 + 2, ca cau ZETA that bai van tinh phi)");
if (baoCao.tongSoBuoc !== 3) throw new Error("tongSoBuoc phai la 3 -- cau hoi thoi tiet khong duoc tinh, no chua tung cham goiMcpTool");
if (baoCao.soLanChanAnToan !== 0) throw new Error("soLanChanAnToan phai la 0 -- bai nay chua co moi de doa nao");
if (demGoiToolThat.soLan !== 3) throw new Error("demGoiToolThat.soLan phai la 3 -- dung so lan goiMcpTool that su goi toi tool");

const q1 = traLoiCauHoi(CATALOG_TOOL_NGHIEN_CUU, "Gia co phieu cua ACME hien tai la bao nhieu");
if (q1.loai !== "thanh_cong" || q1.giaTri !== 42) throw new Error("cau hoi gia ACME phai thanh_cong voi giaTri 42");

const q3 = traLoiCauHoi(CATALOG_TOOL_NGHIEN_CUU, "Thoi tiet hom nay the nao");
if (JSON.stringify(q3) !== JSON.stringify({ loai: "khong_xac_dinh_nhiem_vu" })) {
  throw new Error("cau hoi khong khop tool nao phai tra ve DUNG { loai: 'khong_xac_dinh_nhiem_vu' }, khong duoc goi tool");
}

const q4 = traLoiCauHoi(CATALOG_TOOL_NGHIEN_CUU, "Gia co phieu cua ZETA la bao nhieu");
if (q4.loai !== "loi_thuc_thi" || q4.loi !== "khong_tim_thay_cong_ty") {
  throw new Error("cau hoi gia ZETA (ma hop le nhung khong co trong CSDL_GIA) phai loi_thuc_thi voi loi khong_tim_thay_cong_ty");
}
```

:::hints
- kind: attention
  body: "Hai cho trong, cung trong traLoiCauHoi, SAU dong 'if (chon.loai !== \"da_chon\") return chon;' (nghia la chon.loai chac chan la 'da_chon' o day, chon.loiGoi ton tai). Cho dau: goi PROTOCOL that su -- goiMcpTool(catalog, chon.loiGoi). Cho hai: rap KetQuaTraLoi thanh cong tu ketQua.giaTri VA chon.loiGoi.tenTool."
- kind: strategy
  body: "Cho dau: const ketQua = goiMcpTool(catalog, chon.loiGoi); Cho hai: return { loai: \"thanh_cong\", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };"
- kind: one-line
  body: 'Cho dau la goiMcpTool(catalog, chon.loiGoi), cho hai la { loai: "thanh_cong", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri }.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "{\"tyLeHoanThanh\":0.5,\"tongChiPhi\":5,\"tongSoBuoc\":3,\"soLanChanAnToan\":0} 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0.5` hoàn thành, chi phí `5`, `3` bước thật, `0` lần chặn an toàn — bốn
con số này sẽ ĐI THEO suốt `T9.7`. Bài sau thêm CONTEXT: khi trợ lý cần
NHIỀU lượt hỏi trong CÙNG một phiên, lịch sử phải nằm trong một ngân sách
token, không phình vô hạn.
::::

::::reflect{#nghi-lai}
`traLoiCauHoi` không phát minh gì mới — nó RÁP hai lớp đã học: PROMPT
(đầu ra tool-call có cấu trúc, phân biệt RÕ ba kết cục thay vì một
boolean "hiểu hay không hiểu") và PROTOCOL (schema validate đứng gác
TRƯỚC KHI chạm tool thật, y hệt MCP Ở `T9.6`). Bốn chỉ số đo được
(`tyLeHoanThanh=0.5`, `tongChiPhi=5`, `tongSoBuoc=3`, `soLanChanAnToan=0`)
không phải bốn con số rời rạc — chúng LÀ định nghĩa CHÍNH THỨC của "đo
end-to-end" mà `T9.7` sẽ dùng lại, KHÔNG đổi tên, cho tới bài BOSS cuối
cùng đóng cả Realm 9 và toàn bộ MASTERPLAN.
::::

::::checkpoint{mastery=0.85}
::::
