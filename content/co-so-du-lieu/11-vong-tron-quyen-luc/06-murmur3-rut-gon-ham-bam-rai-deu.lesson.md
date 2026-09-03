---
id: co-so-du-lieu.vong-tron-quyen-luc.murmur3-rut-gon-ham-bam-rai-deu
title: "murmur3 (rút gọn) — hàm băm rải đều"
summary: "bamTe(s) = charCodeAt(0) % 1000 — CHỈ nhìn KÝ TỰ đầu — khiến 'binh', 'bich', 'bao', 'ba' (cùng bắt đầu bằng 'b') dồn CỤM vào ĐÚNG một vị trí (98) trên vòng, dù là bốn khoá khác nhau hoàn toàn. bam (dùng xuyên suốt bài 1-5) là một phiên bản RÚT GỌN của kỹ thuật murmur3 thật — XOR-nhân từng ký tự RỒI một bước 'avalanche' cuối cùng trộn lại mọi bit — rải bốn khoá đó ra bốn vị trí khác hẳn nhau (200/643/492/741)."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.murmur3-rut-gon]
requires: [db.them-bot-node-tren-vong]
concepts: [db.murmur3-rut-gon]
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
`bam` (dùng xuyên suốt bài 1-5) rải khoá ra vòng đủ ĐỀU để mọi ví dụ
Ở trên hoạt động ĐÚNG. Một hàm băm TỆ trông ra sao?
::::

