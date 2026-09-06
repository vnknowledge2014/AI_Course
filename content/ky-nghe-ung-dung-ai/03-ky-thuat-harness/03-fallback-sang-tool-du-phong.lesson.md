---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.fallback-sang-tool-du-phong
title: "Fallback sang tool dự phòng khi retry vẫn thất bại"
summary: "taoToolLuonThatBai() mô phỏng tool chính HỎNG VĨNH VIỄN (mọi lần gọi đều thất bại, không chỉ lần đầu -- retry KHÔNG cứu được). taoToolDuPhong() LUÔN thành công nhưng trả giá trị mặc định kém chính xác hơn (\"gia_tri_mac_dinh\" thay vì \"ket_qua_that\"). goiToolCoFallback(toolChinh, toolDuPhong) thử toolChinh QUA goiToolCoRetry (bài trước); NẾU vẫn thất bại, chuyển sang toolDuPhong. Trên 5 tác vụ với tool chính hỏng vĩnh viễn: completion rate VẪN là 1 (5/5) nhờ fallback, dù tool chính hoàn toàn vô dụng -- nhưng MỌI giá trị trả về đều là \"gia_tri_mac_dinh\", không phải \"ket_qua_that\". Đối chứng: nếu tool chính tự phục hồi qua retry (taoToolLoiTamThoi, bài 1-2), toolDuPhong KHÔNG hề bị gọi (soLanDaGoi=0) -- fallback chỉ là phương án CUỐI CÙNG."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.fallback-sang-tool-du-phong]
requires: [kna.retry-mot-lan-sua-loi-tam-thoi]
concepts: [kna.fallback-sang-tool-du-phong]
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
Retry cứu được lỗi TẠM THỜI — nhưng nếu tool chính hỏng THẬT SỰ, hỏng
Ở MỌI lần gọi chứ không chỉ lần đầu, retry vô dụng: gọi lại bao nhiêu
lần cũng ra đúng một kết quả — thất bại. Bài này thêm một lối thoát
KHÁC: khi tool chính đã hết cách, chuyển hẳn sang một tool DỰ PHÒNG —
chấp nhận kết quả kém chính xác hơn, đổi lấy việc CÓ kết quả.
::::

