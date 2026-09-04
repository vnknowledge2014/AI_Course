---
id: co-so-du-lieu.byteledger-capstone.kien-truc-byteledger
title: "Kiến trúc ByteLedger — tổng quan"
summary: "Replica (q19) giờ mang trong mình một HeThongSoCai ĐẦY ĐỦ (q16 — TaiKhoan/ButToan/cacPendingDangCho/dsIdDaXuLy, không phải chỉ Map<TaiKhoan> đơn giản như q19 dùng) qua trường soCai. taoReplica(0,3,[0,1,2]) rồi apDungQuaSoCai nạp 1000 cho TK1 trực tiếp vào r0.soCai: soDuSoSach=1000, heThongCanBang=true. Một replica KHÁC (r1, cùng tham số) vẫn có soDu TK1=0 — mỗi replica giữ một soCai HOÀN TOÀN riêng, y hệt tinh thần 'trạng thái tách biệt' của q19 bài 2."
locale: vi
track: co-so-du-lieu
module: byteledger-capstone
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.kien-truc-byteledger]
requires: [db.state-transfer-dong-bo-lac-hau]
concepts: [db.kien-truc-byteledger]
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
q19 khép lại VỚI một `Replica` chỉ mang hai bút toán demo trong MỘT
`Map<TaiKhoan>` đơn sơ. q16 xây MỘT sổ cái đầy đủ — double-entry,
128-byte, pending/post/void, idempotent. q21 ghép CẢ hai làm một.
Bước đầu tiên: cho `Replica` MANG trong mình một sổ cái thật.
::::

::::explain{#soCai-vao-replica}
`Replica` (q19 bài 2) giờ có thêm đúng MỘT trường mới: `soCai:
HeThongSoCai` — KHÔNG phải `Map<number, TaiKhoan>` đơn giản q19 dùng
tạm, mà LÀ `HeThongSoCai` đầy đủ (q16 BOSS): `cacTaiKhoan`,
`cacPendingDangCho` (cho pending/post/void), VÀ `dsIdDaXuLy` (cho
idempotent). `taoReplica` giờ nhận THÊM `cacIdTaiKhoan` để khởi tạo
sổ cái đó:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): void {
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0, soCai: taoHeThongSoCai(cacIdTaiKhoan) };
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
apDungQuaSoCai(r0.soCai, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
console.log("r0 -- soDu TK1:", soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!));
console.log("r0 -- canBang:", heThongCanBang(r0.soCai));

const r1 = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
console.log("r1 (rieng biet) -- soDu TK1:", soDuSoSach(r1.soCai.cacTaiKhoan.get(TK1)!));
```

```text title=readonly
r0 -- soDu TK1: 1000
r0 -- canBang: true
r1 (rieng biet) -- soDu TK1: 0
```

`r0.soCai` VÀ `r1.soCai` LÀ hai object HOÀN toàn tách biệt (`taoHeThongSoCai`
gọi `new Map()` MỚI mỗi lần) — áp dụng bút toán VÀO `r0.soCai` không
hề đụng TỚI `r1.soCai`, y hệt cách `r0.log` VÀ `r1.log` (q19 bài 2)
độc lập với nhau. Bảy trường VSR (`chiSo`...`commitNumber`) GIỮ
NGUYÊN như q19 — CHỈ thêm ĐÚNG một trường `soCai`.
::::

::::example{#khong-phai-map-don-gian}
q19 (bài 3-13) áp dụng bút toán TRỰC TIẾP vào một
`Map<number,TaiKhoan>` đơn — KHÔNG hề có `cacPendingDangCho` (không
CÓ pending/post/void) VÀ KHÔNG có `dsIdDaXuLy` (không CÓ idempotent
THẬT — chỉ có `opNumber` bảo VỆ khỏi double-commit, MỘT lớp bảo vệ
KHÁC hẳn). `HeThongSoCai` LÀ nguyên VẸN kết quả của q16 BOSS — KHÔNG
đơn giản hoá thêm GÌ. Đây chính LÀ "nâng cấp struct" quest NÀY hứa Ở
đầu: một `Replica` giờ LÀ một node đồng thuận MANG một sổ cái THẬT,
không phải một demo hai dòng.
::::

::::predict{#doan-hai-replica-doc-lap commitOnce}
Tạo `r2 = taoReplica(2, 3, [NGUON_NGOAI, TK1, TK2])`. Áp dụng
`apDungQuaSoCai(r0.soCai, ...)` MỘT bút toán `500` NỮA (chuyển từ TK1
sang TK2, sau readonly Ở trên — `r0` đã có `soDu TK1=1000`). `soDu`
TK1 CỦA `r2.soCai` (chưa từng bị đụng TỚI) LÀ bao nhiêu?
:::opt{correct}
`0` — `r2` LÀ một replica MỚI, hoàn toàn tách biệt VỚI `r0`; KHÔNG có
cơ chế nào (ngoài đúng VSR sẽ học Ở bài SAU) đồng bộ `soCai` giữa các
replica — áp dụng TRỰC TIẾP vào `r0.soCai` không lan sang `r2.soCai`
:::
:::opt
`500` — hai replica CÙNG tham gia một cụm (`tongSo=3`) NÊN chia sẻ
CHUNG một trạng thái sổ cái
::why
Trực giác NÀY nhầm "CÙNG một cụm" VỚI "CÙNG một bộ nhớ" — nhưng VSR
(và MỌI hệ đồng thuận) tồn tại CHÍNH VÌ mỗi replica giữ bản SAO riêng
của nó, KHÔNG chia sẻ bộ nhớ VẬT lý.

Chỗ lệch: `taoReplica` gọi `taoHeThongSoCai` MỚI cho MỖI replica —
`r0.soCai` VÀ `r2.soCai` LÀ hai `object` khác nhau TRONG bộ nhớ.
`apDungQuaSoCai(r0.soCai, ...)` chỉ ghi VÀO đúng object ĐƯỢC truyền
làm tham số — `r2.soCai` không hề được nhắc TỚI, giữ NGUYÊN trạng
thái khởi tạo (`soDu TK1 = 0`). Đồng BỘ giữa các replica LÀ việc CỦA
giao thức đồng thuận (bài 2 trở đi), KHÔNG phải một tính chất TỰ
động của việc "cùng cụm".
::
:::
::::

::::code{#viet_tao_replica_bytelledger}
Hoàn thiện `taoReplica` — thêm trường `soCai: taoHeThongSoCai(cacIdTaiKhoan)`
VÀO object trả về, GIỮ NGUYÊN sáu trường VSR đã có từ q19.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): void {
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return {
    chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0,
    ___
  };
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
apDungQuaSoCai(r0.soCai, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
console.log(soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

interface HeThongSoCai {
  cacTaiKhoan: Map<number, TaiKhoan>;
  cacPendingDangCho: Map<number, ButToan>;
  dsIdDaXuLy: Set<number>;
}
function taoHeThongSoCai(cacId: number[]): HeThongSoCai {
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of cacId) cacTaiKhoan.set(id, taoTaiKhoan(id));
  return { cacTaiKhoan, cacPendingDangCho: new Map(), dsIdDaXuLy: new Set() };
}
function heThongCanBang(soCai: HeThongSoCai): boolean {
  let no = 0, co = 0;
  for (const [, tk] of soCai.cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}
function apDungQuaSoCai(soCai: HeThongSoCai, bt: ButToan): void {
  soCai.cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  soCai.cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
  soCai: HeThongSoCai;
}
function taoReplica(chiSo: number, tongSo: number, cacIdTaiKhoan: number[]): Replica {
  return {
    chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0,
    soCai: taoHeThongSoCai(cacIdTaiKhoan),
  };
}

const NGUON_NGOAI = 0, TK1 = 1, TK2 = 2;
const r0 = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
apDungQuaSoCai(r0.soCai, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 1000 });
console.log(soDuSoSach(r0.soCai.cacTaiKhoan.get(TK1)!));
```

