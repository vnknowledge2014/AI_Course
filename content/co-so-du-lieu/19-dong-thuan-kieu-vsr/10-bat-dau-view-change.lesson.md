---
id: co-so-du-lieu.dong-thuan-kieu-vsr.bat-dau-view-change
title: "Bắt đầu View Change"
summary: "nhanStartViewChange gộp phiếu vào Set theo viewNumber (bản thân replica tự tính LÀ một phiếu khi Set vừa tạo, giống nhanPrepareOk ở bài 6) -- đủ nguongQuorum thì trả true. N=5 (quorum=3): tự-vote+1 phiếu KHÁC (=2) CHƯA đủ -- false; thêm 1 phiếu KHÁC nữa (=3) đủ -- true; phiếu TRÙNG LẶP (cùng tu gửi lại) không làm giảm, vẫn true nhờ Set khử trùng. View KHÁC nhau tính tách biệt hoàn toàn."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.bat-dau-view-change]
requires: [db.phat-hien-primary-chet]
concepts: [db.bat-dau-view-change]
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
Một backup vừa nghi ngờ (bài trước), tăng `viewNumber`, chuyển
`'view-change'`. Nhưng MỘT mình nó không đủ tư CÁCH đổi primary — nó
phải rủ REPLICA khác, VÀ đợi đa số cùng đồng Ý view MỚI đó.
::::

