---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.goi-tool-la-kleisli-retry-la-alternative
title: "Cầu nối FP — gọi tool LÀ Kleisli arrow, retry/fallback LÀ Alternative"
summary: "Cầu nối FP (dựa trên chapter-28-applicative-validation.chapter.md, phần 'ap' -- Applicative kết hợp NHIỀU phép tính ĐỘC LẬP, KHÁC hẳn Monad chain tuần tự): một 'Kleisli arrow' LÀ dạng tối giản một hàm (a: A) => KetQua<B, E> -- nhận giá trị THƯỜNG, trả về giá trị LỒNG trong KetQua (đã học T9.3 bài 5). kleisliCompose(k1, k2) = (a) => andThen(k1(a), k2) GHÉP HAI Kleisli arrow thành MỘT arrow mới -- verify bằng số: kleisliCompose lặp lại ĐÚNG chuỗi 3 bước goiToolAnToan→parseKetQua→dinhDangLai của T9.3 bài 5 (demGoi khớp (1,1,1)/(1,1,0)/(1,0,0) trên ba đầu vào so_hop_le/khong_hop_le/loi_tool). 'Alternative' (<|> trong Haskell -- thử nhánh A, KHÔNG được thì thử nhánh B, GIỮ MỘT kết quả DUY NHẤT, KHÁC Applicative gộp CẢ HAI lỗi) LÀ chính xác hình dạng chonMotTrongHai(a, b) = a() thành công thì giữ, không thì thử b() -- verify: goiToolCoFallback (T9.3 bài 3) viết lại QUA chonMotTrongHai cho ĐÚNG kết quả cũ (tool chính hỏng vĩnh viễn → gia_tri_mac_dinh qua nhánh b; tool chính tự phục hồi qua retry → nhánh b KHÔNG BAO GIỜ bị gọi, soLanDaGoi=0)."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.goi-tool-la-kleisli-retry-la-alternative]
requires: [kna.vong-lap-goi-tool-co-retry]
concepts: [kna.goi-tool-la-kleisli-retry-la-alternative]
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
T9.3 bài `5` đặt tên "ROP" cho `KetQua<T, E>` VÀ `andThen`. Bài này đi
xa hơn MỘT bước: cái HÌNH DẠNG của `goiToolAnToan`, `parseKetQua`,
`dinhDangLai` — mỗi hàm nhận giá trị thường, trả về giá trị LỒNG trong
`KetQua` — có tên riêng trong lý thuyết phạm trù: Kleisli arrow. VÀ cái
hình dạng của `goiToolCoFallback` — thử tool chính, không được thì thử
tool dự phòng — LÀ đúng hình dạng của một khái niệm HỌ HÀNG VỚI
Applicative (chapter `28`) mà chương đó CHƯA dùng tới: Alternative.
::::

::::explain{#kleisli_arrow_va_kleisli_compose}
`chapter-28-applicative-validation.chapter.md` dạy Applicative: kết hợp
NHIỀU phép tính ĐỘC LẬP, gộp CẢ HAI lỗi nếu cả hai đều thất bại — khác
hẳn Monad `chain` (T9.3 bài `5`), vốn DỪNG NGAY Ở lỗi đầu tiên. Một
"Kleisli arrow" LÀ dạng tối giản của MỘT hàm `(a: A) => KetQua<B, E>` —
đúng hình dạng `goiToolAnToan: (dauVao: string) => KetQua<string, string>`
đã viết Ở T9.3 bài `5`, giờ đặt TÊN CHÍNH THỨC. `kleisliCompose` GHÉP
HAI Kleisli arrow thành MỘT arrow MỚI — qua `andThen` (T9.3 bài `5`),
KHÔNG bọc thêm lớp nào:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
}

type KleisliArrow<A, B, E> = (a: A) => KetQua<B, E>;

function kleisliCompose<A, B, C, E>(
  k1: KleisliArrow<A, B, E>,
  k2: KleisliArrow<B, C, E>,
): KleisliArrow<A, C, E> {
  return (a: A) => andThen(k1(a), k2);
}

const chiaCho2: KleisliArrow<number, number, string> = (n) =>
  n % 2 === 0 ? { thanhCong: true, giaTri: n / 2 } : { thanhCong: false, loi: "so_le_khong_chia_het" };
const congThem10: KleisliArrow<number, number, string> = (n) => ({ thanhCong: true, giaTri: n + 10 });

