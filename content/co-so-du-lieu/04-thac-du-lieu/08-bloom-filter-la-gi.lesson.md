---
id: co-so-du-lieu.thac-du-lieu.bloom-filter-la-gi
title: Bloom filter là gì
summary: "Một bloom filter trả lời NHANH câu hỏi 'khoá này CÓ THỂ nằm trong SSTable không?' bằng vài bit, không cần quét gì. Nó KHÔNG BAO GIỜ báo sai kiểu 'chắc chắn không có' khi THẬT SỰ có (không âm tính giả) — nhưng CÓ THỂ báo sai kiểu 'có thể có' khi THẬT SỰ không có (dương tính giả)."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.bloom-filter-idea]
requires: [db.sparse-index]
concepts: [db.bloom-filter-idea]
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
Chỉ mục thưa (bài TRƯỚC) vẫn phải quét MỘT vùng nhỏ trước khi biết
chắc "khoá này KHÔNG có". Có cách nào biết TRƯỚC, gần như tức THÌ?
::::

::::explain{#bloom-filter}
Một bloom filter LÀ một mảng bit, cùng vài HÀM băm cố định. Thêm
một khoá — BẬT các bit Ở vị trí băm ra. Hỏi "có thể CÓ khoá này
không" — kiểm TRA đúng những bit đó có ĐANG bật hết không:

```typescript title=readonly
function bamDon(s: string, seed: number, kichThuoc: number): number {
  let h = seed;
  for (let i = 0; i < s.length; i++) {
    h = (h * 31 + s.charCodeAt(i)) >>> 0;
  }
  return h % kichThuoc;
}

interface BoLocBloom {
  bit: boolean[];
  kichThuoc: number;
  soHam: number;
}

function taoBoLocBloom(kichThuoc: number, soHam: number): BoLocBloom {
  return { bit: new Array(kichThuoc).fill(false), kichThuoc, soHam };
}

function themBloom(bo: BoLocBloom, khoa: string): void {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    bo.bit[viTri] = true;
  }
}

function coTheCoBloom(bo: BoLocBloom, khoa: string): boolean {
  for (let seed = 0; seed < bo.soHam; seed++) {
    const viTri = bamDon(khoa, seed, bo.kichThuoc);
    if (bo.bit[viTri] !== true) return false;
  }
  return true;
}

const bo = taoBoLocBloom(8, 2);
themBloom(bo, "cam");
themBloom(bo, "buoi");
console.log(coTheCoBloom(bo, "cam"), coTheCoBloom(bo, "tao"));
```

```text title=readonly
true false
```

`"cam"` ĐÃ thêm — cả hai bit của nó (hai hàm băm, `seed=0` VÀ
`seed=1`) ĐANG bật, `coTheCoBloom` trả về `true`. `"tao"` CHƯA
từng thêm — Ít nhất MỘT bit của nó chưa bật, trả về `false`: đây LÀ
một ÂM tính THẬT — bloom filter đúng khi nó nói "không có".
::::

::::example{#duong-tinh-gia}
Nhưng bloom filter CÓ THỂ nói "có thể có" cho một khoá CHƯA từng
được thêm — nếu MỌI bit của nó tình cờ ĐÃ bật SẴN bởi khoá khác:

```typescript title=readonly
console.log(coTheCoBloom(bo, "chanh"));
console.log(bamDon("chanh", 0, 8), bamDon("chanh", 1, 8));
```

```text title=readonly
true
6 5
```

`"chanh"` CHƯA bao giờ được `themBloom`. Nhưng hai bit của NÓ (vị
trí `6` VÀ `5`) tình cờ TRÙNG với hai bit ĐÃ bật bởi `"cam"`/`"buoi"`
— `coTheCoBloom` trả về `true`, một DƯƠNG tính GIẢ: bloom filter nói
"có thể có" cho một khoá THẬT SỰ không có. Đây LÀ chi phí của việc
nén thông tin xuống VÀI bit — càng ÍT bit trên mỗi khoá, càng DỄ va
chạm.
::::

::::predict{#doan-am-tinh-that commitOnce}
Byte hỏi bloom filter Ở TRÊN về một khoá KHÁC, `"le"`:

```typescript
console.log(coTheCoBloom(bo, "le"));
```

Byte biết TRƯỚC (đã tính SẴN): `bamDon("le", 0, 8) = 1` VÀ
`bamDon("le", 1, 8) = 2` — CẢ hai bit đó ĐANG tắt (`false`) trong
`bo.bit` (chỉ CÓ bit `5, 6, 7` được bật bởi `"cam"`/`"buoi"`). Dòng
cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì bloom filter LUÔN thiên về báo "có thể có" để KHÔNG bỏ
sót khoá nào, giống HỆT trường hợp `"chanh"` Ở trên
::why
Gần đúng ở việc bạn nhớ ĐÚNG tính chất "không bỏ sót" của bloom
filter (không âm tính GIẢ) — một quan sát chính XÁC về THIẾT kế của
nó.

Chỗ lệch: "không bỏ SÓT" nghĩa LÀ không bao giờ báo `false` cho một
khoá THẬT SỰ có — nó KHÔNG có nghĩa LÀ luôn báo `true`. `"chanh"`
báo `true` (SAI, dương tính giả) LÀ vì hai bit của nó TÌNH CỜ trùng
bit đã bật SẴN. `"le"` có hai bit (`1` VÀ `2`) đều CHƯA từng bật —
`coTheCoBloom` kiểm tra thấy bit `1` tắt, trả VỀ `false` ngay, không
cần xét TIẾP bit thứ hai.
::
:::

:::opt
Máy báo lỗi — vì `"le"` chưa từng được `themBloom`, gọi
`coTheCoBloom` trên một khoá CHƯA thêm là thao tác không hợp LỆ
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "phải thêm TRƯỚC khi
hỏi" — MỘT trực giác hợp lý cho một số cấu trúc dữ liệu KHÁC.

Chỗ lệch: TOÀN bộ mục ĐÍCH của bloom filter LÀ trả lời cho những
khoá CHƯA biết có được thêm hay CHƯA — `coTheCoBloom` chỉ đơn thuần
kiểm TRA bit, không hề `throw`, hoạt động BÌNH thường với BẤT kỳ
chuỗi nào, kể cả chưa từng thêm.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không âm tính giả, có THỂ dương tính giả — bloom filter trả lời
NHANH câu hỏi "chắc chắn KHÔNG có" mà không cần quét gì. Ghép NÓ
với SSTable, khi nào thì DÙNG được?
::::

::::reflect{#nghi-lai}
Bloom filter đánh đổi ĐỘ chắc chắn lấy TỐC độ — trả lời gần như tức
THÌ bằng vài phép băm VÀ tra bit, không cần chạm VÀO dữ liệu thật.
Tính chất CỐT lõi: không BAO GIỜ báo sai kiểu "chắc chắn không có"
khi THẬT sự có (an TOÀN tuyệt đối theo hướng đó) — nhưng CÓ thể báo
sai kiểu "có thể có" khi thật sự KHÔNG có (chấp nhận ĐƯỢC, vì chỉ
tốn thêm MỘT lần kiểm tra thừa, không bao giờ BỎ sót dữ liệu thật).
Ghép NÓ với mỗi SSTable — hỏi bloom filter TRƯỚC khi quét, bỏ qua
NGAY những SSTable chắc chắn không CÓ — sẽ tiết kiệm được gì?
::::

::::checkpoint{mastery=0.8}
::::
