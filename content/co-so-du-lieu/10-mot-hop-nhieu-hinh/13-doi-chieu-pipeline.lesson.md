---
id: co-so-du-lieu.mot-hop-nhieu-hinh.doi-chieu-pipeline
title: "Ráp một pipeline nhỏ đối chiếu"
summary: "chonVaNhom ghép chonTheoTuoi (bài 2, lọc) VỚI nhomVaDem (bài 10, đếm theo nhóm) — đúng thứ tự lọc TRƯỚC nhóm SAU (pushdown, q08 bài 7). SELECT thanh_pho, count() AS so_luong FROM nguoi WHERE tuoi > 18 GROUP BY thanh_pho (SurrealDB thật) và chonVaNhom (TypeScript) trên CÙNG dữ liệu cho ĐÚNG cùng kết quả — một pipeline hai bước, hai bộ máy, một kết quả."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.doi-chieu-pipeline]
requires: [db.hanh-vi-khop-boolean]
concepts: [db.doi-chieu-pipeline]
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
Lọc (bài 2) khớp. Nhóm-đếm (bài 10) khớp. GHÉP cả hai lại thành một
pipeline — lọc RỒI nhóm — hai bộ máy có VẪN đồng thuận không?
::::

::::explain{#pipeline-that}
Bốn người, hai thành phố. Lọc TUỔI lớn hơn `18`, RỒI đếm theo thành
phố trên PHẦN còn lại:

```text title=readonly
SELECT thanh_pho, count() AS so_luong FROM nguoi
  WHERE tuoi > 18 GROUP BY thanh_pho;
```

```text title=readonly
[{ thanh_pho: "HN", so_luong: 1 }, { thanh_pho: "SG", so_luong: 2 }]
```

`An` (`15`, HN) bị LOẠI ngay TỪ bước lọc — HN chỉ CÒN `Binh` (`1`
người). `SG` giữ nguyên cả `Chi` VÀ `Dung` (`2` người). Thứ TỰ CHẠY
LÀ lọc TRƯỚC, nhóm SAU — đúng nguyên tắc pushdown (q08 bài 7): áp
điều kiện WHERE CÀNG sớm càng TỐT, trước khi làm bất KỲ việc tốn
kém hơn (Ở đây LÀ đếm theo nhóm).
::::

::::example{#ghep-hai-ham-da-viet}
Bài 2 ĐÃ viết `chonTheoTuoi` (lọc). Bài 10 ĐÃ viết `nhomVaDem`
(đếm). Ghép CHÚNG lại — lọc TRƯỚC, nhóm SAU — ĐÚNG thứ tự SurrealDB
thật chạy:

```typescript title=readonly
function chonVaNhom(bang: Nguoi[], nguongDuoi: number): NhomDem[] {
  const daLoc = chonTheoTuoi(bang, nguongDuoi);
  return nhomVaDem(daLoc);
}
```

KHÔNG hàm MỚI nào cần viết TỪ đầu — `chonVaNhom` chỉ LÀ việc gọi
ĐÚNG thứ tự hai hàm ĐÃ có sẵn TỪ hai bài trước.
::::

::::predict{#doan-doi-thu-tu commitOnce}
Nếu ĐỔI thứ tự — nhóm TRƯỚC, lọc SAU (`nhomVaDem` chạy trên TOÀN
bộ bảng, RỒI mới lọc kết quả NHÓM theo một điều kiện VỀ `soLuong`,
ví dụ "chỉ giữ nhóm CÓ từ 2 người trở lên") — kết quả CÓ giống hệt
`chonVaNhom` (lọc TRƯỚC theo `tuoi`, nhóm SAU) không?

:::opt{correct}
KHÔNG — hai phép LỌC khác nhau hoàn toàn (lọc THEO `tuoi` của TỪNG
người TRƯỚC khi nhóm, so với lọc THEO `soLuong` của CẢ nhóm sau khi
đếm) — không CÓ lý do gì để chúng trùng KẾT quả
:::

:::opt
CÓ — vì `AND`/lọc LÀ phép giao HOÁN, đổi thứ tự các bước xử LÝ
không ảnh hưởng tới kết QUẢ cuối cùng, giống pushdown (q08 bài 7)
đã dạy
::why
Gần đúng ở việc bạn nhớ TỚI pushdown (q08 bài 7) — nơi đổi thứ tự
CÁC bước ĐÚNG là không ảnh hưởng KẾT quả, MIỄN là bước đó LỌC trên
CÙNG một điều kiện.

Chỗ lệch: câu hỏi Ở đây KHÔNG phải "đổi thứ tự CÙNG một phép lọc"
(như pushdown) — nó LÀ hai phép lọc HOÀN toàn khác NHAU: lọc THEO
`tuoi` của từng NGƯỜI (trước khi nhóm) so VỚI lọc THEO `soLuong` của
CẢ nhóm (sau khi đếm). "Chỉ giữ nhóm CÓ ≥2 người" VÀ "chỉ giữ người
CÓ tuổi > 18" LÀ hai tiêu chí hoàn toàn ĐỘC lập — không CÓ căn cứ
NÀO để tin CHÚNG cho cùng kết quả.
::
:::
::::

::::code{#viet_chon_va_nhom}
Hoàn thiện `chonVaNhom` — gọi `chonTheoTuoi` TRƯỚC (lọc), RỒI
`nhomVaDem` trên kết QUẢ đã lọc (nhóm).

```typescript title=starter
interface Nguoi {
  ten: string;
  tuoi: number;
  thanhPho: string;
}

interface NhomDem {
  thanhPho: string;
  soLuong: number;
}

function chonTheoTuoi(bang: Nguoi[], nguongDuoi: number): Nguoi[] {
  const ketQua: Nguoi[] = [];
  for (const hang of bang) {
    if (hang.tuoi > nguongDuoi) {
      ketQua.push(hang);
    }
  }
  return ketQua;
}

function nhomVaDem(bang: Nguoi[]): NhomDem[] {
  const dem = new Map<string, number>();
  for (const hang of bang) {
    const hienTai = dem.get(hang.thanhPho) ?? 0;
    dem.set(hang.thanhPho, hienTai + 1);
  }
  const ketQua: NhomDem[] = [];
  for (const [thanhPho, soLuong] of dem) {
    ketQua.push({ thanhPho, soLuong });
  }
  return ketQua;
}

function chonVaNhom(bang: Nguoi[], nguongDuoi: number): NhomDem[] {
  ___
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15, thanhPho: "HN" },
  { ten: "Binh", tuoi: 20, thanhPho: "HN" },
  { ten: "Chi", tuoi: 25, thanhPho: "SG" },
  { ten: "Dung", tuoi: 30, thanhPho: "SG" },
];
const ketQua = chonVaNhom(bang, 18);
console.log(ketQua.map((n) => `${n.thanhPho}:${n.soLuong}`).join(","));
```

```typescript title=solution
interface Nguoi {
  ten: string;
  tuoi: number;
  thanhPho: string;
}

interface NhomDem {
  thanhPho: string;
  soLuong: number;
}

function chonTheoTuoi(bang: Nguoi[], nguongDuoi: number): Nguoi[] {
  const ketQua: Nguoi[] = [];
  for (const hang of bang) {
    if (hang.tuoi > nguongDuoi) {
      ketQua.push(hang);
    }
  }
  return ketQua;
}

function nhomVaDem(bang: Nguoi[]): NhomDem[] {
  const dem = new Map<string, number>();
  for (const hang of bang) {
    const hienTai = dem.get(hang.thanhPho) ?? 0;
    dem.set(hang.thanhPho, hienTai + 1);
  }
  const ketQua: NhomDem[] = [];
  for (const [thanhPho, soLuong] of dem) {
    ketQua.push({ thanhPho, soLuong });
  }
  return ketQua;
}

function chonVaNhom(bang: Nguoi[], nguongDuoi: number): NhomDem[] {
  const daLoc = chonTheoTuoi(bang, nguongDuoi);
  return nhomVaDem(daLoc);
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15, thanhPho: "HN" },
  { ten: "Binh", tuoi: 20, thanhPho: "HN" },
  { ten: "Chi", tuoi: 25, thanhPho: "SG" },
  { ten: "Dung", tuoi: 30, thanhPho: "SG" },
];
const ketQua = chonVaNhom(bang, 18);
console.log(ketQua.map((n) => `${n.thanhPho}:${n.soLuong}`).join(","));
```

```typescript title=test
const bang2: Nguoi[] = [
  { ten: "An", tuoi: 15, thanhPho: "HN" },
  { ten: "Binh", tuoi: 20, thanhPho: "HN" },
  { ten: "Chi", tuoi: 25, thanhPho: "SG" },
  { ten: "Dung", tuoi: 30, thanhPho: "SG" },
];
const ketQua2 = chonVaNhom(bang2, 18);
console.log(ketQua2.map((n) => `${n.thanhPho}:${n.soLuong}`).join(","));
if (ketQua2.length !== 2) throw new Error("phai co dung 2 nhom sau khi loc");
if (ketQua2[0]?.thanhPho !== "HN" || ketQua2[0]?.soLuong !== 1) throw new Error("nhom HN sau loc phai con dung 1 (An bi loai)");
if (ketQua2[1]?.thanhPho !== "SG" || ketQua2[1]?.soLuong !== 2) throw new Error("nhom SG sau loc phai con dung 2 (Chi va Dung)");

const ketQuaNguongCao = chonVaNhom(bang2, 100);
if (ketQuaNguongCao.length !== 0) throw new Error("nguong cao hon moi tuoi phai cho ra danh sach nhom rong");
```

:::hints
- kind: attention
  body: "Loc truoc bang chonTheoTuoi, roi nhom ket qua da loc bang nhomVaDem -- hai dong."
- kind: strategy
  body: "const daLoc = chonTheoTuoi(bang, nguongDuoi); return nhomVaDem(daLoc);"
- kind: one-line
  body: "return nhomVaDem(chonTheoTuoi(bang, nguongDuoi));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "HN:1,SG:2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Pipeline hai bước, hai bộ máy, một kết quả. Ráp TOÀN bộ q10 lại
thành BOSS cuối cùng trông ra sao?
::::

::::reflect{#nghi-lai}
`chonVaNhom` KHÔNG viết hàm MỚI — nó chỉ GHÉP đúng thứ tự hai hàm
ĐÃ xác nhận riêng LẺ (bài 2, bài 10). Đây LÀ đúng tinh THẦN "pipeline"
mà q08 dạy: MỘT hệ thống thực thi THẬT không phải một khối mã DUY
nhất, mà LÀ nhiều bước NHỎ, mỗi bước ĐÃ đúng RIÊNG lẻ, ghép LẠI theo
đúng THỨ tự. Q10 CÒN một mảnh chưa GHÉP: đồ thị (bài 5-6). BOSS sẽ
ghép TẤT cả.
::::

::::checkpoint{mastery=0.85}
::::
