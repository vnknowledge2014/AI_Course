---
id: ky-nghe-ung-dung-ai.boss-ky-nghe-ung-dung-ai.ngu-canh-nhieu-luot-trong-ngan-sach
title: "T9.7 bài 2 — phiên nhiều lượt, lịch sử phải nằm trong ngân sách token"
summary: "chayPhienNhieuLuot(catalog, cacCauHoi, nganSach): KetQuaPhien lặp traLoiCauHoi (bài 1) cho MỖI câu hỏi trong MỘT phiên, ghi mỗi kết quả vào lichSu (CONTEXT), cắt bằng catCuaSoTruotLichSu (tái dùng nguyên vẹn từ q9.2a) MỖI KHI vượt nganSach. Trên BO_CAU_HOI_L2 (5 câu, nganSach=50): lichSuCuoiCung chỉ còn 2/5 mục (lượt 4 và 5, tổng 50 token, khớp CHÍNH XÁC ngân sách) — NHƯNG baoCao (tyLeHoanThanh=0.8, tongChiPhi=8, tongSoBuoc=5) tính trên TOÀN BỘ 5 lượt, KHÔNG đổi dù lịch sử đã bị cắt còn 2. Bài học cốt lõi: đo end-to-end và quản lý ngữ cảnh là HAI việc độc lập — cắt cửa sổ không được phép làm sai lệch số liệu đo."
locale: vi
track: ky-nghe-ung-dung-ai
module: boss-ky-nghe-ung-dung-ai
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.ngu-canh-nhieu-luot-trong-ngan-sach]
requires: [kna.dat-bai-toan-tro-ly-nghien-cuu-qua-mcp]
concepts: [kna.ngu-canh-nhieu-luot-trong-ngan-sach]
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
Bài `1` xử lý MỘT câu hỏi, MỘT lượt. Người dùng thật hỏi NHIỀU câu trong
CÙNG một phiên — và lịch sử phiên đó, nếu không quản lý, phình vô hạn.
Bài này ráp CONTEXT (`q9.2a`) vào: cắt cửa sổ trượt khi vượt ngân sách
token, TÁCH BIỆT với việc đo end-to-end.
::::

