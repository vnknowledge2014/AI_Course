---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.chuan-hoa-va-khu-trung
title: "Chuẩn hoá URL: hai chuỗi khác nhau, một đích đến"
summary: "chuanHoaUrl bỏ fragment (#...), sắp xếp query param theo alphabet, lowercase host, và chuẩn hoá đường dẫn: domain trần (không path) LUÔN thành \"/\", đường dẫn dài hơn 1 ký tự kết thúc bằng \"/\" thì bỏ dấu \"/\" thừa. Nhờ quy ước này, \"http://vidu.com/\" VÀ \"http://VIDU.com\" (không path) đều chuẩn hoá về CÙNG \"http://vidu.com/\". khuTrungDanhSachUrl dùng Set các chuỗi đã chuẩn hoá: 8 URL khác chuỗi (khác hoa/thường, khác fragment, khác thứ tự query, khác trailing slash) khử trùng còn đúng 4 đích đến DUY NHẤT."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.chuan-hoa-va-khu-trung]
requires: [sd.hang-doi-bfs]
concepts: [sd.chuan-hoa-va-khu-trung]
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
BFS (bài trước) khám phá đúng THỨ tự — nhưng chưa hề hỏi câu QUAN
trọng: "trang NÀY đã thăm chưa?". Vấn đề LÀ, hai chuỗi khác NHAU
hoàn toàn có thể trỏ TỚI đúng một trang.
::::

