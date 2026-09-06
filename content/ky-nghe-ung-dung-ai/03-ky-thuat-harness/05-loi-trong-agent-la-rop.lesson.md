---
id: ky-nghe-ung-dung-ai.ky-thuat-harness.loi-trong-agent-la-rop
title: "Cầu nối FP — lỗi trong agent là Railway-Oriented Programming"
summary: "Cầu nối FP (dựa trên chapter-22-error-handling-rop.chapter.md): type KetQua<T, E> = { thanhCong: true; giaTri: T } | { thanhCong: false; loi: E } là discriminated union thay throw/try-catch -- ĐÚNG hình dạng KetQuaGoiTool bốn bài trước đã dùng, giờ đặt tên chính thức. andThen<T, U, E>(kq, buoc) nối hai bước: nếu kq THÀNH CÔNG thì chạy buoc(kq.giaTri); nếu THẤT BẠI thì trả về NGUYÊN kq, KHÔNG gọi buoc -- 'trượt' qua như đường ray lỗi. Chuỗi 3 bước goiToolAnToan → parseKetQua → dinhDangLai (nối bằng andThen hai lần), đo qua bộ đếm demGoi: đầu vào hợp lệ chạy CẢ BA bước (1,1,1); lỗi ở bước 2 (parse) chặn bước 3 KHÔNG chạy (1,1,0), lỗi giữ nguyên; lỗi NGAY bước 1 (tool) chặn CẢ bước 2 VÀ 3 (1,0,0), lỗi gốc giữ nguyên tới cuối chuỗi."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-harness
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.loi-trong-agent-la-rop]
requires: [kna.rang-buoc-so-lan-thu-lai-toi-da]
concepts: [kna.loi-trong-agent-la-rop]
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
Bốn bài trước LUÔN làm MỘT việc, dưới một cái tên chưa gọi ra: biểu
diễn "thành công hoặc thất bại" bằng một GIÁ TRỊ trả về, không hề dùng
`throw`. Lập trình hàm có TÊN CHÍNH THỨC cho hình dạng này —
Railway-Oriented Programming (ROP): lỗi LÀ một đường ray phụ, VÀ một
khi tàu rẽ vào đó, mọi trạm SAU trên đường ray chính bị bỏ qua cho tới
ga cuối.
::::

::::explain{#ket_qua_va_and_then}
`KetQua<T, E>` LÀ đúng hình dạng `KetQuaGoiTool` bốn bài trước đã dùng
— giờ viết Ở dạng TỔNG QUÁT (generic), CHO PHÉP kiểu giá trị `T` VÀ
kiểu lỗi `E` khác nhau Ở MỖI bước. `andThen` LÀ "đường ray": nối một
`KetQua` VỚI bước xử lý TIẾP THEO — CHỈ chạy bước đó khi bước TRƯỚC đã
thành công; nếu thất bại, TRẢ VỀ NGUYÊN `kq` (giữ đúng lỗi gốc), KHÔNG
gọi `buoc`:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
}

const ok: KetQua<number, string> = { thanhCong: true, giaTri: 10 };
const loi: KetQua<number, string> = { thanhCong: false, loi: "hong" };

const sauOk = andThen(ok, (n: number) => ({ thanhCong: true as const, giaTri: n * 2 }));
const sauLoi = andThen(loi, (n: number) => ({ thanhCong: true as const, giaTri: n * 2 }));
console.log(JSON.stringify(sauOk));
console.log(JSON.stringify(sauLoi));
```

```text title=readonly
{"thanhCong":true,"giaTri":20}
{"thanhCong":false,"loi":"hong"}
```

`sauOk` chạy `buoc` (nhân đôi `10` thành `20`) vì `ok.thanhCong` LÀ
`true`. `sauLoi` KHÔNG chạy `buoc` — `andThen` trả về NGUYÊN `loi`
("hong"), y hệt lỗi gốc, không hề bị `buoc` chạm vào. Không có
`try`/`catch` nào Ở đây cả — lỗi LÀ một GIÁ TRỊ, đi qua chương trình
như bất kỳ giá trị nào khác.
::::

::::example{#chuoi_ba_buoc_qua_and_then}
Một chuỗi harness THẬT gồm ba bước: gọi tool → parse kết quả thô →
định dạng lại. Mỗi bước LÀ một hàm trả về `KetQua`; `demGoi` đếm số
lần MỖI bước THẬT SỰ chạy, để chứng minh bước SAU một lỗi không hề
được gọi tới:

```typescript title=readonly
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
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

function chayChuoiBaBuoc(dauVao: string): KetQua<string, string> {
  const b1 = goiToolAnToan(dauVao);
  const b2 = andThen(b1, parseKetQua);
  return andThen(b2, dinhDangLai);
}

