---
id: ky-nghe-phan-mem.bao-mat-ung-dung.tham-so-hoa-truy-van
title: "Tham số hoá truy vấn — tách LỆNH khỏi DỮ LIỆU"
summary: "Giải pháp thật: KHÔNG nối chuỗi — tách RIÊNG câu lệnh SQL (cố định, chứa $1/$2 chỗ giữ) khỏi DỮ LIỆU (mảng params, truyền riêng cho driver). sql KHÔNG BAO GIỜ chứa chuỗi người dùng, driver BINDS giá trị chứ không re-parse như code."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [bmud.parameterized-queries]
requires: [bmud.sql-injection-attack]
concepts: [bmud.parameterized-queries]
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
Vấn đề GỐC là nối chuỗi TRỰC TIẾP. Nếu TÁCH RIÊNG câu lệnh SQL (cố
định) khỏi DỮ LIỆU (truyền riêng), database có còn hiểu nhầm không?
::::

::::explain{#tach-lenh-khoi-du-lieu}
Giải pháp THẬT: **KHÔNG nối chuỗi** — tách RIÊNG câu lệnh SQL (**CỐ
ĐỊNH**, chứa `$1`/`$2` là CHỖ GIỮ) khỏi **DỮ LIỆU** (mảng `params`,
TRUYỀN RIÊNG cho driver database):

```typescript
type Query = { sql: string; params: unknown[] };

function buildSafeQuery(email: string): Query {
  return { sql: "SELECT * FROM users WHERE email = $1", params: [email] };
}

const q = buildSafeQuery("'; DROP TABLE users; --");
console.log(q.sql);
console.log(JSON.stringify(q.params));
console.log(q.sql.includes("DROP TABLE"));
```

```text
SELECT * FROM users WHERE email = $1
["'; DROP TABLE users; --"]
false
```

`q.sql` **KHÔNG BAO GIỜ** chứa chuỗi người dùng — nó LUÔN LÀ chuỗi
CỐ ĐỊNH `"SELECT * FROM users WHERE email = $1"`, BẤT KỂ `email`
truyền vào là gì. Dữ liệu ĐỘC HẠI (`"'; DROP TABLE users; --"`) chỉ
nằm TRONG `params` — driver database **BINDS** giá trị đó vào vị trí
`$1` NHƯ MỘT GIÁ TRỊ ĐƠN THUẦN (KHÔNG BAO GIỜ re-parse như CÂU LỆNH
mới), NÊN `q.sql.includes("DROP TABLE")` là `false`.
::::

::::example{#nhieu-tham-so}
Mở rộng cho NHIỀU tham số — MỖI giá trị người dùng đi vào MỘT vị trí
`$N` RIÊNG, KHÔNG BAO GIỜ chèn TRỰC TIẾP vào chuỗi `sql`:

```typescript title=readonly
type Query = { sql: string; params: unknown[] };
function buildSafeOrderQuery(id: string, trangThai: string): Query {
  return { sql: "SELECT * FROM orders WHERE id = $1 AND status = $2", params: [id, trangThai] };
}

const q2 = buildSafeOrderQuery("DH-01", "'; DROP TABLE orders; --");
console.log(q2.sql);
console.log(JSON.stringify(q2.params));
console.log(q2.sql.includes("DROP TABLE"));
```

```text title=readonly
SELECT * FROM orders WHERE id = $1 AND status = $2
["DH-01","'; DROP TABLE orders; --"]
false
```

DÙ `trangThai` chứa chuỗi ĐỘC HẠI, `q2.sql` VẪN KHÔNG ĐỔI — CHỈ hai
CHỖ GIỮ `$1`/`$2` cố định. Kỹ thuật NÀY áp dụng được cho BAO NHIÊU
tham số tuỳ ý — mỗi giá trị người dùng ĐI VÀO `params`, KHÔNG BAO GIỜ
đi vào `sql`.
::::

::::predict{#doan-thu-tu-params-quan-trong commitOnce}
```typescript
type Query = { sql: string; params: unknown[] };
function buildSafeOrderQuery(id: string, trangThai: string): Query {
  return { sql: "SELECT * FROM orders WHERE id = $1 AND status = $2", params: [id, trangThai] };
}

// GỌI với thứ tự tham số BỊ ĐẢO NGƯỢC so với ý định (id và trangThai hoán đổi)
const q = buildSafeOrderQuery("da_giao", "DH-99");
console.log(q.params[0]);
```

Dòng cuối in ra gì?

:::opt{correct}
`da_giao`
:::

:::opt
`DH-99` — vì `params` LUÔN sắp xếp theo Ý NGHĨA của TÊN tham số
(`id` trước `trangThai`), TypeScript tự ĐỘNG sắp xếp lại đúng thứ tự
NGỮ NGHĨA dù người gọi truyền SAI thứ tự
::why
Gần đúng ở việc bạn nhớ ĐÚNG hàm khai `id` TRƯỚC `trangThai` (đúng
THỨ TỰ tham số trong chữ ký hàm) — quan sát về CHỮ KÝ đó đúng.

Chỗ lệch: TypeScript (và JavaScript) **KHÔNG** "hiểu Ý NGHĨA" tên
tham số để tự sắp xếp lại — nó CHỈ gán giá trị theo **VỊ TRÍ**. Lời
gọi `buildSafeOrderQuery("da_giao", "DH-99")`: đối số THỨ NHẤT
(`"da_giao"`) được gán cho `id` (tham số ĐẦU tiên trong chữ ký), đối
số THỨ HAI (`"DH-99"`) được gán cho `trangThai` — **DÙ TÊN GỌI** nghe
"ngược" (mã đơn hàng lẽ ra phải LÀ `id`, trạng thái lẽ ra phải LÀ
`trangThai`), TypeScript CHỈ theo VỊ TRÍ, không theo "Ý NGHĨA hợp lý"
của giá trị. `q.params[0]` LÀ `"da_giao"` — giá trị ĐẦU TIÊN được
truyền, ĐÚNG vị trí, DÙ nghe "sai chỗ" về mặt NGHIỆP VỤ.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `buildSafeOrderQuery("da_giao", "DH-99")`
không hợp lệ vì TypeScript phát hiện giá trị "da_giao" trông giống
TRẠNG THÁI hơn MÃ ĐƠN HÀNG, không khớp Ý NGHĨA của tham số `id`
::why
Gần đúng ở việc bạn để ý `"da_giao"` (đọc như MỘT trạng thái) và
`"DH-99"` (đọc như MỘT mã đơn hàng) LOOKS "ngược chỗ" so với tên tham
số `id`/`trangThai` — một quan sát THÔNG MINH về mặt NGỮ NGHĨA con
người đọc được.

Chỗ lệch: CẢ HAI tham số `id: string` VÀ `trangThai: string` đều có
kiểu **`string`** — TypeScript KHÔNG PHÂN BIỆT được "chuỗi trông như
mã đơn hàng" và "chuỗi trông như trạng thái" (cả hai đều CHỈ là
`string` Ở TẦNG KIỂU). Truyền BẤT KỲ hai chuỗi nào, theo BẤT KỲ THỨ
TỰ nào, ĐỀU biên dịch sạch — việc "đúng Ý NGHĨA nghiệp vụ" là trách
nhiệm của NGƯỜI GỌI, không phải điều compiler kiểm được.
::
:::
::::

::::code{#viet_buildsafequery}
Tự viết `buildSafeQuery` và `buildSafeOrderQuery`.

```typescript title=starter
type Query = { sql: string; params: unknown[] };

function buildSafeQuery(email: string): Query {
  return { sql: "SELECT * FROM users WHERE email = $1", params: ___ };
}

function buildSafeOrderQuery(id: string, trangThai: string): Query {
  return { sql: "SELECT * FROM orders WHERE id = $1 AND status = $2", params: ___ };
}

console.log(JSON.stringify(buildSafeQuery("an@shop.vn")));
```

```typescript title=solution
type Query = { sql: string; params: unknown[] };

function buildSafeQuery(email: string): Query {
  return { sql: "SELECT * FROM users WHERE email = $1", params: [email] };
}

function buildSafeOrderQuery(id: string, trangThai: string): Query {
  return { sql: "SELECT * FROM orders WHERE id = $1 AND status = $2", params: [id, trangThai] };
}

console.log(JSON.stringify(buildSafeQuery("an@shop.vn")));
```

```typescript title=test
const qTest = buildSafeQuery("'; DROP TABLE users; --");
if (qTest.sql.includes("DROP TABLE")) throw new Error("sql KHÔNG được chứa dữ liệu người dùng, kể cả độc hại");
if (JSON.stringify(qTest.params) !== JSON.stringify(["'; DROP TABLE users; --"])) throw new Error("params phải chứa ĐÚNG giá trị email, nguyên vẹn");
if (!qTest.sql.includes("email")) throw new Error("sql của buildSafeQuery phải là câu lệnh về email/users, không lẫn với buildSafeOrderQuery");
if (qTest.sql.includes("orders")) throw new Error("sql của buildSafeQuery không được lẫn nội dung của buildSafeOrderQuery");

const qOrderTest = buildSafeOrderQuery("DH-77", "cho_thanh_toan");
if (qOrderTest.sql.includes("DH-77") || qOrderTest.sql.includes("cho_thanh_toan")) throw new Error("sql KHÔNG được chứa dữ liệu, dù không độc hại");
if (qOrderTest.params[0] !== "DH-77") throw new Error("params[0] phải đúng id, không lẫn với trangThai");
if (qOrderTest.params[1] !== "cho_thanh_toan") throw new Error("params[1] phải đúng trangThai, không lẫn với id");
if (!qOrderTest.sql.includes("orders")) throw new Error("sql của buildSafeOrderQuery phải là câu lệnh về orders, không lẫn với buildSafeQuery");
if (qOrderTest.sql.includes("email")) throw new Error("sql của buildSafeOrderQuery không được lẫn nội dung của buildSafeQuery");
```

:::hints
- kind: attention
  body: "params LUÔN là một MẢNG chứa giá trị theo ĐÚNG thứ tự các $N xuất hiện trong sql — buildSafeQuery có $1 nên params có MỘT phần tử; buildSafeOrderQuery có $1 và $2 nên params có HAI phần tử theo đúng thứ tự."
- kind: strategy
  body: "[email] : [id, trangThai] — mảng một phần tử cho một $N, mảng hai phần tử theo đúng thứ tự cho hai $N."
- kind: one-line
  body: "___ (buildSafeQuery) = [email]\n___ (buildSafeOrderQuery) = [id, trangThai]"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "SELECT"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tham số hoá: sql cố định, dữ liệu tách riêng, driver BINDS chứ không
re-parse. Bước tiếp theo: lỗ hổng KHÁC — dữ liệu render thành HTML mà
không escape.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`SQL Injection` là dữ liệu trở thành LỆNH cho database. Nếu nội dung
người dùng render THẲNG thành HTML, KHÔNG escape — vấn đề TƯƠNG TỰ
xảy ra ở đâu?
::::

::::checkpoint{mastery=0.8}
::::
