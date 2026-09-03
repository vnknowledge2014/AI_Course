---
id: co-so-du-lieu.vong-tron-quyen-luc.vnode-nhieu-diem-ao-moi-node
title: "Vnode — nhiều điểm ảo mỗi node"
summary: "xayVongVnode ánh xạ MỖI node vật lý thành N điểm ảo (vnode) rải khắp vòng — bam(`${ten}#${i}`) cho i chạy từ 0 tới N-1. Với 20 vnode/node (80 điểm thay vì 4), phân bố trên cùng 100 khoá cải thiện từ {alpha:35,beta:16,gamma:13,delta:36} (bài 7, chênh 2.77 lần) xuống {alpha:29,beta:25,gamma:15,delta:31} (chênh 2.07 lần) — tốt hơn, KHÔNG hoàn hảo: vnode giảm nhưng không xoá sạch may rủi của vị trí ngẫu nhiên."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.vnode-nhieu-diem-ao]
requires: [db.van-de-phan-bo-khong-deu]
concepts: [db.vnode-nhieu-diem-ao]
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
Mỗi node MỘT điểm trên vòng khiến phân bố LỆCH gấp `2.77` lần (bài
trước) — dù `bam` (bài 6) LÀ một hàm băm tốt. Cho mỗi node NHIỀU
điểm hơn — trông ra sao?
::::