console.log(JSON.stringify(chayChuoiBaBuoc("so_hop_le")), JSON.stringify(demGoi));
demGoi.goiTool = 0; demGoi.parse = 0; demGoi.dinhDang = 0;

console.log(JSON.stringify(chayChuoiBaBuoc("khong_hop_le")), JSON.stringify(demGoi));
demGoi.goiTool = 0; demGoi.parse = 0; demGoi.dinhDang = 0;

console.log(JSON.stringify(chayChuoiBaBuoc("loi_tool")), JSON.stringify(demGoi));
```

```text title=readonly
{"thanhCong":true,"giaTri":"Ket qua: 42"} {"goiTool":1,"parse":1,"dinhDang":1}
{"thanhCong":false,"loi":"parse_that_bai_khong_phai_so"} {"goiTool":1,"parse":1,"dinhDang":0}
{"thanhCong":false,"loi":"tool_that_bai"} {"goiTool":1,"parse":0,"dinhDang":0}
```

Ba kết quả LÀ ba câu chuyện khác nhau. `"so_hop_le"`: CẢ BA bước chạy
đúng một lần. `"khong_hop_le"`: bước `1` (tool) thành công, nhưng bước
`2` (parse) thất bại Ở `"khong_phai_so"` — `dinhDang` KHÔNG hề chạy
(`0` lần), VÀ lỗi cuối cùng chính LÀ lỗi CỦA bước `2`, không bị bước
`3` ghi đè. `"loi_tool"`: NGAY bước `1` đã thất bại — CẢ `parse` LẪN
`dinhDang` đều `0` lần, lỗi giữ nguyên LÀ `"tool_that_bai"` TỚI TẬN
cuối chuỗi.
::::

::::predict{#doan-buoc-2-that-bai commitOnce}
NẾU bước `2` (`parseKetQua`) trả về thất bại, `andThen(b2, dinhDangLai)`
Ở dòng cuối `chayChuoiBaBuoc` CÓ gọi `dinhDangLai` không?

:::opt{correct}
Không — `andThen` kiểm `kq.thanhCong` TRƯỚC: khi đó LÀ `false`, nó
`return kq;` NGAY (trả nguyên lỗi của bước `2`), KHÔNG bao giờ gọi
`buoc` (Ở đây LÀ `dinhDangLai`)
:::
:::opt
Có — `andThen` LUÔN gọi `buoc` để thử ĐỊNH DẠNG lại LỖI đó thành một
thông báo dễ đọc hơn
::why
Nhầm `andThen` VỚI một hàm kiểu `mapErr` (biến đổi LỖI) — nhưng
`andThen` chỉ nhận `buoc: (giaTri: T) => KetQua<U, E>`, một hàm xử lý
GIÁ TRỊ THÀNH CÔNG, không phải lỗi.

Chỗ lệch: khi `kq.thanhCong` LÀ `false`, nhánh `else` của phép toán
ba ngôi (`kq.thanhCong ? buoc(kq.giaTri) : kq`) LÀ `kq` — trả thẳng
lỗi gốc, không đụng tới `buoc` chút nào.
::
:::
:::opt
Không xác định — phụ thuộc `parseKetQua` trả lỗi kiểu gì
::why
Nhầm rằng LOẠI lỗi ảnh hưởng tới việc `buoc` có được gọi hay không —
nhưng `andThen` chỉ nhìn vào CỜ `thanhCong` (một `boolean`), không hề
đọc NỘI DUNG của `loi`.

Chỗ lệch: dù `loi` LÀ chuỗi gì (`"parse_that_bai_khong_phai_so"` hay
bất kỳ chuỗi nào khác), MIỄN LÀ `thanhCong === false`, `andThen` LUÔN
trả về nguyên `kq`, không bao giờ gọi `buoc`.
::
:::
::::

::::code{#viet_and_then}
Hoàn thiện `andThen` — NẾU `kq.thanhCong` LÀ `true`, gọi `buoc(kq.giaTri)`
VÀ trả về kết quả đó; NẾU KHÔNG, trả về NGUYÊN `kq`. Hoàn thiện
`chayChuoiBaBuoc` — nối `goiToolAnToan(dauVao)` VỚI `parseKetQua` bằng
`andThen`, rồi nối kết quả đó VỚI `dinhDangLai` bằng `andThen` một lần
nữa, trả về kết quả CUỐI CÙNG.

```typescript title=starter
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

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

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  ___
}

function chayChuoiBaBuoc(dauVao: string): KetQua<string, string> {
  ___
}

const ketQuaOk = chayChuoiBaBuoc("so_hop_le");
console.log(JSON.stringify(ketQuaOk));
```

```typescript title=solution
type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };

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

function andThen<T, U, E>(kq: KetQua<T, E>, buoc: (giaTri: T) => KetQua<U, E>): KetQua<U, E> {
  return kq.thanhCong ? buoc(kq.giaTri) : kq;
}

