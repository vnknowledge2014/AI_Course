---
id: ky-nghe-ung-dung-ai.boss-ky-nghe-ung-dung-ai.harness-retry-fallback-quanh-loi-mcp
title: "T9.7 bài 3 — harness: retry + fallback quanh một tool MCP lỗi tạm thời"
summary: "traLoiCauHoiCoHarness(ngu, catalog, cauHoi, soLanThuLaiToiDa): KetQuaTraLoi bọc goiMcpTool (bài 1) qua goiMcpToolCoRetry (retry, q9.3a) rồi goiMcpToolCoRetryVaFallback (fallback, q9.3a) — áp dụng cho tool THỨ BA, tra_cuu_tin_tuc, cố ý lỗi tạm thời ('loi_tam_thoi_ket_noi') ở HAI lần gọi đầu tiên xuyên suốt phiên. Trên BO_CAU_HOI_L3 (2 câu tin tức, soLanThuLaiToiDa=1): traLoiCauHoiDonGian (không harness) hoàn thành 0/2; traLoiCauHoiCoHarness hoàn thành 2/2 — câu đầu qua FALLBACK (giá trị mặc định, vì retry không đủ cứu 2 lần lỗi liên tiếp), câu sau qua GIÁ TRỊ THẬT (lỗi đã hết hiệu lực). demGoiToolThat.soLan tăng từ 2 (đơn giản) lên 3 (harness, retry tốn thêm một lượt gọi thật) dù tongChiPhi (tính theo Ý ĐỊNH câu hỏi, không theo số lần thử) giữ nguyên là 6 ở cả hai."
locale: vi
track: ky-nghe-ung-dung-ai
module: boss-ky-nghe-ung-dung-ai
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kna.harness-retry-fallback-quanh-loi-mcp]
requires: [kna.ngu-canh-nhieu-luot-trong-ngan-sach]
concepts: [kna.harness-retry-fallback-quanh-loi-mcp]
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
Hai bài trước giả định mọi tool LUÔN đáp ứng đúng như thiết kế. Thêm tool
thứ ba — `tra_cuu_tin_tuc` — cố ý LỖI TẠM THỜI Ở những lần gọi đầu tiên
của phiên, giống mọi kết nối mạng thật. Bài này ráp HARNESS (`q9.3a`):
retry rồi fallback, quanh MỘT lệnh gọi MCP.
::::