::::explain{#xay-vong-vnode}
`xayVongVnode` ánh XẠ MỖI tên node thành `soVnodeMoiNode` điểm ảo —
`bam(\`${ten}#${i}\`)` VỚI `i` chạy từ `0` tới `soVnodeMoiNode-1`,
TẤT cả cùng mang TÊN node gốc:

```typescript title=readonly
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function demPhanBo(vong: DiemNode[], cacKhoa: string[]): Record<string, number> {
  const dem: Record<string, number> = {};
  for (const khoa of cacKhoa) {
    const ten = timNodeChiuTrachNhiem(vong, khoa);
    dem[ten] = (dem[ten] ?? 0) + 1;
  }
  return dem;
}

const vongVnode = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(vongVnode.length);
const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
console.log(demPhanBo(vongVnode, cacKhoa));
```

```text title=readonly
80
{ alpha: 29, beta: 25, gamma: 15, delta: 31 }
```

`4` node × `20` vnode = `80` điểm trên vòng, thay VÌ `4`. TRÊN CÙNG
`100` khoá đã dùng Ở bài 7: `gamma` giờ nhận `15` khoá (trước LÀ
`13`), `alpha`/`delta` giảm CÒN `29`/`31` (trước LÀ `35`/`36`) —
khoảng CHÊNH lệch (`max/min`) giảm TỪ `36/13≈2.77` LẦN xuống còn
`31/15≈2.07` lần.
::::

::::example{#khong-hoan-hao}
`2.07` lần VẪN chưa phải "hoàn toàn ĐỀU" (`1` lần) — vnode GIẢM may
rủi của việc "CHỈ một điểm ngẫu nhiên MỖI node", nhưng KHÔNG xoá
sạch NÓ: `80` điểm vẫn LÀ một con số HỮU hạn, VÀ vị trí của chúng
vẫn phụ THUỘC vào `bam` áp DỤNG lên `"gamma#0"`, `"gamma#1"`, v.v —
NGẪU nhiên (theo nghĩa "khó đoán TRƯỚC", không phải thực sự ngẫu
nhiên) NHƯNG vẫn hữu HẠN. Hệ THẬT (ScyllaDB, Cassandra) thường dùng
HÀNG trăm vnode CHO mỗi node vật lý CHÍNH vì lý DO này — CÀNG nhiều
điểm đại diện, phân bố CÀNG hội tụ VỀ đều, nhưng KHÔNG có một con số
"đủ" tuyệt đối, chỉ CÓ "đủ tốt cho quy MÔ dữ liệu đang xử lý".
::::

::::predict{#doan-nhieu-vnode-hon commitOnce}
Nếu tăng `soVnodeMoiNode` từ `20` LÊN `2000` (mỗi node CÓ `2000`
điểm ảo, thay VÌ `20`), phân bố tải TRÊN cùng `100` khoá CÓ CHẮC
chắn đều HƠN không?

:::opt{correct}
Nhiều khả NĂNG đều hơn, NHƯNG không chắc CHẮN tuyệt đối — càng nhiều
điểm CÀNG hội tụ về đều VỀ mặt THỐNG kê, nhưng với đúng `100` khoá
CỤ thể, kết quả VẪN phụ thuộc và VỊ trí thật của TỪNG điểm
:::

:::opt
CÓ, chắc chắn tuyệt đối — số vnode CÀNG lớn thì sai SỐ càng tiến VỀ
`0`, đây LÀ một định lý TOÁN học không CÓ ngoại lệ
::why
Gần đúng ở việc bạn nắm ĐÚNG xu hướng THỐNG kê (nhiều điểm hơn →
phân bố kỳ VỌNG đều hơn) — xu hướng NÀY LÀ thật, VÀ LÀ lý do hệ
thống thật dùng NHIỀU vnode.

Chỗ lệch: "xu hướng THỐNG kê" khác VỚI "chắc chắn tuyệt đối cho MỘT
bộ khoá cụ thể". VỚI đúng `100` khoá CỐ định (không phải MỘT tập vô
hạn), một VỊ trí vnode cụ thể VẪN có thể rơi LỆCH — tăng số vnode
làm khả NĂNG lệch nghiêm trọng giảm ĐI rất nhiều, nhưng KHÔNG có gì
đảm bảo tuyệt đối `100%` cho MỘT bộ dữ liệu hữu hạn cụ thể.
::
:::
::::

::::code{#viet_xay_vong_vnode}
Hoàn thiện `xayVongVnode` — VỚI mỗi node, tạo `soVnodeMoiNode` điểm
ảo, đặt TÊN vnode LÀ `"tenNode#i"`.

```typescript title=starter
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      ___
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

const vongVnode = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(vongVnode.length);
```

```typescript title=solution
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

interface DiemNode { viTri: number; ten: string; }

function timNodeChiuTrachNhiem(vong: DiemNode[], khoa: string): string {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.ten;
  }
  return vong[0]!.ten;
}

function xayVongVnode(tenCacNode: string[], soVnodeMoiNode: number): DiemNode[] {
  const vong: DiemNode[] = [];
  for (const ten of tenCacNode) {
    for (let i = 0; i < soVnodeMoiNode; i++) {
      vong.push({ viTri: bam(`${ten}#${i}`), ten });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

const vongVnode = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(vongVnode.length);
```

```typescript title=test
function demPhanBo(vong: DiemNode[], cacKhoa: string[]): Record<string, number> {
  const dem: Record<string, number> = {};
  for (const khoa of cacKhoa) {
    const ten = timNodeChiuTrachNhiem(vong, khoa);
    dem[ten] = (dem[ten] ?? 0) + 1;
  }
  return dem;
}

const vongVnode2 = xayVongVnode(["alpha", "beta", "gamma", "delta"], 20);
console.log(vongVnode2.length);
if (vongVnode2.length !== 80) throw new Error("4 node nhan 20 vnode moi node phai co dung 80 diem tren vong");

const chiAlpha = vongVnode2.filter((d) => d.ten === "alpha");
if (chiAlpha.length !== 20) throw new Error("alpha phai co dung 20 vnode");

const cacKhoa: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa.push("nguoidung" + i);
const ketQua = demPhanBo(vongVnode2, cacKhoa);
if (ketQua["alpha"] !== 29) throw new Error("voi vnode, alpha phai nhan dung 29 khoa");
if (ketQua["beta"] !== 25) throw new Error("voi vnode, beta phai nhan dung 25 khoa");
if (ketQua["gamma"] !== 15) throw new Error("voi vnode, gamma phai nhan dung 15 khoa");
if (ketQua["delta"] !== 31) throw new Error("voi vnode, delta phai nhan dung 31 khoa");
```

:::hints
- kind: attention
  body: "Day vao vong mot diem moi: viTri la bam cua chuoi ten+'#'+i, ten la ten node -- mot dong."
- kind: strategy
  body: "vong.push({ viTri: bam(`${ten}#${i}`), ten });"
- kind: one-line
  body: "vong.push({ viTri: bam(`${ten}#${i}`), ten });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "80"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hàm băm rải đều, vòng consistent hashing, vnode cân bằng tải — mọi
mảnh đã sẵn sàng. Ráp thành một hệ định tuyến hoàn chỉnh trông ra
sao?
::::

::::reflect{#nghi-lai}
Vnode LÀ một ý tưởng đơn GIẢN đến bất ngờ: KHÔNG cần thay đổi GÌ Ở
`timNodeChiuTrachNhiem` (bài 4) — nó vẫn CHỈ tìm "node đầu tiên gặp
được TRÊN vòng". Cái THAY đổi duy nhất LÀ cách XÂY vòng: thay VÌ một
điểm cho MỖI node, giờ LÀ nhiều điểm — VÀ vì `timNodeChiuTrachNhiem`
CHỈ quan tâm tới TRƯỜNG `ten` của điểm tìm được (KHÔNG quan tâm đó
LÀ vnode thứ mấy), mọi LOGIC khác Ở các bài TRƯỚC (bài 2, 5, 7) vẫn
dùng ĐƯỢC nguyên vẹn TRÊN vòng vnode NÀY.
::::

::::checkpoint{mastery=0.85}
::::
