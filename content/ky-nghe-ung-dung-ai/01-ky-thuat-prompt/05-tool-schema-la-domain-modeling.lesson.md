---
id: ky-nghe-ung-dung-ai.ky-thuat-prompt.tool-schema-la-domain-modeling
title: "Cầu nối FP: tool schema là Domain Modeling Made Functional"
summary: "type KetQuaGoiTool = {loai:\"thanh_cong\";duLieu:KetQuaPhanLoai} | {loai:\"loi_parse\";chuoiGoc:string} | {loai:\"thieu_khoa\";khoaThieu:string[]} -- discriminated union mo hinh hoa BA cach that bai KHAC NHAU cua mot loi goi tool, khong gop chung thanh null/Error nhu bai 3. goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool phan biet: JSON.parse nem loi hoac ket qua khong phai object -> loi_parse (kem chuoiGoc de debug); parse duoc nhung thieu/sai khoa -> thieu_khoa (kem DANH SACH ten khoa thieu); dung ca hai -> thanh_cong (kem duLieu). Switch tren truong loai la EXHAUSTIVE -- compiler TypeScript bao loi neu thieu mot case. Nguon cam hung: content/legacy/typescript/part-4-ddd/chapter-20-domain-modeling-with-dus.chapter.md (state machine qua discriminated union, 'illegal states unrepresentable')."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-prompt
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.tool-schema-la-domain-modeling]
requires: [kna.few-shot-sua-dinh-dang]
concepts: [kna.tool-schema-la-domain-modeling]
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
`thuParseKetQua` (bài `structured-output-va-schema-json`) trả về
`KetQuaPhanLoai | null` — MỘT giá trị `null` gộp chung MỌI lý do thất
bại: chuỗi không phải JSON, chuỗi LÀ JSON nhưng sai khoá, chuỗi LÀ
`null`... Tất cả trông GIỐNG HỆT nhau Ở phía gọi. Nhưng ba lý do đó cần
BA cách xử lý khác nhau — VÀ đây chính LÀ chỗ TypeScript làm được điều
Python không làm sạch được.
::::

