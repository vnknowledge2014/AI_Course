---
id: ky-nghe-ung-dung-ai.giao-thuc-mcp-a2a.mcp-resources-list-va-read
title: "MCP resources — \"resources/list\" và \"resources/read\""
summary: "docResourceCoKiemDanhSach(danhSachCongBo, uri) đọc một resource CHỈ SAU KHI xác nhận uri đó CÓ nằm trong danhSachCongBo (kết quả của 'resources/list') — khác hẳn docResourceKhongKiem(uri), một hàm NAIVE tra thẳng vào kho nội dung BANG_NOI_DUNG mà KHÔNG hề kiểm danh sách công bố. Trên DANH_SACH_RESOURCE gồm 3 mục công bố (quy1, quy2, quy3) và BANG_NOI_DUNG gồm 3 khoá THẬT SỰ có nội dung (quy1, quy2, VÀ 'noi-bo/luong-nhan-vien' — một resource CHƯA BAO GIỜ công bố): docResourceKhongKiem('file:///noi-bo/luong-nhan-vien.txt') RÒ RỈ nội dung nội bộ ('Luong CEO: 500 trieu/thang') dù resource này không hề xuất hiện ở resources/list; docResourceCoKiemDanhSach cùng uri đó bị CHẶN với lỗi nêu rõ 'khong nam trong danh sach cong bo'. Trên uri 'file:///bao-cao/quy3.txt' (CÓ công bố nhưng BANG_NOI_DUNG không có nội dung — dữ liệu chưa sẵn sàng): docResourceCoKiemDanhSach trả một lỗi KHÁC — 'khong tim thay noi dung' — chứng minh hai lớp lỗi tách biệt (không công bố vs công bố-nhưng-thiếu-nội-dung) đi qua đúng hai bước kiểm riêng, theo đúng thứ tự: catalog trước, nội dung sau."
locale: vi
track: ky-nghe-ung-dung-ai
module: giao-thuc-mcp-a2a
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kna.mcp-resources-list-va-read]
requires: [kna.ma-loi-json-rpc-day-du]
concepts: [kna.mcp-resources-list-va-read]
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
Ba bài đầu q9.6a chỉ đụng tới `tools` — agent GỌI một hành động. MCP còn
một khả năng khác: `resources` — agent ĐỌC dữ liệu, không gọi hành động
nào cả. `"resources/list"` công bố NHỮNG uri nào tồn tại; `"resources/
read"` đọc MỘT uri cụ thể. Câu hỏi của bài này: đọc một uri KHÔNG NẰM
trong danh sách vừa công bố — có nên được phép không?
::::