::::explain{#dem-phieu-view-change}
`nhanStartViewChange` gộp phiếu VÀO một `Set` THEO `viewNumber` —
CHÍNH replica đang ĐẾM tự tính LÀ một phiếu ngay khi `Set` được TẠO
(giống hệt cách `nhanPrepareOk` tự tính primary Ở bài 6). Đủ
`nguongQuorum` thì trả `true`:

```typescript title=readonly
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanStartViewChange(
  replica: Replica, phieuTheoView: Map<number, Set<number>>, td: ThongDiep,
): boolean {
  let phieu = phieuTheoView.get(td.viewNumber);
  if (!phieu) { phieu = new Set<number>([replica.chiSo]); phieuTheoView.set(td.viewNumber, phieu); }
  phieu.add(td.tu);
  return phieu.size >= nguongQuorum(replica.tongSo);
}

const backup1 = taoReplica(1, 5); // N=5, quorum=3
backup1.status = 'view-change'; backup1.viewNumber = 1;
const phieuTheoView = new Map<number, Set<number>>();
console.log("phieu tu backup0:", nhanStartViewChange(backup1, phieuTheoView, { loai: 'StartViewChange', tu: 0, den: 1, viewNumber: 1 }));
console.log("phieu tu backup3:", nhanStartViewChange(backup1, phieuTheoView, { loai: 'StartViewChange', tu: 3, den: 1, viewNumber: 1 }));
```

```text title=readonly
phieu tu backup0: false
phieu tu backup3: true
```

`N=5` cần `3` phiếu (bài 1). Tự-VOTE (`backup1`) `+` phiếu TỪ
`backup0` `= 2` — CHƯA đủ. Thêm phiếu TỪ `backup3` `= 3` — ĐỦ quorum,
trả `true`. Sẵn SÀNG bước sang DoViewChange (bài 11).
::::

::::example{#tai-sao-set-khu-trung}
`Set` tự động khử TRÙNG: nếu `backup3` gửi LẠI (mạng lặp gói, hoặc
retry giống bài 8) MỘT `StartViewChange` khác cho CÙNG `viewNumber`,
`phieu.add(3)` KHÔNG làm `size` tăng THÊM — quorum vẫn ĐÚNG `3`,
không bị "tăng ảo" thành `4`. Đây LÀ lý do dùng `Set` (đếm SỐ NGƯỜI
khác nhau) thay VÌ một bộ đếm số nguyên (đếm SỐ LẦN nhận, dễ bị lặp
gói làm SAI).
::::

::::predict{#doan-view-khac-nhau commitOnce}
`backup1` (Ở TRÊN) ĐÃ đạt quorum cho `viewNumber=1`. Nó nhận THÊM một
`StartViewChange` cho `viewNumber=2` (MỘT view HOÀN toàn khác, TỪ
`backup4`). `nhanStartViewChange` LẦN này trả về GÌ?
:::opt{correct}
`false` — `phieuTheoView.get(2)` chưa TỪNG tồn tại, `Set` MỚI được
tạo VỚI tự-vote (`backup1`) `+` phiếu TỪ `backup4` `= 2`, CHƯA đạt
quorum `3` của `viewNumber=2` — HOÀN toàn tách biệt VỚI tiến độ đã
đạt Ở `viewNumber=1`
:::
:::opt
`true` — replica ĐÃ đạt quorum một LẦN (Ở view khác) thì mọi view SAU
đó cũng tự động ĐƯỢC coi LÀ đủ quorum
::why
Trực giác NÀY nhầm "ĐÃ có đủ phiếu" (một trạng thái CHUNG của replica)
với "ĐÃ có đủ phiếu CHO đúng view NÀY" — nhưng `phieuTheoView` LÀ một
`Map` khoá THEO `viewNumber`, mỗi view CÓ `Set` phiếu RIÊNG.

Chỗ lệch: dòng `phieuTheoView.get(td.viewNumber)` LUÔN tra CỨU đúng
view ĐANG xét — `viewNumber=2` chưa hề CÓ entry nào trong `Map`
TRƯỚC đó, nên bắt ĐẦU lại từ `Set` rỗng (RỒI tự-vote). Mỗi cuộc bầu
view MỚI LÀ một cuộc đếm phiếu HOÀN toàn độc lập.
::
:::
::::

::::code{#viet_nhan_start_view_change}
Hoàn thiện `nhanStartViewChange` — gộp phiếu VÀO `Set` theo
`viewNumber`, trả `true` NẾU đủ quorum.

```typescript title=starter
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanStartViewChange(
  replica: Replica, phieuTheoView: Map<number, Set<number>>, td: ThongDiep,
): boolean {
  let phieu = phieuTheoView.get(td.viewNumber);
  if (!phieu) { phieu = new Set<number>([replica.chiSo]); phieuTheoView.set(td.viewNumber, phieu); }
  phieu.add(td.tu);
  ___
}

const backup1 = taoReplica(1, 3);
backup1.status = 'view-change'; backup1.viewNumber = 1;
console.log(nhanStartViewChange(backup1, new Map(), { loai: 'StartViewChange', tu: 2, den: 1, viewNumber: 1 }));
```

```typescript title=solution
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
function nguongQuorum(tongSo: number): number { return Math.floor(tongSo / 2) + 1; }
interface ThongDiep {
  loai: 'Prepare' | 'PrepareOk' | 'Commit' | 'StartViewChange' | 'DoViewChange' | 'StartView' | 'YeuCauDongBo' | 'PhanHoiDongBo';
  tu: number; den: number; viewNumber: number;
  opNumber?: number; bt?: ButToan; commitNumber?: number; log?: LogEntry[];
}
function nhanStartViewChange(
  replica: Replica, phieuTheoView: Map<number, Set<number>>, td: ThongDiep,
): boolean {
  let phieu = phieuTheoView.get(td.viewNumber);
  if (!phieu) { phieu = new Set<number>([replica.chiSo]); phieuTheoView.set(td.viewNumber, phieu); }
  phieu.add(td.tu);
  return phieu.size >= nguongQuorum(replica.tongSo);
}

const backup1 = taoReplica(1, 3);
backup1.status = 'view-change'; backup1.viewNumber = 1;
console.log(nhanStartViewChange(backup1, new Map(), { loai: 'StartViewChange', tu: 2, den: 1, viewNumber: 1 }));
```

```typescript title=test
const b2 = taoReplica(2, 5); // N=5, quorum=3
b2.status = 'view-change'; b2.viewNumber = 1;
const pv = new Map<number, Set<number>>();
const r1 = nhanStartViewChange(b2, pv, { loai: 'StartViewChange', tu: 0, den: 2, viewNumber: 1 });
if (r1 !== false) throw new Error("N=5, quorum=3: chinh minh(tu-vote) + 1 phieu = 2, CHUA du quorum -- phai la false");
const r2 = nhanStartViewChange(b2, pv, { loai: 'StartViewChange', tu: 3, den: 2, viewNumber: 1 });
if (r2 !== true) throw new Error("them 1 phieu tu KHAC nua = 3 = quorum(5) -- phai la true");
const r3 = nhanStartViewChange(b2, pv, { loai: 'StartViewChange', tu: 3, den: 2, viewNumber: 1 });
if (r3 !== true) throw new Error("da du quorum roi, nhan lai phieu TRUNG LAP van phai la true (Set khu trung)");
const pv2 = new Map<number, Set<number>>();
const rKhacView1 = nhanStartViewChange(b2, pv2, { loai: 'StartViewChange', tu: 0, den: 2, viewNumber: 7 });
if (rKhacView1 !== false) throw new Error("view=7 la view MOI, chi moi 2 phieu (tu-vote+1) -- phai la false");
```

:::hints
- kind: attention
  body: "Tra ve phieu.size >= nguongQuorum(replica.tongSo) -- mot dong."
- kind: strategy
  body: "return phieu.size >= nguongQuorum(replica.tongSo);"
- kind: one-line
  body: "return phieu.size >= nguongQuorum(replica.tongSo);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đủ quorum đồng Ý đổi view. Bước tiếp theo: ai LÀ primary mới, VÀ log
của ai được dùng làm bản CHÍNH thức?
::::

::::reflect{#nghi-lai}
`nhanStartViewChange` tái sử dụng ĐÚNG khuôn "đếm phiếu VÀO Set theo
khoá" của `nhanPrepareOk` (bài 6) — chỉ đổi KHOÁ từ `opNumber` sang
`viewNumber`. Đây LÀ minh chứng: VSR không CÓ hai cơ chế đồng thuận
riêng biệt cho "commit MỘT bút toán" VÀ "bầu MỘT primary mới" — cả
hai đều LÀ cùng một câu HỎI ("đủ đa số CHƯA?"), chỉ khác đối tượng
được BỎ phiếu.
::::

::::checkpoint{mastery=0.85}
::::
