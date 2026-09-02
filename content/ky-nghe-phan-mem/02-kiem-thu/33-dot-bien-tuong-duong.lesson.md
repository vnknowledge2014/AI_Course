---
id: ky-nghe-phan-mem.kiem-thu.dot-bien-tuong-duong
title: "Đột biến TƯƠNG ĐƯƠNG — thay đổi CÚ PHÁP, KHÔNG đổi HÀNH VI"
summary: "Giới hạn thành thật của mutation testing: MỘT SỐ đột biến thay đổi cú pháp nhưng KHÔNG đổi hành vi thật — gọi là 'đột biến tương đương'. n*2 đổi thành n+n giống hệt nhau với MỌI n, không test nào (dù viết tốt tới đâu) giết được nó. Nhận diện được đột biến tương đương là một kỹ năng thực tế."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 33
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.equivalent-mutants]
requires: [kt.mutation-value-swap]
concepts: [kt.equivalent-mutants]
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
`n * 2` đổi thành `n + n` — về mặt TOÁN HỌC, GIỐNG HỆT nhau với MỌI
`n`. Có test NÀO (dù viết tốt tới đâu) giết được đột biến NÀY không?
::::

::::explain{#dot-bien-tuong-duong-gioi-han-that}
**GIỚI HẠN THÀNH THẬT** của mutation testing: MỘT SỐ đột biến thay
đổi **CÚ PHÁP** NHƯNG **KHÔNG** đổi **HÀNH VI THẬT** — GỌI LÀ "đột
biến TƯƠNG ĐƯƠNG" (equivalent mutant):

```typescript
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function nhanDoi(n: number): number {
  return n * 2;
}

// DOT BIEN: * 2 thanh + n
function nhanDoiDotBien(n: number): number {
  return n + n;
}

// kiem tinh chat: HAI ham co LUON cho CUNG ket qua khong, tren HANG TRAM gia tri ngau nhien
kiemTraTinhChat(soNguyen(-100000, 100000), (n) => nhanDoi(n) === nhanDoiDotBien(n), 500);
```

```text
[PASS] tinh chat dung tren ca 500 lan
```

VỀ MẶT TOÁN HỌC, `n * 2` VÀ `n + n` **GIỐNG HỆT NHAU** VỚI **MỌI**
`n` — KHÔNG test NÀO (DÙ viết TỐT tới ĐÂU, DÙ thử BAO NHIÊU giá trị)
CÓ THỂ GIẾT được đột biến NÀY, VÀ ĐÓ **HOÀN TOÀN KHÔNG PHẢI** LỖI của
bộ test.
::::

::::example{#du-an-nay-cung-co-danh-sach-mien-tru}
Nhận diện được đột biến TƯƠNG ĐƯƠNG (THAY VÌ CỐ viết test VÔ VỌNG để
giết nó) LÀ MỘT kỹ năng THỰC TẾ — CÔNG cụ mutation testing THẬT (KỂ
CẢ `tools/kiem_dot_bien.mjs` CỦA CHÍNH dự án NÀY) ĐỀU có "danh sách
MIỄN TRỪ" cho trường hợp NÀY:

```typescript title=readonly
// vi du trich tu content/curriculum/dot-bien-bo-qua.yaml CUA CHINH du an nay:
//
// # TƯƠNG ĐƯƠNG THẬT — cấu trúc riêng của HÀM CLAMP. dieuChinhKhoangGiaTri
// # trả "min" khi gia < min, và trả CHÍNH "gia" khi gia đã hợp lệ. Tại ĐÚNG
// # điểm biên gia === min, HAI nhánh cho ra CÙNG một GIÁ TRỊ SỐ...
// - 'ky-nghe-phan-mem.kiem-thu.mau-bat-bien · viet_dieuchinhkhoanggiatri
//    · đổi MỌI dấu < thành <= (1 chỗ)'

function dieuChinhKhoangGiaTri(gia: number, min: number, max: number): number {
  if (gia < min) return min;
  if (gia > max) return max;
  return gia;
}

// tai DUNG bien (gia === min), doi < thanh <= KHONG doi ket qua:
// gia < min: false -> tra ve gia (= min, vi gia===min)
// gia <= min: true -> tra ve min (= gia, vi gia===min)
// CA HAI cho CUNG mot GIA TRI SO -- day CHINH LA vi sao bai 21 da khai MIEN TRU
console.log(dieuChinhKhoangGiaTri(10, 10, 20));
```

```text title=readonly
10
```

BÀI **21** (mẫu Bất Biến, `dieuChinhKhoangGiaTri`) THỰC SỰ ĐÃ gặp
đúng TÌNH HUỐNG NÀY — HAI đột biến `<`↔`<=` VÀ `>`↔`>=` SỐNG SÓT
KHÔNG PHẢI VÌ bộ test THIẾU SÓT, MÀ VÌ CHÚNG LÀ đột biến TƯƠNG ĐƯƠNG
THẬT (đã KHAI vào `dot-bien-bo-qua.yaml`, KHÔNG force-fix bằng CÁCH
nới LỎNG lời giải).
::::

::::predict{#doan-khong-phai-moi-thu-deu-tuong-duong commitOnce}
```typescript
function nhanDoi(n: number): number {
  return n * 2;
}
function nhanDoiDotBienKhac(n: number): number {
  return n * n; // MOT dot bien KHAC -- co PHAI tuong duong khong?
}

console.log(nhanDoi(3), nhanDoiDotBienKhac(3));
```

Dòng cuối in ra gì?

:::opt{correct}
`6 9`
:::

:::opt
`6 6` — vì `n * n` (bình phương) VÀ `n * 2` (nhân đôi) ĐỀU LÀ "phép
nhân LIÊN QUAN tới `n`", NÊN CHÚNG CŨNG LÀ đột biến TƯƠNG ĐƯƠNG,
GIỐNG `n + n` Ở PHẦN TRƯỚC
::why
Gần đúng ở việc bạn nhớ `n + n` VÀ `n * 2` LÀ TƯƠNG ĐƯƠNG THẬT (đã
xác NHẬN Ở phần TRƯỚC) — MỘT sự kiện ĐÚNG, NHƯNG bạn suy RỘNG "MỌI
phép nhân liên quan `n`" ĐỀU tương ĐƯƠNG — ĐÂY LÀ SUY LUẬN QUÁ VỘI.

Chỗ lệch: `n * n` (bình PHƯƠNG) VÀ `n * 2` (nhân ĐÔI) LÀ HAI phép
tính **HOÀN TOÀN KHÁC NHAU** VỀ MẶT toán HỌC — chúng CHỈ trùng NHAU
TẠI `n = 0` (`0*0=0=0*2`) VÀ `n = 2` (`2*2=4=2*2`), **KHÔNG PHẢI**
VỚI MỌI `n`. TẠI `n = 3`: `nhanDoi(3) = 3 * 2 = 6`. `nhanDoiDotBienKhac
(3) = 3 * 3 = 9`. HAI kết quả **KHÁC NHAU RÕ RỆT** — đột biến NÀY
**KHÔNG PHẢI** tương đương, MỘT test ĐƠN GIẢN (`n=3`) ĐÃ đủ GIẾT nó.
Nhận diện đột biến TƯƠNG ĐƯƠNG đòi HỎI KIỂM TRA CẨN THẬN (TOÁN HỌC
HOẶC chạy THỬ), KHÔNG PHẢI đoán "trông giống NHAU".
::
:::

:::opt
Máy báo lỗi biên dịch — HAI hàm `nhanDoi`/`nhanDoiDotBienKhac` CÙNG
NHẬN `n: number` NHƯNG MỘT hàm TÊN CÓ "DotBien", TypeScript ĐÒI cặp
hàm SO SÁNH NHAU (`console.log` gọi CẢ HAI TRÊN CÙNG dòng) PHẢI CÓ
TÊN "khớp mẫu" (CÙNG tiền TỐ, chỉ khác hậu TỐ)
::why
Gần đúng ở việc bạn để ý HAI hàm CÓ TÊN "liên QUAN" (`nhanDoi`,
`nhanDoiDotBienKhac`) — một quan sát ĐÚNG về QUY ƯỚC đặt tên.

Chỗ lệch: TypeScript KHÔNG có RÀNG BUỘC "khớp MẪU tên" giữa các hàm
được GỌI TRÊN CÙNG MỘT dòng (HAY BẤT KỲ đâu) — TÊN hàm CHỈ LÀ nhãn
tham chiếu, HOÀN TOÀN ĐỘC LẬP với NHAU. `console.log` chấp NHẬN BẤT
KỲ số lượng đối SỐ nào, THUỘC BẤT KỲ hàm NÀO, MIỄN các LỜI GỌI hàm
HỢP LỆ RIÊNG rẽ. Biên dịch sạch.
::
:::
::::

::::code{#viet_latuongduong}
Tự viết `laTuongDuong` — hàm kiểm HAI hàm có TƯƠNG ĐƯƠNG hay KHÔNG.

```typescript title=starter
function laTuongDuong(hamA: (n: number) => number, hamB: (n: number) => number, soLan: number): boolean {
  for (let i = 0; i < soLan; i++) {
    const n = Math.floor(Math.random() * 200001) - 100000;
    if (___) return ___;
  }
  return true;
}

function nhanDoi(n: number): number { return n * 2; }
function nhanDoiDotBien(n: number): number { return n + n; }

const ketQuaTuongDuong = laTuongDuong(nhanDoi, nhanDoiDotBien, 300);
console.log(ketQuaTuongDuong ? "[PASS] n*2 va n+n phai duoc nhan dien la tuong duong" : "[FAIL] n*2 va n+n phai duoc nhan dien la tuong duong");
```

```typescript title=solution
function laTuongDuong(hamA: (n: number) => number, hamB: (n: number) => number, soLan: number): boolean {
  for (let i = 0; i < soLan; i++) {
    const n = Math.floor(Math.random() * 200001) - 100000;
    if (hamA(n) !== hamB(n)) return false;
  }
  return true;
}

function nhanDoi(n: number): number { return n * 2; }
function nhanDoiDotBien(n: number): number { return n + n; }

const ketQuaTuongDuong = laTuongDuong(nhanDoi, nhanDoiDotBien, 300);
console.log(ketQuaTuongDuong ? "[PASS] n*2 va n+n phai duoc nhan dien la tuong duong" : "[FAIL] n*2 va n+n phai duoc nhan dien la tuong duong");
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function nhanDoiA(n: number): number { return n * 2; }
function nhanDoiTuongDuong(n: number): number { return n + n; }
function nhanDoiKhongTuongDuong(n: number): number { return n * n; }

assertEqual(laTuongDuong(nhanDoiA, nhanDoiTuongDuong, 300), true, "n*2 va n+n PHAI duoc nhan dien la tuong duong");
assertEqual(laTuongDuong(nhanDoiA, nhanDoiKhongTuongDuong, 300), false, "n*2 va n*n KHONG duoc nhan dien nham la tuong duong");
```

:::hints
- kind: attention
  body: "Nếu tìm được MỘT giá trị n mà hai hàm cho kết quả KHÁC nhau, chúng KHÔNG tương đương — trả về false ngay. Nếu chạy hết vòng lặp mà không tìm được khác biệt nào, coi là tương đương (true)."
- kind: strategy
  body: 'hamA(n) !== hamB(n) : false — điều kiện phát hiện khác biệt, giá trị trả khi phát hiện.'
- kind: one-line
  body: '___ (dieu kien) = hamA(n) !== hamB(n)\n___ (gia tri tra) = false'
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
Đột biến tương đương: giới hạn thành thật, nhận diện thay vì cố giết.
Bài BOSS: săn đột biến THẬT — thêm test cho tới khi giết hết.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`phanLoaiTuoi(tuoi)` phân loại "trẻ em"/"thiếu niên"/"người lớn" —
bộ test BAN ĐẦU chỉ có BA ví dụ RÕ RÀNG. Ghép TRỌN kỷ luật cả track
để thêm đủ test giết HẾT đột biến còn sống — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
