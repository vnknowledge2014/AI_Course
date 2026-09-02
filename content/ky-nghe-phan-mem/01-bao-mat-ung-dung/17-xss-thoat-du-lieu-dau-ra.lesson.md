---
id: ky-nghe-phan-mem.bao-mat-ung-dung.xss-thoat-du-lieu-dau-ra
title: "XSS — escape MỌI biên xuất ra HTML, kể cả vector không phải <script>"
summary: "Cross-Site Scripting: nội dung người dùng CHƯA escape render THẲNG thành HTML thực thi NHƯ CODE. escapeHtml thay NĂM ký tự đặc biệt THEO ĐÚNG THỨ TỰ (& trước tiên, nếu không sẽ escape ĐÈ lên chính escape sequence vừa tạo). Vector KHÔNG PHẢI <script>: onerror cũng bị chặn."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [bmud.xss-escape-output]
requires: [bmud.parameterized-queries]
concepts: [bmud.xss-escape-output]
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
`SQL Injection` là dữ liệu trở thành LỆNH cho database. Nếu nội dung
người dùng render THẲNG thành HTML, KHÔNG escape — vấn đề TƯƠNG TỰ ở
đâu?
::::

::::explain{#escape-html}
**Cross-Site Scripting (XSS)**: nội dung người dùng **CHƯA escape**
render THẲNG thành HTML **THỰC THI NHƯ CODE**. `escapeHtml(unsafe)`
thay **NĂM** ký tự đặc biệt **THEO ĐÚNG THỨ TỰ**:

```typescript
function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

console.log(escapeHtml("<script>alert(1)</script>"));
```

```text
&lt;script&gt;alert(1)&lt;/script&gt;
```

`<script>...</script>` bị escape thành CHUỖI VĂN BẢN THUẦN (`&lt;`
thay `<`) — trình duyệt HIỂN THỊ nó như VĂN BẢN (`<script>...`), KHÔNG
THỰC THI như một thẻ script. Escape `&` **TRƯỚC TIÊN** RẤT quan
trọng: NẾU escape `&` **SAU** các ký tự khác, nó sẽ escape **ĐÈ LÊN**
CHÍNH escape sequence VỪA TẠO (`&lt;` → escape `&` trong ĐÓ → `&amp;lt;`
— HỎNG).
::::

::::example{#vector-khong-phai-script}
Vector **KHÔNG PHẢI** `<script>`: `onerror` (thuộc tính HTML kích
hoạt JavaScript khi ẢNH tải LỖI) — escape ĐÚNG cả `"`/`<`/`>` chặn
được vector NÀY:

```typescript title=readonly
function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

console.log(escapeHtml('<img src=x onerror="alert(1)">'));
```

```text title=readonly
&lt;img src=x onerror=&quot;alert(1)&quot;&gt;
```

`<img src=x onerror="alert(1)">` (KHÔNG có `<script>` nào cả — thẻ
`<img>` với `src` CỐ Ý SAI, kích hoạt `onerror` khi tải ảnh THẤT BẠI)
VẪN bị escape SẠCH — `<`/`>` biến thành thẻ VÔ HẠI (KHÔNG PHẢI thẻ
HTML thật NỮA), `"` biến `onerror="alert(1)"` thành VĂN BẢN, KHÔNG
PHẢI thuộc tính THỰC THI được. Escape MỌI ký tự đặc biệt (KHÔNG CHỈ
`<`/`>`) chặn được NHIỀU vector, không CHỈ `<script>` trực tiếp.
::::

::::predict{#doan-thu-tu-escape-quan-trong commitOnce}
```typescript
function escapeHtmlSaiThuTu(unsafe: string): string {
  return unsafe
    .replace(/</g, "&lt;")
    .replace(/&/g, "&amp;"); // escape & SAU -- SAI thứ tự
}

console.log(escapeHtmlSaiThuTu("<b>"));
```

Dòng cuối in ra gì?

:::opt{correct}
`&amp;lt;b>`
:::

:::opt
`&lt;b&gt;` — vì `escapeHtmlSaiThuTu` VẪN thực hiện ĐỦ hai bước thay
thế (`<` rồi `&`), thứ tự CHỈ ảnh hưởng tới HIỆU SUẤT, không ảnh
hưởng tới KẾT QUẢ CUỐI CÙNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG hàm CÓ thực hiện ĐỦ hai bước `.replace`
(escape `<`, RỒI escape `&`) — cả hai bước ĐỀU chạy, không bước nào
bị bỏ qua, quan sát ĐÓ đúng.

Chỗ lệch: THỨ TỰ **CÓ** ảnh hưởng tới KẾT QUẢ, vì bước THỨ HAI chạy
TRÊN **KẾT QUẢ** của bước ĐẦU (KHÔNG PHẢI trên chuỗi GỐC). Bước 1:
`"<b>".replace(/</g, "&lt;")` → `"&lt;b>"` (dấu `<` GỐC đã biến thành
`&lt;`, chứa MỘT dấu `&` MỚI). Bước 2: `"&lt;b>".replace(/&/g,
"&amp;")` — TÌM `&` TRONG chuỗi HIỆN TẠI (`"&lt;b>"`), tìm THẤY dấu
`&` CỦA CHÍNH `&lt;` vừa tạo, escape NÓ THÀNH `&amp;` — kết quả
`"&amp;lt;b>"` (chữ `l`, `t` VÀ dấu `>` GỐC KHÔNG hề bị đổi, `>` vẫn
CÒN NGUYÊN vì bước escape `>` KHÔNG có trong hàm SAI THỨ TỰ này).
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `.replace()` HAI LẦN LIÊN TIẾP (method
chaining) trên biểu thức KẾT QUẢ của `.replace()` trước đó không hợp
lệ với regex TOÀN CỤC (`/g`)
::why
Gần đúng ở việc bạn để ý CÓ HAI lời gọi `.replace()` NỐI TIẾP nhau
(method chaining) — một quan sát ĐÚNG về CẤU TRÚC code.

Chỗ lệch: `String.prototype.replace` LUÔN trả về MỘT `string` MỚI —
gọi `.replace()` TIẾP TỤC trên kết quả ĐÓ (chaining) là thao tác
**HOÀN TOÀN CHUẨN**, dùng khắp nơi trong JavaScript, KHÔNG có hạn chế
đặc biệt nào cho regex có cờ `/g` (global). Biên dịch VÀ chạy hoàn
toàn bình thường — vấn đề CHỈ là kết quả CUỐI CÙNG không đúng Ý MUỐN,
không phải lỗi cú pháp/kiểu.
::
:::
::::

::::code{#viet_escapehtml}
Tự viết BA bước `.replace` đầu tiên trong `escapeHtml` (`&`, `<`, `>`).

```typescript title=starter
function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, ___)
    .replace(/</g, ___)
    .replace(/>/g, ___)
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

console.log(escapeHtml("<script>alert(1)</script>"));
```

```typescript title=solution
function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

console.log(escapeHtml("<script>alert(1)</script>"));
```

```typescript title=test
if (escapeHtml("<b>") !== "&lt;b&gt;") throw new Error("phải escape < và > đúng, theo đúng thứ tự (& trước)");
if (escapeHtml("a & b") !== "a &amp; b") throw new Error("phải escape & thành &amp;");
if (escapeHtml('<img src=x onerror="alert(1)">') !== "&lt;img src=x onerror=&quot;alert(1)&quot;&gt;") throw new Error("phải escape đúng vector onerror, không chỉ <script>");
if (escapeHtml("van ban binh thuong") !== "van ban binh thuong") throw new Error("văn bản không có ký tự đặc biệt phải giữ nguyên");
```

:::hints
- kind: attention
  body: "& PHẢI escape TRƯỚC TIÊN (thành &amp;), rồi mới tới < (thành &lt;) và > (thành &gt;) — nếu đảo thứ tự, escape & SAU sẽ escape ĐÈ lên chính &lt;/&gt; vừa tạo."
- kind: strategy
  body: '"&amp;" : "&lt;" : "&gt;" — ba chuỗi thay thế chuẩn HTML entity, đúng thứ tự & trước, </>  sau.'
- kind: one-line
  body: '___ (&) = "&amp;"\n___ (<) = "&lt;"\n___ (>) = "&gt;"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "&lt;"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
XSS: escape mọi biên xuất ra HTML, thứ tự quan trọng. Bước tiếp theo:
lớp phòng thủ THỨ HAI — chặn script LẠ dù escape có bị bỏ sót ở đâu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Escape (bài này) là lớp phòng thủ Ở TẦNG CODE (dễ quên một chỗ). Có
cách nào ra lệnh cho TRÌNH DUYỆT tự chặn script LẠ, dù escape có sót
ở đâu đó không?
::::

::::checkpoint{mastery=0.8}
::::