```typescript title=test
const rT = taoReplica(2, 5, [NGUON_NGOAI, TK1, TK2]);
if (rT.chiSo !== 2 || rT.tongSo !== 5) throw new Error("chiSo/tongSo phai dung tham so");
if (rT.viewNumber !== 0 || rT.status !== 'normal' || rT.opNumber !== 0 || rT.commitNumber !== 0 || rT.log.length !== 0) {
  throw new Error("cac truong VSR (q19 bai 2) phai khoi tao dung nhu cu -- khong bi doi khi them soCai");
}
if (!(rT.soCai.cacTaiKhoan instanceof Map)) throw new Error("soCai.cacTaiKhoan phai la Map");
if (rT.soCai.cacTaiKhoan.size !== 3) throw new Error("soCai phai co dung 3 tai khoan theo cacIdTaiKhoan truyen vao");
if (rT.soCai.cacPendingDangCho.size !== 0) throw new Error("cacPendingDangCho ban dau phai rong");
if (rT.soCai.dsIdDaXuLy.size !== 0) throw new Error("dsIdDaXuLy ban dau phai rong");
if (!heThongCanBang(rT.soCai)) throw new Error("so cai moi (rong) phai can bang");

const rA = taoReplica(0, 3, [NGUON_NGOAI, TK1, TK2]);
const rB = taoReplica(1, 3, [NGUON_NGOAI, TK1, TK2]);
apDungQuaSoCai(rA.soCai, { id: 1, debitAccountId: NGUON_NGOAI, creditAccountId: TK1, amount: 500 });
if (soDuSoSach(rA.soCai.cacTaiKhoan.get(TK1)!) !== 500) throw new Error("rA phai co soDu 500 sau khi ap dung");
if (soDuSoSach(rB.soCai.cacTaiKhoan.get(TK1)!) !== 0) throw new Error("rB (replica KHAC) khong duoc bi anh huong -- moi replica giu soCai RIENG");
if (!heThongCanBang(rA.soCai)) throw new Error("rA phai can bang sau 1 but toan hop le");
```

:::hints
- kind: attention
  body: "Them dung mot truong soCai: taoHeThongSoCai(cacIdTaiKhoan) vao object tra ve -- mot truong."
- kind: strategy
  body: "soCai: taoHeThongSoCai(cacIdTaiKhoan),"
- kind: one-line
  body: "soCai: taoHeThongSoCai(cacIdTaiKhoan),"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Struct đã ghép. Nhưng ÁP dụng trực tiếp NHƯ vừa làm không LÀ VSR —
làm sao đúng một bút toán đi qua ĐƯỜNG prepare/prepare_ok/commit,
VÀO đúng cái sổ cái ĐẦY đủ này?
::::

::::reflect{#nghi-lai}
`taoReplica` giờ trả VỀ một object CÓ đúng tám trường thay VÌ bảy —
KHÔNG đổi Ý nghĩa của bảy trường CŨ, chỉ thêm MỘT cánh cửa VÀO sổ cái
thật. Đây LÀ bước "ghép" đầu tiên trong MƯỜI bước của q21: mọi bài
SAU đều XÂY trên đúng struct NÀY — VSR (bài 2), fault injection (bài
3-4), crash recovery (bài 5), view change (bài 6-7), mini-VOPR (bài
8), audit (bài 9), VÀ BOSS (bài 10).
::::

::::checkpoint{mastery=0.8}
::::