::::explain{#danh_sach_cong_bo_vs_kho_noi_dung}
`McpResourceDefinition` LÀ một mục trong kết quả `"resources/list"` —
`uri`, `name`, `mimeType`. Đây LÀ danh sách CÔNG KHAI agent nhìn thấy.
Nhưng nội dung THẬT SỰ nằm Ở một nơi KHÁC — `BANG_NOI_DUNG` — VÀ hai
tập hợp này KHÔNG BẮT BUỘC khớp nhau: có thể có nội dung tồn tại nhưng
CHƯA BAO GIỜ được công bố Ở `"resources/list"`:

```typescript title=readonly
type McpResourceDefinition = { uri: string; name: string; mimeType: string };
type McpResourceContent = { uri: string; mimeType: string; text: string };

const DANH_SACH_RESOURCE: McpResourceDefinition[] = [
  { uri: "file:///bao-cao/quy1.txt", name: "Bao cao quy 1", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy2.txt", name: "Bao cao quy 2", mimeType: "text/plain" },
];

const BANG_NOI_DUNG: Record<string, string> = {
  "file:///bao-cao/quy1.txt": "Doanh thu quy 1: 120 trieu",
  "file:///bao-cao/quy2.txt": "Doanh thu quy 2: 150 trieu",
  "file:///noi-bo/luong-nhan-vien.txt": "Luong CEO: 500 trieu/thang",
};

function docResourceKhongKiem(uri: string): McpResourceContent | { error: string } {
  const text = BANG_NOI_DUNG[uri];
  if (text === undefined) return { error: `khong tim thay noi dung: ${uri}` };
  return { uri, mimeType: "text/plain", text };
}

console.log(JSON.stringify(docResourceKhongKiem("file:///noi-bo/luong-nhan-vien.txt")));
```

```text title=readonly
{"uri":"file:///noi-bo/luong-nhan-vien.txt","mimeType":"text/plain","text":"Luong CEO: 500 trieu/thang"}
```

`docResourceKhongKiem` đọc THẲNG vào `BANG_NOI_DUNG` — VÀ `BANG_NOI_DUNG`
CÓ khoá `"file:///noi-bo/luong-nhan-vien.txt"`, dù khoá NÀY chưa từng
xuất hiện Ở `DANH_SACH_RESOURCE`. Hàm rò rỉ đúng nội dung một resource
KHÔNG HỀ được công bố công khai.
::::

::::example{#doc_co_kiem_danh_sach}
`docResourceCoKiemDanhSach` thêm ĐÚNG một bước TRƯỚC khi chạm
`BANG_NOI_DUNG`: xác nhận `uri` CÓ nằm trong `danhSachCongBo` hay không.
Hai lớp lỗi TÁCH BIỆT: "không công bố" (chặn Ở bước đầu) VÀ "công bố
nhưng thiếu nội dung" (chặn Ở bước hai) — thứ tự MATTER: catalog trước,
nội dung sau:

```typescript title=readonly
type McpResourceDefinition = { uri: string; name: string; mimeType: string };
type McpResourceContent = { uri: string; mimeType: string; text: string };

const DANH_SACH_RESOURCE: McpResourceDefinition[] = [
  { uri: "file:///bao-cao/quy1.txt", name: "Bao cao quy 1", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy2.txt", name: "Bao cao quy 2", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy3.txt", name: "Bao cao quy 3", mimeType: "text/plain" },
];

const BANG_NOI_DUNG: Record<string, string> = {
  "file:///bao-cao/quy1.txt": "Doanh thu quy 1: 120 trieu",
  "file:///bao-cao/quy2.txt": "Doanh thu quy 2: 150 trieu",
  "file:///noi-bo/luong-nhan-vien.txt": "Luong CEO: 500 trieu/thang",
};

function locResourceTrongDanhSach(danhSach: McpResourceDefinition[], uri: string): McpResourceDefinition | undefined {
  return danhSach.find((r) => r.uri === uri);
}

function docResourceCoKiemDanhSach(
  danhSachCongBo: McpResourceDefinition[],
  uri: string,
): McpResourceContent | { error: string } {
  const dinhNghia = locResourceTrongDanhSach(danhSachCongBo, uri);
  if (dinhNghia === undefined) {
    return { error: `resource khong nam trong danh sach cong bo (resources/list): ${uri}` };
  }
  const text = BANG_NOI_DUNG[uri];
  if (text === undefined) {
    return { error: `khong tim thay noi dung: ${uri}` };
  }
  return { uri, mimeType: dinhNghia.mimeType, text };
}

console.log(JSON.stringify(docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///noi-bo/luong-nhan-vien.txt")));
console.log(JSON.stringify(docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///bao-cao/quy3.txt")));
console.log(JSON.stringify(docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///bao-cao/quy1.txt")));
```

```text title=readonly
{"error":"resource khong nam trong danh sach cong bo (resources/list): file:///noi-bo/luong-nhan-vien.txt"}
{"error":"khong tim thay noi dung: file:///bao-cao/quy3.txt"}
{"uri":"file:///bao-cao/quy1.txt","mimeType":"text/plain","text":"Doanh thu quy 1: 120 trieu"}
```

Ba lời gọi, BA kết quả khác nhau: resource nội bộ bị chặn VỚI lý do
"không công bố"; `quy3` (CÓ công bố nhưng thiếu nội dung) bị chặn VỚI
lý do KHÁC "không tìm thấy nội dung"; `quy1` (công bố VÀ có nội dung)
đọc thành công. Cùng LÀ thất bại, nhưng lý do PHẢN ÁNH ĐÚNG bước nào
chặn.
::::

::::predict{#doan-thu-tu-hai-buoc-kiem commitOnce}
Nếu ĐẢO thứ tự hai bước kiểm — kiểm `BANG_NOI_DUNG` TRƯỚC, kiểm
`DANH_SACH_RESOURCE` SAU — thì gọi
`docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///noi-bo/luong-nhan-vien.txt")`
sẽ trả về lỗi NÀO?

:::opt{correct}
Vẫn LÀ một lỗi (uri này không công bố), NHƯNG thông điệp SẼ SAI — vì
`BANG_NOI_DUNG` CÓ khoá này (có nội dung THẬT), bước kiểm nội dung sẽ
QUA, VÀ hàm sẽ đi tới bước kiểm danh sách — nhưng nếu bước ĐÓ bị đặt
SAU cùng dưới dạng "else trả nội dung luôn" thay vì kiểm tiếp, nội dung
NỘI BỘ vẫn bị RÒ — đây chính LÀ lý do thứ tự "catalog trước, nội dung
sau" không phải ngẫu nhiên
:::
:::opt
Kết quả HỆT NHAU dù đảo thứ tự — vì cả hai bước đều LÀ điều kiện độc
lập, không ảnh hưởng lẫn nhau
::why
Nhầm rằng hai bước kiểm ĐỘC LẬP hoàn toàn — nhưng chúng chia sẻ CHUNG
một `uri`: `BANG_NOI_DUNG` CÓ chứa khoá
`"file:///noi-bo/luong-nhan-vien.txt"` (thật sự có nội dung), nên NẾU
bước kiểm nội dung đứng TRƯỚC VÀ được viết theo kiểu "qua thì trả luôn",
nó sẽ bỏ lỡ mất bước kiểm catalog phía sau — thứ tự các bước quyết định
CHÍNH XÁC lỗi nào bị bỏ lỡ.

Chỗ lệch: hai tập dữ liệu (`DANH_SACH_RESOURCE` VÀ `BANG_NOI_DUNG`)
KHÔNG khớp nhau hoàn toàn — chênh lệch đó chính LÀ nơi thứ tự kiểm gây
ra khác biệt quan sát được.
::
:::
:::opt
Lỗi biên dịch — TypeScript không cho phép đảo thứ tự hai câu lệnh `if`
::why
Nhầm rằng THỨ TỰ các câu lệnh Ở TẦNG NGÔN NGỮ bị TypeScript ràng buộc —
nhưng đảo thứ tự hai khối `if` độc lập (không phụ thuộc kiểu dữ liệu
lẫn nhau) LÀ hợp lệ VỀ MẶT CÚ PHÁP VÀ KIỂU; đây LÀ một quyết định THIẾT
KẾ (chọn bước nào chặn TRƯỚC), không phải một ràng buộc ngôn ngữ.

Chỗ lệch: chương trình VẪN biên dịch bình thường dù đảo thứ tự — chỉ
HÀNH VI lúc chạy (lỗi nào được báo, hay tệ hơn LÀ rò rỉ) mới thay đổi.
::
:::
::::

::::code{#viet_doc_resource_co_kiem}
Hoàn thiện `locResourceTrongDanhSach` — dùng `.find()` tìm phần tử
`danhSach` CÓ `uri` khớp. Hoàn thiện `docResourceCoKiemDanhSach` — GỌI
`locResourceTrongDanhSach` TRƯỚC (NẾU `undefined`, trả lỗi "khong nam
trong danh sach cong bo"); CHỈ SAU KHI catalog xác nhận mới tra
`BANG_NOI_DUNG` (NẾU `undefined`, trả lỗi "khong tim thay noi dung");
NGƯỢC LẠI trả về `McpResourceContent` đầy đủ.

```typescript title=starter
type McpResourceDefinition = { uri: string; name: string; mimeType: string };
type McpResourceContent = { uri: string; mimeType: string; text: string };

const DANH_SACH_RESOURCE: McpResourceDefinition[] = [
  { uri: "file:///bao-cao/quy1.txt", name: "Bao cao quy 1", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy2.txt", name: "Bao cao quy 2", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy3.txt", name: "Bao cao quy 3", mimeType: "text/plain" },
];

const BANG_NOI_DUNG: Record<string, string> = {
  "file:///bao-cao/quy1.txt": "Doanh thu quy 1: 120 trieu",
  "file:///bao-cao/quy2.txt": "Doanh thu quy 2: 150 trieu",
  "file:///noi-bo/luong-nhan-vien.txt": "Luong CEO: 500 trieu/thang",
};

function locResourceTrongDanhSach(danhSach: McpResourceDefinition[], uri: string): McpResourceDefinition | undefined {
  ___
}

function docResourceCoKiemDanhSach(
  danhSachCongBo: McpResourceDefinition[],
  uri: string,
): McpResourceContent | { error: string } {
  ___
}

const ketQuaHopLe = docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///bao-cao/quy1.txt");
console.log(JSON.stringify(ketQuaHopLe));
```

```typescript title=solution
type McpResourceDefinition = { uri: string; name: string; mimeType: string };
type McpResourceContent = { uri: string; mimeType: string; text: string };

const DANH_SACH_RESOURCE: McpResourceDefinition[] = [
  { uri: "file:///bao-cao/quy1.txt", name: "Bao cao quy 1", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy2.txt", name: "Bao cao quy 2", mimeType: "text/plain" },
  { uri: "file:///bao-cao/quy3.txt", name: "Bao cao quy 3", mimeType: "text/plain" },
];

const BANG_NOI_DUNG: Record<string, string> = {
  "file:///bao-cao/quy1.txt": "Doanh thu quy 1: 120 trieu",
  "file:///bao-cao/quy2.txt": "Doanh thu quy 2: 150 trieu",
  "file:///noi-bo/luong-nhan-vien.txt": "Luong CEO: 500 trieu/thang",
};

function locResourceTrongDanhSach(danhSach: McpResourceDefinition[], uri: string): McpResourceDefinition | undefined {
  return danhSach.find((r) => r.uri === uri);
}

function docResourceCoKiemDanhSach(
  danhSachCongBo: McpResourceDefinition[],
  uri: string,
): McpResourceContent | { error: string } {
  const dinhNghia = locResourceTrongDanhSach(danhSachCongBo, uri);
  if (dinhNghia === undefined) {
    return { error: `resource khong nam trong danh sach cong bo (resources/list): ${uri}` };
  }
  const text = BANG_NOI_DUNG[uri];
  if (text === undefined) {
    return { error: `khong tim thay noi dung: ${uri}` };
  }
  return { uri, mimeType: dinhNghia.mimeType, text };
}

const ketQuaHopLe = docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///bao-cao/quy1.txt");
console.log(JSON.stringify(ketQuaHopLe));
```

```typescript title=test
if ("error" in ketQuaHopLe) throw new Error("quy1 co cong bo VA co noi dung -- khong duoc tra ve error");
if ((ketQuaHopLe as McpResourceContent).text !== "Doanh thu quy 1: 120 trieu") throw new Error("noi dung tra ve phai dung noi dung that cua quy1");

const noiBo = docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///noi-bo/luong-nhan-vien.txt");
if (!("error" in noiBo)) throw new Error("resource noi bo KHONG cong bo -- phai bi chan");
if (!(noiBo as { error: string }).error.includes("khong nam trong danh sach cong bo")) throw new Error("ly do chan phai la 'khong nam trong danh sach cong bo', khong phai ly do khac");

const thieuNoiDung = docResourceCoKiemDanhSach(DANH_SACH_RESOURCE, "file:///bao-cao/quy3.txt");
if (!("error" in thieuNoiDung)) throw new Error("quy3 co cong bo nhung KHONG co noi dung -- phai bi chan");
if (!(thieuNoiDung as { error: string }).error.includes("khong tim thay noi dung")) throw new Error("ly do chan phai la 'khong tim thay noi dung', KHAC voi ly do 'khong nam trong danh sach cong bo'");
if ((thieuNoiDung as { error: string }).error.includes("khong nam trong danh sach cong bo")) throw new Error("quy3 CO trong danh sach cong bo -- khong duoc bao nham ly do 'khong nam trong danh sach'");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (locResourceTrongDanhSach): mot dong danhSach.find((r) => r.uri === uri). Cho hai (docResourceCoKiemDanhSach): goi locResourceTrongDanhSach TRUOC (chan neu undefined), CHI SAU DO moi tra BANG_NOI_DUNG (chan neu undefined), cuoi cung moi tra ve noi dung day du."
- kind: strategy
  body: "Cho dau: return danhSach.find((r) => r.uri === uri); Cho hai: const dinhNghia = locResourceTrongDanhSach(danhSachCongBo, uri); if (dinhNghia === undefined) return { error: `resource khong nam trong danh sach cong bo (resources/list): ${uri}` }; const text = BANG_NOI_DUNG[uri]; if (text === undefined) return { error: `khong tim thay noi dung: ${uri}` }; return { uri, mimeType: dinhNghia.mimeType, text };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, GIU NGUYEN thu tu hai buoc kiem."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"uri\":\"file:///bao-cao/quy1.txt\",\"mimeType\":\"text/plain\",\"text\":\"Doanh thu quy 1: 120 trieu\"}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`resources` VÀ `tools` giải quyết CÙNG một bài học Ở HAI hình dạng khác
nhau: hàng rào phải đứng TRƯỚC hành động (tools) hoặc TRƯỚC lần đọc
(resources), không phải sau. Bài sau chuyển sang một chiều mới của A2A:
KHÔNG PHẢI một task đơn, mà MỘT LUỒNG nhiều message tích luỹ theo thời
gian.
::::

::::reflect{#nghi-lai}
Sự khác biệt quan trọng nhất Ở bài này KHÔNG PHẢI "resources khác tools"
Ở tầng CÚ PHÁP — mà LÀ: `docResourceKhongKiem` chỉ hỏi "nội dung này CÓ
tồn tại không", trong khi `docResourceCoKiemDanhSach` hỏi ĐÚNG câu hỏi
cần hỏi — "nội dung này có được PHÉP lộ ra qua kênh này không". Một kho
nội dung (`BANG_NOI_DUNG`) VÀ một danh sách công bố (`DANH_SACH_RESOURCE`)
LÀ HAI tập hợp khác nhau VỚI HAI mục đích khác nhau: tồn tại không có
nghĩa LÀ được phép đọc — VÀ khoảng cách giữa hai tập hợp đó chính LÀ nơi
một resource "bí mật" có thể bị lộ nếu không ai kiểm catalog trước khi
đọc.
::::

::::checkpoint{mastery=0.86}
::::
