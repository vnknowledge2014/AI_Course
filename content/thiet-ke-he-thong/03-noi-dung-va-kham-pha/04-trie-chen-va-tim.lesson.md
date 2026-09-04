---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.trie-chen-va-tim
title: "Trie: chèn và tìm theo tiền tố"
summary: "chenTuVaoTrie/timMoiTuTheoTienTo tự viết node-based (Map<string,NutTrie>, KHONG dung thu vien). Chen 4 tu (may, may tinh, may anh, meo): tim tien to 'may' tra ve dung 3 ket qua ['may','may tinh','may anh'] -- 'may' XUAT HIEN trong chinh ket qua cua no du cung la tien to cua 2 tu kia (co laTuHoanChinh doc lap voi viec co node con hay khong). Tien to 'me' tra ve dung ['meo']; tien to khong ton tai tra ve mang rong."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.trie-chen-va-tim]
requires: [sd.nguoi-noi-tieng-va-hybrid]
concepts: [sd.trie-chen-va-tim]
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
News Feed xong. Bài toán KHÁC hẳn: gõ MỖI ký tự vào ô tìm kiếm, hệ thống
phải gợi Ý ngay lập tức MỌI từ khớp tiền tố. Cấu trúc phù hợp NHẤT cho
việc này LÀ một cây tiền tố — trie.
::::

