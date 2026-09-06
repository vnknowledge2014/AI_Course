---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.rang-buoc-so-lan-thu-lai-toi-da
title: "Ràng buộc số lần thử lại — chặn vòng lặp vô hạn"
summary: "goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa) tổng quát hoá goiToolCoRetry (bài 2, luôn đúng 1 lần) thành MỘT NGÂN SÁCH bất kỳ: gọi tool 1 lần, rồi thử lại LIÊN TỤC cho tới khi thành công HOẶC đã thử lại đủ soLanThuLaiToiDa lần thì DỪNG. Trên taoToolLuonThatBai (bài 3, hỏng vĩnh viễn, KHÔNG dùng fallback ở bài này): với soLanThuLaiToiDa=3, tool bị gọi ĐÚNG 4 lần (1 lần đầu + 3 lần thử lại) rồi trả về thất bại -- KHÔNG lặp vô hạn. Đổi soLanThuLaiToiDa sang 0 => đúng 1 lần gọi; sang 5 => đúng 6 lần gọi -- xác nhận tham số THẬT SỰ ràng buộc số lần gọi, không phải hằng số ẩn."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.rang-buoc-so-lan-thu-lai-toi-da]
requires: [kna.fallback-sang-tool-du-phong]
concepts: [kna.rang-buoc-so-lan-thu-lai-toi-da]
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
`goiToolCoRetry` Ở bài `2` LUÔN thử lại ĐÚNG một lần — con số đó bị
"đóng cứng" ngay trong thân hàm. Nhưng một hệ thống thật cần TỰ CHỌN
con số đó: một tool quan trọng có thể đáng thử lại `5` lần, một tool rẻ
tiền chỉ đáng thử `1` lần. Bài này tách con số đó ra thành một THAM SỐ
— VÀ xử lý đúng câu hỏi mà "thử lại nhiều lần" luôn kéo theo: nếu tool
KHÔNG BAO GIỜ thành công, cái gì chặn vòng lặp lại KHÔNG chạy mãi?
::::

