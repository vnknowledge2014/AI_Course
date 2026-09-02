---
id: ky-nghe-phan-mem.kiem-thu.test-toan-bo-luong-su-kien
title: "Test toàn bộ LUỒNG — phát lại một chuỗi sự kiện, kiểm trạng thái CUỐI"
summary: "Ngoài test TỪNG bước riêng (bài 7-8), phát lại MỘT chuỗi sự kiện qua transition (vòng lặp), kiểm trạng thái CUỐI CÙNG — 'integration test' thu nhỏ nhưng VẪN thuần túy, VẪN đồng bộ. Nếu MỘT sự kiện GIỮA chuỗi bị từ chối, chuỗi DỪNG ở trạng thái TRƯỚC ĐÓ — kiểm cả case này."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.test-full-workflow]
requires: [kt.test-invalid-transitions]
concepts: [kt.test-full-workflow]
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
Đã test TỪNG bước riêng. Nhưng KHÁCH HÀNG THẬT trải nghiệm CẢ luồng —
xác nhận, rồi thanh toán, rồi giao, rồi nhận. Test CẢ luồng thế nào?
::::

::::explain{#phat-lai-mot-chuoi-su-kien}
NGOÀI test TỪNG bước RIÊNG (bài 7-8), **phát lại** MỘT **CHUỖI** sự
kiện qua `transition` (vòng lặp), kiểm trạng thái **CUỐI CÙNG** —
một "integration test" THU NHỎ nhưng VẪN thuần túy, VẪN đồng bộ
(KHÔNG database, KHÔNG mạng, CHỈ gọi hàm LẶP LẠI):

```typescript
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_nhan" };
  return { tag: "loi" };
}

function chayChuoi(trangThaiBatDau: TrangThai, cacSuKien: SuKien[]): TrangThai {
  let hienTai = trangThaiBatDau;
  for (const sk of cacSuKien) {
    const kq = transition(hienTai, sk);
    if (kq.tag === "loi") break;
    hienTai = kq.trangThaiMoi;
  }
  return hienTai;
}

function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(
  chayChuoi("nhap", ["xac_nhan", "thanh_toan", "giao_hang", "nhan_hang"]),
  "da_nhan",
  "chuoi day du tu nhap den da_nhan"
);
```

```text
[PASS] chuoi day du tu nhap den da_nhan
```

`chayChuoi` **VẪN LÀ HÀM THUẦN** — nó gọi `transition` LẶP LẠI, KHÔNG
lưu trữ, KHÔNG chờ đợi. Test NÀY khác test bài 7-8 Ở CHỖ: nó kiểm
**TOÀN BỘ LUỒNG** hoạt động ĐÚNG khi ghép LẠI VỚI NHAU — MỘT lỗi nhỏ
Ở BẤT KỲ bước NÀO trong chuỗi (ví dụ bước 3 SAI) sẽ làm trạng thái
CUỐI SAI, VÀ assertion NÀY bắt được.
::::

::::example{#chuoi-dung-lai-khi-bi-tu-choi}
Nếu MỘT sự kiện GIỮA chuỗi bị TỪ CHỐI, chuỗi **DỪNG LẠI** Ở trạng
thái TRƯỚC ĐÓ — KHÔNG PHẢI lỗi crash, cũng KHÔNG PHẢI "bỏ qua sự kiện
lỗi rồi tiếp tục":

```typescript title=readonly
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  return { tag: "loi" };
}

function chayChuoi(trangThaiBatDau: TrangThai, cacSuKien: SuKien[]): TrangThai {
  let hienTai = trangThaiBatDau;
  for (const sk of cacSuKien) {
    const kq = transition(hienTai, sk);
    if (kq.tag === "loi") break;
    hienTai = kq.trangThaiMoi;
  }
  return hienTai;
}

function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// "giao_hang" khong hop le tu "cho_thanh_toan" (thieu buoc thanh_toan)
assertEqual(
  chayChuoi("nhap", ["xac_nhan", "giao_hang"]),
  "cho_thanh_toan",
  "chuoi dung lai o trang thai truoc khi su kien bi tu choi"
);
```

```text title=readonly
[PASS] chuoi dung lai o trang thai truoc khi su kien bi tu choi
```

`"xac_nhan"` (bước 1) THÀNH CÔNG, đưa trạng thái tới `"cho_thanh_toan"`.
`"giao_hang"` (bước 2) BỊ TỪ CHỐI (KHÔNG có nhánh khớp `"cho_thanh_toan"`
+ `"giao_hang"`) — vòng lặp `break` NGAY, `hienTai` **GIỮ NGUYÊN** giá
trị TRƯỚC ĐÓ (`"cho_thanh_toan"`), KHÔNG bị ghi đè bởi kết quả LỖI.
::::

::::predict{#doan-chuoi-rong commitOnce}
```typescript
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  return { tag: "loi" };
}

function chayChuoi(trangThaiBatDau: TrangThai, cacSuKien: SuKien[]): TrangThai {
  let hienTai = trangThaiBatDau;
  for (const sk of cacSuKien) {
    const kq = transition(hienTai, sk);
    if (kq.tag === "loi") break;
    hienTai = kq.trangThaiMoi;
  }
  return hienTai;
}

// mảng RỖNG -- không có sự kiện nào cả
console.log(chayChuoi("nhap", []));
```

Dòng cuối in ra gì?

:::opt{correct}
`nhap`
:::

:::opt
Máy báo lỗi lúc chạy — vòng lặp `for...of` trên mảng RỖNG khiến biến
`hienTai` KHÔNG BAO GIỜ được gán giá trị, ĐỌC một biến `let` chưa gán
sẽ NÉM lỗi
::why
Gần đúng ở việc bạn để ý mảng `cacSuKien` LÀ RỖNG (`[]`) — một quan
sát ĐÚNG rằng vòng lặp `for...of` sẽ KHÔNG chạy LẦN nào.

Chỗ lệch: `hienTai` **ĐÃ** được gán giá trị TRƯỚC vòng lặp (`let
hienTai = trangThaiBatDau;` — dòng NÀY chạy VÔ ĐIỀU KIỆN, KHÔNG phụ
thuộc mảng RỖNG hay không), nên nó KHÔNG PHẢI biến "chưa gán". Vòng
lặp `for...of` trên mảng RỖNG đơn giản **KHÔNG THỰC THI THÂN LẶP LẦN
NÀO** (giống HỆT `for (const x of []) {...}` — bỏ qua HOÀN TOÀN, KHÔNG
lỗi), hàm nhảy thẳng tới `return hienTai;` — trả LẠI ĐÚNG giá trị BAN
ĐẦU, `"nhap"`.
::
:::

:::opt
Máy báo lỗi biên dịch — hàm `transition` trong đoạn NÀY LUÔN trả `{
tag: "loi" }`, KHÔNG bao giờ trả `{ tag: "ok", ... }`, nên TypeScript
suy kiểu trả về THỰC TẾ LÀ `{ tag: "loi" }` (không phải `KetQuaChuyen`
đầy đủ) và `chayChuoi` gọi `kq.trangThaiMoi` sẽ báo lỗi truy cập field
không tồn tại
::why
Gần đúng ở việc bạn nhận ra `transition` Ở đoạn NÀY LUÔN trả về CÙNG
MỘT giá trị `{ tag: "loi" }` — một quan sát ĐÚNG về HÀNH VI thực tế
của hàm.

Chỗ lệch: TypeScript suy kiểu trả về theo **CHỮ KÝ ĐÃ KHAI BÁO**
(`): KetQuaChuyen`), KHÔNG PHẢI theo "phân tích xem thân hàm THỰC SỰ
trả những gì" — dù thân hàm CHỈ CÓ một `return`, kiểu trả VẪN LÀ
`KetQuaChuyen` (union đầy đủ) NHƯ đã khai báo. Bên trong `chayChuoi`,
`kq.trangThaiMoi` CHỈ được truy cập SAU khi `if (kq.tag === "loi")
break;` đã LOẠI trừ nhánh `"loi"` — an toàn kiểu, giống MỌI lần dùng
trước. Với mảng RỖNG, đoạn code TRUY CẬP `kq.trangThaiMoi` KHÔNG BAO
GIỜ thực sự CHẠY (vòng lặp không lặp lần nào) — nhưng dù CÓ chạy,
biên dịch VẪN sạch nhờ narrowing. Biên dịch sạch, chạy KHÔNG lỗi.
::
:::
::::

::::code{#viet_chaychuoi}
Cho `transition` ĐÃ cài đặt sẵn (đầy đủ, giống bài 7-8). Tự viết phần
lõi của `chayChuoi`.

```typescript title=starter
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_nhan" };
  return { tag: "loi" };
}

function chayChuoi(trangThaiBatDau: TrangThai, cacSuKien: SuKien[]): TrangThai {
  let hienTai = trangThaiBatDau;
  for (const sk of cacSuKien) {
    const kq = transition(hienTai, sk);
    if (kq.tag === "loi") break;
    hienTai = ___;
  }
  return hienTai;
}

function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(chayChuoi("nhap", ["xac_nhan"]), "cho_thanh_toan", "mot buoc don gian");
```

```typescript title=solution
type TrangThai = "nhap" | "cho_thanh_toan" | "da_thanh_toan" | "da_giao" | "da_nhan" | "da_huy";
type SuKien = "xac_nhan" | "thanh_toan" | "giao_hang" | "nhan_hang" | "huy";
type KetQuaChuyen = { tag: "ok"; trangThaiMoi: TrangThai } | { tag: "loi" };

function transition(trangThaiHienTai: TrangThai, suKien: SuKien): KetQuaChuyen {
  if (trangThaiHienTai === "nhap" && suKien === "xac_nhan") return { tag: "ok", trangThaiMoi: "cho_thanh_toan" };
  if (trangThaiHienTai === "cho_thanh_toan" && suKien === "thanh_toan") return { tag: "ok", trangThaiMoi: "da_thanh_toan" };
  if (trangThaiHienTai === "da_thanh_toan" && suKien === "giao_hang") return { tag: "ok", trangThaiMoi: "da_giao" };
  if (trangThaiHienTai === "da_giao" && suKien === "nhan_hang") return { tag: "ok", trangThaiMoi: "da_nhan" };
  return { tag: "loi" };
}

function chayChuoi(trangThaiBatDau: TrangThai, cacSuKien: SuKien[]): TrangThai {
  let hienTai = trangThaiBatDau;
  for (const sk of cacSuKien) {
    const kq = transition(hienTai, sk);
    if (kq.tag === "loi") break;
    hienTai = kq.trangThaiMoi;
  }
  return hienTai;
}

function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(chayChuoi("nhap", ["xac_nhan"]), "cho_thanh_toan", "mot buoc don gian");
```

```typescript title=test
assertEqual(chayChuoi("nhap", ["xac_nhan", "thanh_toan", "giao_hang", "nhan_hang"]), "da_nhan", "chuoi day du den da_nhan");
assertEqual(chayChuoi("nhap", ["xac_nhan", "giao_hang"]), "cho_thanh_toan", "dung lai truoc su kien bi tu choi");
assertEqual(chayChuoi("nhap", []), "nhap", "chuoi rong giu nguyen trang thai ban dau");
assertEqual(chayChuoi("nhap", ["xac_nhan", "thanh_toan"]), "da_thanh_toan", "chuoi hai buoc dung dan");
```

:::hints
- kind: attention
  body: "Sau khi if(kq.tag === \"loi\") break; đã chạy qua (nghĩa là kq KHÔNG phải lỗi), TypeScript thu hẹp kq về đúng dạng { tag: \"ok\"; trangThaiMoi }. Cập nhật hienTai bằng field đó."
- kind: strategy
  body: "hienTai = kq.trangThaiMoi — đọc trạng thái mới ra từ kết quả chuyển đổi vừa tính, gán đè lên hienTai để vòng lặp tiếp tục từ đó."
- kind: one-line
  body: "___ = kq.trangThaiMoi"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phát lại chuỗi sự kiện, kiểm trạng thái cuối — vẫn thuần túy, vẫn
đồng bộ. Bước tiếp theo: TDD một máy trạng thái MỚI từ đầu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đã học trọn chu trình: Đỏ→Xanh, test đường hợp lệ, test đường bị từ
chối, test cả luồng. Áp DỤNG toàn bộ vào MỘT máy trạng thái HOÀN TOÀN
MỚI (đèn giao thông) — chỉ với một đặc tả THUẦN VĂN BẢN — sẽ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