::::explain{#discriminated-union-cho-loi}
`chapter-20-domain-modeling-with-dus.chapter.md` (Realm cầu nối FP) dạy
nguyên tắc "make illegal states unrepresentable" — một discriminated
union chỉ cho phép NHỮNG trạng thái HỢP LỆ tồn tại, mỗi trạng thái
mang ĐÚNG dữ liệu nó cần, không hơn không kém. Ở đó, nguyên tắc NÀY
áp dụng cho một ĐƠN HÀNG (draft/paid/shipped); Ở đây, áp dụng cho MỘT
LỜI GỌI TOOL:

```typescript title=readonly
interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };
```

Ba biến thể (`variant`) — KHÔNG phải MỘT kiểu chung có trường
`optional`. `{ loai: "loi_parse" }` LUÔN đi kèm `chuoiGoc` (chuỗi gốc
để debug); `{ loai: "thieu_khoa" }` LUÔN đi kèm `khoaThieu` (DANH SÁCH
tên khoá còn thiếu, không phải MỘT thông báo lỗi chung chung); `{ loai:
"thanh_cong" }` LUÔN đi kèm `duLieu` ĐÃ được xác nhận đúng hình dạng.
Không có trạng thái nào "thành công nhưng thiếu duLieu", hay "lỗi
parse nhưng vẫn có khoaThieu" — CHÍNH việc đó LÀ "illegal state" bị
loại bỏ NGAY từ định nghĩa kiểu, không cần kiểm tra runtime để phát
hiện.
::::

::::example{#goi_tool_va_switch_exhaustive}
`goiToolPhanLoai` phân biệt RÕ ba trường hợp NGAY trong thân hàm, VÀ
`xuLyKetQuaGoiTool` xử lý TỪNG trường hợp bằng một `switch` EXHAUSTIVE
trên trường `loai` — thiếu MỘT case, TypeScript báo lỗi biên dịch
NGAY, không đợi tới lúc chạy:

```typescript title=readonly
interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}

type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };

function goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoiTraVe);
  } catch {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  if (typeof giaTri !== "object" || giaTri === null) {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  const ghi = giaTri as Record<string, unknown>;
  const coNhiemVu = laNhiemVuHopLe(ghi.nhiemVu);
  const coDoTinCay = typeof ghi.doTinCay === "number";
  if (!coNhiemVu || !coDoTinCay) {
    const khoaThieu: string[] = [];
    if (!coNhiemVu) khoaThieu.push("nhiemVu");
    if (!coDoTinCay) khoaThieu.push("doTinCay");
    return { loai: "thieu_khoa", khoaThieu };
  }
  return {
    loai: "thanh_cong",
    duLieu: { nhiemVu: ghi.nhiemVu as "tom_tat" | "dich" | "dem_so", doTinCay: ghi.doTinCay as number },
  };
}

function xuLyKetQuaGoiTool(kq: KetQuaGoiTool): string {
  switch (kq.loai) {
    case "thanh_cong":
      return `OK: nhiem vu ${kq.duLieu.nhiemVu}, do tin cay ${kq.duLieu.doTinCay}`;
    case "loi_parse":
      return `LOI PARSE: khong phai JSON hop le -- "${kq.chuoiGoc}"`;
    case "thieu_khoa":
      return `THIEU KHOA: ${kq.khoaThieu.join(", ")}`;
    // Bo mot case Ở day (vi du "thieu_khoa") se lam TypeScript bao loi
    // "Function lacks ending return statement" -- switch KHONG con exhaustive.
  }
}

const chuoiHopLe = JSON.stringify({ nhiemVu: "dich", doTinCay: 0.9 });
const chuoiProse = "Toi nghi day la nhiem vu dich, do tin cay khoang 0.9.";
const chuoiSaiKhoa = JSON.stringify({ loaiNhiemVu: "dich", mucTinCay: 0.9 });

console.log(goiToolPhanLoai(chuoiHopLe));
console.log(goiToolPhanLoai(chuoiProse));
console.log(goiToolPhanLoai(chuoiSaiKhoa));
console.log(xuLyKetQuaGoiTool(goiToolPhanLoai(chuoiHopLe)));
console.log(xuLyKetQuaGoiTool(goiToolPhanLoai(chuoiProse)));
console.log(xuLyKetQuaGoiTool(goiToolPhanLoai(chuoiSaiKhoa)));
```

```text title=readonly
{"loai":"thanh_cong","duLieu":{"nhiemVu":"dich","doTinCay":0.9}}
{"loai":"loi_parse","chuoiGoc":"Toi nghi day la nhiem vu dich, do tin cay khoang 0.9."}
{"loai":"thieu_khoa","khoaThieu":["nhiemVu","doTinCay"]}
OK: nhiem vu dich, do tin cay 0.9
LOI PARSE: khong phai JSON hop le -- "Toi nghi day la nhiem vu dich, do tin cay khoang 0.9."
THIEU KHOA: nhiemVu, doTinCay
```

Ba chuỗi ĐẦU vào, ba KẾT QUẢ khác hẳn NHAU Ở CẤU TRÚC, không chỉ Ở nội
dung: `chuoiHopLe` cho `loai: "thanh_cong"` kèm `duLieu` ĐẦY ĐỦ;
`chuoiProse` cho `loai: "loi_parse"` kèm ĐÚNG chuỗi gốc để debug;
`chuoiSaiKhoa` (JSON hợp lệ nhưng SAI cả hai tên khoá) cho `loai:
"thieu_khoa"` kèm DANH SÁCH `["nhiemVu","doTinCay"]` — biết CHÍNH XÁC
khoá nào thiếu, không CHỈ biết "có lỗi". `xuLyKetQuaGoiTool` xử lý
TỪNG trường hợp bằng đúng dữ liệu CỦA trường hợp đó — không cần `if
(kq !== null)` rồi đoán tiếp lý do thất bại LÀ gì.
::::

::::predict{#doan_thieu_mot_khoa commitOnce}
Gọi `goiToolPhanLoai` trên chuỗi `'{"nhiemVu":"dich","doTinCay":"cao"}'`
— khoá `nhiemVu` ĐÚNG giá trị hợp lệ, nhưng `doTinCay` LÀ một chuỗi
(`"cao"`), không phải `number`. Trường `khoaThieu` của kết quả LÀ gì?

:::opt{correct}
`["doTinCay"]` — CHỈ một phần tử. `coNhiemVu` LÀ `true` (giá trị
`"dich"` hợp lệ) nên `khoaThieu.push("nhiemVu")` KHÔNG chạy; `coDoTinCay`
LÀ `false` (`"cao"` không phải `number`) nên `khoaThieu.push("doTinCay")`
CÓ chạy — mảng chỉ chứa ĐÚNG khoá THẬT SỰ sai
:::
:::opt
`["nhiemVu", "doTinCay"]` — cả hai khoá đều bị liệt kê, vì object
KHÔNG khớp HOÀN TOÀN với `KetQuaPhanLoai` thì nên coi LÀ thiếu CẢ hai
::why
Nhầm "không khớp schema hoàn toàn" VỚI "MỌI khoá đều sai" — nhưng
`goiToolPhanLoai` kiểm tra TỪNG khoá RIÊNG BIỆT bằng hai biến
`coNhiemVu`/`coDoTinCay` ĐỘC LẬP, không gộp chung một điều kiện.

Chỗ lệch: `khoaThieu` được XÂY bằng HAI dòng `if` tách biệt — mỗi dòng
chỉ `push` ĐÚNG MỘT tên khoá khi CHÍNH khoá đó sai. Với `nhiemVu="dich"`
(hợp lệ), dòng `if (!coNhiemVu)` không chạy; CHỈ dòng `if
(!coDoTinCay)` chạy, VÌ vậy mảng cuối cùng CHỈ chứa `"doTinCay"`.
::
:::
:::opt
`[]` (mảng rỗng) — vì `nhiemVu` đúng, hàm coi object LÀ ĐỦ gần đúng để
không liệt kê khoá nào thiếu
::why
Gần đúng Ở việc bạn nhận ra `nhiemVu` THẬT SỰ hợp lệ trong ví dụ này —
quan sát đó đúng.

Chỗ lệch: nhánh `if (!coNhiemVu || !coDoTinCay)` kiểm tra BẤT KỲ MỘT
trong hai điều kiện sai LÀ ĐÃ đủ để coi LÀ `"thieu_khoa"` — không cần
CẢ HAI cùng sai. Vì `coDoTinCay` LÀ `false`, nhánh NÀY vẫn chạy, VÀ nó
xây `khoaThieu` dựa trên TỪNG điều kiện — không hề trả về mảng rỗng.
::
:::
::::

::::code{#viet_goi_tool_phan_loai}
Hoàn thiện `goiToolPhanLoai` — sau khi ĐÃ có `coNhiemVu` VÀ
`coDoTinCay` (kiểm tra RIÊNG biệt hai khoá), xây nhánh `thieu_khoa` LIỆT
KÊ ĐÚNG những khoá THẬT SỰ sai, rồi xây nhánh `thanh_cong` khi CẢ hai
khoá đều hợp lệ.

```typescript title=starter
interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}

type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };

function goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoiTraVe);
  } catch {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  if (typeof giaTri !== "object" || giaTri === null) {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  const ghi = giaTri as Record<string, unknown>;
  const coNhiemVu = laNhiemVuHopLe(ghi.nhiemVu);
  const coDoTinCay = typeof ghi.doTinCay === "number";
  ___
  ___
}

const chuoiX = JSON.stringify({ nhiemVu: "tom_tat", doTinCay: 0.8 });
console.log(goiToolPhanLoai(chuoiX));
```

```typescript title=solution
interface KetQuaPhanLoai {
  nhiemVu: "tom_tat" | "dich" | "dem_so";
  doTinCay: number;
}

function laNhiemVuHopLe(x: unknown): x is "tom_tat" | "dich" | "dem_so" {
  return x === "tom_tat" || x === "dich" || x === "dem_so";
}

type KetQuaGoiTool =
  | { loai: "thanh_cong"; duLieu: KetQuaPhanLoai }
  | { loai: "loi_parse"; chuoiGoc: string }
  | { loai: "thieu_khoa"; khoaThieu: string[] };

function goiToolPhanLoai(chuoiTraVe: string): KetQuaGoiTool {
  let giaTri: unknown;
  try {
    giaTri = JSON.parse(chuoiTraVe);
  } catch {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  if (typeof giaTri !== "object" || giaTri === null) {
    return { loai: "loi_parse", chuoiGoc: chuoiTraVe };
  }
  const ghi = giaTri as Record<string, unknown>;
  const coNhiemVu = laNhiemVuHopLe(ghi.nhiemVu);
  const coDoTinCay = typeof ghi.doTinCay === "number";
  if (!coNhiemVu || !coDoTinCay) {
    const khoaThieu: string[] = [];
    if (!coNhiemVu) khoaThieu.push("nhiemVu");
    if (!coDoTinCay) khoaThieu.push("doTinCay");
    return { loai: "thieu_khoa", khoaThieu };
  }
  return {
    loai: "thanh_cong",
    duLieu: { nhiemVu: ghi.nhiemVu as "tom_tat" | "dich" | "dem_so", doTinCay: ghi.doTinCay as number },
  };
}

const chuoiX = JSON.stringify({ nhiemVu: "tom_tat", doTinCay: 0.8 });
console.log(goiToolPhanLoai(chuoiX));
```

```typescript title=test
const kqHopLe = goiToolPhanLoai(chuoiX);
if (kqHopLe.loai !== "thanh_cong" || kqHopLe.duLieu.nhiemVu !== "tom_tat" || kqHopLe.duLieu.doTinCay !== 0.8) {
  throw new Error("JSON hop le phai tra ve thanh_cong voi dung duLieu");
}

const kqProse = goiToolPhanLoai("Toi nghi day la nhiem vu dich.");
if (kqProse.loai !== "loi_parse" || kqProse.chuoiGoc !== "Toi nghi day la nhiem vu dich.") {
  throw new Error("chuoi khong phai JSON phai tra ve loi_parse VOI dung chuoiGoc");
}

const kqNull = goiToolPhanLoai("null");
if (kqNull.loai !== "loi_parse") throw new Error("chuoi 'null' (JSON hop le nhung la null) phai tra ve loi_parse, khong duoc nem loi runtime");

const chuoiSaiCaHaiKhoa = JSON.stringify({ loaiNhiemVu: "dich", mucTinCay: 0.9 });
const kqSaiCaHai = goiToolPhanLoai(chuoiSaiCaHaiKhoa);
if (kqSaiCaHai.loai !== "thieu_khoa") throw new Error("JSON dung nhung SAI ca hai ten khoa phai tra ve thieu_khoa");
if (kqSaiCaHai.loai === "thieu_khoa" && (kqSaiCaHai.khoaThieu.length !== 2 || !kqSaiCaHai.khoaThieu.includes("nhiemVu") || !kqSaiCaHai.khoaThieu.includes("doTinCay"))) {
  throw new Error("thieu CA HAI khoa thi khoaThieu phai liet ke DUNG CA HAI ten");
}

const chuoiSaiMotKhoa = JSON.stringify({ nhiemVu: "dich", doTinCay: "cao" });
const kqSaiMot = goiToolPhanLoai(chuoiSaiMotKhoa);
if (kqSaiMot.loai !== "thieu_khoa") throw new Error("doTinCay sai kieu (khong phai number) phai tra ve thieu_khoa");
if (kqSaiMot.loai === "thieu_khoa" && (kqSaiMot.khoaThieu.length !== 1 || kqSaiMot.khoaThieu[0] !== "doTinCay")) {
  throw new Error("CHI doTinCay sai thi khoaThieu phai liet ke DUNG MOT minh 'doTinCay', khong duoc them 'nhiemVu'");
}
```

:::hints
- kind: attention
  body: "Hai cho trong, ca hai la cau lenh return. Cho dau: neu (!coNhiemVu || !coDoTinCay) thi xay mang khoaThieu (push tung ten khoa THAT SU sai), return { loai: 'thieu_khoa', khoaThieu }. Cho hai: return { loai: 'thanh_cong', duLieu: { nhiemVu: ghi.nhiemVu ep kieu, doTinCay: ghi.doTinCay ep kieu } } -- day la nhanh CUOI, chi chay khi CA HAI khoa da hop le."
- kind: strategy
  body: "Cho dau: if (!coNhiemVu || !coDoTinCay) { const khoaThieu: string[] = []; if (!coNhiemVu) khoaThieu.push(\"nhiemVu\"); if (!coDoTinCay) khoaThieu.push(\"doTinCay\"); return { loai: \"thieu_khoa\", khoaThieu }; } Cho hai: return { loai: \"thanh_cong\", duLieu: { nhiemVu: ghi.nhiemVu as \"tom_tat\" | \"dich\" | \"dem_so\", doTinCay: ghi.doTinCay as number } };"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy, DUNG THU TU: nhanh thieu_khoa truoc, nhanh thanh_cong sau."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"loai\":\"thanh_cong\",\"duLieu\":{\"nhiemVu\":\"tom_tat\",\"doTinCay\":0.8}}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba trường hợp lỗi, ba hình dạng dữ liệu KHÁC nhau, MỘT switch
exhaustive không bỏ sót trường hợp nào — compiler TỰ kiểm tra điều đó,
không phải người viết code nhớ thủ công. Quest CÒN một bài: ráp CẢ NĂM
kỹ thuật lại, đo pass@1 tổng hợp trên đúng bộ test đã dùng xuyên suốt.
::::

::::reflect{#nghi-lai}
`KetQuaGoiTool` không hề LÀ một cách viết khác của `KetQuaPhanLoai |
null` — nó LÀ một câu hỏi khác hẳn: KHÔNG PHẢI "có kết quả hay không",
mà LÀ "kết quả này, hay CHÍNH XÁC lý do thất bại nào". `null` xoá SẠCH
thông tin VỀ lý do; discriminated union GIỮ NGUYÊN nó, gắn liền với
đúng dữ liệu cần để XỬ LÝ từng trường hợp riêng. Đây chính LÀ điều
`chapter-20-domain-modeling-with-dus` gọi LÀ "illegal states
unrepresentable", áp dụng vào một miền hoàn toàn khác: không phải
trạng thái đơn hàng, mà LÀ trạng thái của một lời gọi tool.
::::

::::checkpoint{mastery=0.84}
::::
