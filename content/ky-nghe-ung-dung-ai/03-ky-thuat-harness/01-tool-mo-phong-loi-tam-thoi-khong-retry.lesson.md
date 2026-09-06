---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.tool-mo-phong-loi-tam-thoi-khong-retry
title: "Tool mô phỏng có lỗi tạm thời — harness KHÔNG retry"
summary: "taoToolLoiTamThoi() dựng MỘT tool rule-based có trạng thái (đếm soLanDaGoi qua closure): LẦN GỌI ĐẦU TIÊN của MỖI tool mới LUÔN thất bại (mô phỏng rate-limit/timeout tạm thời), lần gọi thứ hai trở đi LUÔN thành công. goiToolKhongRetry(tool) gọi tool ĐÚNG MỘT LẦN, không thử lại. chayHarnessTrenNTacVu(soTacVu) chạy harness này trên soTacVu tác vụ, MỖI tác vụ dùng MỘT tool MỚI (trạng thái không rò rỉ giữa các tác vụ). Trên 5 tác vụ: cả 5 đều thất bại (mỗi tool mới luôn thất bại đúng ở lần gọi đầu) — tyLeHoanThanh = 0/5 = 0. Đây là baseline: model/tool không đổi, nhưng harness không có bất kỳ cơ chế phục hồi nào."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [kna.tool-mo-phong-loi-tam-thoi-khong-retry]
requires: [kna.boss-ky-nghe-hoa-ngu-canh]
concepts: [kna.tool-mo-phong-loi-tam-thoi-khong-retry]
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
Realm 9 tới đây đã học prompt VÀ ngữ cảnh — nhưng ngay cả prompt hoàn
hảo VÀ ngữ cảnh gọn gàng cũng KHÔNG cứu được một agent khi chính TOOL
nó gọi báo lỗi. Quest này bắt đầu Ở đúng chỗ đó: khi tool trả lỗi,
CHƯƠNG TRÌNH bao quanh model — harness — phải quyết định phải làm gì.
Bài đầu tiên dựng một tool LUÔN hỏng Ở lần gọi đầu, VÀ một harness đơn
giản nhất có thể: gọi đúng một lần, không hỏi han gì thêm.
::::

