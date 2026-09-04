---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.mot-bang-cho-moi-cau-hoi
title: "Một bảng cho mỗi câu hỏi"
summary: "demTinNhanPhongChuanHoa quét MỘT bảng phẳng (chuẩn hoá, kiểu quan hệ) để tìm tin nhắn của một phòng — chạm CẢ 1000 tin nhắn dù phòng đó chỉ có 20. xayBangTheoPhong tổ chức lại dữ liệu THÀNH bảng riêng theo phongId (wide-column) — cùng câu hỏi giờ chỉ chạm đúng 20 tin nhắn của phòng đó, không hơn. Cùng dữ liệu, hai cách tổ chức khác nhau cho hai kết quả CHI PHÍ khác hẳn — mô hình hoá THEO câu hỏi nghĩa là: có bao nhiêu câu hỏi thường dùng, có bấy nhiêu bảng."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.mot-bang-cho-moi-cau-hoi]
requires: [db.so-sanh-ba-chien-luoc-nen]
concepts: [db.mot-bang-cho-moi-cau-hoi]
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
q07-q14 xây MỌI cơ chế của một hệ phân tán — quorum, hint, read
repair, compaction. NHƯNG chưa hề hỏi: dữ liệu nên được TỔ CHỨC như
thế nào ngay từ đầu? Câu trả lời đổi hẳn cách bảng được thiết kế.
::::