::::explain{#chuan-hoa-url}
`http://Vidu.Com/trang/`, `http://vidu.com/trang#gioi-thieu`, VÀ
`http://vidu.com/trang` LÀ ba chuỗi khác nhau — nhưng CÙNG một đích.
`chuanHoaUrl` đưa mọi biến thể VỀ một dạng duy nhất qua BA bước: bỏ
fragment (`#...`), sắp xếp query param theo alphabet, VÀ chuẩn hoá
scheme/host/đường dẫn (host luôn lowercase, "domain trần" không path
được coi LÀ có `"/"`, đường dẫn dài hơn `"/"` thì bỏ dấu `/` thừa Ở
cuối):

```typescript title=readonly
function chuanHoaUrl(urlGoc: string): string {
  // 1) bo fragment (moi thu tu # tro di)
  const khongFragment = urlGoc.split("#")[0] ?? "";

  // 2) tach query, sap xep tham so theo alphabet
  const viTriHoi = khongFragment.indexOf("?");
  const phanTruocQuery = viTriHoi === -1 ? khongFragment : khongFragment.slice(0, viTriHoi);
  const chuoiQuery = viTriHoi === -1 ? "" : khongFragment.slice(viTriHoi + 1);
  const thamSoSapXep = chuoiQuery === "" ? "" : chuoiQuery.split("&").sort().join("&");

  // 3) tach scheme://host/path, lowercase host, chuan hoa duong dan
  const khopScheme = /^([a-zA-Z][a-zA-Z0-9+.-]*:\/\/)([^/]+)(\/.*)?$/.exec(phanTruocQuery);
  let phanGoc: string;
  if (khopScheme) {
    const scheme = khopScheme[1] ?? "";
    const host = (khopScheme[2] ?? "").toLowerCase();
    // khong co duong dan (domain tran) thi coi la goc "/"; duong dan dai hon
    // 1 ky tu ma ket thuc bang "/" thi bo dau "/" thua
    let duong = khopScheme[3] ?? "/";
    if (duong.length > 1 && duong.endsWith("/")) duong = duong.slice(0, -1);
    phanGoc = scheme + host + duong;
  } else {
    phanGoc = phanTruocQuery;
  }

  return thamSoSapXep === "" ? phanGoc : `${phanGoc}?${thamSoSapXep}`;
}

console.log(chuanHoaUrl("http://Vidu.Com/trang/"));
console.log(chuanHoaUrl("http://vidu.com/trang#gioi-thieu"));
console.log(chuanHoaUrl("http://vidu.com/trang?b=2&a=1"));
console.log(chuanHoaUrl("http://vidu.com/trang?a=1&b=2"));
console.log(chuanHoaUrl("http://vidu.com/"));
console.log(chuanHoaUrl("http://VIDU.com"));
```

```text title=readonly
http://vidu.com/trang
http://vidu.com/trang
http://vidu.com/trang?a=1&b=2
http://vidu.com/trang?a=1&b=2
http://vidu.com/
http://vidu.com/
```

Bốn dòng ĐẦU cho thấy trailing slash, fragment, VÀ thứ tự query đều
bị "san PHẲNG" về đúng một dạng. Hai dòng CUỐI xác nhận một quy ước
tinh TẾ hơn: `"http://vidu.com/"` (có dấu `/`) VÀ `"http://VIDU.com"`
(không path GÌ cả) đều quy VỀ đúng `"http://vidu.com/"` — vì hàm cố
Ý coi "không path" LÀ tương đương "path gốc `/`".
::::

::::example{#khu-trung-bang-set}
Ghép `chuanHoaUrl` VỚI một `Set` các chuỗi đã chuẩn hoá tạo THÀNH một
bộ khử trùng: chỉ giữ LẠI URL đầu tiên của mỗi ĐÍCH đến duy nhất, bỏ
qua mọi biến THỂ trùng lặp sau đó:

```typescript title=readonly
function chuanHoaUrl(urlGoc: string): string {
  const khongFragment = urlGoc.split("#")[0] ?? "";
  const viTriHoi = khongFragment.indexOf("?");
  const phanTruocQuery = viTriHoi === -1 ? khongFragment : khongFragment.slice(0, viTriHoi);
  const chuoiQuery = viTriHoi === -1 ? "" : khongFragment.slice(viTriHoi + 1);
  const thamSoSapXep = chuoiQuery === "" ? "" : chuoiQuery.split("&").sort().join("&");

  const khopScheme = /^([a-zA-Z][a-zA-Z0-9+.-]*:\/\/)([^/]+)(\/.*)?$/.exec(phanTruocQuery);
  let phanGoc: string;
  if (khopScheme) {
    const scheme = khopScheme[1] ?? "";
    const host = (khopScheme[2] ?? "").toLowerCase();
    let duong = khopScheme[3] ?? "/";
    if (duong.length > 1 && duong.endsWith("/")) duong = duong.slice(0, -1);
    phanGoc = scheme + host + duong;
  } else {
    phanGoc = phanTruocQuery;
  }

  return thamSoSapXep === "" ? phanGoc : `${phanGoc}?${thamSoSapXep}`;
}

function khuTrungDanhSachUrl(urls: string[]): string[] {
  const daThay = new Set<string>();
  const ketQua: string[] = [];
  for (const u of urls) {
    const chuan = chuanHoaUrl(u);
    if (!daThay.has(chuan)) {
      daThay.add(chuan);
      ketQua.push(chuan);
    }
  }
  return ketQua;
}

const boUrlTrungLap = [
  "http://vidu.com/trang/",
  "http://vidu.com/trang#gioi-thieu",
  "http://VIDU.com/trang",
  "http://vidu.com/trang?b=2&a=1",
  "http://vidu.com/trang?a=1&b=2",
  "http://vidu.com/",
  "http://VIDU.com",
  "http://vidu.com/khac",
];

console.log("dau vao:", boUrlTrungLap.length, "URL");
const daKhuTrung = khuTrungDanhSachUrl(boUrlTrungLap);
console.log("sau khu trung:", daKhuTrung.length, "URL");
console.log(daKhuTrung);
```

```text title=readonly
dau vao: 8 URL
sau khu trung: 4 URL
[
  'http://vidu.com/trang',
  'http://vidu.com/trang?a=1&b=2',
  'http://vidu.com/',
  'http://vidu.com/khac'
]
```

TÁM chuỗi đầu vào, chỉ BỐN đích đến DUY nhất. `"http://vidu.com/"`
VÀ `"http://VIDU.com"` (hai chuỗi trông rất khác) khử TRÙNG thành một
mục — đúng như quy tắc "domain trần" Ở khối trên.
::::

::::predict{#doan-goc-domain commitOnce}
`chuanHoaUrl("http://vidu.com")` (KHÔNG có dấu `/` nào Ở cuối, path
hoàn toàn vắng mặt) VÀ `chuanHoaUrl("http://vidu.com/")` (CÓ dấu `/`
gốc) — hai LỜI gọi này có trả về CÙNG một chuỗi không?

:::opt{correct}
CÓ — cả hai đều LÀ `"http://vidu.com/"`, vì khi `khopScheme[3]`
(nhóm đường dẫn) LÀ `undefined` (không có path), hàm mặc định
`duong = "/"`
:::
:::opt
KHÔNG — `"http://vidu.com"` giữ nguyên không dấu `/`, còn
`"http://vidu.com/"` giữ nguyên CÓ dấu `/`; hàm chỉ XOÁ dấu `/` thừa,
không hề THÊM dấu `/` vào chỗ chưa CÓ
::why
Nhầm "hàm chỉ có nhiệm vụ XOÁ ký tự thừa" với toàn bộ LOGIC thật của
`chuanHoaUrl` — dòng `let duong = khopScheme[3] ?? "/";` không CHỈ
xoá, nó còn CHỦ ĐỘNG gán `"/"` khi hoàn toàn không CÓ path.

Chỗ lệch: với `"http://vidu.com"`, regex khớp `scheme="http://"`,
`host="vidu.com"`, VÀ nhóm thứ BA (đường dẫn) không khớp gì cả nên LÀ
`undefined` — toán tử `?? "/"` biến nó THÀNH `"/"` ngay lập tức. Với
`"http://vidu.com/"`, nhóm thứ ba khớp đúng `"/"`, giữ NGUYÊN vì độ
dài `1` không thoả điều kiện cắt (`duong.length > 1`). Cả hai đường
đều DỪNG lại ở cùng giá trị `duong = "/"`.
::
:::
::::

::::code{#viet_chuan_hoa_url}
Hoàn thiện `chuanHoaUrl` — sau khi xác định đường dẫn mặc định
(dòng TRÊN), bỏ dấu `/` thừa Ở CUỐI nếu đường dẫn dài hơn một ký tự.

```typescript title=starter
function chuanHoaUrl(urlGoc: string): string {
  const khongFragment = urlGoc.split("#")[0] ?? "";
  const viTriHoi = khongFragment.indexOf("?");
  const phanTruocQuery = viTriHoi === -1 ? khongFragment : khongFragment.slice(0, viTriHoi);
  const chuoiQuery = viTriHoi === -1 ? "" : khongFragment.slice(viTriHoi + 1);
  const thamSoSapXep = chuoiQuery === "" ? "" : chuoiQuery.split("&").sort().join("&");

  const khopScheme = /^([a-zA-Z][a-zA-Z0-9+.-]*:\/\/)([^/]+)(\/.*)?$/.exec(phanTruocQuery);
  let phanGoc: string;
  if (khopScheme) {
    const scheme = khopScheme[1] ?? "";
    const host = (khopScheme[2] ?? "").toLowerCase();
    let duong = khopScheme[3] ?? "/";
    ___
    phanGoc = scheme + host + duong;
  } else {
    phanGoc = phanTruocQuery;
  }

  return thamSoSapXep === "" ? phanGoc : `${phanGoc}?${thamSoSapXep}`;
}

console.log(chuanHoaUrl("http://Vidu.Com/trang/"));
```

```typescript title=solution
function chuanHoaUrl(urlGoc: string): string {
  const khongFragment = urlGoc.split("#")[0] ?? "";
  const viTriHoi = khongFragment.indexOf("?");
  const phanTruocQuery = viTriHoi === -1 ? khongFragment : khongFragment.slice(0, viTriHoi);
  const chuoiQuery = viTriHoi === -1 ? "" : khongFragment.slice(viTriHoi + 1);
  const thamSoSapXep = chuoiQuery === "" ? "" : chuoiQuery.split("&").sort().join("&");

  const khopScheme = /^([a-zA-Z][a-zA-Z0-9+.-]*:\/\/)([^/]+)(\/.*)?$/.exec(phanTruocQuery);
  let phanGoc: string;
  if (khopScheme) {
    const scheme = khopScheme[1] ?? "";
    const host = (khopScheme[2] ?? "").toLowerCase();
    let duong = khopScheme[3] ?? "/";
    if (duong.length > 1 && duong.endsWith("/")) duong = duong.slice(0, -1);
    phanGoc = scheme + host + duong;
  } else {
    phanGoc = phanTruocQuery;
  }

  return thamSoSapXep === "" ? phanGoc : `${phanGoc}?${thamSoSapXep}`;
}

console.log(chuanHoaUrl("http://Vidu.Com/trang/"));
```

```typescript title=test
if (chuanHoaUrl("http://Vidu.Com/trang/") !== "http://vidu.com/trang") throw new Error("host phai lowercase, trailing slash o duong dan dai phai bi bo");
if (chuanHoaUrl("http://vidu.com/trang#gioi-thieu") !== "http://vidu.com/trang") throw new Error("fragment (#...) phai bi bo hoan toan");
if (chuanHoaUrl("http://vidu.com/trang?b=2&a=1") !== chuanHoaUrl("http://vidu.com/trang?a=1&b=2")) throw new Error("thu tu query param khac nhau van phai chuan hoa ve CUNG mot chuoi");
if (chuanHoaUrl("http://vidu.com/") !== "http://vidu.com/") throw new Error("goc domain co dau / phai GIU nguyen dau /");
if (chuanHoaUrl("http://VIDU.com") !== "http://vidu.com/") throw new Error("goc domain KHONG co dau / phai duoc coi la co dau / (giong het truong hop co san dau /)");
if (chuanHoaUrl("http://vidu.com/trang") !== "http://vidu.com/trang") throw new Error("duong dan da khong co trailing slash thi giu nguyen");
```

:::hints
- kind: attention
  body: "Chi cat dau '/' cuoi khi duong dan DAI HON mot ky tu -- neu chi la '/' don doc thi GIU nguyen, khong cat."
- kind: strategy
  body: "if (duong.length > 1 && duong.endsWith('/')) duong = duong.slice(0, -1);"
- kind: one-line
  body: "if (duong.length > 1 && duong.endsWith(\"/\")) duong = duong.slice(0, -1);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "http://vidu.com/trang"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trùng lặp đã bị dẹp. Nhưng một crawler THẬT còn có thể chạy MÃI —
những trang tự sinh ra liên KẾT vô hạn LÀ một cái bẫy cần chặn TRƯỚC.
::::

::::reflect{#nghi-lai}
`chuanHoaUrl` không hề "sửa" URL — nó CHỈ định nghĩa một dạng CHUẨN
DUY nhất để so sánh. Tập quy tắc chọn ở đây (bỏ fragment, sắp xếp
query, coi domain TRẦN như có `/`) LÀ một lựa chọn CÓ CHỦ đích, không
phải LUẬT bắt buộc — một hệ thống khác hoàn toàn có thể chọn quy tắc
riêng, miễn LÀ áp dụng nhất quán cho MỌI URL đi qua nó.
::::

::::checkpoint{mastery=0.77}
::::