function chayChuoiBaBuoc(dauVao: string): KetQua<string, string> {
  const b1 = goiToolAnToan(dauVao);
  const b2 = andThen(b1, parseKetQua);
  return andThen(b2, dinhDangLai);
}

const ketQuaOk = chayChuoiBaBuoc("so_hop_le");
console.log(JSON.stringify(ketQuaOk));
```

```typescript title=test
if (!ketQuaOk.thanhCong) throw new Error("chuoi 3 buoc voi dau vao hop le phai THANH CONG");
if (ketQuaOk.giaTri !== "Ket qua: 42") throw new Error("chuoi 3 buoc voi dau vao hop le phai co gia tri 'Ket qua: 42'");
if (demGoi.goiTool !== 1 || demGoi.parse !== 1 || demGoi.dinhDang !== 1) {
  throw new Error("dau vao hop le phai chay CA BA buoc, dung mot lan moi buoc");
}

demGoi.goiTool = 0; demGoi.parse = 0; demGoi.dinhDang = 0;
const ketQuaLoiParse = chayChuoiBaBuoc("khong_hop_le");
if (ketQuaLoiParse.thanhCong) throw new Error("dau vao khong parse duoc thanh so phai THAT BAI o buoc 2");
if (ketQuaLoiParse.loi !== "parse_that_bai_khong_phai_so") {
  throw new Error("loi o buoc parse phai GIU NGUYEN dung loi do, khong bi buoc dinh dang lai ghi de");
}
if (demGoi.goiTool !== 1 || demGoi.parse !== 1 || demGoi.dinhDang !== 0) {
  throw new Error("loi o buoc parse (buoc 2) phai chan buoc dinh dang lai (buoc 3) KHONG chay");
}

demGoi.goiTool = 0; demGoi.parse = 0; demGoi.dinhDang = 0;
const ketQuaLoiTool = chayChuoiBaBuoc("loi_tool");
if (ketQuaLoiTool.thanhCong) throw new Error("dau vao 'loi_tool' phai THAT BAI ngay o buoc 1");
if (ketQuaLoiTool.loi !== "tool_that_bai") {
  throw new Error("loi ngay o buoc goi tool (buoc 1) phai GIU NGUYEN dung loi do toi cuoi chuoi");
}
if (demGoi.goiTool !== 1 || demGoi.parse !== 0 || demGoi.dinhDang !== 0) {
  throw new Error("loi o buoc 1 phai chan CA buoc 2 VA buoc 3 khong chay");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (andThen): dung phep toan ba ngoi tren kq.thanhCong -- neu true thi goi buoc(kq.giaTri) va return ket qua do, nguoc lai return thang kq (KHONG goi buoc). Cho hai (chayChuoiBaBuoc): goi goiToolAnToan(dauVao) luu vao b1, roi andThen(b1, parseKetQua) luu vao b2, roi return andThen(b2, dinhDangLai)."
- kind: strategy
  body: "Cho dau: return kq.thanhCong ? buoc(kq.giaTri) : kq; Cho hai: const b1 = goiToolAnToan(dauVao); const b2 = andThen(b1, parseKetQua); return andThen(b2, dinhDangLai);"
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
  expect: "{\"thanhCong\":true,\"giaTri\":\"Ket qua: 42\"}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba kịch bản, ba kết quả — VÀ trong CẢ hai kịch bản lỗi, số lần gọi
CHỨNG MINH bằng số rằng bước sau một thất bại KHÔNG hề chạy. Bốn bài
đầu quest này đã dùng ĐÚNG hình dạng này, không hề gọi tên nó. BOSS
khép quest ráp lại retry, fallback, retry-budget VÀ hình dạng ROP này
thành hai harness — VÀ so sánh chúng trên CÙNG một model, một tool.
::::

::::reflect{#nghi-lai}
`KetQua<T, E>` VÀ `andThen` không phải LÀ khái niệm MỚI — chúng LÀ cái
TÊN chính thức cho đúng thứ bốn bài trước đã xây: `KetQuaGoiTool` (bài
`1`), if/return sớm trong `goiToolCoRetry` (bài `2`), if/return sớm
trong `goiToolCoFallback` (bài `3`), VÀ vòng `while` dừng đúng lúc
trong `goiToolCoRetryCoGioiHan` (bài `4`) — TẤT CẢ đều LÀ cách viết
tay của "đường ray lỗi", không hề dùng `throw`/`try`/`catch` Ở BẤT KỲ
đâu. `andThen` LÀ bản TỔNG QUÁT của đúng mẫu if/return đó — nối được
BAO NHIÊU bước tuỳ ý, mỗi bước có thể ĐỔI cả kiểu giá trị LẪN không hề
phải viết lại phép kiểm `if (...thanhCong)` mỗi lần nối thêm một bước.
::::

::::checkpoint{mastery=0.9}
::::