::::explain{#trie-node-based}
Một trie LÀ một cây MÀ mỗi cạnh mang ĐÚNG một ký tự — đi TỪ gốc theo một
chuỗi ký tự SẼ dẫn tới đúng node đại diện cho tiền tố ĐÓ. Mỗi node giữ
một `Map` con TRỎ theo ký tự, VÀ một cờ đánh dấu "đây có LÀ một từ hoàn
chỉnh hay chỉ LÀ tiền tố trung gian":

```typescript title=readonly
interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }

function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrie();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}

function timNutTheoTienTo(goc: NutTrie, tienTo: string): NutTrie | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}

function thuThapTatCaTu(nut: NutTrie, tienToHienTai: string, ketQua: string[]): void {
  if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
  for (const [kyTu, con] of nut.con) thuThapTatCaTu(con, tienToHienTai + kyTu, ketQua);
}

function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  const nutGoc = timNutTheoTienTo(goc, tienTo);
  if (nutGoc === undefined) return [];
  const ketQua: string[] = [];
  thuThapTatCaTu(nutGoc, tienTo, ketQua);
  return ketQua;
}

const goc = taoNutTrie();
chenTuVaoTrie(goc, "may");
chenTuVaoTrie(goc, "may tinh");
chenTuVaoTrie(goc, "may anh");
chenTuVaoTrie(goc, "meo");
console.log('tim tien to "may":', JSON.stringify(timMoiTuTheoTienTo(goc, "may")));
console.log('tim tien to "me":', JSON.stringify(timMoiTuTheoTienTo(goc, "me")));
```

```text title=readonly
tim tien to "may": ["may","may tinh","may anh"]
tim tien to "me": ["meo"]
```

`chenTuVaoTrie` đi TỪNG ký tự, tạo node MỚI nếu chưa CÓ cạnh tương ứng,
RỒI đánh dấu `laTuHoanChinh = true` TẠI node cuối cùng. `timMoiTuTheoTienTo`
đi TỚI đúng node của tiền tố, RỒI duyệt (DFS) toàn bộ cây CON để gom mọi
từ hoàn chỉnh — `"may"` khớp `3` từ ĐÃ chèn, `"me"` chỉ khớp `"meo"`.
::::

::::example{#tien-to-khong-ton-tai-va-tien-to-rong}
Hai trường hợp BIÊN đáng chú ý: một tiền tố KHÔNG hề xuất hiện trong bất
kỳ từ nào ĐÃ chèn (không CÓ node tương ứng), VÀ tiền tố RỖNG (khớp MỌI
từ, vì mọi chuỗi đều "bắt đầu" bằng chuỗi rỗng):

```typescript title=readonly
interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }

function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrie();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}

function timNutTheoTienTo(goc: NutTrie, tienTo: string): NutTrie | undefined {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return undefined;
    hienTai = con;
  }
  return hienTai;
}

function thuThapTatCaTu(nut: NutTrie, tienToHienTai: string, ketQua: string[]): void {
  if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
  for (const [kyTu, con] of nut.con) thuThapTatCaTu(con, tienToHienTai + kyTu, ketQua);
}

function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  const nutGoc = timNutTheoTienTo(goc, tienTo);
  if (nutGoc === undefined) return [];
  const ketQua: string[] = [];
  thuThapTatCaTu(nutGoc, tienTo, ketQua);
  return ketQua;
}

// tai lap dung trang thai da chen o khoi truoc: chen lai ca 4 tu
const goc = taoNutTrie();
chenTuVaoTrie(goc, "may");
chenTuVaoTrie(goc, "may tinh");
chenTuVaoTrie(goc, "may anh");
chenTuVaoTrie(goc, "meo");

console.log('tim tien to "xyz" (khong ton tai):', JSON.stringify(timMoiTuTheoTienTo(goc, "xyz")));
console.log('so tu khop tien to rong "":', timMoiTuTheoTienTo(goc, "").length);
```

```text title=readonly
tim tien to "xyz" (khong ton tai): []
so tu khop tien to rong "": 4
```

`"xyz"` không hề CÓ cạnh nào trong trie NÊN `timNutTheoTienTo` trả về
`undefined` NGAY từ ký tự đầu, VÀ `timMoiTuTheoTienTo` trả VỀ mảng rỗng
mà không hề đụng TỚI `thuThapTatCaTu`. Tiền tố rỗng thì NGƯỢC lại — vòng
lặp `for` không chạy lần NÀO, `nutGoc` chính LÀ `goc`, nên nó khớp CẢ
`4` từ đã chèn.
::::

::::predict{#doan-tu-vua-la-tien-to-vua-la-ket-qua commitOnce}
Trie đã chèn `"may"`, `"may tinh"`, `"may anh"`. `"may"` VỪA là một từ
hoàn chỉnh (đã chèn RIÊNG), VỪA là tiền tố chung của `"may tinh"` VÀ
`"may anh"`. Gọi `timMoiTuTheoTienTo(goc, "may")` — kết quả CÓ chứa
chính `"may"` không?

:::opt{correct}
CÓ — `laTuHoanChinh` đánh dấu ĐỘC LẬP tại node `"may"`, VÀ `thuThapTatCaTu`
kiểm tra cờ đó NGAY khi bước vào node (trước khi đệ quy xuống con), nên
`"may"` được thu thập dù nó CÒN có node con
:::
:::opt
KHÔNG — `"may"` chỉ đóng vai TRÒ tiền tố trung gian dẫn tới `"may tinh"`
VÀ `"may anh"`, một tiền tố trung gian không được TÍNH là kết quả riêng
::why
Nhầm "có node con" VỚI "không phải là một từ hoàn chỉnh" — nhưng hai
thuộc tính NÀY của một node hoàn toàn ĐỘC LẬP với nhau trong cách
`NutTrie` được thiết kế.

Chỗ lệch: `hienTai.laTuHoanChinh = true` được ĐẶT tại đúng node cuối
cùng của MỖI lần `chenTuVaoTrie` chạy — node `"may"` được đánh dấu
`laTuHoanChinh = true` từ LẦN chèn riêng của chính nó, HOÀN TOÀN không
phụ thuộc việc nó SAU đó còn có thêm node con (`" tinh"`, `" anh"`) hay
không. `thuThapTatCaTu` kiểm tra `nut.laTuHoanChinh` ở DÒNG đầu tiên,
trước cả vòng lặp duyệt con — nên `"may"` luôn có mặt trong kết quả.
::
:::
::::

::::code{#viet_chen_tu_vao_trie}
Hoàn thiện `chenTuVaoTrie` — khi ký tự HIỆN tại chưa có cạnh tương ứng
(`con === undefined`), tạo một node MỚI và gắn nó vào `hienTai.con`.

```typescript title=starter
interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }

function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      ___
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}

function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return [];
    hienTai = con;
  }
  const ketQua: string[] = [];
  const thuThap = (nut: NutTrie, tienToHienTai: string): void => {
    if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
    for (const [k, c] of nut.con) thuThap(c, tienToHienTai + k);
  };
  thuThap(hienTai, tienTo);
  return ketQua;
}

const goc = taoNutTrie();
chenTuVaoTrie(goc, "may");
chenTuVaoTrie(goc, "meo");
console.log(JSON.stringify(timMoiTuTheoTienTo(goc, "m")));
```

```typescript title=solution
interface NutTrie { con: Map<string, NutTrie>; laTuHoanChinh: boolean; }
function taoNutTrie(): NutTrie { return { con: new Map(), laTuHoanChinh: false }; }

function chenTuVaoTrie(goc: NutTrie, tu: string): void {
  let hienTai = goc;
  for (const kyTu of tu) {
    let con = hienTai.con.get(kyTu);
    if (con === undefined) {
      con = taoNutTrie();
      hienTai.con.set(kyTu, con);
    }
    hienTai = con;
  }
  hienTai.laTuHoanChinh = true;
}

function timMoiTuTheoTienTo(goc: NutTrie, tienTo: string): string[] {
  let hienTai = goc;
  for (const kyTu of tienTo) {
    const con = hienTai.con.get(kyTu);
    if (con === undefined) return [];
    hienTai = con;
  }
  const ketQua: string[] = [];
  const thuThap = (nut: NutTrie, tienToHienTai: string): void => {
    if (nut.laTuHoanChinh) ketQua.push(tienToHienTai);
    for (const [k, c] of nut.con) thuThap(c, tienToHienTai + k);
  };
  thuThap(hienTai, tienTo);
  return ketQua;
}

const goc = taoNutTrie();
chenTuVaoTrie(goc, "may");
chenTuVaoTrie(goc, "meo");
console.log(JSON.stringify(timMoiTuTheoTienTo(goc, "m")));
```

```typescript title=test
const gocT = taoNutTrie();
chenTuVaoTrie(gocT, "may");
chenTuVaoTrie(gocT, "may tinh");
chenTuVaoTrie(gocT, "may anh");
chenTuVaoTrie(gocT, "meo");

const ketQuaMay = timMoiTuTheoTienTo(gocT, "may").sort();
if (JSON.stringify(ketQuaMay) !== JSON.stringify(["may", "may anh", "may tinh"])) {
  throw new Error("tien to 'may' phai tra ve dung 3 tu: may, may anh, may tinh");
}
if (!ketQuaMay.includes("may")) throw new Error("'may' phai co mat trong ket qua cua chinh no");

const ketQuaMe = timMoiTuTheoTienTo(gocT, "me");
if (JSON.stringify(ketQuaMe) !== JSON.stringify(["meo"])) throw new Error("tien to 'me' chi phai tra ve ['meo']");

if (timMoiTuTheoTienTo(gocT, "xyz").length !== 0) throw new Error("tien to khong ton tai phai tra ve mang rong");
if (timMoiTuTheoTienTo(gocT, "may bay").length !== 0) throw new Error("tien to dai hon moi tu hien co phai tra ve mang rong");

const ketQuaRong = timMoiTuTheoTienTo(gocT, "").sort();
if (JSON.stringify(ketQuaRong) !== JSON.stringify(["may", "may anh", "may tinh", "meo"])) {
  throw new Error("tien to rong phai khop TOAN BO 4 tu da chen");
}

chenTuVaoTrie(gocT, "meo");
if (timMoiTuTheoTienTo(gocT, "meo").length !== 1) throw new Error("chen trung mot tu khong duoc tao ban sao trong ket qua");
```

:::hints
- kind: attention
  body: "Khi con === undefined, phai TAO mot node moi (dung taoNutTrie) VA GAN no vao hienTai.con tai dung ky tu kyTu."
- kind: strategy
  body: "con = taoNutTrie() tao node; hienTai.con.set(kyTu, con) gan node do lam canh moi tu hienTai."
- kind: one-line
  body: "con = taoNutTrie(); hienTai.con.set(kyTu, con);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "may"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Trie chèn VÀ tìm đã xong. Nhưng mỗi lần gõ thêm MỘT ký tự, hệ thống lại
duyệt LẠI toàn bộ cây con — với hàng triệu từ, việc đó CHẬM đi rõ rệt.
::::

::::reflect{#nghi-lai}
`chenTuVaoTrie` VÀ `timMoiTuTheoTienTo` không hề PHỨC tạp hơn một cây
thường — cái LÀM nó thành trie LÀ cạnh MANG ký tự, còn node chỉ giữ MỘT
cờ boolean. `laTuHoanChinh` độc lập VỚI việc có con hay KHÔNG chính LÀ
điều cho phép một từ vừa LÀ kết quả, vừa LÀ tiền tố của từ khác.
::::

::::checkpoint{mastery=0.71}
::::