::::explain{#tool-mo-phong-co-trang-thai}
Một "tool" trong quest này KHÔNG gọi mạng, KHÔNG gọi model thật — nó LÀ
một hàm TypeScript tất định, có THỂ cố ý thất bại theo một QUY LUẬT rõ
ràng. `taoToolLoiTamThoi` dựng một tool như vậy: nó đóng gói một
`TrangThaiTool` (đếm `soLanDaGoi`) qua closure, VÀ quy luật của nó LÀ
**lần gọi đầu tiên LUÔN thất bại, lần gọi thứ hai trở đi LUÔN thành
công** — mô phỏng đúng dáng một lỗi TẠM THỜI (rate-limit, timeout) mà
hệ thống thật hay gặp:

```typescript title=readonly
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
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

const mot = taoToolLoiTamThoi();
console.log(JSON.stringify(mot.goi()));
console.log(JSON.stringify(mot.goi()));
console.log(JSON.stringify(mot.goi()));
```

```text title=readonly
{"thanhCong":false,"loi":"loi_tam_thoi_rate_limit"}
{"thanhCong":true,"giaTri":"ket_qua_that"}
{"thanhCong":true,"giaTri":"ket_qua_that"}
```

`trangThai` nằm NGOÀI object trả về nhưng vẫn được tham chiếu bên
trong `goi()` qua closure — mỗi lần gọi `taoToolLoiTamThoi()` tạo một
`trangThai` HOÀN TOÀN mới, không chia sẻ với bất kỳ tool nào khác. Đây
LÀ điều kiện bắt buộc để đo công bằng: nếu hai tool CHIA SẺ chung một
`trangThai`, "lần gọi đầu tiên" của tool THỨ HAI thật ra LÀ lần gọi
THỨ BA của trạng thái dùng chung — sai hoàn toàn quy luật đã định
nghĩa. Không hề dùng `Math.random()` hay `Date.now()` — quy luật thất
bại HOÀN TOÀN tất định, chỉ phụ thuộc `soLanDaGoi`.
::::

::::example{#harness_khong_retry_tren_n_tac_vu}
`goiToolKhongRetry` LÀ harness ĐƠN GIẢN nhất có thể: gọi tool đúng
MỘT LẦN, trả về kết quả — dù thành công hay thất bại — KHÔNG thử lại:

```typescript title=readonly
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
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

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  return tool.goi();
}

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

const SO_TAC_VU = 5;
const ketQuaKhongRetry: KetQuaGoiTool[] = [];
for (let i = 0; i < SO_TAC_VU; i++) {
  const tool = taoToolLoiTamThoi();
  ketQuaKhongRetry.push(goiToolKhongRetry(tool));
}
console.log(JSON.stringify(ketQuaKhongRetry.map((k) => k.thanhCong)));
console.log(tinhTyLeHoanThanh(ketQuaKhongRetry));
```

```text title=readonly
[false,false,false,false,false]
0
```

MỖI tác vụ trong 5 tác vụ dùng một `taoToolLoiTamThoi()` MỚI — nghĩa
là MỖI tác vụ gặp ĐÚNG "lần gọi đầu tiên" của tool riêng nó, VÀ lần đó
LUÔN thất bại theo đúng quy luật đã định nghĩa Ở bài trên. Completion
rate rơi thẳng xuống `0` — KHÔNG phải vì model/tool tệ (tool THẬT SỰ
thành công từ lần gọi thứ hai trở đi), mà vì HARNESS không hề cho nó
CƠ HỘI thứ hai đó.
::::

::::predict{#doan-tang-n-tac-vu commitOnce}
Nếu tăng `SO_TAC_VU` từ `5` lên `100` (nhưng VẪN dùng `taoToolLoiTamThoi()`
MỚI cho MỖI tác vụ, đúng như hiện tại) — `tinhTyLeHoanThanh` trên 100
tác vụ đó LÀ bao nhiêu?

:::opt{correct}
Vẫn LÀ `0` — mỗi tác vụ vẫn dùng một tool HOÀN TOÀN mới, trạng thái
KHÔNG tích luỹ giữa các tác vụ, nên tác vụ nào cũng gặp đúng "lần gọi
đầu tiên" LUÔN thất bại của CHÍNH nó, bất kể tổng số tác vụ LÀ bao
nhiêu
:::
:::opt
Giảm dần về gần `0` nhưng không bằng `0` — càng nhiều tác vụ, càng có
VÀI tác vụ may mắn thành công
::why
Nhầm rằng có yếu tố NGẪU NHIÊN nào đó trong tool — nhưng
`taoToolLoiTamThoi` HOÀN TOÀN tất định, không dùng `Math.random()`
hay bất kỳ nguồn ngẫu nhiên nào.

Chỗ lệch: quy luật thất bại chỉ phụ thuộc `soLanDaGoi` CỦA CHÍNH tool
đó — VÀ vì mỗi tác vụ tạo một tool MỚI (bắt đầu lại từ `soLanDaGoi =
0`), TẤT CẢ 100 tác vụ đều gặp đúng "lần gọi đầu tiên", không có tác
vụ nào "may mắn" khác đi.
::
:::
:::opt
Tăng dần lên gần `1` — sau nhiều tác vụ, hệ thống "học" được cách
tránh lỗi tạm thời
::why
Nhầm tool rule-based với một hệ thống có khả năng HỌC — nhưng
`taoToolLoiTamThoi` không hề lưu lại bất cứ điều gì XUYÊN QUA các lần
gọi `taoToolLoiTamThoi()` khác nhau.

Chỗ lệch: mỗi lần gọi `taoToolLoiTamThoi()` tạo một `trangThai` MỚI,
độc lập hoàn toàn Ở mức `{ soLanDaGoi: 0 }` — không có bộ nhớ nào nối
giữa tác vụ này với tác vụ trước.
::
:::
::::

::::code{#viet_goi_tool_khong_retry}
Hoàn thiện `goiToolKhongRetry` — gọi `tool.goi()` VÀ trả thẳng kết quả
(không kiểm tra, không thử lại). Hoàn thiện `chayHarnessTrenNTacVu` —
với MỖI trong `soTacVu` tác vụ: tạo một `taoToolLoiTamThoi()` MỚI, gọi
`goiToolKhongRetry` trên nó, đẩy kết quả vào mảng trả về.

```typescript title=starter
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };

type TrangThaiTool = { soLanDaGoi: number };

interface ToolMoPhong {
  trangThai: TrangThaiTool;
  goi(): KetQuaGoiTool;
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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  ___
}

function chayHarnessTrenNTacVu(soTacVu: number): KetQuaGoiTool[] {
  ___
}

const ketQua5 = chayHarnessTrenNTacVu(5);
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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolKhongRetry(tool: ToolMoPhong): KetQuaGoiTool {
  return tool.goi();
}

function chayHarnessTrenNTacVu(soTacVu: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoToolLoiTamThoi();
    ketQua.push(goiToolKhongRetry(tool));
  }
  return ketQua;
}

const ketQua5 = chayHarnessTrenNTacVu(5);
console.log(JSON.stringify(ketQua5.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua5));
```

```typescript title=test
if (ketQua5.length !== 5) throw new Error("chayHarnessTrenNTacVu(5) phai tra ve mang dung 5 phan tu");
if (JSON.stringify(ketQua5.map((k) => k.thanhCong)) !== JSON.stringify([false, false, false, false, false])) {
  throw new Error("moi tac vu dung MOT tool MOI, lan goi dau tien LUON that bai -- ca 5 phai la false");
}
if (tinhTyLeHoanThanh(ketQua5) !== 0) throw new Error("completion rate tren 5 tac vu KHONG retry phai la 0 (gan 0%)");

const ketQua3 = chayHarnessTrenNTacVu(3);
if (ketQua3.length !== 3) throw new Error("doi so tac vu tu 5 sang 3 phai doi do dai mang tra ve -- tham so phai duoc dung that");

const toolRieng = taoToolLoiTamThoi();
const kq1 = goiToolKhongRetry(toolRieng);
if (kq1.thanhCong !== false) throw new Error("goiToolKhongRetry goi tool MOI lan dau phai that bai (dung quy luat cua taoToolLoiTamThoi)");
if (toolRieng.trangThai.soLanDaGoi !== 1) throw new Error("goiToolKhongRetry phai goi tool DUNG MOT LAN, khong hon -- day la diem khac biet voi bai sau (co retry)");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (goiToolKhongRetry): chi can goi tool.goi() va return thang ket qua, khong kiem tra gi them. Cho hai (chayHarnessTrenNTacVu): mot vong for chay dung soTacVu lan, moi lan TAO MOT tool bang taoToolLoiTamThoi(), roi day ket qua cua goiToolKhongRetry(tool) vao mot mang, cuoi cung return mang do."
- kind: strategy
  body: "Cho dau: return tool.goi(); Cho hai: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const tool = taoToolLoiTamThoi(); ketQua.push(goiToolKhongRetry(tool)); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu tao tool roi moi goi goiToolKhongRetry."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[false,false,false,false,false] 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
5 tác vụ, 5 lần thất bại, completion rate `0`. Tool THẬT SỰ vẫn thành
công từ lần gọi thứ hai — nhưng `goiToolKhongRetry` không bao giờ cho
nó cơ hội đó. Bài sau đổi ĐÚNG MỘT dòng harness — thêm một lần thử lại
— và xem completion rate đổi thế nào, dù model VÀ tool giữ NGUYÊN
không đổi một chữ.
::::

::::reflect{#nghi-lai}
Điều quan trọng nhất bài này không nằm Ở `taoToolLoiTamThoi` — nó nằm
Ở khoảng cách giữa NĂNG LỰC THẬT của tool (thành công từ lần gọi thứ
hai) VÀ kết quả QUAN SÁT được (completion rate `0`). Model tốt, tool
tốt, nhưng harness đơn giản nhất — gọi đúng một lần rồi dừng — biến
một hệ thống CÓ THỂ hoạt động thành một hệ thống LUÔN thất bại. Đây
chính LÀ luận điểm mở đầu của T9.3: harness — chương trình bao quanh
model — LÀ nơi lỗi "model+prompt+context đều tốt vẫn fail" thật sự
nằm Ở đó, KHÔNG nằm Ở model.
::::

::::checkpoint{mastery=0.8}
::::