::::explain{#phien_nhieu_luot_va_ngan_sach}
`chayPhienNhieuLuot` chạy `traLoiCauHoi` (bài `1`) cho MỖI câu hỏi
TUẦN TỰ, ghi từng kết quả thành MỘT mục lịch sử (`LichSuMuc`), rồi kiểm
tổng token — vượt `nganSach` thì CẮT (`catCuaSoTruotLichSu`, tái dùng
nguyên vẹn thuật toán cửa sổ trượt của `q9.2a` bài `cua-so-truot`: giữ
phần ĐUÔI, bỏ dần từ ĐẦU cho tới khi vừa ngân sách):

```typescript title=readonly
type ChatLichSu = { luot: number; noiDung: string };

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongTokenLichSu(ds: ChatLichSu[]): number {
  return ds.reduce((t, m) => t + demTokenGiaLap(m.noiDung), 0);
}
function catCuaSoTruotLichSu(ds: ChatLichSu[], nganSach: number): ChatLichSu[] {
  let batDau = 0;
  while (batDau < ds.length && tinhTongTokenLichSu(ds.slice(batDau)) > nganSach) batDau++;
  return ds.slice(batDau);
}

function tomTatDoEndToEnd(ketQua: KetQuaTraLoi[]): BaoCaoDoEndToEnd {
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

type KetQuaPhien = {
  soCauDaXuLy: number;
  lichSuCuoiCung: ChatLichSu[];
  tongTokenCuoiCung: number;
  baoCao: BaoCaoDoEndToEnd;
};

function chayPhienNhieuLuot(catalog: McpToolDefinition[], cacCauHoi: string[], nganSach: number): KetQuaPhien {
  let lichSu: ChatLichSu[] = [];
  const dsKetQua: KetQuaTraLoi[] = [];
  cacCauHoi.forEach((cauHoi, i) => {
    const ketQua = traLoiCauHoi(catalog, cauHoi);
    dsKetQua.push(ketQua);
    lichSu = [...lichSu, { luot: i + 1, noiDung: JSON.stringify(ketQua) }];
    if (tinhTongTokenLichSu(lichSu) > nganSach) {
      lichSu = catCuaSoTruotLichSu(lichSu, nganSach);
    }
  });
  return {
    soCauDaXuLy: cacCauHoi.length,
    lichSuCuoiCung: lichSu,
    tongTokenCuoiCung: tinhTongTokenLichSu(lichSu),
    baoCao: tomTatDoEndToEnd(dsKetQua),
  };
}
```

Điểm THIẾT KẾ quan trọng nhất: `baoCao` được tính từ `dsKetQua` — mảng
GIỮ NGUYÊN TOÀN BỘ kết quả của MỌI lượt, KHÔNG BAO GIỜ bị cắt. `lichSu`
(bị cắt) và `dsKetQua` (không bị cắt) LÀ HAI cấu trúc dữ liệu TÁCH BIỆT.
Nếu lỡ tính `baoCao` từ `lichSu` đã cắt, con số đo được sẽ THAY ĐỔI theo
ngân sách — một lỗi kỹ nghệ nghiêm trọng, vì "đo end-to-end" phải là một
sự thật KHÁCH QUAN, không phụ thuộc cấu hình bộ nhớ hiển thị.
::::

::::example{#nam_cau_hoi_ngan_sach_50}
```typescript title=readonly
const BO_CAU_HOI_L2 = [
  "Gia co phieu cua ACME hien tai la bao nhieu",
  "Cho toi thong tin ho so cua GLOB",
  "Gia co phieu cua GLOB hien tai la bao nhieu",
  "Cho toi thong tin ho so cua ACME",
  "Gia co phieu cua ZETA la bao nhieu",
];

const phien = chayPhienNhieuLuot(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L2, 50);
console.log(phien.soCauDaXuLy, phien.lichSuCuoiCung.length, phien.tongTokenCuoiCung, JSON.stringify(phien.baoCao));
console.log(JSON.stringify(phien.lichSuCuoiCung.map((m) => m.luot)));
```

```text title=readonly
5 2 50 {"tyLeHoanThanh":0.8,"tongChiPhi":8,"tongSoBuoc":5,"soLanChanAnToan":0}
[4,5]
```

Năm câu hỏi (token mỗi lượt lần lượt LÀ `17,29,17,28,22` — kết quả JSON
mỗi lượt dài ngắn khác nhau vì hồ sơ công ty dài hơn giá cổ phiếu), ngân
sách `50`: SAU lượt `3` tổng đã vượt `50`, cắt bỏ lượt `1`; sau lượt `4`
lại vượt, cắt bỏ lượt `2`; sau lượt `5` lại vượt, cắt bỏ lượt `3` —
`lichSuCuoiCung` CHỈ CÒN lượt `4` VÀ `5`, tổng ĐÚNG BẰNG `50`. NHƯNG
`baoCao` vẫn đếm ĐỦ cả `5` lượt: `tyLeHoanThanh = 0.8` (`4/5` — chỉ câu
`ZETA` thất bại), `tongChiPhi = 8` (`2+1+2+1+2`), `tongSoBuoc = 5` (không
câu nào bị từ chối Ở PROMPT). Cắt cửa sổ hiển thị KHÔNG hề làm mất một
đơn vị đo lường nào.
::::

::::predict{#doan_tang_ngan_sach_khong_bao_gio_cat commitOnce}
Tăng `nganSach` từ `50` lên `200` (đủ lớn để KHÔNG BAO GIỜ kích hoạt cắt)
— `lichSuCuoiCung.length` và `phien.baoCao` đổi ra sao so với bản gốc?

:::opt{correct}
`lichSuCuoiCung.length` tăng từ `2` lên `5` (giữ TRỌN VẸN cả năm lượt,
không lượt nào bị cắt) — NHƯNG `phien.baoCao` (`tyLeHoanThanh=0.8`,
`tongChiPhi=8`, `tongSoBuoc=5`) HOÀN TOÀN KHÔNG ĐỔI, vì nó tính từ
`dsKetQua`, độc lập với `nganSach`
:::
:::opt
`phien.baoCao` cũng đổi theo — `tyLeHoanThanh` tăng lên vì giờ có nhiều
lịch sử hơn để "tham khảo" khi trả lời
::why
Nhầm rằng `traLoiCauHoi` (bài `1`) ĐỌC `lichSu` để trả lời câu hỏi tiếp
theo — nhưng nó KHÔNG hề nhận `lichSu` làm tham số; mỗi câu hỏi được xử
lý ĐỘC LẬP, không phụ thuộc bất kỳ lượt nào trước đó.

Chỗ lệch: `dsKetQua` (nguồn của `baoCao`) được tính TRƯỚC khi `lichSu`
có cơ hội bị cắt hay không bị cắt — thứ tự trong `chayPhienNhieuLuot` LÀ
gọi `traLoiCauHoi` rồi MỚI append/cắt `lichSu`, không phải ngược lại.
::
:::
:::opt
`lichSuCuoiCung.length` không đổi (vẫn `2`), vì `catCuaSoTruotLichSu` chỉ
kích hoạt đúng một lần bất kể ngân sách lớn cỡ nào
::why
Gần đúng Ở việc bạn nhớ CÓ một điều kiện kích hoạt cắt (`vượt nganSach`)
— quan sát Ề CÓ điều kiện đó đúng.

Chỗ lệch: điều kiện đó SO SÁNH VỚI `nganSach` MỖI LẦN — với `nganSach=200`,
tổng token tối đa xuyên suốt năm lượt (`17+29+17+28+22=113`) KHÔNG BAO GIỜ
vượt `200`, nên nhánh cắt (`if (tinhTongTokenLichSu(lichSu) > nganSach)`)
không kích hoạt Ở BẤT KỲ lượt nào — `lichSu` giữ NGUYÊN cả năm mục.
::
:::
::::

::::code{#viet_chay_phien_nhieu_luot}
Hoàn thiện `chayPhienNhieuLuot`: sau khi gọi `traLoiCauHoi` và append vào
`lichSu`, cắt bằng `catCuaSoTruotLichSu` KHI vượt ngân sách; trả về đủ
bốn trường của `KetQuaPhien`.

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

type BaoCaoDoEndToEnd = {
  tyLeHoanThanh: number;
  tongChiPhi: number;
  tongSoBuoc: number;
  soLanChanAnToan: number;
};

type ChatLichSu = { luot: number; noiDung: string };

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongTokenLichSu(ds: ChatLichSu[]): number {
  return ds.reduce((t, m) => t + demTokenGiaLap(m.noiDung), 0);
}
function catCuaSoTruotLichSu(ds: ChatLichSu[], nganSach: number): ChatLichSu[] {
  let batDau = 0;
  while (batDau < ds.length && tinhTongTokenLichSu(ds.slice(batDau)) > nganSach) batDau++;
  return ds.slice(batDau);
}

function tomTatDoEndToEnd(ketQua: KetQuaTraLoi[]): BaoCaoDoEndToEnd {
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

type KetQuaPhien = {
  soCauDaXuLy: number;
  lichSuCuoiCung: ChatLichSu[];
  tongTokenCuoiCung: number;
  baoCao: BaoCaoDoEndToEnd;
};

function chayPhienNhieuLuot(catalog: McpToolDefinition[], cacCauHoi: string[], nganSach: number): KetQuaPhien {
  let lichSu: ChatLichSu[] = [];
  const dsKetQua: KetQuaTraLoi[] = [];
  cacCauHoi.forEach((cauHoi, i) => {
    const ketQua = traLoiCauHoi(catalog, cauHoi);
    dsKetQua.push(ketQua);
    lichSu = [...lichSu, { luot: i + 1, noiDung: JSON.stringify(ketQua) }];
    if (tinhTongTokenLichSu(lichSu) > nganSach) {
      ___
    }
  });
  return ___;
}

const BO_CAU_HOI_L2 = [
  "Gia co phieu cua ACME hien tai la bao nhieu",
  "Cho toi thong tin ho so cua GLOB",
  "Gia co phieu cua GLOB hien tai la bao nhieu",
  "Cho toi thong tin ho so cua ACME",
  "Gia co phieu cua ZETA la bao nhieu",
];

const phien = chayPhienNhieuLuot(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L2, 50);
console.log(phien.soCauDaXuLy, phien.lichSuCuoiCung.length, phien.tongTokenCuoiCung, JSON.stringify(phien.baoCao));
console.log(JSON.stringify(phien.lichSuCuoiCung.map((m) => m.luot)));
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

type BaoCaoDoEndToEnd = {
  tyLeHoanThanh: number;
  tongChiPhi: number;
  tongSoBuoc: number;
  soLanChanAnToan: number;
};

type ChatLichSu = { luot: number; noiDung: string };

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongTokenLichSu(ds: ChatLichSu[]): number {
  return ds.reduce((t, m) => t + demTokenGiaLap(m.noiDung), 0);
}
function catCuaSoTruotLichSu(ds: ChatLichSu[], nganSach: number): ChatLichSu[] {
  let batDau = 0;
  while (batDau < ds.length && tinhTongTokenLichSu(ds.slice(batDau)) > nganSach) batDau++;
  return ds.slice(batDau);
}

function tomTatDoEndToEnd(ketQua: KetQuaTraLoi[]): BaoCaoDoEndToEnd {
  return {
    tyLeHoanThanh: tinhTyLeHoanThanh(ketQua),
    tongChiPhi: tinhChiPhi(ketQua),
    tongSoBuoc: tinhSoBuoc(ketQua),
    soLanChanAnToan: 0,
  };
}

type KetQuaPhien = {
  soCauDaXuLy: number;
  lichSuCuoiCung: ChatLichSu[];
  tongTokenCuoiCung: number;
  baoCao: BaoCaoDoEndToEnd;
};

function chayPhienNhieuLuot(catalog: McpToolDefinition[], cacCauHoi: string[], nganSach: number): KetQuaPhien {
  let lichSu: ChatLichSu[] = [];
  const dsKetQua: KetQuaTraLoi[] = [];
  cacCauHoi.forEach((cauHoi, i) => {
    const ketQua = traLoiCauHoi(catalog, cauHoi);
    dsKetQua.push(ketQua);
    lichSu = [...lichSu, { luot: i + 1, noiDung: JSON.stringify(ketQua) }];
    if (tinhTongTokenLichSu(lichSu) > nganSach) {
      lichSu = catCuaSoTruotLichSu(lichSu, nganSach);
    }
  });
  return {
    soCauDaXuLy: cacCauHoi.length,
    lichSuCuoiCung: lichSu,
    tongTokenCuoiCung: tinhTongTokenLichSu(lichSu),
    baoCao: tomTatDoEndToEnd(dsKetQua),
  };
}

const BO_CAU_HOI_L2 = [
  "Gia co phieu cua ACME hien tai la bao nhieu",
  "Cho toi thong tin ho so cua GLOB",
  "Gia co phieu cua GLOB hien tai la bao nhieu",
  "Cho toi thong tin ho so cua ACME",
  "Gia co phieu cua ZETA la bao nhieu",
];

const phien = chayPhienNhieuLuot(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L2, 50);
console.log(phien.soCauDaXuLy, phien.lichSuCuoiCung.length, phien.tongTokenCuoiCung, JSON.stringify(phien.baoCao));
console.log(JSON.stringify(phien.lichSuCuoiCung.map((m) => m.luot)));
```

```typescript title=test
if (phien.soCauDaXuLy !== 5) throw new Error("soCauDaXuLy phai la 5");
if (phien.lichSuCuoiCung.length !== 2) throw new Error("lichSuCuoiCung phai chi con 2 muc, sau khi cat cua so voi nganSach=50");
if (JSON.stringify(phien.lichSuCuoiCung.map((m) => m.luot)) !== JSON.stringify([4, 5])) {
  throw new Error("lichSuCuoiCung phai giu lai DUNG luot 4 va 5 (phan DUOI), khong phai luot khac");
}
if (phien.tongTokenCuoiCung !== 50) throw new Error("tongTokenCuoiCung phai khop CHINH XAC nganSach la 50");

if (phien.baoCao.tyLeHoanThanh !== 0.8) throw new Error("baoCao.tyLeHoanThanh phai la 0.8 (4/5) -- tinh tren TOAN BO 5 luot, khong phai lichSu da cat");
if (phien.baoCao.tongChiPhi !== 8) throw new Error("baoCao.tongChiPhi phai la 8 (2+1+2+1+2)");
if (phien.baoCao.tongSoBuoc !== 5) throw new Error("baoCao.tongSoBuoc phai la 5");
if (phien.baoCao.soLanChanAnToan !== 0) throw new Error("baoCao.soLanChanAnToan phai la 0");

const phienNganSachLon = chayPhienNhieuLuot(CATALOG_TOOL_NGHIEN_CUU, BO_CAU_HOI_L2, 200);
if (phienNganSachLon.lichSuCuoiCung.length !== 5) throw new Error("voi nganSach=200 (du lon), lichSuCuoiCung phai giu DU CA 5 luot");
if (JSON.stringify(phienNganSachLon.baoCao) !== JSON.stringify(phien.baoCao)) {
  throw new Error("baoCao phai GIONG HET nhau du doi nganSach -- no khong phu thuoc viec lichSu co bi cat hay khong");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (ben trong if vuot nganSach): MOT dong gan lai -- lichSu = catCuaSoTruotLichSu(lichSu, nganSach). Cho hai (return cuoi ham, SAU vong forEach): tra ve DU BON truong cua KetQuaPhien -- soCauDaXuLy (cacCauHoi.length), lichSuCuoiCung (lichSu), tongTokenCuoiCung (tinhTongTokenLichSu(lichSu)), baoCao (tomTatDoEndToEnd(dsKetQua) -- CHU Y dsKetQua, KHONG PHAI lichSu)."
- kind: strategy
  body: "Cho dau: lichSu = catCuaSoTruotLichSu(lichSu, nganSach); Cho hai: return { soCauDaXuLy: cacCauHoi.length, lichSuCuoiCung: lichSu, tongTokenCuoiCung: tinhTongTokenLichSu(lichSu), baoCao: tomTatDoEndToEnd(dsKetQua) };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri -- baoCao PHAI tinh tu dsKetQua, khong phai lichSu."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "5 2 50 {\"tyLeHoanThanh\":0.8,\"tongChiPhi\":8,\"tongSoBuoc\":5,\"soLanChanAnToan\":0}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`lichSuCuoiCung` chỉ còn `2` mục, NHƯNG `baoCao` vẫn đếm đủ `5` lượt —
CONTEXT quản lý HIỂN THỊ, không quản lý SỰ THẬT. Bài sau thêm HARNESS:
một trong hai tool sẽ lỗi TẠM THỜI, và trợ lý cần retry + fallback để
không sụp hoàn toàn.
::::

::::reflect{#nghi-lai}
Bài này KHÔNG dạy một thuật toán cắt mới — `catCuaSoTruotLichSu` LÀ
`catCuaSoTruot` của `q9.2a`, đổi tên biến cho khớp miền dữ liệu. Điều
bài này THẬT SỰ dạy LÀ một ranh giới kiến trúc: `lichSu` (thứ agent
"nhìn thấy" Ở lượt tiếp theo) và `dsKetQua` (thứ hệ thống "đo" để báo
cáo) PHẢI LÀ hai luồng dữ liệu tách biệt, dù chúng xuất phát từ CÙNG một
`traLoiCauHoi`. Trộn chúng làm MỘT — tính `baoCao` từ `lichSu` đã cắt —
sẽ khiến một con số vận hành quan trọng (tỉ lệ hoàn thành) phụ thuộc
NGẪU NHIÊN vào một tham số hiển thị (ngân sách cửa sổ), điều không một
kỹ sư nào chấp nhận được khi đo hệ thống thật.
::::

::::checkpoint{mastery=0.87}
::::
