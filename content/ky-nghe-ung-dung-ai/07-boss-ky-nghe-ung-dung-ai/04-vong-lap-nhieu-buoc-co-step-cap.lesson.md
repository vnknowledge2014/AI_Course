---
id: ky-nghe-ung-dung-ai.boss-ky-nghe-ung-dung-ai.vong-lap-nhieu-buoc-co-step-cap
title: "T9.7 bài 4 — vòng lặp nhiều bước, step cap tường minh (hetBuoc khác loi khác thanhCong)"
summary: "chayVongLapTraLoi(ngu, catalog, cauHoi, soBuocToiDa, soLanThuLaiToiDa): KetQuaVongLap dùng quyetDinhBuocTiepTheo để QUYẾT ĐỊNH runtime tool nào còn thiếu (ho_so/gia), gọi qua goiMcpToolCoRetryVaFallback (bài 3) mỗi bước, dừng ở du_thong_tin (thanhCong) hoặc hetBuoc (chạm soBuocToiDa) hoặc loi (dữ liệu không tồn tại/nhiệm vụ không rõ) — ba trạng thái TÁCH BIỆT, không trạng thái nào thay thế được trạng thái khác. Trên 5 kịch bản hỗn hợp (câu hỏi ghép cần 2 tool, cùng câu với step cap=1, câu đơn cần 1 tool, câu hỏi mã không tồn tại, câu không xác định nhiệm vụ): tyLeHoanThanh=0.4 (2/5), tyLeChamCap=0.2 (1/5). Điểm mấu chốt: câu ghép cần soBuocToiDa=3 (không phải 2) để thành công dù chỉ tốn ĐÚNG 2 lượt gọi tool thật — vòng lặp cần MỘT bước rảnh để XÁC NHẬN đã đủ thông tin."
locale: vi
track: ky-nghe-ung-dung-ai
module: boss-ky-nghe-ung-dung-ai
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kna.vong-lap-nhieu-buoc-co-step-cap]
requires: [kna.harness-retry-fallback-quanh-loi-mcp]
concepts: [kna.vong-lap-nhieu-buoc-co-step-cap]
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
Ba bài trước: MỘT câu hỏi luôn cần ĐÚNG một lượt gọi tool. Câu hỏi thật
đôi khi cần NHIỀU thứ — "cho tôi cả hồ sơ LẪN giá cổ phiếu". Bài này ráp
LOOP (`q9.4a`/`q9.4b`): agent tự QUYẾT ĐỊNH bước tiếp theo dựa trên những
gì ĐÃ thu thập, với một step cap tường minh.
::::