::::explain{#ngan_sach_thu_lai}
`goiToolCoRetryCoGioiHan` gọi tool một lần, rồi LẶP: nếu kết quả VẪN
thất bại VÀ số lần đã thử lại CHƯA đạt `soLanThuLaiToiDa`, gọi thêm
một lần VÀ tăng bộ đếm. Vòng lặp dừng ngay khi MỘT trong hai điều xảy
ra — thành công, HOẶC đã thử lại đủ số lần cho phép:

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

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

const tool3 = taoToolLuonThatBai();
const kq3 = goiToolCoRetryCoGioiHan(tool3, 3);
console.log(JSON.stringify(kq3), tool3.trangThai.soLanDaGoi);
```

```text title=readonly
{"thanhCong":false,"loi":"loi_vinh_vien"} 4
```

`taoToolLuonThatBai` KHÔNG BAO GIỜ thành công — vậy vòng `while` không
bao giờ dừng nhờ điều kiện ĐẦU (`!ketQua.thanhCong` luôn đúng). Điều
DỪNG nó lại LÀ điều kiện THỨ HAI: `soLanDaThuLai < soLanThuLaiToiDa`.
Với ngân sách `3`, tool bị gọi ĐÚNG `4` lần — một lần đầu, cộng `3` lần
thử lại — rồi DỪNG, trả về thất bại. Không phải vòng lặp vô hạn, cũng
không phải một con số bí ẩn: đúng `soLanThuLaiToiDa + 1`.
::::

::::example{#doi_ngan_sach_doi_so_lan_goi}
Đổi `soLanThuLaiToiDa` VÀ quan sát số lần gọi thay đổi THEO ĐÚNG công
thức `soLanThuLaiToiDa + 1` — CHỨNG MINH tham số này thật sự ràng buộc,
không phải một con số trang trí:

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

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

const tool0 = taoToolLuonThatBai();
goiToolCoRetryCoGioiHan(tool0, 0);
console.log("ngan sach 0:", tool0.trangThai.soLanDaGoi);

const tool5 = taoToolLuonThatBai();
goiToolCoRetryCoGioiHan(tool5, 5);
console.log("ngan sach 5:", tool5.trangThai.soLanDaGoi);
```

```text title=readonly
ngan sach 0: 1
ngan sach 5: 6
```

Ngân sách `0` (không cho thử lại) vẫn gọi tool ĐÚNG một lần — lần gọi
ĐẦU TIÊN luôn xảy ra trước khi vòng `while` được xét tới. Ngân sách `5`
gọi tool `6` lần. KHÔNG có con số nào Ở đây bị "đóng cứng": đổi tham
số LÀ đổi hành vi thật.
::::

::::predict{#doan-ngan-sach-0 commitOnce}
Với `soLanThuLaiToiDa = 0` (không cho phép thử lại nào), gọi
`goiToolCoRetryCoGioiHan` trên một `taoToolLuonThatBai()` MỚI — tool
đó bị gọi tất cả bao nhiêu lần?

:::opt{correct}
Đúng `1` lần — dòng `let ketQua = tool.goi();` LUÔN chạy trước tiên
(ngoài vòng `while`); điều kiện `soLanDaThuLai (0) < soLanThuLaiToiDa
(0)` LÀ `false` ngay từ đầu, nên vòng `while` không chạy lần nào
:::
:::opt
`0` lần — ngân sách `0` nghĩa LÀ không được phép gọi tool
::why
Nhầm "không được thử LẠI" VỚI "không được gọi LẦN NÀO CẢ" — nhưng
lần gọi ĐẦU TIÊN không phải LÀ một lần "thử lại", nó LÀ lần gọi CHÍNH,
xảy ra TRƯỚC khi bất kỳ ngân sách nào được xét tới.

Chỗ lệch: dòng `let ketQua = tool.goi();` nằm HOÀN TOÀN ngoài, VÀ
trước, vòng `while` — nó LUÔN thực thi, bất kể `soLanThuLaiToiDa` LÀ
bao nhiêu.
::
:::
:::opt
Lặp vô hạn — vì `taoToolLuonThatBai` không bao giờ thành công, điều
kiện dừng của `while` không bao giờ đúng
::why
Nhầm một Ở HAI điều kiện dừng của `while` VỚI TOÀN BỘ điều kiện — vòng
lặp dừng khi MỘT TRONG HAI vế của `&&`... à không, đây LÀ `&&` giữa
`!ketQua.thanhCong` VÀ `soLanDaThuLai < soLanThuLaiToiDa`, vòng lặp
CHẠY TIẾP chỉ khi CẢ HAI đúng, VÀ dừng ngay khi MỘT trong hai SAI.

Chỗ lệch: đúng LÀ `!ketQua.thanhCong` luôn `true` (tool không bao giờ
thành công) — nhưng `soLanDaThuLai < soLanThuLaiToiDa` LÀ `0 < 0`, đã
SAI ngay từ đầu, nên `&&` sai, vòng lặp không chạy một vòng nào.
::
:::
::::

::::code{#viet_goi_tool_co_retry_co_gioi_han}
Hoàn thiện `goiToolCoRetryCoGioiHan` — gọi `tool.goi()` một lần lưu
vào `ketQua`; sau đó lặp `while`: NẾU `ketQua` chưa thành công VÀ số
lần đã thử lại còn nhỏ hơn `soLanThuLaiToiDa`, gọi lại `tool.goi()`
(cập nhật `ketQua`) VÀ tăng bộ đếm; cuối cùng trả về `ketQua`. Hoàn
thiện `chayHarnessRetryBudgetTrenNTacVu` — với MỖI trong `soTacVu` tác
vụ: tạo một `taoToolLuonThatBai()` MỚI, gọi
`goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa)`, đẩy kết quả vào mảng
trả về.

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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  ___
}

function chayHarnessRetryBudgetTrenNTacVu(soTacVu: number, soLanThuLaiToiDa: number): KetQuaGoiTool[] {
  ___
}

const ketQua4 = chayHarnessRetryBudgetTrenNTacVu(4, 3);
console.log(JSON.stringify(ketQua4.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua4));
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

function tinhTyLeHoanThanh(ketQua: KetQuaGoiTool[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function goiToolCoRetryCoGioiHan(tool: ToolMoPhong, soLanThuLaiToiDa: number): KetQuaGoiTool {
  let ketQua = tool.goi();
  let soLanDaThuLai = 0;
  while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) {
    ketQua = tool.goi();
    soLanDaThuLai++;
  }
  return ketQua;
}

function chayHarnessRetryBudgetTrenNTacVu(soTacVu: number, soLanThuLaiToiDa: number): KetQuaGoiTool[] {
  const ketQua: KetQuaGoiTool[] = [];
  for (let i = 0; i < soTacVu; i++) {
    const tool = taoToolLuonThatBai();
    ketQua.push(goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa));
  }
  return ketQua;
}

const ketQua4 = chayHarnessRetryBudgetTrenNTacVu(4, 3);
console.log(JSON.stringify(ketQua4.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua4));
```

```typescript title=test
if (ketQua4.length !== 4) throw new Error("chayHarnessRetryBudgetTrenNTacVu(4, 3) phai tra ve mang dung 4 phan tu");
if (JSON.stringify(ketQua4.map((k) => k.thanhCong)) !== JSON.stringify([false, false, false, false])) {
  throw new Error("taoToolLuonThatBai khong bao gio thanh cong -- ca 4 tac vu phai la false, bai nay KHONG co fallback");
}
if (tinhTyLeHoanThanh(ketQua4) !== 0) throw new Error("completion rate phai la 0 -- retry-budget KHONG cuu duoc tool hong vinh vien, chi CHAN vong lap vo han");

const ketQua2 = chayHarnessRetryBudgetTrenNTacVu(2, 3);
if (ketQua2.length !== 2) throw new Error("doi so_tac_vu tu 4 sang 2 phai doi do dai mang tra ve -- tham so phai duoc dung that");

const toolNganSach3 = taoToolLuonThatBai();
const kqNganSach3 = goiToolCoRetryCoGioiHan(toolNganSach3, 3);
if (kqNganSach3.thanhCong !== false) throw new Error("tool hong vinh vien phai van THAT BAI sau khi het ngan sach");
if (toolNganSach3.trangThai.soLanDaGoi !== 4) throw new Error("ngan sach 3 phai goi tool DUNG 4 lan (1 lan dau + 3 lan thu lai), khong hon khong kem");

const toolNganSach0 = taoToolLuonThatBai();
goiToolCoRetryCoGioiHan(toolNganSach0, 0);
if (toolNganSach0.trangThai.soLanDaGoi !== 1) throw new Error("ngan sach 0 phai VAN goi tool DUNG 1 lan (lan dau khong tinh la thu lai), khong phai 0 lan");

const toolNganSach5 = taoToolLuonThatBai();
goiToolCoRetryCoGioiHan(toolNganSach5, 5);
if (toolNganSach5.trangThai.soLanDaGoi !== 6) throw new Error("doi ngan sach tu 3 sang 5 phai doi so lan goi thanh 6 (1 + 5) -- tham so PHAI thuc su rang buoc so lan goi");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (goiToolCoRetryCoGioiHan): let ketQua = tool.goi(); let soLanDaThuLai = 0; roi mot vong while: dieu kien LA !ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa, than vong CAP NHAT ketQua = tool.goi() va tang soLanDaThuLai; cuoi cung return ketQua. Cho hai (chayHarnessRetryBudgetTrenNTacVu): giong bai truoc nhung nhan THEM tham so soLanThuLaiToiDa va truyen no vao goiToolCoRetryCoGioiHan."
- kind: strategy
  body: "Cho dau: let ketQua = tool.goi(); let soLanDaThuLai = 0; while (!ketQua.thanhCong && soLanDaThuLai < soLanThuLaiToiDa) { ketQua = tool.goi(); soLanDaThuLai++; } return ketQua; Cho hai: const ketQua: KetQuaGoiTool[] = []; for (let i = 0; i < soTacVu; i++) { const tool = taoToolLuonThatBai(); ketQua.push(goiToolCoRetryCoGioiHan(tool, soLanThuLaiToiDa)); } return ketQua;"
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
  expect: "[false,false,false,false] 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`soLanThuLaiToiDa + 1` lần gọi, đúng ĐẾN TỪNG con số, dù đổi ngân sách
LÀ `0`, `3` hay `5`. Không có vòng lặp vô hạn nào — VÀ cũng không có
fallback nào Ở bài này để "cứu" completion rate: retry-budget CHỈ làm
đúng MỘT việc, chặn một vòng lặp gọi lại không có điểm dừng. Bài sau
đặt tên chính thức cho cách biểu diễn "thành công hoặc thất bại" mà
MỌI hàm Ở bốn bài vừa qua đều dùng, không hề gọi tới `throw`.
::::

::::reflect{#nghi-lai}
Retry-budget tách khỏi retry (bài `2`) VÀ fallback (bài `3`) một câu
hỏi RIÊNG: không phải "làm sao PHỤC HỒI" mà LÀ "làm sao ĐẢM BẢO một
hệ thống thử-lại không BAO GIỜ chạy mãi". Hai bài trước LUÔN thử lại
đúng một số lần CỐ ĐỊNH (một trong `goiToolCoRetry`, hai trong
`goiToolCoFallback`) — con số đó ẩn trong code. Bài này LÀM RÕ con số
đó bằng cách biến nó thành một THAM SỐ tường minh, VÀ chứng minh (bằng
ba giá trị `0`/`3`/`5`) rằng tham số đó thật sự điều khiển hành vi,
không phải một con số trang trí không ai kiểm tra.
::::

::::checkpoint{mastery=0.85}
::::
