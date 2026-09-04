---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.do-ty-le-di-chuyen-vnode-loi
title: "Đo tỷ lệ khoá di chuyển: modulo so với vnode"
summary: "demSoKhoaDoiVongLoi đo số khoá đổi lõi khi thêm một lõi trên vòng vnode — ĐÚNG demSoKhoaDoiVong của q11 bài 5, ở quy mô lõi. Với 12 khoá mẫu, thêm lõi thứ tư (3→4, 20 vnode/lõi) chỉ 3/12 (25%) đổi — so với 9/12 (75%) của % soLoi ngây thơ (bài 7) trên CÙNG dữ liệu. Trên 100 khoá: 30/100 (30%) so với 77/100 (77%) — vnode giảm rehash xuống chưa tới một nửa."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.do-ty-le-di-chuyen-vnode-loi]
requires: [db.vong-tron-o-quy-mo-loi]
concepts: [db.do-ty-le-di-chuyen-vnode-loi]
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
Vòng vnode CHO lõi ĐÃ xây (bài TRƯỚC). Đo THẬT xem thêm một lõi TRÊN
vòng đó gây rehash ÍT hơn `% soLoi` (bài 7) bao NHIÊU.
::::

::::explain{#dem-doi-vong-loi}
`demSoKhoaDoiVongLoi` so sánh lõi CHỊU trách nhiệm CHO mỗi khoá GIỮA
hai vòng (TRƯỚC VÀ sau khi thêm lõi) — ĐÚNG `demSoKhoaDoiVong` (q11
bài 5), CHỈ đổi tên:

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

function demSoKhoaDoiVongLoi(cacKhoa: string[], vongTruoc: DiemLoi[], vongSau: DiemLoi[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timLoiChiuTrachNhiem(vongTruoc, khoa) !== timLoiChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const vongTruoc = xayVongVnodeLoi(3, 20);
const vongSau = xayVongVnodeLoi(4, 20);
const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiVongLoi(cacKhoa, vongTruoc, vongSau));
```

```text title=readonly
3
```

`3` TRÊN `12` khoá — `25%` — đổi lõi khi THÊM lõi thứ TƯ trên vòng
vnode. SO VỚI `% soLoi` (bài 7) TRÊN CÙNG bộ khoá: `9` trên `12`
(`75%`) — vòng vnode giảm rehash XUỐNG còn MỘT phần ba.
::::

::::example{#tren-100-khoa}
CÙNG phép SO sánh, TRÊN `100` khoá:

```typescript title=readonly
const cacKhoa100: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa100.push("nguoidung" + i);
console.log(demSoKhoaDoiVongLoi(cacKhoa100, xayVongVnodeLoi(3, 20), xayVongVnodeLoi(4, 20)));
```

```text title=readonly
30
```

`30` trên `100` (`30%`) VỚI vnode, SO với `77` trên `100` (`77%`) VỚI
`% soLoi` (bài 7) — VẪN chưa tới MỘT nửa. Kết quả NHẤT quán Ở CẢ hai
cỡ dữ liệu (`12` VÀ `100` khoá): vnode LUÔN giảm rehash đáng KỂ, dù
KHÔNG xoá SẠCH nó (thêm lõi VẪN luôn kéo THEO một PHẦN khoá di
chuyển — không CÓ cách nào tránh HẲN điều đó).
::::

::::predict{#doan-nhieu-vnode-hon-nua commitOnce}
Tăng `soVnodeMoiLoi` TỪ `20` lên `200` (mười LẦN nhiều hơn). Tỷ lệ
khoá đổi LÕI khi thêm lõi thứ TƯ CÓ chắc chắn giảm ĐI đáng kể so VỚI
`25%` (`20` vnode) không?

:::opt{correct}
Nhiều khả NĂNG giảm, NHƯNG không PHẢI một quy LUẬT tuyệt đối — nhiều
vnode HƠN giúp phân bố ĐỀU hơn Ề THỐNG kê (q11 bài 8), NHƯNG kết quả
CHÍNH xác vẫn phụ thuộc VỊ trí thật của TỪNG điểm ảo VỚI đúng bộ khoá
đang xét
:::

:::opt
KHÔNG đổi GÌ — tỷ lệ khoá di chuyển khi THÊM một lõi CHỈ phụ thuộc
VÀO số lõi TRƯỚC/sau, không liên QUAN tới số vnode MỖI lõi
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng SỐ khoá bị ảnh hưởng LUÔN nằm
TRONG "đoạn" quanh lõi MỚI (bài 8's Ý tưởng), MỘT quan sát cấu trúc
đúng đắn.

Chỗ lệch: ĐỘ dài của "đoạn" ĐÓ, VÀ CÓ bao nhiêu khoá rơi VÀO nó, phụ
thuộc và CHÍNH VỊ trí các điểm ảo TRÊN vòng — VÀ vị trí ĐÓ đổi khi số
vnode MỖI lõi đổi (nhiều vnode hơn NGHĨA là nhiều điểm CHIA vòng
thành nhiều đoạn NHỎ hơn, đoạn CỦA lõi mới CŨNG nhỏ hơn theo). Số
vnode KHÔNG phải một tham số "trang trí" — nó ẢNH hưởng trực tiếp
tới CẢ độ cân bằng tải (q11 bài 8) LẪN tỷ lệ rehash khi thêm/bớt lõi.
::
:::
::::

::::code{#viet_dem_so_khoa_doi_vong_loi}
Hoàn thiện `demSoKhoaDoiVongLoi` — đếm số khoá CÓ lõi chịu trách
nhiệm TRƯỚC khác lõi chịu trách nhiệm SAU.

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
      vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function demSoKhoaDoiVongLoi(cacKhoa: string[], vongTruoc: DiemLoi[], vongSau: DiemLoi[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (___) {
      dem = dem + 1;
    }
  }
  return dem;
}

const vongTruoc = xayVongVnodeLoi(3, 20);
const vongSau = xayVongVnodeLoi(4, 20);
const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiVongLoi(cacKhoa, vongTruoc, vongSau));
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

function demSoKhoaDoiVongLoi(cacKhoa: string[], vongTruoc: DiemLoi[], vongSau: DiemLoi[]): number {
  let dem = 0;
  for (const khoa of cacKhoa) {
    if (timLoiChiuTrachNhiem(vongTruoc, khoa) !== timLoiChiuTrachNhiem(vongSau, khoa)) {
      dem = dem + 1;
    }
  }
  return dem;
}

const vongTruoc = xayVongVnodeLoi(3, 20);
const vongSau = xayVongVnodeLoi(4, 20);
const cacKhoa: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa.push("nguoidung" + i);
console.log(demSoKhoaDoiVongLoi(cacKhoa, vongTruoc, vongSau));
```

```typescript title=test
const vongTruoc2 = xayVongVnodeLoi(3, 20);
const vongSau2 = xayVongVnodeLoi(4, 20);
const cacKhoa2: string[] = [];
for (let i = 0; i < 12; i++) cacKhoa2.push("nguoidung" + i);
console.log(demSoKhoaDoiVongLoi(cacKhoa2, vongTruoc2, vongSau2));
if (demSoKhoaDoiVongLoi(cacKhoa2, vongTruoc2, vongSau2) !== 3) throw new Error("them loi thu tu (20 vnode) tren 12 khoa phai co dung 3 khoa doi loi");
if (demSoKhoaDoiVongLoi(cacKhoa2, vongTruoc2, vongTruoc2) !== 0) throw new Error("cung mot vong thi khong khoa nao duoc doi");
if (demSoKhoaDoiVongLoi([], vongTruoc2, vongSau2) !== 0) throw new Error("danh sach khoa rong thi dem phai la 0");

const cacKhoa100: string[] = [];
for (let i = 0; i < 100; i++) cacKhoa100.push("nguoidung" + i);
if (demSoKhoaDoiVongLoi(cacKhoa100, xayVongVnodeLoi(3, 20), xayVongVnodeLoi(4, 20)) !== 30) throw new Error("100 khoa phai co dung 30 khoa doi loi");
```

:::hints
- kind: attention
  body: "Dieu kien dem: loi chiu trach nhiem Ở vongTruoc KHAC loi chiu trach nhiem Ở vongSau -- mot dong."
- kind: strategy
  body: "timLoiChiuTrachNhiem(vongTruoc, khoa) !== timLoiChiuTrachNhiem(vongSau, khoa)"
- kind: one-line
  body: "if (timLoiChiuTrachNhiem(vongTruoc, khoa) !== timLoiChiuTrachNhiem(vongSau, khoa)) {"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vnode giảm rehash Ở quy mô lõi ĐÚNG như Ở quy mô máy. Ráp TOÀN bộ q12
thành một hệ shard-per-core hoàn chỉnh trông ra sao?
::::

::::reflect{#nghi-lai}
`25%` (vnode) SO với `75%` (`% soLoi`) — ĐÚNG cùng mức cải THIỆN q11
đã đo Ở quy mô MÁY (`38%` so VỚI `85%`, tỷ lệ tương ĐƯƠNG). Đây LÀ
bằng chứng CHO một Ý tưởng đã ngụ Ý xuyên suốt q12: vấn đề "định
tuyến tất định, chịu được thay đổi SỐ lượng ứng viên" LÀ MỘT vấn đề
DUY nhất, xuất hiện Ở NHIỀU quy mô khác nhau (máy TRONG một cụm, lõi
TRONG một máy) — VÀ đúng MỘT lời giải (vòng hash + vnode) giải quyết
được CẢ hai, không cần phát MINH lại điều gì MỚI. Mọi mảnh — kho
riêng (bài 3), routing (bài 4, 8), tin NHẮN (bài 6), đo rehash (bài
NÀY) — đã sẵn SÀNG cho một BOSS ráp trọn.
::::

::::checkpoint{mastery=0.85}
::::
</content>