::::explain{#quyet_dinh_runtime_va_ba_trang_thai}
`quyetDinhBuocTiepTheo` không có danh sách bước CỐ ĐỊNH — nó ĐỌC câu hỏi
VÀ những gì `ketQuaThuThap` ĐÃ có, rồi quyết định: còn thiếu hồ sơ thì
gọi `tra_cuu_ho_so`; còn thiếu giá thì gọi `tra_cuu_gia_co_phieu`; đủ cả
hai (hoặc không cần cái nào nữa) thì báo `"du_thong_tin"`:

```typescript title=readonly
type QuyetDinhBuoc =
  | { loai: "goi_tool"; tenTool: string; thamSo: Record<string, unknown> }
  | { loai: "du_thong_tin" }
  | { loai: "khong_xac_dinh" };

function quyetDinhBuocTiepTheo(cauHoi: string, ketQuaThuThap: Record<string, unknown>): QuyetDinhBuoc {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  if (maCongTy === null) return { loai: "khong_xac_dinh" };
  const canHoSo = c.includes("ho so");
  const canGia = c.includes("gia") || c.includes("co phieu");
  if (!canHoSo && !canGia) return { loai: "khong_xac_dinh" };
  if (canHoSo && !("ho_so" in ketQuaThuThap)) {
    return { loai: "goi_tool", tenTool: "tra_cuu_ho_so", thamSo: { maCongTy } };
  }
  if (canGia && !("gia" in ketQuaThuThap)) {
    return { loai: "goi_tool", tenTool: "tra_cuu_gia_co_phieu", thamSo: { maCongTy } };
  }
  return { loai: "du_thong_tin" };
}

function nhanCuaTool(tenTool: string): string {
  return tenTool === "tra_cuu_ho_so" ? "ho_so" : "gia";
}

type KetQuaVongLap =
  | { trangThai: "thanhCong"; ketQuaThuThap: Record<string, unknown>; soBuocDaDung: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number };

function chayVongLapTraLoi(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  cauHoi: string,
  soBuocToiDa: number,
  soLanThuLaiToiDa: number,
): KetQuaVongLap {
  let ketQuaThuThap: Record<string, unknown> = {};
  let buoc = 0;
  while (buoc < soBuocToiDa) {
    const quyetDinh = quyetDinhBuocTiepTheo(cauHoi, ketQuaThuThap);
    if (quyetDinh.loai === "khong_xac_dinh") return { trangThai: "loi", loi: "khong_xac_dinh_nhiem_vu", soBuocDaDung: buoc };
    if (quyetDinh.loai === "du_thong_tin") return { trangThai: "thanhCong", ketQuaThuThap, soBuocDaDung: buoc };
    buoc++;
    const loiGoi: LoiGoiTool = { tenTool: quyetDinh.tenTool, thamSo: quyetDinh.thamSo };
    const ketQuaGoi = goiMcpToolCoRetryVaFallback(ngu, catalog, loiGoi, soLanThuLaiToiDa);
    if (!ketQuaGoi.thanhCong) return { trangThai: "loi", loi: ketQuaGoi.loi, soBuocDaDung: buoc };
    ketQuaThuThap = { ...ketQuaThuThap, [nhanCuaTool(quyetDinh.tenTool)]: ketQuaGoi.giaTri };
  }
  return { trangThai: "hetBuoc", soBuocDaDung: buoc };
}
```

BA trạng thái, KHÔNG trạng thái nào thay thế được trạng thái khác:
`"thanhCong"` (đã đủ thông tin), `"loi"` (một lượt gọi tool thất bại VĨNH
VIỄN — dữ liệu không tồn tại, hoặc câu hỏi không xác định được nhiệm vụ
ngay từ đầu), `"hetBuoc"` (chạm `soBuocToiDa` mà VẪN chưa tới
`"du_thong_tin"`). Để Ý: kiểm tra `buoc < soBuocToiDa` đứng Ở ĐẦU vòng
`while` — CẢ bước gọi tool LẪN bước "chỉ để xác nhận đã đủ" đều PHẢI
nằm trong ngân sách đó.
::::

::::example{#nam_kich_ban_hon_hop}
```typescript title=readonly
type KichBanL4 = { cauHoi: string; soBuocToiDa: number };
const BO_KICH_BAN_L4: KichBanL4[] = [
  { cauHoi: "Cho toi ho so va gia co phieu cua ACME", soBuocToiDa: 3 },
  { cauHoi: "Cho toi ho so va gia co phieu cua ACME", soBuocToiDa: 1 },
  { cauHoi: "Gia co phieu cua GLOB la bao nhieu", soBuocToiDa: 3 },
  { cauHoi: "Cho toi ho so va gia co phieu cua ZETA", soBuocToiDa: 3 },
  { cauHoi: "Thoi tiet hom nay the nao", soBuocToiDa: 3 },
];

const ketQuaL4 = BO_KICH_BAN_L4.map((kb) => chayVongLapTraLoi(taoNguCanhThucThi(), CATALOG_TOOL_NGHIEN_CUU, kb.cauHoi, kb.soBuocToiDa, 1));
console.log(JSON.stringify(ketQuaL4.map((k) => k.trangThai)));
console.log(JSON.stringify(ketQuaL4.map((k) => k.soBuocDaDung)));
```

```text title=readonly
["thanhCong","hetBuoc","thanhCong","loi","loi"]
[2,1,1,1,0]
```

Kịch bản `1` (câu ghép, `soBuocToiDa=3`): THÀNH CÔNG sau ĐÚNG `2` lượt
gọi tool thật (`ho_so` rồi `gia`) — nhưng cần cap `3`, không phải `2`, vì
vòng LẶP THỨ BA (không tốn lượt gọi tool nào) là lượt DUY NHẤT xác nhận
`"du_thong_tin"`. Kịch bản `2` (CÙNG câu, `soBuocToiDa=1`): CHỈ đủ ngân
sách cho MỘT lượt gọi tool (`ho_so`) — vòng lặp dừng Ở `"hetBuoc"`, VẪN
`soBuocDaDung=1`, dù đã THU được một phần thông tin. Kịch bản `4` (mã
`ZETA` không tồn tại): thất bại NGAY Ở lượt ĐẦU (`"loi"`,
`khong_tim_thay_cong_ty`). Kịch bản `5` (không khớp tool nào): thất bại
Ở `soBuocDaDung=0` — CHƯA từng chạm một tool nào.
::::

::::predict{#doan_giam_cap_xuong_hai commitOnce}
Kịch bản `1` (câu ghép) đang dùng `soBuocToiDa=3` để THÀNH CÔNG với
`soBuocDaDung=2`. Nếu đổi `soBuocToiDa` của CHÍNH kịch bản này xuống `2`
(giữ nguyên câu hỏi) — `trangThai` và `soBuocDaDung` đổi ra sao?

:::opt{correct}
`trangThai` đổi thành `"hetBuoc"`, NHƯNG `soBuocDaDung` VẪN LÀ `2` (giống
hệt bản gốc) — cả hai lượt gọi tool thật (`ho_so` rồi `gia`) VẪN chạy
xong bình thường; chỉ có vòng `while` KHÔNG còn đủ chỗ (`2 < 2` sai) để
gọi `quyetDinhBuocTiepTheo` LẦN THỨ BA — lần đáng lẽ sẽ trả về
`"du_thong_tin"` — nên vòng lặp thoát Ở `"hetBuoc"` trước khi kịp xác
nhận thành công
:::
:::opt
`trangThai` VẪN LÀ `"thanhCong"`, vì cả hai lượt gọi tool cần thiết đều
đã hoàn tất Ở bước `2`
::why
Nhầm rằng "đã thu đủ dữ liệu" đồng nghĩa "đã được xác nhận thành công" —
nhưng `chayVongLapTraLoi` CHỈ trả `"thanhCong"` khi `quyetDinhBuocTiepTheo`
THẬT SỰ trả về `"du_thong_tin"` Ở một VÒNG LẶP kế tiếp; nó không tự suy
luận "chắc đã xong" từ số lượt đã chạy.

Chỗ lệch: vòng lặp CẦN một lượt kiểm tra THỨ BA (không gọi tool) để xác
nhận — với `soBuocToiDa=2`, điều kiện `while (buoc < 2)` chặn lượt kiểm
tra đó lại NGAY SAU KHI `buoc` chạm `2`.
::
:::
:::opt
`soBuocDaDung` giảm xuống `1`, vì ngân sách nhỏ hơn buộc vòng lặp bỏ bớt
một lượt gọi tool
::why
Gần đúng Ở việc bạn cho rằng ngân sách nhỏ hơn LÀM ÍT việc hơn — trực
giác đó ĐÚNG trong nhiều tình huống khác (vd kịch bản `2` Ở ví dụ trên,
`soBuocToiDa=1`).

Chỗ lệch: Ở ĐÂY, `soBuocToiDa=2` VẪN đủ chỗ cho ĐÚNG hai lượt gọi tool
(`buoc` tăng dần `0→1→2`, cả hai lần `buoc < 2` đều đúng TRƯỚC khi tăng)
— chỉ THIẾU đúng MỘT lượt kiểm tra rảnh Ở cuối, không thiếu lượt gọi tool
nào.
::
:::
::::

::::code{#viet_vong_lap_tra_loi}
Hoàn thiện `chayVongLapTraLoi`: MỖI vòng lặp, hỏi `quyetDinhBuocTiepTheo`;
xử lý `"khong_xac_dinh"` và `"du_thong_tin"` ngay; nếu cần gọi tool, tăng
`buoc`, gọi qua `goiMcpToolCoRetryVaFallback` (bài `3`), rồi gộp kết quả
vào `ketQuaThuThap`.

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
const CSDL_TIN_TUC: Record<string, string> = {
  ACME: "ACME mo rong thi truong sang khu vuc mien trung",
  GLOB: "GLOB ra mat dong san pham cong nghe moi",
};

type DemGoi = { soLan: number };
type TrangThaiTinTuc = { soLanDaGoi: number };
type NguCanhThucThi = { demGoiToolThat: DemGoi; trangThaiTinTuc: TrangThaiTinTuc };

function taoNguCanhThucThi(): NguCanhThucThi {
  return { demGoiToolThat: { soLan: 0 }, trangThaiTinTuc: { soLanDaGoi: 0 } };
}

function thucThiTraCuuHoSo(ngu: NguCanhThucThi, thamSo: Record<string, unknown>): KetQua<HoSoCongTy, string> {
  ngu.demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const hoSo = CSDL_HO_SO[maCongTy];
  if (!hoSo) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: hoSo };
}
function thucThiTraCuuGiaCoPhieu(ngu: NguCanhThucThi, thamSo: Record<string, unknown>): KetQua<number, string> {
  ngu.demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const gia = CSDL_GIA[maCongTy];
  if (gia === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: gia };
}
function thucThiTraCuuTinTuc(ngu: NguCanhThucThi, thamSo: Record<string, unknown>): KetQua<{ tieuDe: string }, string> {
  ngu.demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  ngu.trangThaiTinTuc.soLanDaGoi++;
  if (ngu.trangThaiTinTuc.soLanDaGoi <= 2) return { thanhCong: false, loi: "loi_tam_thoi_ket_noi" };
  const tieuDe = CSDL_TIN_TUC[maCongTy];
  if (tieuDe === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: { tieuDe } };
}

const CATALOG_TOOL_NGHIEN_CUU: McpToolDefinition[] = [
  { name: "tra_cuu_ho_so", description: "Tra cuu ho so cong ty theo ma", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_gia_co_phieu", description: "Tra cuu gia co phieu hien tai theo ma cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_tin_tuc", description: "Tra cuu tin tuc gan day ve mot cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
];

function trichMaCongTy(cauHoi: string): string | null {
  const khop = /\b[A-Z]{4}\b/.exec(cauHoi);
  return khop ? khop[0] : null;
}

type LoiGoiTool = { tenTool: string; thamSo: Record<string, unknown> };

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

function thucThiTool(ngu: NguCanhThucThi, tenTool: string, thamSo: Record<string, unknown>): KetQuaGoiTool {
  if (tenTool === "tra_cuu_ho_so") return thucThiTraCuuHoSo(ngu, thamSo);
  if (tenTool === "tra_cuu_gia_co_phieu") return thucThiTraCuuGiaCoPhieu(ngu, thamSo);
  if (tenTool === "tra_cuu_tin_tuc") return thucThiTraCuuTinTuc(ngu, thamSo);
  return { thanhCong: false, loi: `tool_chua_cai_dat:${tenTool}` };
}

function goiMcpTool(ngu: NguCanhThucThi, catalog: McpToolDefinition[], loiGoi: LoiGoiTool): KetQuaGoiTool {
  const dinhNghia = catalog.find((t) => t.name === loiGoi.tenTool);
  if (dinhNghia === undefined) return { thanhCong: false, loi: `tool_khong_ton_tai:${loiGoi.tenTool}` };
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, loiGoi.thamSo)) {
    return { thanhCong: false, loi: `tham_so_khong_hop_le:${loiGoi.tenTool}` };
  }
  return thucThiTool(ngu, loiGoi.tenTool, loiGoi.thamSo);
}

function laLoiTamThoi(loi: string): boolean {
  return loi === "loi_tam_thoi_ket_noi";
}

function goiMcpToolCoRetry(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  loiGoi: LoiGoiTool,
  soLanThuLaiToiDa: number,
): KetQuaGoiTool {
  let ketQua = goiMcpTool(ngu, catalog, loiGoi);
  let soLanDaThu = 0;
  while (!ketQua.thanhCong && laLoiTamThoi(ketQua.loi) && soLanDaThu < soLanThuLaiToiDa) {
    ketQua = goiMcpTool(ngu, catalog, loiGoi);
    soLanDaThu++;
  }
  return ketQua;
}

const GIA_TRI_DU_PHONG_TIN_TUC = { tieuDe: "chua co tin tuc moi, thu lai sau" };

function goiMcpToolCoRetryVaFallback(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  loiGoi: LoiGoiTool,
  soLanThuLaiToiDa: number,
): KetQuaGoiTool {
  const ketQua = goiMcpToolCoRetry(ngu, catalog, loiGoi, soLanThuLaiToiDa);
  if (ketQua.thanhCong) return ketQua;
  if (laLoiTamThoi(ketQua.loi)) return { thanhCong: true, giaTri: GIA_TRI_DU_PHONG_TIN_TUC };
  return ketQua;
}

type QuyetDinhBuoc =
  | { loai: "goi_tool"; tenTool: string; thamSo: Record<string, unknown> }
  | { loai: "du_thong_tin" }
  | { loai: "khong_xac_dinh" };

function quyetDinhBuocTiepTheo(cauHoi: string, ketQuaThuThap: Record<string, unknown>): QuyetDinhBuoc {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  if (maCongTy === null) return { loai: "khong_xac_dinh" };
  const canHoSo = c.includes("ho so");
  const canGia = c.includes("gia") || c.includes("co phieu");
  if (!canHoSo && !canGia) return { loai: "khong_xac_dinh" };
  if (canHoSo && !("ho_so" in ketQuaThuThap)) {
    return { loai: "goi_tool", tenTool: "tra_cuu_ho_so", thamSo: { maCongTy } };
  }
  if (canGia && !("gia" in ketQuaThuThap)) {
    return { loai: "goi_tool", tenTool: "tra_cuu_gia_co_phieu", thamSo: { maCongTy } };
  }
  return { loai: "du_thong_tin" };
}

function nhanCuaTool(tenTool: string): string {
  return tenTool === "tra_cuu_ho_so" ? "ho_so" : "gia";
}

type KetQuaVongLap =
  | { trangThai: "thanhCong"; ketQuaThuThap: Record<string, unknown>; soBuocDaDung: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number };

function chayVongLapTraLoi(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  cauHoi: string,
  soBuocToiDa: number,
  soLanThuLaiToiDa: number,
): KetQuaVongLap {
  let ketQuaThuThap: Record<string, unknown> = {};
  let buoc = 0;
  while (buoc < soBuocToiDa) {
    const quyetDinh = quyetDinhBuocTiepTheo(cauHoi, ketQuaThuThap);
    if (quyetDinh.loai === "khong_xac_dinh") return { trangThai: "loi", loi: "khong_xac_dinh_nhiem_vu", soBuocDaDung: buoc };
    if (quyetDinh.loai === "du_thong_tin") return { trangThai: "thanhCong", ketQuaThuThap, soBuocDaDung: buoc };
    ___
    const loiGoi: LoiGoiTool = { tenTool: quyetDinh.tenTool, thamSo: quyetDinh.thamSo };
    const ketQuaGoi = goiMcpToolCoRetryVaFallback(ngu, catalog, loiGoi, soLanThuLaiToiDa);
    if (!ketQuaGoi.thanhCong) return { trangThai: "loi", loi: ketQuaGoi.loi, soBuocDaDung: buoc };
    ___
  }
  return { trangThai: "hetBuoc", soBuocDaDung: buoc };
}

type KichBanL4 = { cauHoi: string; soBuocToiDa: number };
const BO_KICH_BAN_L4: KichBanL4[] = [
  { cauHoi: "Cho toi ho so va gia co phieu cua ACME", soBuocToiDa: 3 },
  { cauHoi: "Cho toi ho so va gia co phieu cua ACME", soBuocToiDa: 1 },
  { cauHoi: "Gia co phieu cua GLOB la bao nhieu", soBuocToiDa: 3 },
  { cauHoi: "Cho toi ho so va gia co phieu cua ZETA", soBuocToiDa: 3 },
  { cauHoi: "Thoi tiet hom nay the nao", soBuocToiDa: 3 },
];

function tinhTyLeHoanThanhVongLap(ds: KetQuaVongLap[]): number {
  if (ds.length === 0) return 0;
  return ds.filter((k) => k.trangThai === "thanhCong").length / ds.length;
}
function tinhTyLeChamCapVongLap(ds: KetQuaVongLap[]): number {
  if (ds.length === 0) return 0;
  return ds.filter((k) => k.trangThai === "hetBuoc").length / ds.length;
}

const ketQuaL4 = BO_KICH_BAN_L4.map((kb) => chayVongLapTraLoi(taoNguCanhThucThi(), CATALOG_TOOL_NGHIEN_CUU, kb.cauHoi, kb.soBuocToiDa, 1));
console.log(JSON.stringify(ketQuaL4.map((k) => k.trangThai)), tinhTyLeHoanThanhVongLap(ketQuaL4), tinhTyLeChamCapVongLap(ketQuaL4));
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
const CSDL_TIN_TUC: Record<string, string> = {
  ACME: "ACME mo rong thi truong sang khu vuc mien trung",
  GLOB: "GLOB ra mat dong san pham cong nghe moi",
};

type DemGoi = { soLan: number };
type TrangThaiTinTuc = { soLanDaGoi: number };
type NguCanhThucThi = { demGoiToolThat: DemGoi; trangThaiTinTuc: TrangThaiTinTuc };

function taoNguCanhThucThi(): NguCanhThucThi {
  return { demGoiToolThat: { soLan: 0 }, trangThaiTinTuc: { soLanDaGoi: 0 } };
}

function thucThiTraCuuHoSo(ngu: NguCanhThucThi, thamSo: Record<string, unknown>): KetQua<HoSoCongTy, string> {
  ngu.demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const hoSo = CSDL_HO_SO[maCongTy];
  if (!hoSo) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: hoSo };
}
function thucThiTraCuuGiaCoPhieu(ngu: NguCanhThucThi, thamSo: Record<string, unknown>): KetQua<number, string> {
  ngu.demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  const gia = CSDL_GIA[maCongTy];
  if (gia === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: gia };
}
function thucThiTraCuuTinTuc(ngu: NguCanhThucThi, thamSo: Record<string, unknown>): KetQua<{ tieuDe: string }, string> {
  ngu.demGoiToolThat.soLan++;
  const maCongTy = thamSo["maCongTy"];
  if (typeof maCongTy !== "string") return { thanhCong: false, loi: "thieu_ma_cong_ty" };
  ngu.trangThaiTinTuc.soLanDaGoi++;
  if (ngu.trangThaiTinTuc.soLanDaGoi <= 2) return { thanhCong: false, loi: "loi_tam_thoi_ket_noi" };
  const tieuDe = CSDL_TIN_TUC[maCongTy];
  if (tieuDe === undefined) return { thanhCong: false, loi: "khong_tim_thay_cong_ty" };
  return { thanhCong: true, giaTri: { tieuDe } };
}

const CATALOG_TOOL_NGHIEN_CUU: McpToolDefinition[] = [
  { name: "tra_cuu_ho_so", description: "Tra cuu ho so cong ty theo ma", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_gia_co_phieu", description: "Tra cuu gia co phieu hien tai theo ma cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
  { name: "tra_cuu_tin_tuc", description: "Tra cuu tin tuc gan day ve mot cong ty", inputSchema: { type: "object", properties: { maCongTy: { type: "string" } }, required: ["maCongTy"] } },
];

function trichMaCongTy(cauHoi: string): string | null {
  const khop = /\b[A-Z]{4}\b/.exec(cauHoi);
  return khop ? khop[0] : null;
}

type LoiGoiTool = { tenTool: string; thamSo: Record<string, unknown> };

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

function thucThiTool(ngu: NguCanhThucThi, tenTool: string, thamSo: Record<string, unknown>): KetQuaGoiTool {
  if (tenTool === "tra_cuu_ho_so") return thucThiTraCuuHoSo(ngu, thamSo);
  if (tenTool === "tra_cuu_gia_co_phieu") return thucThiTraCuuGiaCoPhieu(ngu, thamSo);
  if (tenTool === "tra_cuu_tin_tuc") return thucThiTraCuuTinTuc(ngu, thamSo);
  return { thanhCong: false, loi: `tool_chua_cai_dat:${tenTool}` };
}

function goiMcpTool(ngu: NguCanhThucThi, catalog: McpToolDefinition[], loiGoi: LoiGoiTool): KetQuaGoiTool {
  const dinhNghia = catalog.find((t) => t.name === loiGoi.tenTool);
  if (dinhNghia === undefined) return { thanhCong: false, loi: `tool_khong_ton_tai:${loiGoi.tenTool}` };
  if (!kiemTraThamSoTheoSchema(dinhNghia.inputSchema, loiGoi.thamSo)) {
    return { thanhCong: false, loi: `tham_so_khong_hop_le:${loiGoi.tenTool}` };
  }
  return thucThiTool(ngu, loiGoi.tenTool, loiGoi.thamSo);
}

function laLoiTamThoi(loi: string): boolean {
  return loi === "loi_tam_thoi_ket_noi";
}

function goiMcpToolCoRetry(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  loiGoi: LoiGoiTool,
  soLanThuLaiToiDa: number,
): KetQuaGoiTool {
  let ketQua = goiMcpTool(ngu, catalog, loiGoi);
  let soLanDaThu = 0;
  while (!ketQua.thanhCong && laLoiTamThoi(ketQua.loi) && soLanDaThu < soLanThuLaiToiDa) {
    ketQua = goiMcpTool(ngu, catalog, loiGoi);
    soLanDaThu++;
  }
  return ketQua;
}

const GIA_TRI_DU_PHONG_TIN_TUC = { tieuDe: "chua co tin tuc moi, thu lai sau" };

function goiMcpToolCoRetryVaFallback(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  loiGoi: LoiGoiTool,
  soLanThuLaiToiDa: number,
): KetQuaGoiTool {
  const ketQua = goiMcpToolCoRetry(ngu, catalog, loiGoi, soLanThuLaiToiDa);
  if (ketQua.thanhCong) return ketQua;
  if (laLoiTamThoi(ketQua.loi)) return { thanhCong: true, giaTri: GIA_TRI_DU_PHONG_TIN_TUC };
  return ketQua;
}

type QuyetDinhBuoc =
  | { loai: "goi_tool"; tenTool: string; thamSo: Record<string, unknown> }
  | { loai: "du_thong_tin" }
  | { loai: "khong_xac_dinh" };

function quyetDinhBuocTiepTheo(cauHoi: string, ketQuaThuThap: Record<string, unknown>): QuyetDinhBuoc {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  if (maCongTy === null) return { loai: "khong_xac_dinh" };
  const canHoSo = c.includes("ho so");
  const canGia = c.includes("gia") || c.includes("co phieu");
  if (!canHoSo && !canGia) return { loai: "khong_xac_dinh" };
  if (canHoSo && !("ho_so" in ketQuaThuThap)) {
    return { loai: "goi_tool", tenTool: "tra_cuu_ho_so", thamSo: { maCongTy } };
  }
  if (canGia && !("gia" in ketQuaThuThap)) {
    return { loai: "goi_tool", tenTool: "tra_cuu_gia_co_phieu", thamSo: { maCongTy } };
  }
  return { loai: "du_thong_tin" };
}

function nhanCuaTool(tenTool: string): string {
  return tenTool === "tra_cuu_ho_so" ? "ho_so" : "gia";
}

type KetQuaVongLap =
  | { trangThai: "thanhCong"; ketQuaThuThap: Record<string, unknown>; soBuocDaDung: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number };

function chayVongLapTraLoi(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  cauHoi: string,
  soBuocToiDa: number,
  soLanThuLaiToiDa: number,
): KetQuaVongLap {
  let ketQuaThuThap: Record<string, unknown> = {};
  let buoc = 0;
  while (buoc < soBuocToiDa) {
    const quyetDinh = quyetDinhBuocTiepTheo(cauHoi, ketQuaThuThap);
    if (quyetDinh.loai === "khong_xac_dinh") return { trangThai: "loi", loi: "khong_xac_dinh_nhiem_vu", soBuocDaDung: buoc };
    if (quyetDinh.loai === "du_thong_tin") return { trangThai: "thanhCong", ketQuaThuThap, soBuocDaDung: buoc };
    buoc++;
    const loiGoi: LoiGoiTool = { tenTool: quyetDinh.tenTool, thamSo: quyetDinh.thamSo };
    const ketQuaGoi = goiMcpToolCoRetryVaFallback(ngu, catalog, loiGoi, soLanThuLaiToiDa);
    if (!ketQuaGoi.thanhCong) return { trangThai: "loi", loi: ketQuaGoi.loi, soBuocDaDung: buoc };
    ketQuaThuThap = { ...ketQuaThuThap, [nhanCuaTool(quyetDinh.tenTool)]: ketQuaGoi.giaTri };
  }
  return { trangThai: "hetBuoc", soBuocDaDung: buoc };
}

type KichBanL4 = { cauHoi: string; soBuocToiDa: number };
const BO_KICH_BAN_L4: KichBanL4[] = [
  { cauHoi: "Cho toi ho so va gia co phieu cua ACME", soBuocToiDa: 3 },
  { cauHoi: "Cho toi ho so va gia co phieu cua ACME", soBuocToiDa: 1 },
  { cauHoi: "Gia co phieu cua GLOB la bao nhieu", soBuocToiDa: 3 },
  { cauHoi: "Cho toi ho so va gia co phieu cua ZETA", soBuocToiDa: 3 },
  { cauHoi: "Thoi tiet hom nay the nao", soBuocToiDa: 3 },
];

function tinhTyLeHoanThanhVongLap(ds: KetQuaVongLap[]): number {
  if (ds.length === 0) return 0;
  return ds.filter((k) => k.trangThai === "thanhCong").length / ds.length;
}
function tinhTyLeChamCapVongLap(ds: KetQuaVongLap[]): number {
  if (ds.length === 0) return 0;
  return ds.filter((k) => k.trangThai === "hetBuoc").length / ds.length;
}

const ketQuaL4 = BO_KICH_BAN_L4.map((kb) => chayVongLapTraLoi(taoNguCanhThucThi(), CATALOG_TOOL_NGHIEN_CUU, kb.cauHoi, kb.soBuocToiDa, 1));
console.log(JSON.stringify(ketQuaL4.map((k) => k.trangThai)), tinhTyLeHoanThanhVongLap(ketQuaL4), tinhTyLeChamCapVongLap(ketQuaL4));
```

```typescript title=test
if (JSON.stringify(ketQuaL4.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "hetBuoc", "thanhCong", "loi", "loi"])) {
  throw new Error("nam trang thai phai la [thanhCong,hetBuoc,thanhCong,loi,loi]");
}
if (JSON.stringify(ketQuaL4.map((k) => k.soBuocDaDung)) !== JSON.stringify([2, 1, 1, 1, 0])) {
  throw new Error("soBuocDaDung cua nam kich ban phai la [2,1,1,1,0]");
}
if (tinhTyLeHoanThanhVongLap(ketQuaL4) !== 0.4) throw new Error("tyLeHoanThanh phai la 0.4 (2/5)");
if (tinhTyLeChamCapVongLap(ketQuaL4) !== 0.2) throw new Error("tyLeChamCap phai la 0.2 (1/5), rieng biet voi loi");

if (ketQuaL4[0]!.trangThai === "thanhCong" && JSON.stringify(ketQuaL4[0]!.ketQuaThuThap) !== JSON.stringify({ ho_so: { maCongTy: "ACME", nganh: "ban le", namThanhLap: 1998 }, gia: 42 })) {
  throw new Error("kich ban 1 (cau ghep, cap=3) phai thu thap DU CA ho_so LAN gia cua ACME");
}

const ketQuaCapHai = chayVongLapTraLoi(taoNguCanhThucThi(), CATALOG_TOOL_NGHIEN_CUU, "Cho toi ho so va gia co phieu cua ACME", 2, 1);
if (ketQuaCapHai.trangThai !== "hetBuoc") throw new Error("cau ghep voi soBuocToiDa=2 phai la hetBuoc, KHONG PHAI thanhCong -- can mot vong ranh de xac nhan du_thong_tin");
if (ketQuaCapHai.soBuocDaDung !== 2) throw new Error("cau ghep voi soBuocToiDa=2 van phai dung DUNG 2 luot goi tool that (giong het truong hop cap=3)");
```

:::hints
- kind: attention
  body: "Hai cho trong, cung trong vong while cua chayVongLapTraLoi, sau khi da xu ly khong_xac_dinh va du_thong_tin. Cho dau (mot dong, KHONG gan bien): tang buoc len 1 -- buoc++;. Cho hai (sau khi ketQuaGoi.thanhCong da duoc xac nhan): gan lai ketQuaThuThap bang spread cu CONG mot khoa moi -- ketQuaThuThap = { ...ketQuaThuThap, [nhanCuaTool(quyetDinh.tenTool)]: ketQuaGoi.giaTri };"
- kind: strategy
  body: "Cho dau: buoc++; Cho hai: ketQuaThuThap = { ...ketQuaThuThap, [nhanCuaTool(quyetDinh.tenTool)]: ketQuaGoi.giaTri };"
- kind: one-line
  body: "Cho dau la buoc++;, cho hai la ketQuaThuThap = { ...ketQuaThuThap, [nhanCuaTool(quyetDinh.tenTool)]: ketQuaGoi.giaTri };"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "[\"thanhCong\",\"hetBuoc\",\"thanhCong\",\"loi\",\"loi\"] 0.4 0.2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tyLeHoanThanh=0.4`, `tyLeChamCap=0.2` — HAI con số TÁCH BIỆT, không con
số nào thay được con số kia. Bài sau ráp AN TOÀN: chống prompt injection
từ CHÍNH nội dung mà tool trả về, và lọc tool "độc" ra khỏi catalog
TRƯỚC KHI agent kịp nhìn thấy nó.
::::

::::reflect{#nghi-lai}
Phát hiện quan trọng nhất bài này KHÔNG phải bảng trạng thái ba nhánh —
mà LÀ con số `soBuocDaDung=2` GIỐNG HỆT nhau Ở CẢ hai cấu hình
(`soBuocToiDa=3` THÀNH CÔNG, `soBuocToiDa=2` HẾT BƯỚC). Step cap không
đo "đã làm bao nhiêu việc" — nó đo "còn chỗ để KIỂM TRA đã xong hay chưa
không". Một vòng lặp CẦN chỗ dự phòng đó để tự tin báo cáo THÀNH CÔNG,
đúng khác biệt LOOP đã dạy suốt `q9.4a`/`q9.4b`: `"hetBuoc"` không phải
LÀ LỖI của tác vụ — nó LÀ lỗi của NGƯỜI CẤU HÌNH ngân sách, quên chừa một
bước cho phép hệ thống tự xác nhận.
::::

::::checkpoint{mastery=0.9}
::::
