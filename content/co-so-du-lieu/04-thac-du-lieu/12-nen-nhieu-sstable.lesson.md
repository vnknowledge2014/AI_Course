---
id: co-so-du-lieu.thac-du-lieu.nen-nhieu-sstable
title: Nén nhiều SSTable — compaction
summary: "gopSSTable nhận NHIỀU SSTable (thứ tự CŨ tới MỚI), gộp thành MỘT — mỗi khoá chỉ giữ giá trị MỚI nhất, rồi loại HẲN mọi bia mộ. Đây LÀ 'gộp toàn bộ' (major compaction): không còn SSTable cũ hơn nào cần bia mộ che chắn nữa."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.compaction-merge]
requires: [db.tombstone-delete]
concepts: [db.compaction-merge]
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
Bia mộ (bài trước) tích luỹ MÃI trên nhiều SSTable — chiếm chỗ vô
ích. Có cách nào dọn SẠCH chúng, gộp nhiều SSTable thành MỘT?
::::

::::explain{#gop-sstable}
`gopSSTable` nhận NHIỀU SSTable (thứ tự CŨ → mới), gộp chúng thành
MỘT — mỗi khoá chỉ giữ giá trị MỚI nhất (SSTable sau ghi ĐÈ SSTable
trước), rồi loại HẲN mọi bia mộ:

```typescript title=readonly
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function gopSSTable(danhSach: SSTable[]): SSTable {
  const banDo = new Map<string, string | null>();
  for (const bang of danhSach) {
    for (let i = 0; i < bang.khoa.length; i++) {
      const k = bang.khoa[i];
      const v = bang.giaTri[i];
      if (k !== undefined) banDo.set(k, v === undefined ? null : v);
    }
  }
  const khoaSapXep = [...banDo.keys()].sort();
  const khoa: string[] = [];
  const giaTri: (string | null)[] = [];
  for (const k of khoaSapXep) {
    const v = banDo.get(k);
    if (v !== null && v !== undefined) { khoa.push(k); giaTri.push(v); }
  }
  return { khoa, giaTri };
}

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const ss3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };

const gop = gopSSTable([ss1, ss2, ss3]);
console.log(gop.khoa.join(","), gop.giaTri.join(","));
```

```text title=readonly
cam,dua,le,tao 5500,3000,4000,12000
```

`ss1` (cũ nhất) có `buoi=8000, cam=5000, tao=12000`. `ss2` ghi ĐÈ
`cam=5500`, thêm `dua=3000`. `ss3` (mới nhất) xoá `buoi` (bia mộ
`null`) VÀ thêm `le=4000`. Gộp CẢ ba: `Map` giữ giá trị MỚI nhất
cho mỗi khoá (duyệt THEO thứ tự cũ→mới, giá trị sau LUÔN ghi đè
giá trị trước) — `buoi` có giá trị CUỐI cùng LÀ `null` (bia mộ),
NÊN bị lọc BỎ hoàn toàn khỏi kết quả.
::::

::::example{#khoa-chi-o-mot-sstable}
Một khoá chỉ xuất hiện Ở ĐÚNG một SSTable — vẫn xuất hiện bình
thường trong kết quả gộp:

```typescript title=readonly
console.log(gop.khoa.includes("tao"), gop.khoa.includes("buoi"));
```

```text title=readonly
true false
```

`"tao"` chỉ CÓ ở `ss1`, không hề bị SSTable nào SAU ghi đè hay xoá
— vẫn có MẶT trong kết quả gộp. `"buoi"` — dù XUẤT hiện Ở `ss1` với
giá trị THẬT — bị `ss3` xoá (bia mộ) LÀ bản GHI cuối cùng cho khoá
đó, nên biến MẤT hoàn toàn khỏi kết quả.
::::

::::predict{#doan-so-khoa-sau-gop commitOnce}
Ba SSTable (`ss1, ss2, ss3` như trên) gộp LẠI. Byte đếm số khoá
CÒN lại:

```typescript
console.log(gop.khoa.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`4`
:::

:::opt
`5` — vì có `5` khoá KHÁC nhau TỔNG cộng xuất hiện Ở đâu đó trong
ba SSTable (`buoi, cam, tao, dua, le`), gộp LẠI phải giữ ĐỦ cả năm
::why
Gần đúng ở việc bạn đếm ĐÚNG số khoá KHÁC nhau xuất hiện — MỘT
bước đếm chính xác.

Chỗ lệch: `buoi` XUẤT hiện, NHƯNG bản ghi CUỐI cùng của nó (Ở
`ss3`, SSTable mới NHẤT) LÀ một bia mộ (`giaTri=null`) — vòng lọc
cuối (`if (v !== null && v !== undefined)`) loại HẲN nó khỏi kết
quả. Chỉ `4` khoá THẬT sự còn giá trị SỐNG: `cam, dua, le, tao`.
::
:::

:::opt
`3` — vì gộp TOÀN bộ chỉ giữ khoá xuất hiện Ở SSTable MỚI nhất
(`ss3`), các SSTable cũ hơn coi NHƯ đã lỗi thời
::why
Gần đúng ở việc bạn nghĩ TỚI ưu tiên "MỚI nhất thắng" — MỘT
nguyên tắc ĐÚNG của LSM, chỉ áp dụng SAI phạm vi.

Chỗ lệch: "mới nhất thắng" áp dụng CHO TỪNG khoá riêng LẺ (giá trị
nào của khoá NÀO là mới nhất), KHÔNG có nghĩa LÀ chỉ giữ khoá xuất
hiện Ở SSTable mới nhất — `"tao"` chỉ CÓ ở `ss1` (cũ nhất) vẫn hoàn
toàn hợp lệ VÀ còn nguyên trong kết quả, vì KHÔNG SSTable nào sau
đó ghi đè hay xoá nó.
::
:::
::::

::::code{#viet_gop_sstable}
Hoàn thiện `gopSSTable` — sau khi đã tính giá trị MỚI nhất cho mỗi
khoá, lọc bỏ bia mộ TRƯỚC khi đưa vào kết quả.

```typescript title=starter
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function gopSSTable(danhSach: SSTable[]): SSTable {
  const banDo = new Map<string, string | null>();
  for (const bang of danhSach) {
    for (let i = 0; i < bang.khoa.length; i++) {
      const k = bang.khoa[i];
      const v = bang.giaTri[i];
      if (k !== undefined) banDo.set(k, v === undefined ? null : v);
    }
  }
  const khoaSapXep = [...banDo.keys()].sort();
  const khoa: string[] = [];
  const giaTri: (string | null)[] = [];
  for (const k of khoaSapXep) {
    const v = banDo.get(k);
    ___
  }
  return { khoa, giaTri };
}

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const ss3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };

const gop = gopSSTable([ss1, ss2, ss3]);
console.log(gop.khoa.join(","), gop.giaTri.join(","));
```

```typescript title=solution
interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function gopSSTable(danhSach: SSTable[]): SSTable {
  const banDo = new Map<string, string | null>();
  for (const bang of danhSach) {
    for (let i = 0; i < bang.khoa.length; i++) {
      const k = bang.khoa[i];
      const v = bang.giaTri[i];
      if (k !== undefined) banDo.set(k, v === undefined ? null : v);
    }
  }
  const khoaSapXep = [...banDo.keys()].sort();
  const khoa: string[] = [];
  const giaTri: (string | null)[] = [];
  for (const k of khoaSapXep) {
    const v = banDo.get(k);
    if (v !== null && v !== undefined) { khoa.push(k); giaTri.push(v); }
  }
  return { khoa, giaTri };
}

const ss1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const ss2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const ss3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };

const gop = gopSSTable([ss1, ss2, ss3]);
console.log(gop.khoa.join(","), gop.giaTri.join(","));
```

```typescript title=test
const t1: SSTable = { khoa: ["buoi", "cam", "tao"], giaTri: ["8000", "5000", "12000"] };
const t2: SSTable = { khoa: ["cam", "dua"], giaTri: ["5500", "3000"] };
const t3: SSTable = { khoa: ["buoi", "le"], giaTri: [null, "4000"] };
const gop2 = gopSSTable([t1, t2, t3]);
if (gop2.khoa.join(",") !== "cam,dua,le,tao") throw new Error("khoa phai la cam,dua,le,tao theo thu tu ABC, buoi phai bien mat");
if (gop2.giaTri.join(",") !== "5500,3000,4000,12000") throw new Error("cam phai mang gia tri MOI nhat (5500 tu ss2), khong phai gia tri cu (5000)");
if (gop2.khoa.includes("buoi")) throw new Error("buoi bi xoa o SSTable moi nhat -- phai bien mat hoan toan sau khi gop TOAN BO");

const chiMotBang: SSTable = { khoa: ["an"], giaTri: ["1"] };
const gopMot = gopSSTable([chiMotBang]);
if (gopMot.khoa.join(",") !== "an") throw new Error("gop mot SSTable duy nhat phai giu nguyen noi dung");
```

:::hints
- kind: attention
  body: "Blank nam trong vong for cuoi cung. Neu v khac null va khac undefined thi day k vao khoa, day v vao giaTri."
- kind: strategy
  body: "if (v !== null && v !== undefined) { khoa.push(k); giaTri.push(v); }"
- kind: one-line
  body: "if (v !== null && v !== undefined) { khoa.push(k); giaTri.push(v); }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "cam,dua,le,tao 5500,3000,4000,12000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nén nhiều SSTable thành một — bia mộ biến MẤT, dữ liệu gọn lại.
Nhưng nén TỐN chi phí — ghi LẠI toàn bộ dữ liệu THÊM một lần. Chi
phí đó lớn CỠ nào?
::::

::::reflect{#nghi-lai}
`gopSSTable` giữ giá trị MỚI nhất cho MỖI khoá (duyệt theo thứ tự
cũ→mới, SSTable sau LUÔN ghi đè), rồi lọc bỏ HẲN mọi bia mộ — ĐÂY
là "gộp toàn bộ" (major compaction): KHÔNG còn SSTable nào cũ hơn
cần bia mộ che chắn nữa. Nhưng gộp nghĩa LÀ ghi lại TOÀN bộ dữ liệu
sống MỘT lần nữa — mỗi byte dữ liệu có thể bị ghi xuống đĩa NHIỀU
lần hơn kích thước THẬT của nó. Chi phí đó đo được KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
