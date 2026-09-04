---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.quyet-dinh-gom-lo
title: "Batching thuần: gom đủ lô hoặc hết giờ chờ — chỉ quyết định, không tự gửi"
summary: "quyetDinhGomLo(buffer, suKienMoi, kichThuocLo, thoiGianChoMs) tra ve { loMoi, bufferMoi } -- ham THUAN, KHONG tu goi ham gui nao, chi QUYET DINH da du lo de gui hay chua (theo KICH THUOC hoac theo THOI GIAN CHO tinh tu su kien dau buffer); giong tinh than quyetDinh cua quest truoc, ap dung vao ngu canh gom lo streaming."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.quyet-dinh-gom-lo]
requires: [sd.fp.gom-theo-khung-thuan]
concepts: [sd.fp.quyet-dinh-gom-lo]
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
`gomTheoKhung` (bài trước) gom sự kiện vào khung thời gian — nhưng
không nói gì về việc GỬI dữ liệu đó đi đâu. Gửi từng sự kiện một, ngay
khi nó tới, thì mỗi lần gửi chỉ mang một mẩu nhỏ xíu — quá lãng phí. Gom
đủ một LÔ rồi mới gửi một lần thì hiệu quả hơn. Nhưng "đủ" là khi nào?
::::

::::explain{#quyet-dinh-khong-tu-gui}
`quyetDinhGomLo` chỉ trả lời một câu hỏi: với `buffer` hiện có VÀ một
`suKienMoi` vừa tới, lô đã đủ để gửi CHƯA? Nó xét HAI điều kiện — buffer
sau khi thêm `suKienMoi` đã đạt `kichThuocLo` CHƯA, HOẶC khoảng cách từ
sự kiện ĐẦU buffer tới `suKienMoi` đã vượt `thoiGianChoMs` CHƯA. Nếu
MỘT trong hai đúng, nó trả về `loMoi` (lô sẵn sàng gửi) VÀ `bufferMoi`
rỗng. Nếu chưa, nó trả về `loMoi: undefined` VÀ `bufferMoi` đã có thêm
sự kiện. Hàm không hề gọi bất kỳ hàm "gửi" nào — nó chỉ QUYẾT ĐỊNH:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }

function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}

const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });

console.log("--- gom du KICH THUOC (kichThuocLo=3) ---");
let bufferA: SuKien[] = [];
let r1 = quyetDinhGomLo(bufferA, suKien("a1", 0, 1), 3, 5000);
console.log("sau su kien 1:", JSON.stringify(r1));
bufferA = r1.bufferMoi;
let r2 = quyetDinhGomLo(bufferA, suKien("a2", 100, 1), 3, 5000);
console.log("sau su kien 2:", JSON.stringify(r2));
bufferA = r2.bufferMoi;
let r3 = quyetDinhGomLo(bufferA, suKien("a3", 200, 1), 3, 5000);
console.log("sau su kien 3 (du 3, phai GOM LO):", JSON.stringify(r3));
bufferA = r3.bufferMoi;

console.log("--- het THOI GIAN CHO (kichThuocLo=100, thoiGianChoMs=1000) ---");
let bufferB: SuKien[] = [];
let s1 = quyetDinhGomLo(bufferB, suKien("b1", 0, 1), 100, 1000);
console.log("sau su kien 1 (t=0):", JSON.stringify(s1));
bufferB = s1.bufferMoi;
let s2 = quyetDinhGomLo(bufferB, suKien("b2", 500, 1), 100, 1000);
console.log("sau su kien 2 (t=500, cach dau 500ms < 1000ms):", JSON.stringify(s2));
bufferB = s2.bufferMoi;
let s3 = quyetDinhGomLo(bufferB, suKien("b3", 1200, 1), 100, 1000);
console.log("sau su kien 3 (t=1200, cach dau 1200ms >= 1000ms, HET GIO):", JSON.stringify(s3));
bufferB = s3.bufferMoi;

