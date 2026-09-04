---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.trie-top-k-goi-y
title: "Trie top-K: cache sẵn gợi ý tại mỗi node"
summary: "Moi NutTrieTanSuat luu san topK (SO_GOI_Y_TOI_DA=2) -- xayDungTopK gop de quy TU la len goc, sort giam dan theo tanSuat, cat con 2. Voi 'may'=10, 'may tinh'=50, 'may anh'=30: layGoiY('may') tra ve dung [{'may tinh',50},{'may anh',30}] -- 'may' (10, thap nhat) BI LOAI du van la tu hop le. layGoiY chi doc mang da cache san tai node (khong duyet lai subtree moi lan goi), doi lay O(do sau tien to) thay vi O(kich thuoc subtree)."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.trie-top-k-goi-y]
requires: [sd.trie-chen-va-tim]
concepts: [sd.trie-top-k-goi-y]
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
Bài trước duyệt LẠI toàn bộ cây con MỖI lần tìm — đúng, nhưng CHẬM dần
khi cây LỚN. Nếu mỗi node TỰ giữ sẵn câu trả LỜI cho "top mấy từ phổ
biến nhất bắt đầu TỪ đây", việc tìm KHÔNG cần duyệt lại gì cả.
::::

::::explain{#topk-cache-tai-node}
Mỗi node giờ giữ THÊM một `tanSuat` (mức phổ biến CỦA chính từ đó, nếu
nó LÀ một từ hoàn chỉnh) VÀ một mảng `topK` — sẵn danh sách TỐI ĐA
`SO_GOI_Y_TOI_DA` từ phổ biến nhất bắt nguồn TỪ node này. `xayDungTopK`
tính từ LÁ lên gốc (đệ quy hậu THỨ tự): gộp chính nó (nếu LÀ từ hoàn
chỉnh) VỚI `topK` của TỪNG con, sắp giảm dần, rồi CẮT còn đúng `SO_GOI_Y_TOI_DA`:

```typescript title=readonly
interface MucTanSuat { tu: string; tanSuat: number; }
interface NutTrieTanSuat {
  con: Map<string, NutTrieTanSuat>;
  laTuHoanChinh: boolean;
  tanSuat: number;
  topK: MucTanSuat[];
}
function taoNutTrieTanSuat(): NutTrieTanSuat {
  return { con: new Map(), laTuHoanChinh: false, tanSuat: 0, topK: [] };
}

function chenTuVoiTanSuat(goc: NutTrieTanSuat, tu: string, tanSuat: number): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrieTanSuat();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
  hienTai.tanSuat = tanSuat;
}

const SO_GOI_Y_TOI_DA = 2;

function xayDungTopK(nut: NutTrieTanSuat, tienToHienTai: string): MucTanSuat[] {
  let gopTatCa: MucTanSuat[] = [];
  if (nut.laTuHoanChinh) gopTatCa.push({ tu: tienToHienTai, tanSuat: nut.tanSuat });
  for (const [kyTu, con] of nut.con) {
    gopTatCa = gopTatCa.concat(xayDungTopK(con, tienToHienTai + kyTu));
  }
  gopTatCa.sort((a, b) => b.tanSuat - a.tanSuat);
  nut.topK = gopTatCa.slice(0, SO_GOI_Y_TOI_DA);
  return nut.topK;
}

function timNutTheoTienTo(goc: NutTrieTanSuat, tienTo: string): NutTrieTanSuat | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}

function layGoiY(goc: NutTrieTanSuat, tienTo: string): MucTanSuat[] {
  const hienTai = timNutTheoTienTo(goc, tienTo);
  if (hienTai === undefined) return [];
  return hienTai.topK;
}

const goc = taoNutTrieTanSuat();
chenTuVoiTanSuat(goc, "may", 10);
chenTuVoiTanSuat(goc, "may tinh", 50);
chenTuVoiTanSuat(goc, "may anh", 30);
chenTuVoiTanSuat(goc, "meo", 5);
xayDungTopK(goc, "");

console.log('goi y cho "may":', JSON.stringify(layGoiY(goc, "may")));
console.log('goi y cho "me":', JSON.stringify(layGoiY(goc, "me")));
```

```text title=readonly
goi y cho "may": [{"tu":"may tinh","tanSuat":50},{"tu":"may anh","tanSuat":30}]
goi y cho "me": [{"tu":"meo","tanSuat":5}]
```

`layGoiY` KHÔNG hề duyệt lại cây — nó chỉ đi TỚI đúng node của tiền tố
rồi TRẢ thẳng `topK` đã có SẴN. Prefix `"may"` có `3` từ hợp LỆ (tần
suất `10`, `50`, `30`) nhưng `topK` chỉ giữ đúng `2` — `"may"` (tần
suất thấp NHẤT) bị loại khỏi kết quả CACHE, dù bản thân nó vẫn LÀ một
từ hợp lệ trong trie.
::::

::::example{#gop-qua-nhieu-nhanh}
Tại node GỐC (tiền tố rỗng), `topK` phải gộp CẢ nhánh `"may"` LẪN nhánh
`"meo"` — VÀ kết quả vẫn chỉ giữ đúng `2` mục PHỔ biến nhất trên TOÀN
bộ trie, bất kể chúng thuộc nhánh NÀO:

```typescript title=readonly
interface MucTanSuat { tu: string; tanSuat: number; }
interface NutTrieTanSuat {
  con: Map<string, NutTrieTanSuat>;
  laTuHoanChinh: boolean;
  tanSuat: number;
  topK: MucTanSuat[];
}
function taoNutTrieTanSuat(): NutTrieTanSuat {
  return { con: new Map(), laTuHoanChinh: false, tanSuat: 0, topK: [] };
}

function chenTuVoiTanSuat(goc: NutTrieTanSuat, tu: string, tanSuat: number): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrieTanSuat();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
  hienTai.tanSuat = tanSuat;
}

const SO_GOI_Y_TOI_DA = 2;

function xayDungTopK(nut: NutTrieTanSuat, tienToHienTai: string): MucTanSuat[] {
  let gopTatCa: MucTanSuat[] = [];
  if (nut.laTuHoanChinh) gopTatCa.push({ tu: tienToHienTai, tanSuat: nut.tanSuat });
  for (const [kyTu, con] of nut.con) {
    gopTatCa = gopTatCa.concat(xayDungTopK(con, tienToHienTai + kyTu));
  }
  gopTatCa.sort((a, b) => b.tanSuat - a.tanSuat);
  nut.topK = gopTatCa.slice(0, SO_GOI_Y_TOI_DA);
  return nut.topK;
}

function timNutTheoTienTo(goc: NutTrieTanSuat, tienTo: string): NutTrieTanSuat | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}

function layGoiY(goc: NutTrieTanSuat, tienTo: string): MucTanSuat[] {
  const hienTai = timNutTheoTienTo(goc, tienTo);
  if (hienTai === undefined) return [];
  return hienTai.topK;
}

// tai lap dung trang thai da chen o khoi truoc: chen lai ca 4 tu roi xay lai topK
const goc = taoNutTrieTanSuat();
chenTuVoiTanSuat(goc, "may", 10);
chenTuVoiTanSuat(goc, "may tinh", 50);
chenTuVoiTanSuat(goc, "may anh", 30);
chenTuVoiTanSuat(goc, "meo", 5);
xayDungTopK(goc, "");

console.log('goi y cho "" (tien to rong, gop ca 2 nhanh may VA meo):', JSON.stringify(layGoiY(goc, "")));
console.log('goi y cho tien to khong ton tai "xyz":', JSON.stringify(layGoiY(goc, "xyz")));
```

```text title=readonly
goi y cho "" (tien to rong, gop ca 2 nhanh may VA meo): [{"tu":"may tinh","tanSuat":50},{"tu":"may anh","tanSuat":30}]
goi y cho tien to khong ton tai "xyz": []
```

`"meo"` (tần suất `5`) hoàn toàn KHÔNG lọt vào `topK` của node GỐC — nó
thua CẢ `3` từ nhánh `"may"`, VÀ `topK` gốc vẫn chỉ giữ `2` mục cao
nhất TOÀN cục: `"may tinh"` (`50`) VÀ `"may anh"` (`30`). Tiền tố không
tồn tại vẫn trả VỀ mảng rỗng, giống hệt bài TRƯỚC.
::::

::::predict{#doan-loai-thap-nhat commitOnce}
`SO_GOI_Y_TOI_DA = 2`. Node `"may"` có `3` từ hợp lệ trong cây con
(`"may"` tần suất `10`, `"may tinh"` tần suất `50`, `"may anh"` tần
suất `30`). Sau khi `xayDungTopK` chạy, `layGoiY(goc, "may")` trả về MẤY
mục, VÀ mục nào bị LOẠI?

:::opt{correct}
Đúng `2` mục (`"may tinh"` VÀ `"may anh"`) — `"may"` (tần suất `10`,
THẤP nhất trong ba) bị loại, vì `slice(0, 2)` sau khi sort GIẢM dần chỉ
giữ lại `2` mục CÓ tần suất cao nhất
:::
:::opt
Đúng `3` mục — vì cả `3` từ ĐỀU hợp lệ VÀ đều bắt nguồn từ node `"may"`,
`topK` phải phản ánh ĐẦY đủ mọi từ có THỂ gợi ý
::why
Nhầm "mọi từ HỢP LỆ trong subtree" VỚI "mọi từ được CACHE trong topK" —
nhưng `topK` LÀ một bộ nhớ đệm bị GIỚI hạn kích thước có CHỦ đích, không
phải một bản sao ĐẦY đủ của subtree.

Chỗ lệch: `xayDungTopK` gọi `gopTatCa.slice(0, SO_GOI_Y_TOI_DA)` NGAY
sau khi sort giảm dần theo `tanSuat` — với `SO_GOI_Y_TOI_DA = 2`, chỉ
`2` phần TỬ đầu tiên (tần suất cao nhất) được GIỮ lại, phần còn lại
(`"may"`, tần suất `10`, thấp NHẤT trong ba) bị cắt bỏ HOÀN TOÀN khỏi
`nut.topK`. Đây chính LÀ đánh đổi: tốn bộ NHỚ lưu sẵn top-K, đổi lấy tốc
độ tra CỨU O(1) tại mỗi node.
::
:::
::::

::::code{#viet_lay_goi_y}
Hoàn thiện `layGoiY` — sau khi tìm được node của tiền tố, TRẢ thẳng
`topK` đã cache sẵn TẠI node đó (không duyệt lại cây).

```typescript title=starter
interface MucTanSuat { tu: string; tanSuat: number; }
interface NutTrieTanSuat {
  con: Map<string, NutTrieTanSuat>;
  laTuHoanChinh: boolean;
  tanSuat: number;
  topK: MucTanSuat[];
}
function taoNutTrieTanSuat(): NutTrieTanSuat {
  return { con: new Map(), laTuHoanChinh: false, tanSuat: 0, topK: [] };
}

function chenTuVoiTanSuat(goc: NutTrieTanSuat, tu: string, tanSuat: number): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrieTanSuat();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
  hienTai.tanSuat = tanSuat;
}

const SO_GOI_Y_TOI_DA = 2;

function xayDungTopK(nut: NutTrieTanSuat, tienToHienTai: string): MucTanSuat[] {
  let gopTatCa: MucTanSuat[] = [];
  if (nut.laTuHoanChinh) gopTatCa.push({ tu: tienToHienTai, tanSuat: nut.tanSuat });
  for (const [kyTu, con] of nut.con) {
    gopTatCa = gopTatCa.concat(xayDungTopK(con, tienToHienTai + kyTu));
  }
  gopTatCa.sort((a, b) => b.tanSuat - a.tanSuat);
  nut.topK = gopTatCa.slice(0, SO_GOI_Y_TOI_DA);
  return nut.topK;
}

function timNutTheoTienTo(goc: NutTrieTanSuat, tienTo: string): NutTrieTanSuat | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}

function layGoiY(goc: NutTrieTanSuat, tienTo: string): MucTanSuat[] {
  const hienTai = timNutTheoTienTo(goc, tienTo);
  if (hienTai === undefined) return [];
  ___
}

const goc = taoNutTrieTanSuat();
chenTuVoiTanSuat(goc, "may", 10);
chenTuVoiTanSuat(goc, "may tinh", 50);
xayDungTopK(goc, "");
console.log(JSON.stringify(layGoiY(goc, "may")));
```

```typescript title=solution
interface MucTanSuat { tu: string; tanSuat: number; }
interface NutTrieTanSuat {
  con: Map<string, NutTrieTanSuat>;
  laTuHoanChinh: boolean;
  tanSuat: number;
  topK: MucTanSuat[];
}
function taoNutTrieTanSuat(): NutTrieTanSuat {
  return { con: new Map(), laTuHoanChinh: false, tanSuat: 0, topK: [] };
}

function chenTuVoiTanSuat(goc: NutTrieTanSuat, tu: string, tanSuat: number): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrieTanSuat();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
  hienTai.tanSuat = tanSuat;
}

const SO_GOI_Y_TOI_DA = 2;

function xayDungTopK(nut: NutTrieTanSuat, tienToHienTai: string): MucTanSuat[] {
  let gopTatCa: MucTanSuat[] = [];
  if (nut.laTuHoanChinh) gopTatCa.push({ tu: tienToHienTai, tanSuat: nut.tanSuat });
  for (const [kyTu, con] of nut.con) {
    gopTatCa = gopTatCa.concat(xayDungTopK(con, tienToHienTai + kyTu));
  }
  gopTatCa.sort((a, b) => b.tanSuat - a.tanSuat);
  nut.topK = gopTatCa.slice(0, SO_GOI_Y_TOI_DA);
  return nut.topK;
}

function timNutTheoTienTo(goc: NutTrieTanSuat, tienTo: string): NutTrieTanSuat | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}

function layGoiY(goc: NutTrieTanSuat, tienTo: string): MucTanSuat[] {
  const hienTai = timNutTheoTienTo(goc, tienTo);
  if (hienTai === undefined) return [];
  return hienTai.topK;
}

const goc = taoNutTrieTanSuat();
chenTuVoiTanSuat(goc, "may", 10);
chenTuVoiTanSuat(goc, "may tinh", 50);
xayDungTopK(goc, "");
console.log(JSON.stringify(layGoiY(goc, "may")));
```

```typescript title=test
const gocT = taoNutTrieTanSuat();
chenTuVoiTanSuat(gocT, "may", 10);
chenTuVoiTanSuat(gocT, "may tinh", 50);
chenTuVoiTanSuat(gocT, "may anh", 30);
chenTuVoiTanSuat(gocT, "meo", 5);
xayDungTopK(gocT, "");

const gyMay = layGoiY(gocT, "may");
if (gyMay.length !== 2) throw new Error("SO_GOI_Y_TOI_DA=2, phai tra ve dung 2 goi y cho tien to 'may'");
if (gyMay[0]?.tu !== "may tinh" || gyMay[0]?.tanSuat !== 50) throw new Error("goi y dau tien phai la 'may tinh' (tan suat cao nhat, 50)");
if (gyMay[1]?.tu !== "may anh" || gyMay[1]?.tanSuat !== 30) throw new Error("goi y thu hai phai la 'may anh' (tan suat 30)");
if (gyMay.some((m) => m.tu === "may")) throw new Error("'may' (tan suat 10, thap nhat) phai BI LOAI khoi top-2");

const gyMe = layGoiY(gocT, "me");
if (gyMe.length !== 1) throw new Error("tien to 'me' chi co 1 ung vien, phai tra ve dung 1 goi y");
if (gyMe[0]?.tu !== "meo") throw new Error("goi y duy nhat cho 'me' phai la 'meo'");

if (layGoiY(gocT, "xyz").length !== 0) throw new Error("tien to khong ton tai phai tra ve mang rong");

const gyRong = layGoiY(gocT, "");
if (gyRong.length !== 2) throw new Error("tien to rong van bi gioi han boi SO_GOI_Y_TOI_DA=2");
if (gyRong[0]?.tu !== "may tinh") throw new Error("goi y cao nhat toan cuc phai la 'may tinh' (tan suat 50, cao nhat trong tat ca)");
```

:::hints
- kind: attention
  body: "layGoiY KHONG duoc duyet lai cay -- chi can doc truc tiep truong topK da duoc xayDungTopK tinh san tai node hienTai."
- kind: strategy
  body: "Sau khi xac dinh hienTai khong undefined, tra ve hienTai.topK -- day chinh la ban chat cua viec cache san."
- kind: one-line
  body: "return hienTai.topK;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "may tinh"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tra cứu giờ O(1) tại mỗi node, không cần duyệt lại subtree. Nhưng
`xayDungTopK` phải chạy LẠI mỗi khi có từ mới — chạy MỖI lần gõ phím thì
quá tốn, cần một nhịp CẬP nhật khác.
::::

::::reflect{#nghi-lai}
`layGoiY` bản THÂN chỉ có một dòng — cái LÀM nó nhanh không nằm Ở CHÍNH
nó, mà nằm Ở việc `xayDungTopK` đã LÀM sẵn công việc NẶNG (gộp, sort,
cắt) TỪ trước. Đây LÀ đánh đổi kinh điển: trả bộ NHỚ (mỗi node giữ thêm
một mảng) để MUA tốc độ đọc.
::::

::::checkpoint{mastery=0.73}
::::