::::explain{#tool_thu_ba_va_ngu_canh_thuc_thi}
`tra_cuu_tin_tuc` lỗi Ở HAI lần gọi ĐẦU TIÊN xuyên suốt một phiên (mô
phỏng một dịch vụ tin tức đang "khởi động nguội"), rồi hoạt động bình
thường mãi mãi. Để đếm ĐÚNG số lần gọi xuyên NHIỀU câu hỏi, trạng thái
đó phải sống trong một `NguCanhThucThi` — một "ngăn kéo" đi kèm CẢ phiên,
không phải một biến cục bộ trong MỘT lệnh gọi:

```typescript title=readonly
type KetQua<T, E> = { thanhCong: true; giaTri: T } | { thanhCong: false; loi: E };
type DemGoi = { soLan: number };
type TrangThaiTinTuc = { soLanDaGoi: number };
type NguCanhThucThi = { demGoiToolThat: DemGoi; trangThaiTinTuc: TrangThaiTinTuc };

function taoNguCanhThucThi(): NguCanhThucThi {
  return { demGoiToolThat: { soLan: 0 }, trangThaiTinTuc: { soLanDaGoi: 0 } };
}

const CSDL_TIN_TUC: Record<string, string> = {
  ACME: "ACME mo rong thi truong sang khu vuc mien trung",
  GLOB: "GLOB ra mat dong san pham cong nghe moi",
};
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

function laLoiTamThoi(loi: string): boolean {
  return loi === "loi_tam_thoi_ket_noi";
}

// goiMcpTool(ngu, catalog, loiGoi) -- ban mo rong cua bai 1, nhan them "ngu"
// de tra dung ham thuc thi co trang thai (xem khoi solution de doc day du).

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
```

`laLoiTamThoi` LÀ ranh giới quan trọng nhất: retry/fallback CHỈ can thiệp
khi lỗi LÀ `"loi_tam_thoi_ket_noi"`. Lỗi khác (`"khong_tim_thay_cong_ty"`,
`"tham_so_khong_hop_le"`) đi thẳng qua KHÔNG hề bị fallback che giấu —
một mã công ty sai KHÔNG BAO GIỜ "tự khỏi" bằng cách thử lại.
::::

::::example{#don_gian_vs_harness_tren_tool_flaky}
```typescript title=readonly
const BO_CAU_HOI_L3 = ["Tin tuc gan day ve ACME co gi", "Tin tuc gan day ve GLOB co gi"];

const nguDonGian = taoNguCanhThucThi();
const ketQuaDonGian = BO_CAU_HOI_L3.map((c) => traLoiCauHoiDonGian(nguDonGian, CATALOG_TOOL_NGHIEN_CUU, c));
const baoCaoDonGian = tomTatDoEndToEnd(ketQuaDonGian);

const nguHarness = taoNguCanhThucThi();
const ketQuaHarness = BO_CAU_HOI_L3.map((c) => traLoiCauHoiCoHarness(nguHarness, CATALOG_TOOL_NGHIEN_CUU, c, 1));
const baoCaoHarness = tomTatDoEndToEnd(ketQuaHarness);

console.log("don gian:", JSON.stringify(baoCaoDonGian), nguDonGian.demGoiToolThat.soLan);
console.log("harness: ", JSON.stringify(baoCaoHarness), nguHarness.demGoiToolThat.soLan);
console.log(JSON.stringify(ketQuaHarness));
```

```text title=readonly
don gian: {"tyLeHoanThanh":0,"tongChiPhi":6,"tongSoBuoc":2,"soLanChanAnToan":0} 2
harness:  {"tyLeHoanThanh":1,"tongChiPhi":6,"tongSoBuoc":2,"soLanChanAnToan":0} 3
[{"loai":"thanh_cong","tenTool":"tra_cuu_tin_tuc","giaTri":{"tieuDe":"chua co tin tuc moi, thu lai sau"}},{"loai":"thanh_cong","tenTool":"tra_cuu_tin_tuc","giaTri":{"tieuDe":"GLOB ra mat dong san pham cong nghe moi"}}]
```

`traLoiCauHoiDonGian` (không harness) hoàn thành `0`/`2` — CẢ HAI câu rơi
đúng vào hai lần lỗi đầu tiên của `tra_cuu_tin_tuc`. `traLoiCauHoiCoHarness`
(retry `1` lần + fallback) hoàn thành `2`/`2`, NHƯNG bằng HAI CƠ CHẾ khác
nhau: câu ĐẦU dùng hết ngân sách retry (`1` lần) MÀ VẪN lỗi (đây LÀ lần
gọi thứ `2` của toàn phiên, vẫn `<= 2`) — nên rơi vào FALLBACK, trả
`GIA_TRI_DU_PHONG_TIN_TUC`; câu SAU gọi lần đầu (lần gọi thứ `3` toàn
phiên) đã VƯỢT qua ngưỡng lỗi, THÀNH CÔNG THẬT với tiêu đề GLOB. Chi phí
GIỮ NGUYÊN LÀ `6` Ở CẢ HAI (tính theo Ý ĐỊNH gọi tool, không theo SỐ LẦN
thử) — nhưng `demGoiToolThat.soLan` tăng từ `2` lên `3`: retry LÀ một
lệnh gọi THẬT, tốn tài nguyên thật, dù nó KHÔNG hiện diện trong
`tongChiPhi`.
::::

::::predict{#doan_bo_retry_van_con_fallback commitOnce}
Giữ NGUYÊN mọi thứ, chỉ đổi `soLanThuLaiToiDa` từ `1` xuống `0` (harness
KHÔNG còn retry, CHỈ còn fallback) khi gọi `traLoiCauHoiCoHarness` cho cả
hai câu Ở `BO_CAU_HOI_L3`. `tyLeHoanThanh` và `nguHarness.demGoiToolThat.soLan`
đổi ra sao?

:::opt{correct}
`tyLeHoanThanh` VẪN LÀ `1` (`2`/`2`) — fallback vẫn cứu CẢ HAI câu, vì nó
kích hoạt bất cứ khi nào lỗi LÀ tạm thời, KHÔNG phụ thuộc việc retry đã
thử hay chưa; NHƯNG `demGoiToolThat.soLan` giảm từ `3` xuống `2` (mỗi câu
giờ chỉ tốn ĐÚNG một lần gọi thật, không có lượt thử lại nào)
:::
:::opt
`tyLeHoanThanh` giảm xuống `0.5`, vì không còn retry để "cứu" câu đầu
tiên
::why
Nhầm rằng FALLBACK phụ thuộc VÀO việc retry đã cố gắng trước đó — nhưng
`goiMcpToolCoRetryVaFallback` kiểm tra `laLoiTamThoi(ketQua.loi)` TRÊN
KẾT QUẢ CUỐI CÙNG của `goiMcpToolCoRetry`, bất kể ngân sách retry LÀ bao
nhiêu (kể cả `0`) — MIỄN LÀ lỗi cuối cùng vẫn LÀ tạm thời, fallback LUÔN
kích hoạt.

Chỗ lệch: với `soLanThuLaiToiDa=0`, `goiMcpToolCoRetry` trả về NGAY kết
quả của LẦN GỌI ĐẦU TIÊN (vòng `while` không chạy lần nào) — kết quả đó
VẪN LÀ lỗi tạm thời (vì lần gọi thứ `1` VÀ thứ `2` của phiên đều lỗi), nên
fallback vẫn kích hoạt Ở CẢ HAI câu.
::
:::
:::opt
`demGoiToolThat.soLan` không đổi (vẫn `3`), vì tổng số câu hỏi không đổi
::why
Gần đúng Ở việc bạn để Ý CÓ một con số đo "tổng số lần gọi" cố định theo
số câu hỏi — quan sát Ề CÓ một mối liên hệ như vậy không sai.

Chỗ lệch: `demGoiToolThat.soLan` đếm SỐ LẦN `thucThiTraCuuTinTuc` được
gọi THẬT, và số đó phụ thuộc TRỰC TIẾP vào số lần RETRY được phép thử —
giảm ngân sách retry xuống `0` nghĩa là MỖI câu hỏi CHỈ gọi đúng MỘT lần
(không có lượt thử lại), nên tổng giảm từ `3` xuống `2`.
::
:::
::::

::::code{#viet_tra_loi_co_harness}
Hoàn thiện `traLoiCauHoiCoHarness`: sau khi `chonToolCoCauTruc` chọn được
tool, gọi qua `goiMcpToolCoRetryVaFallback` (retry + fallback) thay vì
`goiMcpTool` trần trụi, rồi ráp kết quả thành công.

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
type LoiGoiToolTho = LoiGoiTool | null;

function llmMoPhongChonTool(cauHoi: string): LoiGoiToolTho {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  const thamSo: Record<string, unknown> = maCongTy !== null ? { maCongTy } : {};
  if (c.includes("tin tuc")) return { tenTool: "tra_cuu_tin_tuc", thamSo };
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

type KetQuaTraLoi =
  | { loai: "thanh_cong"; tenTool: string; giaTri: unknown }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string }
  | { loai: "loi_thuc_thi"; tenTool: string; loi: string };

function traLoiCauHoiDonGian(ngu: NguCanhThucThi, catalog: McpToolDefinition[], cauHoi: string): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = goiMcpTool(ngu, catalog, chon.loiGoi);
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return { loai: "thanh_cong", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };
}

function traLoiCauHoiCoHarness(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  cauHoi: string,
  soLanThuLaiToiDa: number,
): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = ___;
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return ___;
}

const CHI_PHI_THEO_TOOL: Record<string, number> = { tra_cuu_ho_so: 1, tra_cuu_gia_co_phieu: 2, tra_cuu_tin_tuc: 3 };

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

function tomTatDoEndToEnd(ketQua: KetQuaTraLoi[]): BaoCaoDoEndToEnd {
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

const BO_CAU_HOI_L3 = ["Tin tuc gan day ve ACME co gi", "Tin tuc gan day ve GLOB co gi"];

const nguDonGian = taoNguCanhThucThi();
const ketQuaDonGian = BO_CAU_HOI_L3.map((c) => traLoiCauHoiDonGian(nguDonGian, CATALOG_TOOL_NGHIEN_CUU, c));
const baoCaoDonGian = tomTatDoEndToEnd(ketQuaDonGian);

const nguHarness = taoNguCanhThucThi();
const ketQuaHarness = BO_CAU_HOI_L3.map((c) => traLoiCauHoiCoHarness(nguHarness, CATALOG_TOOL_NGHIEN_CUU, c, 1));
const baoCaoHarness = tomTatDoEndToEnd(ketQuaHarness);

console.log(JSON.stringify(baoCaoDonGian), nguDonGian.demGoiToolThat.soLan);
console.log(JSON.stringify(baoCaoHarness), nguHarness.demGoiToolThat.soLan);
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
type LoiGoiToolTho = LoiGoiTool | null;

function llmMoPhongChonTool(cauHoi: string): LoiGoiToolTho {
  const c = cauHoi.toLowerCase();
  const maCongTy = trichMaCongTy(cauHoi);
  const thamSo: Record<string, unknown> = maCongTy !== null ? { maCongTy } : {};
  if (c.includes("tin tuc")) return { tenTool: "tra_cuu_tin_tuc", thamSo };
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

type KetQuaTraLoi =
  | { loai: "thanh_cong"; tenTool: string; giaTri: unknown }
  | { loai: "khong_xac_dinh_nhiem_vu" }
  | { loai: "thieu_tham_so"; tenTool: string }
  | { loai: "loi_thuc_thi"; tenTool: string; loi: string };

function traLoiCauHoiDonGian(ngu: NguCanhThucThi, catalog: McpToolDefinition[], cauHoi: string): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = goiMcpTool(ngu, catalog, chon.loiGoi);
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return { loai: "thanh_cong", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };
}

function traLoiCauHoiCoHarness(
  ngu: NguCanhThucThi,
  catalog: McpToolDefinition[],
  cauHoi: string,
  soLanThuLaiToiDa: number,
): KetQuaTraLoi {
  const chon = chonToolCoCauTruc(cauHoi);
  if (chon.loai !== "da_chon") return chon;
  const ketQua = goiMcpToolCoRetryVaFallback(ngu, catalog, chon.loiGoi, soLanThuLaiToiDa);
  if (!ketQua.thanhCong) return { loai: "loi_thuc_thi", tenTool: chon.loiGoi.tenTool, loi: ketQua.loi };
  return { loai: "thanh_cong", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };
}

const CHI_PHI_THEO_TOOL: Record<string, number> = { tra_cuu_ho_so: 1, tra_cuu_gia_co_phieu: 2, tra_cuu_tin_tuc: 3 };

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

function tomTatDoEndToEnd(ketQua: KetQuaTraLoi[]): BaoCaoDoEndToEnd {
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

const BO_CAU_HOI_L3 = ["Tin tuc gan day ve ACME co gi", "Tin tuc gan day ve GLOB co gi"];

const nguDonGian = taoNguCanhThucThi();
const ketQuaDonGian = BO_CAU_HOI_L3.map((c) => traLoiCauHoiDonGian(nguDonGian, CATALOG_TOOL_NGHIEN_CUU, c));
const baoCaoDonGian = tomTatDoEndToEnd(ketQuaDonGian);

const nguHarness = taoNguCanhThucThi();
const ketQuaHarness = BO_CAU_HOI_L3.map((c) => traLoiCauHoiCoHarness(nguHarness, CATALOG_TOOL_NGHIEN_CUU, c, 1));
const baoCaoHarness = tomTatDoEndToEnd(ketQuaHarness);

console.log(JSON.stringify(baoCaoDonGian), nguDonGian.demGoiToolThat.soLan);
console.log(JSON.stringify(baoCaoHarness), nguHarness.demGoiToolThat.soLan);
```

```typescript title=test
if (baoCaoDonGian.tyLeHoanThanh !== 0) throw new Error("khong harness: tyLeHoanThanh phai la 0 (0/2) -- ca hai cau roi dung vao loi tam thoi");
if (nguDonGian.demGoiToolThat.soLan !== 2) throw new Error("khong harness: demGoiToolThat.soLan phai la 2 (moi cau dung 1 lan goi, khong retry)");

if (baoCaoHarness.tyLeHoanThanh !== 1) throw new Error("co harness: tyLeHoanThanh phai la 1 (2/2)");
if (baoCaoHarness.tongChiPhi !== 6) throw new Error("co harness: tongChiPhi phai la 6 (3+3, tinh theo Y DINH khong theo so lan thu)");
if (nguHarness.demGoiToolThat.soLan !== 3) throw new Error("co harness: demGoiToolThat.soLan phai la 3 (cau dau ton 2 lan goi vi retry, cau sau 1 lan)");

if (ketQuaHarness[0]!.loai !== "thanh_cong" || JSON.stringify((ketQuaHarness[0]! as { giaTri: unknown }).giaTri) !== JSON.stringify({ tieuDe: "chua co tin tuc moi, thu lai sau" })) {
  throw new Error("cau dau (co harness) phai thanh_cong qua FALLBACK, gia tri la GIA_TRI_DU_PHONG_TIN_TUC");
}
if (ketQuaHarness[1]!.loai !== "thanh_cong" || JSON.stringify((ketQuaHarness[1]! as { giaTri: unknown }).giaTri) !== JSON.stringify({ tieuDe: "GLOB ra mat dong san pham cong nghe moi" })) {
  throw new Error("cau sau (co harness) phai thanh_cong qua GIA TRI THAT (khong phai fallback)");
}

const nguKhongRetry = taoNguCanhThucThi();
const ketQuaKhongRetry = BO_CAU_HOI_L3.map((c) => traLoiCauHoiCoHarness(nguKhongRetry, CATALOG_TOOL_NGHIEN_CUU, c, 0));
if (tinhTyLeHoanThanh(ketQuaKhongRetry) !== 1) throw new Error("soLanThuLaiToiDa=0 van phai hoan thanh 1 (fallback khong can retry truoc)");
if (nguKhongRetry.demGoiToolThat.soLan !== 2) throw new Error("soLanThuLaiToiDa=0 phai chi ton 2 lan goi that (khong co luot thu lai nao)");
```

:::hints
- kind: attention
  body: "Hai cho trong, cung trong traLoiCauHoiCoHarness, SAU dong chon.loai !== 'da_chon'. Cho dau: goi goiMcpToolCoRetryVaFallback (KHONG PHAI goiMcpTool tran trui) voi ca bon tham so (ngu, catalog, chon.loiGoi, soLanThuLaiToiDa). Cho hai: rap KetQuaTraLoi thanh cong tu ketQua.giaTri va chon.loiGoi.tenTool -- giong het cho hai cua bai 1."
- kind: strategy
  body: "Cho dau: const ketQua = goiMcpToolCoRetryVaFallback(ngu, catalog, chon.loiGoi, soLanThuLaiToiDa); Cho hai: return { loai: \"thanh_cong\", tenTool: chon.loiGoi.tenTool, giaTri: ketQua.giaTri };"
- kind: one-line
  body: "Cho dau la goiMcpToolCoRetryVaFallback(ngu, catalog, chon.loiGoi, soLanThuLaiToiDa), cho hai giong het bai 1."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "{\"tyLeHoanThanh\":0,\"tongChiPhi\":6,\"tongSoBuoc\":2,\"soLanChanAnToan\":0} 2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0` so với `1` hoàn thành, trên CÙNG hai câu hỏi — chỉ đổi chương trình
bao quanh tool. Bài sau thêm LOOP: một câu hỏi có thể cần NHIỀU lượt gọi
tool khác nhau, với một step cap tường minh — hetBuoc khác loi khác
thanhCong.
::::

::::reflect{#nghi-lai}
`traLoiCauHoiCoHarness` không viết lại retry hay fallback — nó GỌI ĐÚNG
`goiMcpToolCoRetryVaFallback` của `q9.3a`, chỉ ráp và VÀO đúng vị trí
`goiMcpTool` trần trụi từng đứng Ở bài `1`. Con số bất ngờ nhất KHÔNG phải
`tyLeHoanThanh` (từ `0` lên `1` — dễ đoán) mà LÀ `demGoiToolThat.soLan`
(từ `2` lên `3`): harness không MIỄN PHÍ — retry LÀ một lệnh gọi tool
THẬT, và một hệ thống đo lường tốt phải TÁCH RIÊNG "bao nhiêu câu hỏi
xong" khỏi "tốn bao nhiêu lệnh gọi thật để xong", đúng nguyên tắc
`q9.3b` đã dạy: completion rate và chi phí là HAI trục ĐO khác nhau.
::::

::::checkpoint{mastery=0.88}
::::