console.log("--- tinh THUAN: goi lai cung tham so cho cung ket qua ---");
const bufferGoc: SuKien[] = [suKien("x1", 0, 1), suKien("x2", 100, 1)];
const suKienMoiGoc = suKien("x3", 200, 1);
const lanGoi1 = quyetDinhGomLo(bufferGoc, suKienMoiGoc, 3, 5000);
const lanGoi2 = quyetDinhGomLo(bufferGoc, suKienMoiGoc, 3, 5000);
console.log("lan goi 1:", JSON.stringify(lanGoi1));
console.log("hai lan goi giong het nhau?", JSON.stringify(lanGoi1) === JSON.stringify(lanGoi2));
console.log("bufferGoc van con", bufferGoc.length, "phan tu (khong bi mutate)");
```

```text title=readonly
--- gom du KICH THUOC (kichThuocLo=3) ---
sau su kien 1: {"bufferMoi":[{"id":"a1","nguon":"edge-1","thoiDiem":0,"soLuong":1}]}
sau su kien 2: {"bufferMoi":[{"id":"a1","nguon":"edge-1","thoiDiem":0,"soLuong":1},{"id":"a2","nguon":"edge-1","thoiDiem":100,"soLuong":1}]}
sau su kien 3 (du 3, phai GOM LO): {"loMoi":[{"id":"a1","nguon":"edge-1","thoiDiem":0,"soLuong":1},{"id":"a2","nguon":"edge-1","thoiDiem":100,"soLuong":1},{"id":"a3","nguon":"edge-1","thoiDiem":200,"soLuong":1}],"bufferMoi":[]}
--- het THOI GIAN CHO (kichThuocLo=100, thoiGianChoMs=1000) ---
sau su kien 1 (t=0): {"bufferMoi":[{"id":"b1","nguon":"edge-1","thoiDiem":0,"soLuong":1}]}
sau su kien 2 (t=500, cach dau 500ms < 1000ms): {"bufferMoi":[{"id":"b1","nguon":"edge-1","thoiDiem":0,"soLuong":1},{"id":"b2","nguon":"edge-1","thoiDiem":500,"soLuong":1}]}
sau su kien 3 (t=1200, cach dau 1200ms >= 1000ms, HET GIO): {"loMoi":[{"id":"b1","nguon":"edge-1","thoiDiem":0,"soLuong":1},{"id":"b2","nguon":"edge-1","thoiDiem":500,"soLuong":1},{"id":"b3","nguon":"edge-1","thoiDiem":1200,"soLuong":1}],"bufferMoi":[]}
--- tinh THUAN: goi lai cung tham so cho cung ket qua ---
lan goi 1: {"loMoi":[{"id":"x1","nguon":"edge-1","thoiDiem":0,"soLuong":1},{"id":"x2","nguon":"edge-1","thoiDiem":100,"soLuong":1},{"id":"x3","nguon":"edge-1","thoiDiem":200,"soLuong":1}],"bufferMoi":[]}
hai lan goi giong het nhau? true
bufferGoc van con 2 phan tu (khong bi mutate)
```

Chú ý: mỗi lần `loMoi` là `undefined`, `JSON.stringify` bỏ HẲN field
đó khỏi chuỗi kết quả — chỉ còn `bufferMoi`. `quyetDinhGomLo` không hề
biết "gửi đi" nghĩa là gì — nó chỉ trả về DỮ LIỆU (một lô sẵn sàng,
hoặc `undefined`). Ở kịch bản thứ hai, `kichThuocLo` được đặt RẤT cao
(`100`, không bao giờ đạt tới trong ba sự kiện), NÊN chỉ điều kiện thời
gian chờ mới có thể kích hoạt việc gom lô.
::::

::::example{#gap-nhieu-lan-khong-goi-ham-gui}
Vì `quyetDinhGomLo` là hàm thuần, gọi nó liên tiếp qua `reduce` cho cả
một luồng sự kiện — TÍCH LŨY các lô đã sẵn sàng — mà không có bất kỳ
lời gọi hàm "gửi" nào xen vào giữa các bước:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });

const luong: SuKien[] = [
  suKien("e1", 0, 5),
  suKien("e2", 300, 5),
  suKien("e3", 600, 5),
  suKien("e4", 900, 5),
  suKien("e5", 1400, 5),
];
const ketQua = luong.reduce(
  (tich, sk) => {
    const kq = quyetDinhGomLo(tich.buffer, sk, 3, 100000);
    if (kq.loMoi !== undefined) {
      return { buffer: kq.bufferMoi, cacLo: [...tich.cacLo, kq.loMoi] };
    }
    return { buffer: kq.bufferMoi, cacLo: tich.cacLo };
  },
  { buffer: [] as SuKien[], cacLo: [] as SuKien[][] }
);
console.log("so lo da gom (chi tu QUYET DINH, khong goi ham gui nao):", ketQua.cacLo.length);
console.log("lo dau tien co", ketQua.cacLo[0]?.length, "su kien");
console.log("buffer con lai (chua du de gom):", ketQua.buffer.length);
console.log("khong co dong nao trong quyetDinhGomLo goi console.log, fetch, hay ghiRaNgoai");
```

