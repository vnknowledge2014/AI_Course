---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.vong-tron-o-quy-mo-loi
title: "Vòng tròn, ở quy mô lõi"
summary: "xayVongVnodeLoi ánh xạ MỖI lõi thành soVnodeMoiLoi điểm ảo trên vòng bam [0,999] — ĐÚNG kỹ thuật xayVongVnode của q11 bài 8, chỉ đổi 'node' thành 'lõi'. timLoiChiuTrachNhiem (đã cho sẵn) tìm điểm ảo đầu tiên gặp được theo chiều tăng dần, quấn về điểm đầu tiên nếu không tìm thấy — ĐÚNG timNodeChiuTrachNhiem của q11 bài 4, không đổi gì ngoài tên."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.vong-tron-o-quy-mo-loi]
requires: [db.them-loi-tham-hoa-rehash]
concepts: [db.vong-tron-o-quy-mo-loi]
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
q11 giải quyết ĐÚNG vấn đề "thêm MÁY gây rehash" bằng vòng tròn hash
VÀ vnode (bài 3-8). Kỹ thuật ĐÓ áp dụng LẠI Ở quy mô lõi trông ra
SAO?
::::

::::explain{#xay-vong-vnode-loi}
`xayVongVnodeLoi` ánh XẠ MỖI lõi thành `soVnodeMoiLoi` điểm ẢO trên
vòng `[0,999]` — ĐÚNG `xayVongVnode` (q11 bài 8), CHỈ đổi "node"
thành "lõi":

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

interface DiemLoi { viTri: number; chiSo: number; }

function timLoiChiuTrachNhiem(vong: DiemLoi[], khoa: string): number {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.chiSo;
  }
  return vong[0]!.chiSo;
}