const chiaRoiCong = kleisliCompose(chiaCho2, congThem10);
console.log(JSON.stringify(chiaRoiCong(8)));
console.log(JSON.stringify(chiaRoiCong(7)));
```

```text title=readonly
{"thanhCong":true,"giaTri":14}
{"thanhCong":false,"loi":"so_le_khong_chia_het"}
```

`chiaRoiCong` LÀ MỘT arrow MỚI, tạo ra bằng cách GHÉP `chiaCho2` VÀ
`congThem10`. `8` chia hết cho `2` → `4`, cộng `10` → `14`, thành công.
`7` LÀ số lẻ → `chiaCho2` thất bại NGAY — `congThem10` KHÔNG BAO GIỜ
được gọi, đúng cách `andThen` "trượt" qua một lỗi. Khác VỚI `andThen`
(nhận MỘT giá trị `KetQua` đã có sẵn), `kleisliCompose` nhận HAI ARROW
(hai HÀM) VÀ trả về MỘT ARROW MỚI — ghép Ở TẦNG HÀM, không phải TẦNG
giá trị.
::::

::::example{#hai_cau_noi_tren_ham_da_hoc}
Chuỗi `3` bước T9.3 bài `5` (`goiToolAnToan` → `parseKetQua` →
`dinhDangLai`, nối bằng `andThen` HAI LẦN, viết TAY) chính LÀ MỘT chuỗi
Kleisli arrow — VÀ `kleisliCompose` tái tạo được kết quả Y HỆT.
`goiToolCoFallback` (T9.3 bài `3`) — thử tool chính QUA retry, không
được thì thử tool dự phòng — chính LÀ hình dạng Alternative: `<|>` của
Haskell, thử nhánh A, KHÔNG được thì thử nhánh B, GIỮ MỘT kết quả DUY
NHẤT (KHÁC Applicative gộp CẢ HAI lỗi):

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
}

type KleisliArrow<A, B, E> = (a: A) => KetQua<B, E>;

function kleisliCompose<A, B, C, E>(
  k1: KleisliArrow<A, B, E>,
  k2: KleisliArrow<B, C, E>,
): KleisliArrow<A, C, E> {
  return (a: A) => andThen(k1(a), k2);
}

const demGoi = { goiTool: 0, parse: 0, dinhDang: 0 };

function goiToolAnToan(dauVao: string): KetQua<string, string> {
  demGoi.goiTool++;
  if (dauVao === "loi_tool") return { thanhCong: false, loi: "tool_that_bai" };
  return { thanhCong: true, giaTri: dauVao === "so_hop_le" ? "42" : "khong_phai_so" };
}
function parseKetQua(chuoiTho: string): KetQua<number, string> {
  demGoi.parse++;
  const n = Number(chuoiTho);
  if (Number.isNaN(n)) return { thanhCong: false, loi: "parse_that_bai_khong_phai_so" };
  return { thanhCong: true, giaTri: n };
}
function dinhDangLai(n: number): KetQua<string, string> {
  demGoi.dinhDang++;
  return { thanhCong: true, giaTri: `Ket qua: ${n}` };
}

// Kleisli composition: GHEP hai arrow thanh MOT arrow moi, LAP LAI dung chuoi 3 buoc T9.3 bai 5.
const chuoiKleisli: KleisliArrow<string, string, string> = kleisliCompose(
  kleisliCompose(goiToolAnToan, parseKetQua),
  dinhDangLai,
);

console.log("kleisli:", JSON.stringify(chuoiKleisli("so_hop_le")), JSON.stringify(demGoi));

// Alternative: <|> -- thu a(), KHONG duoc thi thu b(). CA HAI cung kieu vao/ra.
type KetQuaGoiTool =
  | { thanhCong: true; giaTri: string }
  | { thanhCong: false; loi: string };
type TrangThaiTool = { soLanDaGoi: number };
interface ToolMoPhong { trangThai: TrangThaiTool; goi(): KetQuaGoiTool; }

function taoToolLuonThatBai(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return { trangThai, goi(): KetQuaGoiTool { trangThai.soLanDaGoi++; return { thanhCong: false, loi: "loi_vinh_vien" }; } };
}
function taoToolDuPhong(): ToolMoPhong {
  const trangThai: TrangThaiTool = { soLanDaGoi: 0 };
  return { trangThai, goi(): KetQuaGoiTool { trangThai.soLanDaGoi++; return { thanhCong: true, giaTri: "gia_tri_mac_dinh" }; } };
}
function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function chonMotTrongHai<T, E>(a: () => KetQua<T, E>, b: () => KetQua<T, E>): KetQua<T, E> {
  const ketQuaA = a();
  return ketQuaA.thanhCong ? ketQuaA : b();
}

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  return chonMotTrongHai(
    () => goiToolCoRetry(toolChinh),
    () => toolDuPhong.goi(),
  );
}

const toolChinh = taoToolLuonThatBai();
const toolDuPhongX = taoToolDuPhong();
console.log("fallback:", JSON.stringify(goiToolCoFallback(toolChinh, toolDuPhongX)));
```

