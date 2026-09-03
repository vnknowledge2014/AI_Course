---
id: co-so-du-lieu.mot-hop-nhieu-hinh.aggregation-group-by
title: "Aggregation thật: GROUP BY/COUNT"
summary: "SELECT thanh_pho, count() AS so_luong FROM nguoi GROUP BY thanh_pho; đếm số người theo từng thành phố trong MỘT dòng — kỹ thuật (đếm bằng Map, nhóm rồi liệt kê) hoàn toàn không xuất hiện trong q08 (executor chỉ có Quét/Lọc/ChọnCột, không có bước gộp-nhóm nào)."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.aggregation-group-by]
requires: [db.kieu-du-lieu-phong-phu]
concepts: [db.aggregation-group-by]
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
Executor q08 CÓ đúng ba bước: Quét, Lọc, Chọn cột. KHÔNG bước nào
đếm HAY nhóm dữ liệu lại. SurrealDB thật đếm SỐ người theo từng
thành phố NHƯ thế nào?
::::

::::explain{#group-by-count}
`GROUP BY` gộp các HÀNG có cùng giá TRỊ một cột lại VỚI nhau,
`count()` đếm SỐ hàng trong MỖI nhóm:

```text title=readonly
CREATE nguoi:a SET ten = 'An', thanh_pho = 'HN';
CREATE nguoi:b SET ten = 'Binh', thanh_pho = 'HN';
CREATE nguoi:c SET ten = 'Chi', thanh_pho = 'SG';
SELECT thanh_pho, count() AS so_luong FROM nguoi GROUP BY thanh_pho;
```

```text title=readonly
[{ thanh_pho: "HN", so_luong: 2 }, { thanh_pho: "SG", so_luong: 1 }]
```

BA người, HAI thành phố — `HN` có `An` VÀ `Binh` (đếm ĐƯỢC `2`), `SG`
chỉ CÓ `Chi` (đếm được `1`). Ba HÀNG gốc GỘP lại thành HAI hàng kết
quả — MỘT hàng CHO mỗi giá trị `thanh_pho` khác nhau.
::::

::::example{#khong-co-trong-q08}
q08's executor (bài 9-11: `tiep_quet`, `tiep_loc`, `chon_cot`) chỉ
GIỮ hoặc LOẠI từng hàng ĐỘC lập — KHÔNG hàm nào so sánh nhiều hàng
VỚI nhau để GỘP chúng lại. `GROUP BY`/`count()` LÀ một lớp thao tác
hoàn toàn KHÁC: nó cần NHỚ (đếm dồn theo TỪNG giá trị đã gặp) trong
khi quét, không chỉ QUYẾT định "giữ hay bỏ" từng hàng riêng LẺ.
::::

::::predict{#doan-nhom-rong commitOnce}
Bảng `sach` ĐÃ khai (`DEFINE TABLE sach;`) nhưng HOÀN toàn rỗng
(chưa `CREATE` bản ghi nào). `SELECT the_loai, count() AS so_luong
FROM sach GROUP BY the_loai;` trả về gì?

:::opt{correct}
`[]` — mảng RỖNG, không CÓ hàng nào để nhóm
:::

:::opt
`[{ the_loai: null, so_luong: 0 }]` — MỘT nhóm "rỗng" đại diện, thể
hiện rằng phép ĐẾM đã chạy nhưng KHÔNG tìm thấy gì
::why
Gần đúng ở việc bạn nghĩ TỚI việc "trả VỀ một dấu hiệu ĐÃ chạy
xong" — một cách BIỂU diễn hợp lý cho "phép TÍNH toán rỗng" Ở một
số ngôn ngữ LẬP trình (ví dụ `reduce` VỚI giá trị khởi tạo).

Chỗ lệch: `GROUP BY` chỉ tạo RA đúng một hàng kết quả CHO mỗi giá
trị NHÓM thực sự XUẤT hiện trong dữ liệu — KHÔNG có hàng NÀO xuất
hiện (bảng rỗng) nghĩa LÀ không CÓ giá trị nhóm NÀO để tạo hàng kết
quả CHO nó. Kết quả LÀ mảng rỗng THẬT sự, giống hệt `SELECT ... FROM
nguoi WHERE ...` trên MỘT bảng rỗng (không CÓ gì để lọc thì không
CÓ gì để trả VỀ).
::
:::
::::

::::code{#viet_nhom_va_dem}
Hoàn thiện `nhomVaDem` — đếm SỐ người theo TỪNG `thanhPho`, dùng
`Map<string, number>` để dồn SỐ đếm trong lúc quét.

```typescript title=starter
interface Nguoi {
  ten: string;
  thanhPho: string;
}

interface NhomDem {
  thanhPho: string;
  soLuong: number;
}

function nhomVaDem(bang: Nguoi[]): NhomDem[] {
  const dem = new Map<string, number>();
  for (const hang of bang) {
    const hienTai = dem.get(hang.thanhPho) ?? 0;
    ___
  }
  const ketQua: NhomDem[] = [];
  for (const [thanhPho, soLuong] of dem) {
    ketQua.push({ thanhPho, soLuong });
  }
  return ketQua;
}

const bang: Nguoi[] = [
  { ten: "An", thanhPho: "HN" },
  { ten: "Binh", thanhPho: "HN" },
  { ten: "Chi", thanhPho: "SG" },
];
const ketQua = nhomVaDem(bang);
console.log(ketQua.map((n) => `${n.thanhPho}:${n.soLuong}`).join(","));
```

```typescript title=solution
interface Nguoi {
  ten: string;
  thanhPho: string;
}

interface NhomDem {
  thanhPho: string;
  soLuong: number;
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

const bang: Nguoi[] = [
  { ten: "An", thanhPho: "HN" },
  { ten: "Binh", thanhPho: "HN" },
  { ten: "Chi", thanhPho: "SG" },
];
const ketQua = nhomVaDem(bang);
console.log(ketQua.map((n) => `${n.thanhPho}:${n.soLuong}`).join(","));
```

```typescript title=test
const bang2: Nguoi[] = [
  { ten: "An", thanhPho: "HN" },
  { ten: "Binh", thanhPho: "HN" },
  { ten: "Chi", thanhPho: "SG" },
];
const ketQua2 = nhomVaDem(bang2);
console.log(ketQua2.map((n) => `${n.thanhPho}:${n.soLuong}`).join(","));
if (ketQua2.length !== 2) throw new Error("phai co dung 2 nhom (HN va SG)");
if (ketQua2[0]?.thanhPho !== "HN" || ketQua2[0]?.soLuong !== 2) throw new Error("nhom HN phai co so_luong 2, khop SurrealDB that");
if (ketQua2[1]?.thanhPho !== "SG" || ketQua2[1]?.soLuong !== 1) throw new Error("nhom SG phai co so_luong 1");

const bangRong: Nguoi[] = [];
if (nhomVaDem(bangRong).length !== 0) throw new Error("bang rong phai cho ra danh sach nhom rong");
```

:::hints
- kind: attention
  body: "Cap nhat dem cho thanhPho hien tai: hienTai cong 1 -- mot dong."
- kind: strategy
  body: "dem.set(hang.thanhPho, hienTai + 1);"
- kind: one-line
  body: "dem.set(hang.thanhPho, hienTai + 1);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "HN:2,SG:1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Aggregation — một khả năng mini-engine chưa hề chạm. Đủ những điểm
khác biệt rồi — vậy TẠI SAO còn xây mini-engine LÀM gì?
::::

::::reflect{#nghi-lai}
`GROUP BY`/`count()` LÀ ví dụ RÕ nhất cho những gì SurrealDB thật
LÀM được mà q07-q09 KHÔNG hề dạy — không PHẢI vì khó, mà VÌ q08 tập
TRUNG dạy đúng LÕI (Quét → Lọc → ChọnCột) trước khi mở RỘNG. Cùng
Ý tưởng "dồn SỐ đếm trong lúc quét" (dùng `Map`) LÀ một kỹ thuật
hoàn toàn tự nhiên MỞ rộng TỪ những gì đã học — chỉ LÀ chưa từng cần
tới.
::::

::::checkpoint{mastery=0.8}
::::