```text title=readonly
so lo da gom (chi tu QUYET DINH, khong goi ham gui nao): 1
lo dau tien co 3 su kien
buffer con lai (chua du de gom): 2
khong co dong nao trong quyetDinhGomLo goi console.log, fetch, hay ghiRaNgoai
```

`thoiGianChoMs` được đặt RẤT lớn (`100000`) Ở đây, nên chỉ điều kiện
kích thước (`kichThuocLo: 3`) có thể kích hoạt. Năm sự kiện qua
`reduce` tạo đúng MỘT lô (ba sự kiện đầu), còn lại `2` sự kiện nằm
trong `buffer`, chưa đủ để gom tiếp. Toàn bộ quá trình này không hề
"gửi" gì cả — `ketQua.cacLo` chỉ LÀ dữ liệu, một danh sách các lô đã
sẵn sàng, chờ một lớp KHÁC quyết định làm gì với chúng.
::::

::::predict{#doan-ranh-gioi-thoi-gian-cho commitOnce}
`buffer1` chỉ có một sự kiện Ở `thoiDiem: 0`. Gọi
`quyetDinhGomLo(buffer1, suKien("p2", 999, 1), 100, 1000)` (khoảng
cách `999ms`, `kichThuocLo` rất cao nên không thể kích hoạt bằng kích
thước) — VÀ tách biệt, gọi `quyetDinhGomLo(buffer1, suKien("p2", 1000,
1), 100, 1000)` (khoảng cách ĐÚNG BẰNG `1000ms`). Hai lệnh gọi này
khác nhau Ở `loMoi` như thế nào?

:::opt{correct}
Lệnh gọi với khoảng cách `999ms` cho `loMoi: undefined` (chưa hết giờ
chờ); lệnh gọi với khoảng cách ĐÚNG `1000ms` cho `loMoi` LÀ một mảng
CÓ giá trị — điều kiện dùng `>=`, nên chạm ĐÚNG ngưỡng cũng LÀ hết giờ
chờ, không cần vượt quá
:::
:::opt
Cả hai đều cho `loMoi: undefined` — vì "hết giờ chờ" chỉ có nghĩa khi
khoảng cách LỚN HƠN `thoiGianChoMs`, chạm đúng ngưỡng vẫn còn TRONG
giới hạn cho phép
::why
Nhầm ngưỡng so sánh — dòng code dùng đúng toán tử `>=`, không phải
`>`, cho biểu thức `suKienMoi.thoiDiem - dauBuffer.thoiDiem >=
thoiGianChoMs`.

Chỗ lệch: với khoảng cách `1000ms` VÀ `thoiGianChoMs: 1000`, biểu thức
là `1000 >= 1000`, tức `true` — `daHetThoiGianCho` được kích hoạt,
hàm trả về `loMoi` có giá trị. Đây là tình huống "chạm đúng ngưỡng vẫn
hợp lệ" đã gặp Ở bài `quyetDinhDem` của quest "Nhật ký bất biến và
fold" (rút đúng bằng số dư vẫn được chấp nhận) — Ở đây, ngưỡng thời
gian chờ hoạt động theo đúng logic tương tự: chạm mốc LÀ đủ điều kiện,
không cần vượt qua nó.
::
:::
::::

::::code{#viet_quyet_dinh_gom_lo}
Hoàn thiện `quyetDinhGomLo` — tạo `bufferSau` bằng cách thêm
`suKienMoi` vào cuối `buffer` (không sửa `buffer` truyền vào). Nếu
`bufferSau.length >= kichThuocLo`, HOẶC (sự kiện đầu tiên của
`bufferSau` tồn tại VÀ `suKienMoi.thoiDiem` trừ `thoiDiem` của sự kiện
đó `>= thoiGianChoMs`), trả về `{ loMoi: bufferSau, bufferMoi: [] }`.
Ngược lại, trả về `{ loMoi: undefined, bufferMoi: bufferSau }`.

```typescript title=starter
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }

function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  ___
}

const skX1: SuKien = { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 };
const skX2: SuKien = { id: "x2", nguon: "e", thoiDiem: 100, soLuong: 1 };
const rX1 = quyetDinhGomLo([], skX1, 2, 5000);
const rX2 = quyetDinhGomLo(rX1.bufferMoi, skX2, 2, 5000);
console.log(rX2.loMoi?.length, rX2.bufferMoi.length);
```

```typescript title=solution
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }

function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}

const skX1: SuKien = { id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 };
const skX2: SuKien = { id: "x2", nguon: "e", thoiDiem: 100, soLuong: 1 };
const rX1 = quyetDinhGomLo([], skX1, 2, 5000);
const rX2 = quyetDinhGomLo(rX1.bufferMoi, skX2, 2, 5000);
console.log(rX2.loMoi?.length, rX2.bufferMoi.length);
```

```typescript title=test
const tSk = (id: string, thoiDiem: number): SuKien => ({ id, nguon: "e", thoiDiem, soLuong: 1 });

const tR1 = quyetDinhGomLo([], tSk("t1", 0), 3, 1000);
if (tR1.loMoi !== undefined) throw new Error("1 su kien voi kichThuocLo=3 chua du, loMoi phai la undefined");
if (tR1.bufferMoi.length !== 1) throw new Error("bufferMoi phai co 1 phan tu");

const tR2 = quyetDinhGomLo(tR1.bufferMoi, tSk("t2", 100), 3, 1000);
if (tR2.loMoi !== undefined) throw new Error("2 su kien van chua du kichThuocLo=3");
if (tR2.bufferMoi.length !== 2) throw new Error("bufferMoi phai co 2 phan tu");

const tR3 = quyetDinhGomLo(tR2.bufferMoi, tSk("t3", 200), 3, 1000);
if (tR3.loMoi === undefined) throw new Error("du 3 su kien (kichThuocLo=3) phai tra ve loMoi");
if (tR3.loMoi.length !== 3) throw new Error("loMoi phai co dung 3 su kien");
if (tR3.bufferMoi.length !== 0) throw new Error("bufferMoi phai RONG sau khi gom lo");

const tBufferTimeout: SuKien[] = [tSk("u1", 0)];
const tRTimeout = quyetDinhGomLo(tBufferTimeout, tSk("u2", 1000), 100, 1000);
if (tRTimeout.loMoi === undefined) throw new Error("cach dau buffer DUNG BANG thoiGianChoMs (1000>=1000) phai duoc gom lo");

const tBufferChuaTimeout: SuKien[] = [tSk("v1", 0)];
const tRChuaTimeout = quyetDinhGomLo(tBufferChuaTimeout, tSk("v2", 999), 100, 1000);
if (tRChuaTimeout.loMoi !== undefined) throw new Error("cach dau buffer 999ms < 1000ms KHONG duoc gom lo");

const tBufferGoc: SuKien[] = [tSk("w1", 0), tSk("w2", 50)];
const tSuKienMoiGoc = tSk("w3", 100);
const truocGoi = JSON.stringify(tBufferGoc);
quyetDinhGomLo(tBufferGoc, tSuKienMoiGoc, 10, 1000);
if (JSON.stringify(tBufferGoc) !== truocGoi) throw new Error("quyetDinhGomLo KHONG duoc mutate buffer truyen vao");

const tLanGoi1 = quyetDinhGomLo(tBufferGoc, tSuKienMoiGoc, 3, 1000);
const tLanGoi2 = quyetDinhGomLo(tBufferGoc, tSuKienMoiGoc, 3, 1000);
if (JSON.stringify(tLanGoi1) !== JSON.stringify(tLanGoi2)) throw new Error("quyetDinhGomLo phai THUAN -- cung dau vao phai cho cung ket qua");
```

:::hints
- kind: attention
  body: "bufferSau = [...buffer, suKienMoi]. daDuKichThuoc = bufferSau.length >= kichThuocLo. Lay dauBuffer = bufferSau[0] (co the undefined ve mat kieu, kiem tra truoc khi tru). daHetThoiGianCho = dauBuffer !== undefined && (suKienMoi.thoiDiem - dauBuffer.thoiDiem) >= thoiGianChoMs. Neu MOT trong hai dieu kien dung, tra ve { loMoi: bufferSau, bufferMoi: [] }; nguoc lai { loMoi: undefined, bufferMoi: bufferSau }."
- kind: strategy
  body: "const bufferSau = [...buffer, suKienMoi]; const dauBuffer = bufferSau[0]; const daDuKichThuoc = bufferSau.length >= kichThuocLo; const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs; if (daDuKichThuoc || daHetThoiGianCho) { return { loMoi: bufferSau, bufferMoi: [] }; } return { loMoi: undefined, bufferMoi: bufferSau };"
- kind: one-line
  body: "const bufferSau = [...buffer, suKienMoi]; const dauBuffer = bufferSau[0]; const daDuKichThuoc = bufferSau.length >= kichThuocLo; const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs; if (daDuKichThuoc || daHetThoiGianCho) { return { loMoi: bufferSau, bufferMoi: [] }; } return { loMoi: undefined, bufferMoi: bufferSau };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gom đủ lô hoặc hết giờ chờ — quyết định thuần, không một lời gọi gửi
nào. Nhưng nếu buffer nhận sự kiện nhanh hơn tốc độ gom lô kịp xử lý,
điều gì ngăn nó phình to vô hạn?
::::

::::reflect{#nghi-lai}
`quyetDinhGomLo` chỉ trả lời "lô đã sẵn sàng CHƯA" — nó không hề biết
"sẵn sàng rồi thì làm gì". Tách biệt này lặp lại đúng bài học của
`quyetDinh` Ở quest "Nhật ký bất biến và fold": một hàm thuần chỉ ra
quyết định, và một lớp KHÁC (chưa xuất hiện Ở bài này) sẽ đọc quyết
định đó rồi hành động. Khác biệt duy nhất: Ở đó quyết định là "chấp
nhận hay từ chối MỘT lệnh"; Ở đây là "lô đã đủ hay chưa" — cùng một
khuôn dạng tư duy, áp dụng cho một bài toán hoàn toàn khác.
::::

::::checkpoint{mastery=0.74}
::::