::::explain{#tool_hong_vinh_vien_va_tool_du_phong}
`taoToolLuonThatBai` mô phỏng đúng lớp lỗi retry KHÔNG cứu được: MỌI
lần gọi đều thất bại, không riêng lần đầu. `taoToolDuPhong` thì ngược
lại — LUÔN thành công, nhưng giá trị nó trả về LÀ một giá trị MẶC ĐỊNH
("gia_tri_mac_dinh"), kém chính xác hơn kết quả THẬT ("ket_qua_that")
mà một tool khoẻ mạnh trả về:

```typescript title=readonly
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

const chinh = taoToolLuonThatBai();
console.log(JSON.stringify(chinh.goi()), JSON.stringify(chinh.goi()), JSON.stringify(chinh.goi()));
const duPhong = taoToolDuPhong();
console.log(JSON.stringify(duPhong.goi()));
```

```text title=readonly
{"thanhCong":false,"loi":"loi_vinh_vien"} {"thanhCong":false,"loi":"loi_vinh_vien"} {"thanhCong":false,"loi":"loi_vinh_vien"}
{"thanhCong":true,"giaTri":"gia_tri_mac_dinh"}
```

`taoToolLuonThatBai` thất bại Ở CẢ BA lần gọi liên tiếp — khác hẳn
`taoToolLoiTamThoi` (bài `1`-`2`) chỉ thất bại đúng lần đầu. Đây LÀ lý
do retry (bài trước) không giúp được gì với tool NÀY: gọi lại bao
nhiêu lần cũng ra thất bại.
::::

::::example{#goi_tool_co_fallback}
`goiToolCoFallback` thử tool chính QUA `goiToolCoRetry` (bài trước —
đã gồm cả lần đầu VÀ một lần thử lại); NẾU kết quả đó VẪN thất bại,
chuyển sang gọi tool dự phòng:

```typescript title=readonly
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  const ketQuaChinh = goiToolCoRetry(toolChinh);
  if (ketQuaChinh.thanhCong) return ketQuaChinh;
  return toolDuPhong.goi();
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

const SO_TAC_VU = 5;
const ketQuaCoFallback: KetQuaGoiTool[] = [];
for (let i = 0; i < SO_TAC_VU; i++) {
  const toolChinh = taoToolLuonThatBai();
  const toolDuPhong = taoToolDuPhong();
  ketQuaCoFallback.push(goiToolCoFallback(toolChinh, toolDuPhong));
}
console.log(JSON.stringify(ketQuaCoFallback.map((k) => k.thanhCong)));
console.log(JSON.stringify(ketQuaCoFallback.map((k) => (k.thanhCong ? k.giaTri : null))));
console.log(tinhTyLeHoanThanh(ketQuaCoFallback));
```

```text title=readonly
[true,true,true,true,true]
["gia_tri_mac_dinh","gia_tri_mac_dinh","gia_tri_mac_dinh","gia_tri_mac_dinh","gia_tri_mac_dinh"]
1
```

Tool chính hỏng VĨNH VIỄN — retry (2 lần gọi) không cứu được MỘT tác
vụ nào. Nhưng completion rate VẪN LÀ `1`: fallback bắt lấy MỌI thất
bại đó, trả về `"gia_tri_mac_dinh"` thay vì `"ket_qua_that"`. Completion
rate cao, nhưng CHẤT LƯỢNG kết quả thấp hơn — đây LÀ đánh đổi thật của
fallback, không phải một chiến thắng miễn phí.
::::

::::predict{#doan-tool-chinh-tu-phuc-hoi commitOnce}
Nếu `toolChinh` truyền vào `goiToolCoFallback` LÀ một `taoToolLoiTamThoi()`
(chỉ thất bại lần đầu, đã học Ở bài `1`-`2`) thay vì
`taoToolLuonThatBai()` — `toolDuPhong` có bị gọi không?

:::opt{correct}
Không — `goiToolCoRetry(toolChinh)` (gồm cả lần đầu VÀ lần thử lại)
đã đủ để tool chính THÀNH CÔNG, nên `ketQuaChinh.thanhCong` LÀ `true`,
hàm `return` sớm ngay dòng đó, KHÔNG bao giờ chạy tới dòng gọi
`toolDuPhong.goi()`
:::
:::opt
Có — `goiToolCoFallback` luôn gọi CẢ HAI tool để so sánh, chọn kết quả
tốt hơn
::why
Nhầm với chiến lược "gọi song song rồi so sánh" — nhưng đó KHÔNG phải
thiết kế Ở đây.

Chỗ lệch: `goiToolCoFallback` dùng `if (ketQuaChinh.thanhCong) return
ketQuaChinh;` — một `return` SỚM. Khi điều kiện đó đúng, hàm KẾT THÚC
ngay, dòng gọi `toolDuPhong.goi()` phía dưới không bao giờ được thực
thi.
::
:::
:::opt
Không xác định được — phụ thuộc tool nào PHẢN HỒI nhanh hơn
::why
Nhầm với việc gọi bất đồng bộ (race giữa hai lời gọi song song) —
nhưng mọi lời gọi Ở đây LÀ đồng bộ, tuần tự.

Chỗ lệch: `const ketQuaChinh = goiToolCoRetry(toolChinh);` chạy XONG
HOÀN TOÀN trước khi dòng `if` kế tiếp được xét — không có khái niệm
"nhanh hơn" nào áp dụng cho hai lời gọi hàm đồng bộ nối tiếp nhau.
::
:::
::::

::::code{#viet_goi_tool_co_fallback}
Hoàn thiện `goiToolCoFallback` — gọi `goiToolCoRetry(toolChinh)`, NẾU
kết quả đó thành công thì trả về NGAY; nếu KHÔNG, gọi
`toolDuPhong.goi()` và trả về kết quả đó. Hoàn thiện
`chayHarnessCoFallbackTrenNTacVu` — với MỖI trong `soTacVu` tác vụ: tạo
MỘT `taoToolLuonThatBai()` (tool chính) VÀ MỘT `taoToolDuPhong()` (tool
dự phòng) MỚI, gọi `goiToolCoFallback` trên cặp đó, đẩy kết quả vào
mảng trả về.

```typescript title=starter
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function taoToolLoiTamThoi(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  ___
}

function chayHarnessCoFallbackTrenNTacVu(soTacVu: number): KetQuaGoiTool[] {
  ___
}

const ketQua5 = chayHarnessCoFallbackTrenNTacVu(5);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
```

```typescript title=solution
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
}

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: false, loi: "loi_vinh_vien" };
    },
  };
}

function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      return { thanhCong: true, giaTri: "gia_tri_mac_dinh" };
    },
  };
}

function taoToolLoiTamThoi(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return {
    trangThai,
    goi(): KetQuaGoiTool {
      trangThai.soLanDaGoi++;
      if (trangThai.soLanDaGoi === 1) {
        return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      }
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  const ketQuaChinh = goiToolCoRetry(toolChinh);
  if (ketQuaChinh.thanhCong) return ketQuaChinh;
  return toolDuPhong.goi();
}

function chayHarnessCoFallbackTrenNTacVu(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const toolChinh = taoToolLuonThatBai();
    const toolDuPhong = taoToolDuPhong();
    ketQua.push(goiToolCoFallback(toolChinh, toolDuPhong));
  }
  return ketQua;
}

const ketQua5 = chayHarnessCoFallbackTrenNTacVu(5);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
```

```typescript title=test
if (ketQua5.length !== 5) throw new Error("chayHarnessCoFallbackTrenNTacVu(5) phai tra ve mang dung 5 phan tu");
if (JSON.stringify(ketQua5.map((k) => k.thanhCong)) !== JSON.stringify([true, true, true, true, true])) {
  throw new Error("fallback phai cuu duoc CA 5 tac vu du tool chinh hong vinh vien");
}
for (const k of ketQua5) {
  if (!k.thanhCong) throw new Error("fallback phai cuu duoc CA 5 tac vu du tool chinh hong vinh vien");
  if (k.giaTri !== "gia_tri_mac_dinh") {
    throw new Error("moi ket qua phai la GIA TRI MAC DINH (tu tool du phong), khong phai ket_qua_that -- tool chinh hong hoan toan");
  }
}
if (tinhTyLeHoanThanh(ketQua5) !== 1) throw new Error("completion rate voi fallback phai la 1 (100%) du tool chinh vo dung");

const ketQua2 = chayHarnessCoFallbackTrenNTacVu(2);
if (ketQua2.length !== 2) throw new Error("doi so tac vu tu 5 sang 2 phai doi do dai mang tra ve -- tham so phai duoc dung that");

const toolChinhRieng = taoToolLuonThatBai();
const toolDuPhongRieng = taoToolDuPhong();
const kqRieng = goiToolCoFallback(toolChinhRieng, toolDuPhongRieng);
if (!kqRieng.thanhCong) throw new Error("goiToolCoFallback phai thanh cong qua fallback khi tool chinh hong vinh vien");
if (kqRieng.giaTri !== "gia_tri_mac_dinh") throw new Error("ket qua fallback phai la gia_tri_mac_dinh");
if (toolChinhRieng.trangThai.soLanDaGoi !== 2) throw new Error("tool chinh phai duoc thu qua goiToolCoRetry DUNG 2 lan (lan dau + 1 lan thu lai) truoc khi fallback");
if (toolDuPhongRieng.trangThai.soLanDaGoi !== 1) throw new Error("tool du phong phai duoc goi DUNG 1 lan khi can fallback");

const toolChinhOn = taoToolLoiTamThoi();
const toolDuPhongOn = taoToolDuPhong();
const kqOn = goiToolCoFallback(toolChinhOn, toolDuPhongOn);
if (!kqOn.thanhCong) throw new Error("neu tool chinh TU PHUC HOI qua retry, ket qua phai THANH CONG");
if (kqOn.giaTri !== "ket_qua_that") throw new Error("neu tool chinh TU PHUC HOI qua retry, ket qua phai la ket_qua_that (KHONG dung fallback)");
if (toolDuPhongOn.trangThai.soLanDaGoi !== 0) throw new Error("tool du phong KHONG duoc goi khi tool chinh da thanh cong qua retry -- fallback chi la phuong an CUOI CUNG");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (goiToolCoFallback): goi goiToolCoRetry(toolChinh) luu vao mot bien; neu no thanh cong thi return NGAY bien do; nguoc lai goi toolDuPhong.goi() va return ket qua do. Cho hai (chayHarnessCoFallbackTrenNTacVu): mot vong for tao CA HAI tool (chinh + du phong) MOI moi lan, goi goiToolCoFallback tren cap do, day ket qua vao mang."
- kind: strategy
  body: "Cho dau: const ketQuaChinh = goiToolCoRetry(toolChinh); if (ketQuaChinh.thanhCong) return ketQuaChinh; return toolDuPhong.goi(); Cho hai: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const toolChinh = taoToolLuonThatBai(); const toolDuPhong = taoToolDuPhong(); ketQua.push(goiToolCoFallback(toolChinh, toolDuPhong)); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu tao hai tool roi moi goi goiToolCoFallback."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[true,true,true,true,true] 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tool chính hoàn toàn hỏng — nhưng completion rate VẪN `1`. Fallback
đổi lấy điều đó bằng CHẤT LƯỢNG: mọi kết quả đều LÀ giá trị mặc định,
không phải giá trị thật. Nhưng cả retry VÀ fallback Ở hai bài này đều
LUÔN dừng lại sau một số lần thử CỐ ĐỊNH — bài sau tách RIÊNG câu hỏi
đó: ai chặn một tool THẬT SỰ không bao giờ dừng gọi lại?
::::

::::reflect{#nghi-lai}
Fallback KHÔNG phải LÀ retry mạnh hơn — nó LÀ một chiến lược khác hẳn:
retry đặt cược tool chính sẽ TỰ khỏi; fallback thừa nhận tool chính CÓ
THỂ không bao giờ khỏi, VÀ chuẩn bị sẵn một phương án khác để completion
rate không phụ thuộc vào một điểm hỏng DUY NHẤT. Nhưng phương án đó có
giá — `"gia_tri_mac_dinh"` không phải `"ket_qua_that"`. Đối chứng
`taoToolLoiTamThoi` Ở cuối bài quan trọng ngang bài chính: một harness
fallback ĐÚNG đắn phải BIẾT khi nào KHÔNG cần fallback — gọi tool dự
phòng một cách bừa bãi (kể cả khi tool chính đã ổn) làm hỏng chính lợi
ích của retry Ở bài trước.
::::

::::checkpoint{mastery=0.85}
::::