function xayVongVnodeLoi(soLoi: number, soVnodeMoiLoi: number): DiemLoi[] {
  const vong: DiemLoi[] = [];
  for (let loi = 0; loi < soLoi; loi++) {
    for (let i = 0; i < soVnodeMoiLoi; i++) {
      vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

const vong = xayVongVnodeLoi(4, 20);
console.log(vong.length);
console.log(timLoiChiuTrachNhiem(vong, "nguoidung0"));
```

```text title=readonly
80
1
```

`4` lõi × `20` vnode MỖI lõi = `80` điểm TRÊN vòng. `timLoiChiuTrachNhiem`
KHÔNG đổi gì cả SO với `timNodeChiuTrachNhiem` (q11 bài 4) — nó CHỈ
quan tâm TRƯỜNG `chiSo` của điểm tìm ĐƯỢC, không quan tâm đó LÀ vnode
thứ MẤY hay thuộc LÕI nào theo cách NÀO.
::::

::::example{#tach-biet-khoi-boi-cong-thuc-cu}
"Điểm ảo" TRÊN vòng ĐỘC lập hoàn TOÀN VỚI công thức `% soLoi` (bài
1-7) — `chonLoiChoKhoa("nguoidung0", 4)` VÀ `timLoiChiuTrachNhiem(vong,
"nguoidung0")` LÀ hai CÁCH tính hoàn toàn KHÁC nhau, KHÔNG hứa cho
CÙNG một kết quả:

```typescript title=readonly
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
console.log(chonLoiChoKhoa("nguoidung0", 4));
console.log(timLoiChiuTrachNhiem(vong, "nguoidung0"));
```

```text title=readonly
2
1
```

`% soLoi` (bài 4) routing `"nguoidung0"` VỀ lõi `2`. Vòng VNODE
(bài NÀY) routing CÙNG khoá ĐÓ về lõi `1` — HAI hệ thống định TUYẾN
KHÁC nhau, KHÔNG tương thích VỚI nhau. Một hệ THẬT chỉ dùng ĐÚNG một
trong hai — CHUYỂN từ hệ này sang hệ KIA nghĩa LÀ di chuyển LẠI toàn
bộ dữ liệu MỘT lần, giống hệt việc THÊM một node/lõi.
::::

::::predict{#doan-so-diem-moi-loi commitOnce}
`xayVongVnodeLoi(6, 15)` — SÁU lõi, MỖI lõi `15` vnode. Trong mảng
kết QUẢ, có bao NHIÊU điểm mang `chiSo === 3` (LÕI thứ tư, đếm TỪ
`0`)?

:::opt{correct}
`15` — ĐÚNG bằng `soVnodeMoiLoi`, VÌ mỗi lõi (bất kể chỉ SỐ nào) đều
được gán đúng số vnode NHƯ nhau
:::

:::opt
Ít hơn `15` — lõi thứ TƯ (chỉ số `3`) không phải lõi ĐẦU tiên hay
CUỐI cùng, CÓ thể bị "che khuất" MỘT phần bởi các lõi lân CẬN trên
vòng
::why
Gần đúng ở việc bạn nghĩ TỚI vị trí "Ở giữa" NHƯ một yếu tố BẤT lợi —
MỘT trực giác dễ hiểu NẾU liên tưởng tới việc CHIA sẻ không gian VẬT
lý.

Chỗ lệch: `xayVongVnodeLoi` tạo ĐÚNG `soVnodeMoiLoi` điểm CHO MỖI
lõi Ở vòng `for` NGOÀI (`for (let loi = 0; loi < soLoi; loi++)`) —
không PHÂN biệt lõi ĐÓ đứng "giữa" hay Ở "rìa" khi sắp XẾP theo VỊ
trí. SỐ lượng điểm ẢO của MỖI lõi LÀ một hằng SỐ cố định
(`soVnodeMoiLoi`), hoàn TOÀN không phụ thuộc VỊ trí các điểm đó rơi
Ở ĐÂU trên vòng SAU khi sắp xếp.
::
:::
::::

::::code{#viet_xay_vong_vnode_loi}
Hoàn thiện `xayVongVnodeLoi` — VỚI mỗi lõi, tạo `soVnodeMoiLoi` điểm
ảo, đặt TÊN vnode LÀ `` `loi${loi}#${i}` ``.

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

interface DiemLoi { viTri: number; chiSo: number; }

function timLoiChiuTrachNhiem(vong: DiemLoi[], khoa: string): number {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.chiSo;
  }
  return vong[0]!.chiSo;
}

function xayVongVnodeLoi(soLoi: number, soVnodeMoiLoi: number): DiemLoi[] {
  const vong: DiemLoi[] = [];
  for (let loi = 0; loi < soLoi; loi++) {
    for (let i = 0; i < soVnodeMoiLoi; i++) {
      ___
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

const vong = xayVongVnodeLoi(4, 20);
console.log(vong.length);
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

interface DiemLoi { viTri: number; chiSo: number; }

function timLoiChiuTrachNhiem(vong: DiemLoi[], khoa: string): number {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.chiSo;
  }
  return vong[0]!.chiSo;
}

function xayVongVnodeLoi(soLoi: number, soVnodeMoiLoi: number): DiemLoi[] {
  const vong: DiemLoi[] = [];
  for (let loi = 0; loi < soLoi; loi++) {
    for (let i = 0; i < soVnodeMoiLoi; i++) {
      vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

const vong = xayVongVnodeLoi(4, 20);
console.log(vong.length);
```

```typescript title=test
const vong2 = xayVongVnodeLoi(4, 20);
console.log(vong2.length);
if (vong2.length !== 80) throw new Error("4 loi, 20 vnode moi loi phai co dung 80 diem tren vong");

const chiLoi0 = vong2.filter((d) => d.chiSo === 0);
if (chiLoi0.length !== 20) throw new Error("loi 0 phai co dung 20 vnode");
const chiLoi3 = vong2.filter((d) => d.chiSo === 3);
if (chiLoi3.length !== 20) throw new Error("loi 3 phai co dung 20 vnode");

if (timLoiChiuTrachNhiem(vong2, "nguoidung0") !== 1) throw new Error("nguoidung0 phai thuoc ve loi 1 tren vong vnode nay");

const vong6 = xayVongVnodeLoi(6, 15);
if (vong6.length !== 90) throw new Error("6 loi, 15 vnode moi loi phai co dung 90 diem");
```

:::hints
- kind: attention
  body: "Day vao vong mot diem moi: viTri la bam cua chuoi loi+chiSo+'#'+i, chiSo la chi so loi -- mot dong."
- kind: strategy
  body: "vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });"
- kind: one-line
  body: "vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });"
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
Vòng vnode CHO lõi ĐÃ xây được. Nó CÓ THẬT sự giảm được rehash SO với
`% soLoi` không — đo THẬT xem sao?
::::

::::reflect{#nghi-lai}
`xayVongVnodeLoi`/`timLoiChiuTrachNhiem` KHÔNG viết thêm Ý tưởng MỚI
nào — CHÚNG LÀ đúng `xayVongVnode`/`timNodeChiuTrachNhiem` (q11 bài
4, 8), đổi ĐÚNG một chữ ("node" → "lõi"). ĐIỀU đó tự nó LÀ một bài
học: kỹ thuật "đặt lên vòng, dùng NHIỀU điểm ảo" không hề GẮN với
khái niệm "MÁY" — nó giải quyết đúng bài TOÁN "routing tất định,
CHỊU được thay đổi SỐ lượng ứng viên", bất kể ỨNG viên đó LÀ máy vật
lý HAY lõi CPU. Đã xây được — bây GIỜ đo THẬT xem nó giảm rehash BAO
nhiêu SO với `% soLoi`.
::::

::::checkpoint{mastery=0.85}
::::
</content>
