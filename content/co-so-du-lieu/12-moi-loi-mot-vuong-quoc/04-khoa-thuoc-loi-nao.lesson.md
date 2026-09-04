---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.khoa-thuoc-loi-nao
title: "Khoá thuộc lõi nào"
summary: "chonLoiChoKhoa(khoa, soLoi) = bam(khoa) % soLoi — TÁI DÙNG đúng hàm bam (q11 bài 6) và đúng công thức chonNodeNgayTho (q11 bài 1), chỉ đổi 'node' thành 'lõi'. Cùng một khoá LUÔN routing về đúng một lõi (tất định) — mọi lõi tự tính được câu trả lời mà không cần hỏi ai, giống hệt lý do băm-rồi-chia hoạt động được giữa nhiều máy ở q11."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.khoa-thuoc-loi-nao]
requires: [db.share-nothing-chia-theo-loi]
concepts: [db.khoa-thuoc-loi-nao]
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
MỖI lõi CÓ kho RIÊNG (bài TRƯỚC). NHƯNG khi CÓ một khoá MỚI cần lưu,
LÀM sao biết nó THUỘC kho của lõi NÀO?
::::

::::explain{#chon-loi-cho-khoa}
`chonLoiChoKhoa` dùng ĐÚNG hàm `bam` (q11 bài 6) VÀ đúng công THỨC
`chonNodeNgayTho` (q11 bài 1) — CHỈ đổi "node" (máy) thành "lõi"
(CPU):

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

function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}

console.log(chonLoiChoKhoa("nguoidung0", 4));
console.log(chonLoiChoKhoa("nguoidung1", 4));
console.log(chonLoiChoKhoa("nguoidung2", 4));
console.log(chonLoiChoKhoa("nguoidung3", 4));
```

```text title=readonly
2
0
2
2
```

VỚI `4` lõi (đánh SỐ `0` đến `3`), MỖI khoá băm RỒI lấy PHẦN dư — cho
ra đúng MỘT chỉ số lõi hợp LỆ. `nguoidung0`, `nguoidung2`, VÀ
`nguoidung3` cùng rơi VÀO lõi `2` — HOÀN toàn bình THƯỜNG, đúng như
q11 bài 1 đã dạy: phép chia LẤY dư CÓ thể ánh xạ nhiều khoá KHÁC nhau
VỀ cùng một chỉ số.
::::

::::example{#tat-dinh-khong-can-hoi-ai}
GỌI LẠI `chonLoiChoKhoa` VỚI cùng khoá VÀ cùng `soLoi` — LUÔN ra CÙNG
kết quả, KHÔNG cần lưu bảng tra CỨU nào:

```typescript title=readonly
console.log(chonLoiChoKhoa("nguoidung0", 4));
console.log(chonLoiChoKhoa("nguoidung0", 4));
```

```text title=readonly
2
2
```

MỖI lõi TỰ tính được "khoá NÀY có phải CỦA mình không" bằng CÁCH gọi
ĐÚNG hàm NÀY — không CẦN hỏi lõi khác, không CẦN một "lõi quản LÝ"
trung tâm biết hết mọi thứ.
::::

::::predict{#doan-route-sai commitOnce}
Một phiên bản `chonLoiChoKhoa` viết SAI: LUÔN trả về `0` (bỏ QUA
`bam`/`% soLoi` hoàn TOÀN, VÍ dụ `return 0;`). Chương trình CÓ báo
lỗi GÌ không khi chạy VỚI phiên bản NÀY?

:::opt{correct}
KHÔNG báo lỗi GÌ — chương trình VẪN chạy trơn TRU, chỉ LÀ MỌI khoá
đều dồn VÀO lõi `0`, các lõi CÒN lại hoàn toàn RẢNH rỗi
:::

:::opt
CÓ — trả VỀ LUÔN cùng một chỉ số LÀ một lời gọi HÀM không hợp lệ,
TypeScript sẽ từ chối biên DỊCH
::why
Gần đúng ở việc bạn nghĩ TỚI "hành vi kỳ QUẶC" NHƯ dấu hiệu của một
lỗi CÚ pháp hay kiểu — MỘT trực giác hợp LÝ khi mã nguồn LÀM điều gì
đó không mong ĐỢI.

Chỗ lệch: `return 0;` LÀ một hàm TRẢ về `number` HOÀN toàn hợp LỆ Ở
mặt KIỂU — TypeScript không CÓ cách nào biết "hàm NÀY nên PHÂN tán
đều" LÀ một yêu CẦU về mặt logic, chỉ kiểm được KIỂU dữ liệu. Chương
trình chạy hoàn TOÀN bình thường, KHÔNG lỗi — chỉ LÀ MỌI khoá đều
"THUỘC" lõi `0`, biến `soLoi - 1` lõi CÒN lại thành vô DỤNG. Đây
chính LÀ vấn đề "lõi NÓNG" (hot core) — một BÀI toán riêng, không
phải lỗi CHƯƠNG trình.
::
:::
::::

::::code{#viet_chon_loi_cho_khoa}
Hoàn thiện `chonLoiChoKhoa` — băm khoá RỒI lấy phần DƯ chia cho
`soLoi`.

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

function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return ___;
}

console.log(chonLoiChoKhoa("nguoidung0", 4));
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

function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}

console.log(chonLoiChoKhoa("nguoidung0", 4));
```

```typescript title=test
console.log(chonLoiChoKhoa("nguoidung0", 4));
if (chonLoiChoKhoa("nguoidung0", 4) !== 2) throw new Error("nguoidung0 voi 4 loi phai ra loi 2");
if (chonLoiChoKhoa("nguoidung1", 4) !== 0) throw new Error("nguoidung1 voi 4 loi phai ra loi 0");

if (chonLoiChoKhoa("bat_ky_khoa_nao", 1) !== 0) throw new Error("chi co 1 loi thi MOI khoa phai vao loi 0");

const lan1 = chonLoiChoKhoa("nguoidung7", 6);
const lan2 = chonLoiChoKhoa("nguoidung7", 6);
if (lan1 !== lan2) throw new Error("cung khoa, cung so loi phai LUON ra cung mot ket qua (tat dinh)");
if (lan1 < 0 || lan1 >= 6) throw new Error("ket qua phai nam trong [0, soLoi)");
```

:::hints
- kind: attention
  body: "Bam khoa thanh so, roi lay phan du chia cho soLoi -- mot dong."
- kind: strategy
  body: "bam(khoa) % soLoi"
- kind: one-line
  body: "return bam(khoa) % soLoi;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoá routing được VỀ đúng lõi. Điều đó CÓ xoá bỏ hoàn TOÀN nhu cầu
dùng khoá KHÔNG?
::::

::::reflect{#nghi-lai}
`chonLoiChoKhoa` KHÔNG phải một kỹ thuật MỚI — nó LÀ `chonNodeNgayTho`
(q11 bài 1) áp DỤNG Ở một quy MÔ khác: TRƯỚC LÀ máy TRONG một cụm,
GIỜ LÀ lõi TRONG một máy. CÙNG một Ý tưởng ("băm RỒI chia") giải
quyết được cả HAI bài toán, VÌ bản CHẤT của chúng LÀ giống nhau —
định TUYẾN một khoá VỀ đúng một "chủ SỞ hữu" trong một tập CÁC ứng cử
viên cố ĐỊNH. Khoá ĐÃ routing được VỀ đúng lõi — kết HỢP với "kho
riêng MỖI lõi" (bài 3), điều đó nghĩa LÀ gì cho việc CÓ còn cần khoá
(lock) hay KHÔNG?
::::

::::checkpoint{mastery=0.8}
::::
</content>