```text title=readonly
kleisli: {"thanhCong":true,"giaTri":"Ket qua: 42"} {"goiTool":1,"parse":1,"dinhDang":1}
fallback: {"thanhCong":true,"giaTri":"gia_tri_mac_dinh"}
```

`chuoiKleisli` — được TẠO RA bằng `kleisliCompose` — cho ra ĐÚNG kết quả
T9.3 bài `5` đã viết TAY bằng `andThen` hai lần: `demGoi` khớp
`{goiTool:1, parse:1, dinhDang:1}`. `goiToolCoFallback`, viết QUA
`chonMotTrongHai`, tái tạo ĐÚNG hành vi T9.3 bài `3`: tool chính hỏng
VĨNH VIỄN, nhánh `a` thất bại, nhánh `b` (tool dự phòng) được thử VÀ
thắng — `"gia_tri_mac_dinh"`.
::::

::::predict{#doan-fallback-tu-phuc-hoi commitOnce}
Nếu `goiToolCoFallback` (viết QUA `chonMotTrongHai` Ở trên) được gọi
VỚI tool chính LÀ MỘT `taoToolLoiTamThoi()` (T9.3 bài `1`-`2`: chỉ thất
bại lần đầu, tự phục hồi Ở lần thứ hai) THAY VÌ `taoToolLuonThatBai()`
— nhánh `b` (`toolDuPhong.goi()`) có bị gọi KHÔNG?

:::opt{correct}
Không — nhánh `a` (`() => goiToolCoRetry(toolChinh)`) ĐÃ thành công (retry
cứu được lỗi tạm thời), nên `chonMotTrongHai` `return ketQuaA` NGAY Ở
DÒNG ĐÓ — nhánh `b` KHÔNG BAO GIỜ được gọi, đúng bản chất "SHORT-CIRCUIT"
của Alternative: dừng NGAY khi nhánh ĐẦU thành công
:::
:::opt
Có — `chonMotTrongHai` LUÔN gọi CẢ HAI nhánh để đối chiếu, giống cách
Applicative (chapter `28`) chạy TẤT CẢ trường ĐỘC LẬP cùng lúc để gộp lỗi
::why
Nhầm Alternative (thử A, KHÔNG được thì thử B, DỪNG NGAY khi A thành
công) VỚI Applicative (`validateAll`/`ap` — LUÔN chạy CẢ BA trường ĐỘC
LẬP, gộp lỗi nếu CẢ HAI đều thất bại) — chapter `28` dạy CÁI THỨ HAI:
Applicative CHẠY HẾT để thu thập MỌI lỗi, Alternative DỪNG SỚM khi tìm
được MỘT thành công.

Chỗ lệch: `chonMotTrongHai` chỉ gọi `b()` Ở NHÁNH `else` của phép toán
ba ngôi (`ketQuaA.thanhCong ? ketQuaA : b()`) — khi `ketQuaA.thanhCong`
LÀ `true`, `b` không hề xuất hiện Ở nhánh ĐÓ.
::
:::
:::opt
Không xác định được — thứ tự gọi `a()`/`b()` phụ thuộc việc engine chọn
nhánh nào PHẢN HỒI nhanh hơn
::why
Nhầm VỚI mô hình BẤT ĐỒNG BỘ (race giữa hai lời gọi song song) — nhưng
mọi lời gọi Ở đây LÀ ĐỒNG BỘ, tuần tự: `a()` chạy XONG HOÀN TOÀN trước
khi `chonMotTrongHai` xét tới việc có cần gọi `b()` hay không.

Chỗ lệch: `const ketQuaA = a();` LÀ một lời gọi hàm THƯỜNG, chạy xong
NGAY LẬP TỨC trên CÙNG một luồng — không có khái niệm "nhanh hơn" nào
áp dụng cho hai lời gọi hàm đồng bộ nối tiếp nhau.
::
:::
::::

::::code{#viet_kleisli_va_alternative}
Hoàn thiện `kleisliCompose` — trả về MỘT hàm nhận `a: A`, gọi
`andThen(k1(a), k2)`. Hoàn thiện `chonMotTrongHai` — gọi `a()`; NẾU
thành công, trả về NGAY; NẾU KHÔNG, gọi VÀ trả về `b()`.

```typescript title=starter
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
}

type KleisliArrow<A, B, E> = (a: A) => KetQua<B, E>;

const demGoi = { goiTool: 0, parse: 0, dinhDang: 0 };

function goiToolAnToan(dauVao: string): KetQua<string, string> {
  demGoi.goiTool++;
  if (dauVao === "loi_tool") return { thanhCong: false, loi: "tool_that_bai" };
  return { thanhCong: true, giaTri: dauVao === "so_hop_le" ? "42" : "khong_phai_so" };
}

function parseKetQua(chuoiTho: string): KetQua<number, string> {
  demGoi.parse++;
  const n = Number(chuoiTho);
  if (Number.isNaN(n)) return { thanhCong: false, loi: "parse_that_bai_khong_phai_so" };
  return { thanhCong: true, giaTri: n };
}

function dinhDangLai(n: number): KetQua<string, string> {
  demGoi.dinhDang++;
  return { thanhCong: true, giaTri: `Ket qua: ${n}` };
}

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
      if (trangThai.soLanDaGoi === 1) return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function kleisliCompose<A, B, C, E>(
  k1: KleisliArrow<A, B, E>,
  k2: KleisliArrow<B, C, E>,
): KleisliArrow<A, C, E> {
  ___
}

function chonMotTrongHai<T, E>(a: () => KetQua<T, E>, b: () => KetQua<T, E>): KetQua<T, E> {
  ___
}

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  return chonMotTrongHai(
    () => goiToolCoRetry(toolChinh),
    () => toolDuPhong.goi(),
  );
}

const chuoiKleisli: KleisliArrow<string, string, string> = kleisliCompose(
  kleisliCompose(goiToolAnToan, parseKetQua),
  dinhDangLai,
);

const ketQuaKleisli = chuoiKleisli("so_hop_le");
const demSauKleisli = { ...demGoi };

const toolChinhA = taoToolLuonThatBai();
const toolDuPhongA = taoToolDuPhong();
const ketQuaFallback = goiToolCoFallback(toolChinhA, toolDuPhongA);

console.log(JSON.stringify(ketQuaKleisli), JSON.stringify(demSauKleisli), JSON.stringify(ketQuaFallback));
```

```typescript title=solution
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
}

type KleisliArrow<A, B, E> = (a: A) => KetQua<B, E>;

const demGoi = { goiTool: 0, parse: 0, dinhDang: 0 };

function goiToolAnToan(dauVao: string): KetQua<string, string> {
  demGoi.goiTool++;
  if (dauVao === "loi_tool") return { thanhCong: false, loi: "tool_that_bai" };
  return { thanhCong: true, giaTri: dauVao === "so_hop_le" ? "42" : "khong_phai_so" };
}

function parseKetQua(chuoiTho: string): KetQua<number, string> {
  demGoi.parse++;
  const n = Number(chuoiTho);
  if (Number.isNaN(n)) return { thanhCong: false, loi: "parse_that_bai_khong_phai_so" };
  return { thanhCong: true, giaTri: n };
}

function dinhDangLai(n: number): KetQua<string, string> {
  demGoi.dinhDang++;
  return { thanhCong: true, giaTri: `Ket qua: ${n}` };
}

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
      if (trangThai.soLanDaGoi === 1) return { thanhCong: false, loi: "loi_tam_thoi_rate_limit" };
      return { thanhCong: true, giaTri: "ket_qua_that" };
    },
  };
}

function goiToolCoRetry(tool: ToolMoPhong): KetQuaGoiTool {
  const lanDau = tool.goi();
  if (lanDau.thanhCong) return lanDau;
  return tool.goi();
}

function kleisliCompose<A, B, C, E>(
  k1: KleisliArrow<A, B, E>,
  k2: KleisliArrow<B, C, E>,
): KleisliArrow<A, C, E> {
  return (a: A) => andThen(k1(a), k2);
}

function chonMotTrongHai<T, E>(a: () => KetQua<T, E>, b: () => KetQua<T, E>): KetQua<T, E> {
  const ketQuaA = a();
  return ketQuaA.thanhCong ? ketQuaA : b();
}

function goiToolCoFallback(toolChinh: ToolMoPhong, toolDuPhong: ToolMoPhong): KetQuaGoiTool {
  return chonMotTrongHai(
    () => goiToolCoRetry(toolChinh),
    () => toolDuPhong.goi(),
  );
}

const chuoiKleisli: KleisliArrow<string, string, string> = kleisliCompose(
  kleisliCompose(goiToolAnToan, parseKetQua),
  dinhDangLai,
);

const ketQuaKleisli = chuoiKleisli("so_hop_le");
const demSauKleisli = { ...demGoi };

const toolChinhA = taoToolLuonThatBai();
const toolDuPhongA = taoToolDuPhong();
const ketQuaFallback = goiToolCoFallback(toolChinhA, toolDuPhongA);

console.log(JSON.stringify(ketQuaKleisli), JSON.stringify(demSauKleisli), JSON.stringify(ketQuaFallback));
```

```typescript title=test
if (!ketQuaKleisli.thanhCong) throw new Error("chuoi Kleisli voi dau vao hop le phai THANH CONG");
if (ketQuaKleisli.giaTri !== "Ket qua: 42") throw new Error("gia tri phai la 'Ket qua: 42'");
if (demSauKleisli.goiTool !== 1 || demSauKleisli.parse !== 1 || demSauKleisli.dinhDang !== 1) {
  throw new Error("dau vao hop le phai chay CA BA buoc trong chuoi Kleisli, dung mot lan moi buoc");
}

demGoi.goiTool = 0; demGoi.parse = 0; demGoi.dinhDang = 0;
const ketQuaKleisliLoiParse = chuoiKleisli("khong_hop_le");
if (ketQuaKleisliLoiParse.thanhCong) throw new Error("dau vao khong parse duoc phai THAT BAI");
if (ketQuaKleisliLoiParse.loi !== "parse_that_bai_khong_phai_so") throw new Error("loi phai GIU NGUYEN dung loi cua buoc parse");
if (demGoi.goiTool !== 1 || demGoi.parse !== 1 || demGoi.dinhDang !== 0) {
  throw new Error("loi o buoc parse phai chan buoc dinhDangLai KHONG chay -- dung Kleisli composition qua andThen");
}

demGoi.goiTool = 0; demGoi.parse = 0; demGoi.dinhDang = 0;
const ketQuaKleisliLoiTool = chuoiKleisli("loi_tool");
if (ketQuaKleisliLoiTool.thanhCong) throw new Error("dau vao loi_tool phai THAT BAI ngay buoc dau");
if (ketQuaKleisliLoiTool.loi !== "tool_that_bai") throw new Error("loi phai GIU NGUYEN dung loi cua buoc goi tool");
if (demGoi.goiTool !== 1 || demGoi.parse !== 0 || demGoi.dinhDang !== 0) {
  throw new Error("loi o buoc dau (goi tool) phai chan CA HAI buoc sau KHONG chay");
}

if (!ketQuaFallback.thanhCong) throw new Error("goiToolCoFallback (qua chonMotTrongHai) phai THANH CONG du tool chinh hong vinh vien");
if (ketQuaFallback.giaTri !== "gia_tri_mac_dinh") throw new Error("ket qua fallback phai la gia_tri_mac_dinh");
if (toolChinhA.trangThai.soLanDaGoi !== 2) throw new Error("nhanh a() (goiToolCoRetry) phai goi tool chinh DUNG 2 lan (1 goi + 1 retry) truoc khi chuyen sang nhanh b()");
if (toolDuPhongA.trangThai.soLanDaGoi !== 1) throw new Error("nhanh b() chi duoc goi DUNG 1 lan khi nhanh a() that bai");

const toolChinhTuPhucHoi = taoToolLoiTamThoi();
const toolDuPhongKhongDung = taoToolDuPhong();
const ketQuaTuPhucHoi = goiToolCoFallback(toolChinhTuPhucHoi, toolDuPhongKhongDung);
if (!ketQuaTuPhucHoi.thanhCong) throw new Error("neu tool chinh TU PHUC HOI qua retry, ket qua phai THANH CONG");
if (ketQuaTuPhucHoi.giaTri !== "ket_qua_that") throw new Error("neu tool chinh tu phuc hoi, gia tri phai la ket_qua_that (KHONG dung fallback)");
if (toolDuPhongKhongDung.trangThai.soLanDaGoi !== 0) {
  throw new Error("chonMotTrongHai KHONG duoc goi nhanh b() khi nhanh a() DA thanh cong -- day la ban chat short-circuit cua Alternative");
}

let demA = 0;
let demB = 0;
const ketQuaChonA: KetQua<number, string> = chonMotTrongHai(
  () => { demA++; return { thanhCong: true, giaTri: 1 }; },
  () => { demB++; return { thanhCong: true, giaTri: 2 }; },
);
if (ketQuaChonA.thanhCong !== true || ketQuaChonA.giaTri !== 1) throw new Error("chonMotTrongHai phai tra ve ket qua cua nhanh a() khi a() thanh cong");
if (demA !== 1 || demB !== 0) throw new Error("khi a() thanh cong, chonMotTrongHai KHONG duoc goi b() -- demB phai la 0");

let demA2 = 0;
let demB2 = 0;
const ketQuaChonB: KetQua<number, string> = chonMotTrongHai(
  () => { demA2++; return { thanhCong: false, loi: "a_hong" }; },
  () => { demB2++; return { thanhCong: true, giaTri: 99 }; },
);
if (ketQuaChonB.thanhCong !== true || ketQuaChonB.giaTri !== 99) throw new Error("chonMotTrongHai phai tra ve ket qua cua nhanh b() khi a() that bai");
if (demA2 !== 1 || demB2 !== 1) throw new Error("khi a() that bai, chonMotTrongHai PHAI goi b() dung 1 lan");

const kqCompose3: KleisliArrow<number, number, string> = kleisliCompose(
  (n: number) => ({ thanhCong: true as const, giaTri: n + 1 }),
  (n: number) => ({ thanhCong: true as const, giaTri: n * 10 }),
);
const kqCompose3Ket = kqCompose3(4);
if (!kqCompose3Ket.thanhCong || kqCompose3Ket.giaTri !== 50) throw new Error("kleisliCompose phai noi dung hai arrow: (4+1)*10 = 50");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (kleisliCompose): return mot ham nhan a: A, ben trong goi andThen(k1(a), k2) va return ket qua do. Cho hai (chonMotTrongHai): goi a() luu vao mot bien; neu bien do thanh cong thi return NGAY; nguoc lai goi VA return b()."
- kind: strategy
  body: "Cho dau: return (a: A) => andThen(k1(a), k2); Cho hai: const ketQuaA = a(); return ketQuaA.thanhCong ? ketQuaA : b();"
- kind: one-line
  body: "Sao chep dung hai dong o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"thanhCong\":true,\"giaTri\":\"Ket qua: 42\"} {\"goiTool\":1,\"parse\":1,\"dinhDang\":1} {\"thanhCong\":true,\"giaTri\":\"gia_tri_mac_dinh\"}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không MỘT dòng logic MỚI nào Ở bài này — `kleisliCompose` VÀ
`chonMotTrongHai` chỉ đặt TÊN chính thức cho hai hình dạng T9.3 ĐÃ viết
tay. Bài sau đổi hướng: một TỔNG ngân sách bước cố định giờ phải CHIA
cho NHIỀU tác vụ chạy tuần tự — VÀ chiến lược CHIA quyết định tác vụ
NÀO sống sót.
::::

::::reflect{#nghi-lai}
`kleisliCompose` VÀ `chonMotTrongHai` không thêm khả năng MỚI — MỌI kết
quả Ở bài này đều TÁI TẠO ĐÚNG những gì T9.3 bài `3` VÀ bài `5` đã tính
ra bằng tay. Giá trị của việc ĐẶT TÊN nằm Ở chỗ khác: một khi nhận ra
"chuỗi gọi tool" LÀ Kleisli composition, một kỹ sư có thể ÁP DỤNG bất kỳ
định lý nào đã biết về Kleisli category (ví dụ: composition LÀ kết
hợp — `kleisliCompose(kleisliCompose(f, g), h)` VÀ
`kleisliCompose(f, kleisliCompose(g, h))` LUÔN cho cùng một arrow) mà
KHÔNG cần chứng minh lại từ đầu cho riêng trường hợp tool-calling. VÀ
một khi nhận ra "thử fallback" LÀ Alternative — MỘT khái niệm HỌ HÀNG
VỚI Applicative NHƯNG ứng xử NGƯỢC LẠI khi gặp thành công (Applicative
chạy HẾT để gộp lỗi; Alternative DỪNG NGAY khi tìm được một nhánh thắng)
— một kỹ sư biết ngay lúc nào NÊN dùng cái nào: nhiều field ĐỘC LẬP cần
TẤT CẢ lỗi cùng lúc → Applicative; nhiều PHƯƠNG ÁN chỉ cần MỘT phương án
thắng → Alternative.
::::

::::checkpoint{mastery=0.88}
::::