::::explain{#mot-bang-cho-moi-cau-hoi}
CSDL quan hệ "chuẩn hoá": MỘT bảng phẳng chứa mọi tin nhắn, mỗi hàng
một sự kiện, không lặp dữ liệu. Muốn hỏi "tin nhắn của phòng X", phải
QUÉT (hoặc dựa vào một chỉ mục riêng) — `demTinNhanPhongChuanHoa`
làm đúng việc quét đó, đếm luôn số lượt chạm:

```typescript title=readonly
interface TinNhan { id: string; phongId: string; nguoiId: string; thoiGian: number; }

function taoDuLieuMau(soTinNhan: number, soPhong: number): TinNhan[] {
  const ds: TinNhan[] = [];
  for (let i = 0; i < soTinNhan; i++) {
    ds.push({ id: "tn" + i, phongId: "phong" + (i % soPhong), nguoiId: "nguoi" + (i % 20), thoiGian: i });
  }
  return ds;
}

function demTinNhanPhongChuanHoa(tatCa: TinNhan[], phongId: string): { soLuotQuet: number; soTimThay: number } {
  let soLuotQuet = 0;
  let soTimThay = 0;
  for (const t of tatCa) {
    soLuotQuet++;
    if (t.phongId === phongId) soTimThay++;
  }
  return { soLuotQuet, soTimThay };
}

function xayBangTheoPhong(tatCa: TinNhan[]): Map<string, TinNhan[]> {
  const bang = new Map<string, TinNhan[]>();
  for (const t of tatCa) {
    const ds = bang.get(t.phongId) ?? [];
    ds.push(t);
    bang.set(t.phongId, ds);
  }
  return bang;
}

const duLieu = taoDuLieuMau(1000, 50);
console.log("chuan hoa (quet bang phang):", demTinNhanPhongChuanHoa(duLieu, "phong5"));

const bangTheoPhong = xayBangTheoPhong(duLieu);
console.log("wide-column (bang rieng theo phong):", (bangTheoPhong.get("phong5") ?? []).length);
console.log("tong so phong:", bangTheoPhong.size);
```

```text title=readonly
chuan hoa (quet bang phang): { soLuotQuet: 1000, soTimThay: 20 }
wide-column (bang rieng theo phong): 20
tong so phong: 50
```

`1000` tin nhắn CHIA đều cho `50` phòng — mỗi phòng đúng `20` tin
nhắn. Cách chuẩn hoá phải QUÉT hết CẢ `1000` tin nhắn để tìm ra `20`
tin nhắn của phòng `5`. `xayBangTheoPhong` gộp TRƯỚC dữ liệu thành
bảng riêng theo `phongId` — hỏi ĐÚNG câu hỏi đó giờ chỉ chạm ĐÚNG `20`,
không hơn.
::::

::::example{#lap-du-lieu-co-chu-dich}
`xayBangTheoPhong` LƯU lại đúng những tin nhắn NÀY một LẦN nữa, dưới
một khoá khác — đây LÀ "lặp dữ liệu có chủ đích" (denormalization),
KHÔNG phải lỗi thiết kế. Đổi lấy TỐC độ đọc, wide-column chấp nhận
tốn thêm KHÔNG gian lưu trữ VÀ phải ghi dữ liệu và NHIỀU bảng khi có
sự kiện MỚI — cái giá đã thấy trước Ở q14 (mỗi bảng "câu hỏi" riêng
CẦN được cập nhật đồng bộ, giống cách MỘT khoá có nhiều bản sao).
::::

::::predict{#doan-phong-khac commitOnce}
Gọi `demTinNhanPhongChuanHoa(duLieu, "phong49")` (phòng CUỐI cùng,
`49`, thay vì `5`). `soLuotQuet` trong kết quả trả VỀ LÀ bao nhiêu?

:::opt{correct}
`1000` — y hệt phòng `5`; cách chuẩn hoá LUÔN quét hết TOÀN bộ bảng
phẳng, bất kể đang tìm phòng NÀO hay phòng đó nằm Ở đâu trong mảng
:::

:::opt
Lớn hơn `1000` một chút — phòng CUỐI (`49`) nằm Ở CUỐI mảng nên phải
quét xa hơn, kèm chi phí "tìm tới" trước khi bắt đầu đếm
::why
Trực giác NÀY nhầm CÁCH `demTinNhanPhongChuanHoa` hoạt động VỚI một
cấu trúc CÓ chỉ mục (Ở đó vị trí dữ liệu MỚI ảnh hưởng chi phí).

Chỗ lệch: vòng `for` duyệt TUẦN tự hết `tatCa` — KHÔNG có bước "tìm
tới" riêng biệt nào, VÀ không hề dừng SỚM khi đã đếm đủ. Dù `phongId`
đang tìm nằm Ở đâu trong mảng (đầu, giữa, hay cuối), `soLuotQuet` LUÔN
đúng bằng `tatCa.length` — `1000`, không hơn không kém, cho MỌI phòng.
::
:::
::::

::::code{#viet_xay_bang_theo_phong}
Hoàn thiện `xayBangTheoPhong` — với mỗi tin nhắn, thêm nó VÀO đúng
danh sách của phòng nó thuộc về.

```typescript title=starter
interface TinNhan { id: string; phongId: string; nguoiId: string; thoiGian: number; }

function taoDuLieuMau(soTinNhan: number, soPhong: number): TinNhan[] {
  const ds: TinNhan[] = [];
  for (let i = 0; i < soTinNhan; i++) {
    ds.push({ id: "tn" + i, phongId: "phong" + (i % soPhong), nguoiId: "nguoi" + (i % 20), thoiGian: i });
  }
  return ds;
}

function xayBangTheoPhong(tatCa: TinNhan[]): Map<string, TinNhan[]> {
  const bang = new Map<string, TinNhan[]>();
  for (const t of tatCa) {
    ___
  }
  return bang;
}

const duLieu = taoDuLieuMau(1000, 50);
console.log(xayBangTheoPhong(duLieu).get("phong5")!.length);
```

```typescript title=solution
interface TinNhan { id: string; phongId: string; nguoiId: string; thoiGian: number; }

function taoDuLieuMau(soTinNhan: number, soPhong: number): TinNhan[] {
  const ds: TinNhan[] = [];
  for (let i = 0; i < soTinNhan; i++) {
    ds.push({ id: "tn" + i, phongId: "phong" + (i % soPhong), nguoiId: "nguoi" + (i % 20), thoiGian: i });
  }
  return ds;
}

function xayBangTheoPhong(tatCa: TinNhan[]): Map<string, TinNhan[]> {
  const bang = new Map<string, TinNhan[]>();
  for (const t of tatCa) {
    const ds = bang.get(t.phongId) ?? [];
    ds.push(t);
    bang.set(t.phongId, ds);
  }
  return bang;
}

const duLieu = taoDuLieuMau(1000, 50);
console.log(xayBangTheoPhong(duLieu).get("phong5")!.length);
```

```typescript title=test
const duLieu2 = taoDuLieuMau(1000, 50);
const bang2 = xayBangTheoPhong(duLieu2);
if (bang2.size !== 50) throw new Error("phai co dung 50 phong (moi phong it nhat 1 tin nhan)");
if ((bang2.get("phong5") ?? []).length !== 20) throw new Error("phong5 phai co dung 20 tin nhan");
if ((bang2.get("phong0") ?? []).length !== 20) throw new Error("phong0 phai co dung 20 tin nhan");

let tongTatCaPhong = 0;
for (const [, ds] of bang2) tongTatCaPhong += ds.length;
if (tongTatCaPhong !== 1000) throw new Error("tong so tin nhan trong moi phong cong lai phai bang dung 1000, khong mat khong trung");

for (const t of bang2.get("phong5")!) {
  if (t.phongId !== "phong5") throw new Error("moi tin nhan trong danh sach cua phong5 phai thuc su co phongId=phong5");
}

const duLieuRong = taoDuLieuMau(0, 50);
if (xayBangTheoPhong(duLieuRong).size !== 0) throw new Error("khong co tin nhan nao thi bang phai rong");
```

:::hints
- kind: attention
  body: "Lay danh sach hien co cua phong (hoac mang rong neu chua co), them tin nhan vao, roi luu lai -- ba dong."
- kind: strategy
  body: "const ds = bang.get(t.phongId) ?? []; ds.push(t); bang.set(t.phongId, ds);"
- kind: one-line
  body: "const ds = bang.get(t.phongId) ?? []; ds.push(t); bang.set(t.phongId, ds);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bảng riêng cho MỘT câu hỏi — nhưng làm SAO biết chính xác dữ liệu
nào đi VỀ đâu, khi có hàng NGÀN bảng như vậy trên hàng ngàn máy?
::::

::::reflect{#nghi-lai}
`xayBangTheoPhong` KHÔNG phải một cấu trúc dữ liệu mới — nó ĐÚNG LÀ
`Map<string, TinNhan[]>`, y hệt các bảng ĐÃ dùng xuyên suốt q11-q14.
Điều thay đổi LÀ TƯ DUY thiết kế: thay VÌ hỏi "thực thể nào cần một
bảng" (chuẩn hoá, kiểu quan hệ), wide-column hỏi "câu hỏi nào cần
được trả lời NHANH, và bảng nào phục vụ đúng câu hỏi đó". Khoá dùng để
tra bảng (`phongId` Ở ĐÂY) chính LÀ điều bài tiếp theo gọi tên: khoá
phân vùng.
::::

::::checkpoint{mastery=0.8}
::::