::::explain{#ham-bam-te}
`bamTe` CHỈ nhìn đúng KÝ tự đầu tiên của chuỗi:

```typescript title=readonly
function bamTe(s: string): number {
  return s.charCodeAt(0) % 1000;
}

console.log(bamTe("binh"));
console.log(bamTe("bich"));
console.log(bamTe("bao"));
console.log(bamTe("ba"));
```

```text title=readonly
98
98
98
98
```

BỐN khoá KHÁC nhau hoàn toàn — `"binh"`, `"bich"`, `"bao"`, `"ba"`
— nhưng CÙNG bắt đầu BẰNG chữ `'b'`, nên `bamTe` cho ra ĐÚNG cùng
MỘT giá trị: `98`. Trên một vòng consistent hashing, CẢ bốn khoá NÀY
sẽ dồn VÀO đúng MỘT node — một "điểm NÓNG" giả tạo, không PHẢI vì
node ĐÓ thực sự cần xử LÝ nhiều dữ liệu hơn, mà chỉ VÌ hàm băm quá
YẾU.
::::

::::example{#bam-la-mot-phien-ban-rut-gon-cua-murmur3}
`bam` (bài 1-5) mạnh HƠN hẳn — trên ĐÚNG bốn khoá vừa THẤY:

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

console.log(bam("binh"));
console.log(bam("bich"));
console.log(bam("bao"));
console.log(bam("ba"));
```

```text title=readonly
200
643
492
741
```

BỐN giá trị hoàn toàn KHÁC nhau. `bam` LÀM hai việc `bamTe` KHÔNG
làm: (1) VÒNG lặp `for` gộp TỪNG ký tự VÀO `h` (không chỉ ký tự
đầu), VÀ (2) năm dòng CUỐI (dịch bit, NHÂN với hằng số) LÀ một bước
"avalanche" — trộn LẠI mọi bit của `h` sao cho một THAY đổi nhỏ Ở
đầu VÀO (VÍ dụ đổi đúng MỘT ký tự) lan RA khắp kết quả. Đây chính LÀ
kỹ thuật CỐT lõi murmur3 THẬT dùng (nhân VỚI hằng số LỚN, dịch bit,
XOR, lặp lại) — `bam` LÀ một bản RÚT gọn của Ý tưởng đó, KHÔNG phải
cài đặt murmur3 chuẩn XÁC (murmur3 thật xử LÝ khối 32-bit, seed, VÀ
nhiều hằng số khác) — đủ để MINH hoạ vì sao bước "trộn" LẠI quan
trọng, không phải để dùng trong một hệ THẬT.
::::

::::predict{#doan-hai-khoa-mot-ky-tu commitOnce}
Hai khoá CHỈ khác nhau ĐÚNG một ký tự CUỐI: `"nguoidungA"` VÀ
`"nguoidungB"`. VỚI `bamTe` (chỉ nhìn ký tự ĐẦU), hai khoá NÀY có
LUÔN cho CÙNG kết quả không?

:::opt{correct}
CÓ, LUÔN giống nhau — `bamTe` chỉ đọc `s.charCodeAt(0)`, VÀ cả hai
chuỗi đều BẮT đầu bằng `"nguoidung"` (giống hệt NHAU Ở ký tự đầu)
:::

:::opt
KHÔNG — hai chuỗi CÓ độ dài BẰNG nhau nhưng nội DUNG khác Ở CUỐI,
`% 1000` sẽ phản ánh sự khác BIỆT đó
::why
Gần đúng ở việc bạn nghĩ TỚI "khác Ở đâu ĐÓ trong chuỗi thì kết quả
PHẢI khác" — MỘT kỳ vọng hợp lý cho một hàm băm TỐT (VÀ đúng VỚI
`bam`, xem bài NÀY Ở trên).

Chỗ lệch: `bamTe` CHỈ đọc DUY nhất `s.charCodeAt(0)` — KÝ tự đầu
tiên — RỒI `% 1000`. KÝ tự thứ hai TRỞ đi (bao gồm CHỖ hai chuỗi
thực sự khác NHAU, ký tự cuối `'A'`/`'B'`) hoàn toàn KHÔNG được đọc
tới. Hai chuỗi CÙNG ký tự đầu (`'n'`) LUÔN cho ĐÚNG cùng một kết
quả, bất kể PHẦN còn lại của chuỗi LÀ gì.
::
:::
::::

::::code{#viet_bam_te}
Hoàn thiện `bamTe` — CHỈ đọc ký tự ĐẦU tiên của chuỗi.

```typescript title=starter
function bamTe(s: string): number {
  return ___;
}

console.log(bamTe("binh"));
console.log(bamTe("bich"));
```

```typescript title=solution
function bamTe(s: string): number {
  return s.charCodeAt(0) % 1000;
}

console.log(bamTe("binh"));
console.log(bamTe("bich"));
```

```typescript title=test
console.log(bamTe("binh"));
if (bamTe("binh") !== 98) throw new Error("bamTe('binh') phai la 98");
if (bamTe("bich") !== bamTe("binh")) throw new Error("bich va binh cung bat dau bang 'b', PHAI trung gia tri (minh hoa dong cum)");
if (bamTe("bao") !== bamTe("binh")) throw new Error("bao va binh cung bat dau bang 'b', PHAI trung gia tri");
if (bamTe("an") === bamTe("binh")) throw new Error("an va binh khac chu cai dau ('a' khac 'b'), KHONG duoc trung");

if (bamTe("ếch") !== 871) throw new Error("bamTe('ếch') phai la 871 -- ky tu 'ế' co ma Unicode 7871, PHAI thuc su chia lay du cho 1000");
```

:::hints
- kind: attention
  body: "Doc ma ky tu CUA VI TRI DAU TIEN trong chuoi (charCodeAt(0)), roi lay phan du chia cho 1000 -- mot dong."
- kind: strategy
  body: "s.charCodeAt(0) % 1000"
- kind: one-line
  body: "return s.charCodeAt(0) % 1000;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "98"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hàm băm TỐT đủ rải đều — nhưng liệu CHỈ vậy đã đủ để chia tải CÂN
bằng giữa các node?
::::

::::reflect{#nghi-lai}
`bamTe` LÀ một hàm băm hoàn TOÀN hợp lệ về MẶT cú pháp (nhận chuỗi,
trả VỀ số trong `[0,999]`) — cái THIẾU không phải "đúng SAI", mà LÀ
"đủ TRỘN": nó bỏ QUA gần hết thông tin trong đầu VÀO. `bam` khá hơn
chính XÁC vì nó ĐỌC toàn bộ chuỗi VÀ có một bước "avalanche" khuếch
tán THAY đổi nhỏ ra khắp kết QUẢ — kỹ thuật CỐT lõi murmur3 thật
dùng. NHƯNG "hàm băm tốt" mới chỉ LÀ một nửa CÂU chuyện — với CHỈ
BỐN node, MỖI node một điểm DUY nhất trên vòng, phân bố CÓ thực sự
cân bằng không?
::::

::::checkpoint{mastery=0.8}
::::
