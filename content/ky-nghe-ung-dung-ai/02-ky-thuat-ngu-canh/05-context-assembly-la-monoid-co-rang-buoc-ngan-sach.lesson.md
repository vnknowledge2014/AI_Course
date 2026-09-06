---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.context-assembly-la-monoid-co-rang-buoc-ngan-sach
title: "Cầu nối FP — lắp ngữ cảnh là Monoid có ràng buộc ngân sách"
summary: "Cầu nối FP (dựa trên chapter-29-monoids-algebra.chapter.md): type ManhNguCanh = { noiDung: string; token: number } là một \"mảnh\" ngữ cảnh; ganMonoid(a, b): ManhNguCanh nối noiDung + cộng token (phép concat của Semigroup); NGU_CANH_RONG = { noiDung: \"\", token: 0 } là phần tử đơn vị (empty) của Monoid — ganMonoid KHÔNG ràng buộc ngân sách vẫn kết hợp được (associative) như bất kỳ Monoid nào. ganNhieuManhCoNganSach(cacManh: ManhNguCanh[], nganSach: number): ManhNguCanh gấp qua ganMonoid NHƯNG DỪNG LẠI một khi token đã đạt ngưỡng — với 3 mảnh A/B/C (mỗi mảnh 5 token, ngân sách 8): gộp RIÊNG LẺ dừng ở AB/10 token; nhưng nếu B+C được GỘP TRƯỚC thành một khối rồi mới đưa vào cùng thuật toán, kết quả lại LÀ ABC/15 token — CÙNG ba mảnh, CÙNG thứ tự, chỉ khác cách NHÓM, cho hai kết quả khác nhau: ràng buộc ngân sách phá vỡ tính kết hợp thuần tuý của Monoid."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.context-assembly-la-monoid-co-rang-buoc-ngan-sach]
requires: [kna.lap-ngu-canh-theo-uu-tien]
concepts: [kna.context-assembly-la-monoid-co-rang-buoc-ngan-sach]
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
Bốn bài trước LUÔN làm MỘT việc, dưới nhiều tên khác nhau: ghép các
mảnh nội dung lại thành một cửa sổ, cộng dồn token của chúng. Đại số
trừu tượng có một cái TÊN cho chính xác phép toán này: Monoid. Đặt
đúng tên cho nó không phải LÀ trang trí — nó lộ ra một sự thật quan
trọng: ràng buộc NGÂN SÁCH phá vỡ đúng thứ TÍNH CHẤT làm Monoid hữu
ích.
::::

::::explain{#manh-ngu-canh-va-monoid}
Một "mảnh" ngữ cảnh (`ManhNguCanh`) LÀ bất cứ khối nội dung nào sẽ được
LẮP vào cửa sổ — một message đã format, một đoạn tóm tắt, MỘT phần của
cửa sổ đã cắt Ở các bài trước. Nó có nội dung VÀ số token của chính nó:

```typescript title=readonly
type ManhNguCanh = { noiDung: string; token: number };

function ganMonoid(a: ManhNguCanh, b: ManhNguCanh): ManhNguCanh {
  return { noiDung: a.noiDung + b.noiDung, token: a.token + b.token };
}

const NGU_CANH_RONG: ManhNguCanh = { noiDung: "", token: 0 };
```

Đây LÀ đúng hình dạng một **Monoid**: `ganMonoid` LÀ phép kết hợp
(giống `concat` của Semigroup — SGK FP gọi phép NÀY LÀ `concat`, quest
này gọi LÀ `ganMonoid` để rõ đúng ngữ cảnh), `NGU_CANH_RONG` LÀ phần tử
ĐƠN VỊ (`empty`): gộp với nó thì giữ nguyên. Yêu cầu DUY NHẤT của một
Monoid LÀ phép kết hợp phải **associative** — nhóm theo cách nào cũng
ra CÙNG kết quả: `ganMonoid(ganMonoid(a, b), c)` PHẢI bằng
`ganMonoid(a, ganMonoid(b, c))`. `ganMonoid` KHÔNG ràng buộc ngân sách
THOẢ MÃN tính chất này — nó chỉ nối chuỗi VÀ cộng số, hai phép toán
vốn ĐÃ associative.
::::

::::example{#gap-co-nganh-sach-pha-vo-ket-hop}
`ganNhieuManhCoNganSach` gấp (fold) qua `ganMonoid` NHƯNG dừng lại MỘT
KHI token đã đạt ngưỡng — đây CHÍNH LÀ "ràng buộc ngân sách":

```typescript title=readonly
type ManhNguCanh = { noiDung: string; token: number };

function ganMonoid(a: ManhNguCanh, b: ManhNguCanh): ManhNguCanh {
  return { noiDung: a.noiDung + b.noiDung, token: a.token + b.token };
}

const NGU_CANH_RONG: ManhNguCanh = { noiDung: "", token: 0 };

function ganNhieuManhCoNganSach(cacManh: ManhNguCanh[], nganSach: number): ManhNguCanh {
  let ketQua = NGU_CANH_RONG;
  for (const manh of cacManh) {
    if (ketQua.token >= nganSach) break;
    ketQua = ganMonoid(ketQua, manh);
  }
  return ketQua;
}

const MANH_A: ManhNguCanh = { noiDung: "A", token: 5 };
const MANH_B: ManhNguCanh = { noiDung: "B", token: 5 };
const MANH_C: ManhNguCanh = { noiDung: "C", token: 5 };

const rieng = ganNhieuManhCoNganSach([MANH_A, MANH_B, MANH_C], 8);
const gopTruoc = ganNhieuManhCoNganSach([MANH_A, ganMonoid(MANH_B, MANH_C)], 8);
console.log(JSON.stringify(rieng));
console.log(JSON.stringify(gopTruoc));
```

```text title=readonly
{"noiDung":"AB","token":10}
{"noiDung":"ABC","token":15}
```

CÙNG ba mảnh `A`, `B`, `C` (mỗi mảnh `5` token), CÙNG ngân sách `8`,
CÙNG thứ tự — nhưng NHÓM khác nhau. Gộp RIÊNG LẺ từng mảnh: sau `A`
(token `5`, chưa đạt `8`) gộp tiếp `B` → `AB`/`10` (ĐÃ đạt `8`), DỪNG
trước `C`. Nhưng nếu `B` VÀ `C` được gộp THÀNH MỘT KHỐI (`"BC"`, `10`
token) TRƯỚC khi đưa vào thuật toán: sau `A` (token `5`, chưa đạt `8`)
KHÔNG có điểm dừng nào Ở GIỮA khối `BC` — toàn bộ khối `10` token đó
được gộp TRỌN VẸN trong một bước, ra `ABC`/`15`, VƯỢT xa ngân sách `8`.
Đây LÀ ràng buộc ngân sách phá vỡ tính KẾT HỢP: một Monoid THẬT phải
cho CÙNG kết quả dù nhóm kiểu gì — `ganNhieuManhCoNganSach` thì KHÔNG.
::::

::::predict{#doan-nhom-a-b-truoc commitOnce}
Thay vì gộp `B` VÀ `C` trước, giờ gộp `A` VÀ `B` THÀNH MỘT khối trước:
`ganNhieuManhCoNganSach([ganMonoid(MANH_A, MANH_B), MANH_C], 8)`. Kết
quả có KHÁC với `ganNhieuManhCoNganSach([MANH_A, MANH_B, MANH_C], 8)`
(gộp riêng lẻ, LÀ `AB`/`10`) không?

:::opt{correct}
KHÔNG khác — CẢ HAI đều cho `AB`/`10`: điểm kiểm ngân sách TRƯỚC khi
gộp khối `AB` (token `10`) LÀ `ketQua.token = 0`, CHƯA đạt `8`, nên
khối `AB` VẪN được gộp TRỌN VẸN trong một bước — kết quả GIỐNG HỆT như
gộp `A` rồi `B` riêng lẻ, vì điểm DỪNG (sau khi đạt `AB`) rơi ĐÚNG vào
CHỖ mà cả hai cách nhóm đều đồng ý dừng
:::
:::opt
CÓ khác — bất kỳ cách nhóm lại nào (gộp `A+B` trước, hay `B+C` trước)
đều phá vỡ tính kết hợp GIỐNG NHAU, vì `ganNhieuManhCoNganSach` vốn đã
không associative
::why
Nhầm "không associative NÓI CHUNG" VỚI "MỌI cách nhóm cụ thể đều cho
kết quả khác" — nhưng ví dụ `A`+`B` gộp trước lại TÌNH CỜ rơi đúng vào
điểm mà thuật toán ĐÃ dừng khi xử lý riêng lẻ.

Chỗ lệch: tính không-kết-hợp CHỈ lộ ra khi khối được gộp TRƯỚC "nuốt
trọn" luôn điểm dừng lẽ ra phải xảy ra Ở GIỮA khối đó — như trường hợp
`B+C` (ví dụ Ở phần trên): điểm dừng đáng lẽ Ở GIỮA `B` và `C` (sau khi
đạt `AB`/`10`), nhưng khối `BC` đã gộp SẴN không cho phép dừng Ở giữa
nó. Với `A+B`, không có điểm dừng nào đáng lẽ rơi vào GIỮA hai mảnh đó.
::
:::
:::opt
Không xác định được — kết quả phụ thuộc thứ tự CHẠY thực tế của
JavaScript, không đoán trước được
::why
Gần đúng Ở việc bạn nghi ngờ có yếu tố "không đoán trước được" nào đó —
nhưng `ganNhieuManhCoNganSach` LÀ một vòng `for` TUẦN TỰ, tất định
hoàn toàn, không có bất kỳ điều gì phụ thuộc thời điểm chạy thực tế.

Chỗ lệch: kết quả HOÀN TOÀN xác định được TRƯỚC — chỉ cần lần theo
đúng thứ tự `for` xử lý từng mảnh trong `cacManh`, so `ketQua.token`
với `nganSach` Ở đầu MỖI vòng lặp.
::
:::
::::

::::code{#viet_gan_monoid_co_nganh_sach}
Hoàn thiện `ganMonoid` — trả về một `ManhNguCanh` mới, nối `noiDung`
VÀ cộng `token`. Hoàn thiện `ganNhieuManhCoNganSach` — gấp qua
`ganMonoid` bắt đầu từ `NGU_CANH_RONG`, DỪNG LẠI (không gộp thêm mảnh
nào) MỘT KHI `ketQua.token` đã đạt/vượt `nganSach`.

```typescript title=starter
type ManhNguCanh = { noiDung: string; token: number };

function ganMonoid(a: ManhNguCanh, b: ManhNguCanh): ManhNguCanh {
  ___
}

const NGU_CANH_RONG: ManhNguCanh = { noiDung: "", token: 0 };

function ganNhieuManhCoNganSach(cacManh: ManhNguCanh[], nganSach: number): ManhNguCanh {
  ___
}

const MANH_A: ManhNguCanh = { noiDung: "A", token: 5 };
const MANH_B: ManhNguCanh = { noiDung: "B", token: 5 };
const MANH_C: ManhNguCanh = { noiDung: "C", token: 5 };

const rieng = ganNhieuManhCoNganSach([MANH_A, MANH_B, MANH_C], 8);
const gopTruoc = ganNhieuManhCoNganSach([MANH_A, ganMonoid(MANH_B, MANH_C)], 8);
console.log(JSON.stringify(rieng), JSON.stringify(gopTruoc));
```

```typescript title=solution
type ManhNguCanh = { noiDung: string; token: number };

function ganMonoid(a: ManhNguCanh, b: ManhNguCanh): ManhNguCanh {
  return { noiDung: a.noiDung + b.noiDung, token: a.token + b.token };
}

const NGU_CANH_RONG: ManhNguCanh = { noiDung: "", token: 0 };

function ganNhieuManhCoNganSach(cacManh: ManhNguCanh[], nganSach: number): ManhNguCanh {
  let ketQua = NGU_CANH_RONG;
  for (const manh of cacManh) {
    if (ketQua.token >= nganSach) break;
    ketQua = ganMonoid(ketQua, manh);
  }
  return ketQua;
}

const MANH_A: ManhNguCanh = { noiDung: "A", token: 5 };
const MANH_B: ManhNguCanh = { noiDung: "B", token: 5 };
const MANH_C: ManhNguCanh = { noiDung: "C", token: 5 };

const rieng = ganNhieuManhCoNganSach([MANH_A, MANH_B, MANH_C], 8);
const gopTruoc = ganNhieuManhCoNganSach([MANH_A, ganMonoid(MANH_B, MANH_C)], 8);
console.log(JSON.stringify(rieng), JSON.stringify(gopTruoc));
```

```typescript title=test
if (JSON.stringify(ganMonoid(NGU_CANH_RONG, MANH_A)) !== JSON.stringify(MANH_A)) throw new Error("gan voi phan tu rong ben trai phai giu nguyen (identity)");
if (JSON.stringify(ganMonoid(MANH_A, NGU_CANH_RONG)) !== JSON.stringify(MANH_A)) throw new Error("gan voi phan tu rong ben phai phai giu nguyen (identity)");
if (JSON.stringify(ganMonoid(ganMonoid(MANH_A, MANH_B), MANH_C)) !== JSON.stringify(ganMonoid(MANH_A, ganMonoid(MANH_B, MANH_C)))) {
  throw new Error("ganMonoid KHONG rang buoc ngan sach phai KET HOP duoc (associative)");
}

if (JSON.stringify(rieng) !== JSON.stringify({ noiDung: "AB", token: 10 })) throw new Error("gop rieng le toi nguong 8 phai dung lai o AB/10");
if (JSON.stringify(gopTruoc) !== JSON.stringify({ noiDung: "ABC", token: 15 })) throw new Error("gop B+C truoc thanh mot khoi roi moi gan tiep phai cho ABC/15 -- KHAC voi gop rieng le, du CUNG ba manh CUNG thu tu");
if (JSON.stringify(ganNhieuManhCoNganSach([], 10)) !== JSON.stringify(NGU_CANH_RONG)) throw new Error("danh sach rong phai tra ve dung phan tu don vi");

if (JSON.stringify(ganNhieuManhCoNganSach([MANH_A, MANH_B, MANH_C], 15)) !== JSON.stringify({ noiDung: "ABC", token: 15 })) throw new Error("nganSach 15 (du cho ca ba manh) phai gop het ca A, B, C");
if (JSON.stringify(ganNhieuManhCoNganSach([MANH_A, MANH_B, MANH_C], 3)) !== JSON.stringify({ noiDung: "A", token: 5 })) throw new Error("nganSach 3 (nho hon ca mot manh) phai dung lai ngay sau A -- doi nganSach phai doi ket qua");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (ganMonoid): return mot object moi { noiDung: a.noiDung + b.noiDung, token: a.token + b.token }. Cho hai (ganNhieuManhCoNganSach): khoi tao ketQua = NGU_CANH_RONG, roi mot vong for-of qua tung manh trong cacManh -- neu ketQua.token >= nganSach thi break (dung lai NGAY, KHONG gan them), nguoc lai gan ketQua = ganMonoid(ketQua, manh); cuoi cung return ketQua."
- kind: strategy
  body: "Cho dau: return { noiDung: a.noiDung + b.noiDung, token: a.token + b.token }; Cho hai: let ketQua = NGU_CANH_RONG; for (const manh of cacManh) { if (ketQua.token >= nganSach) break; ketQua = ganMonoid(ketQua, manh); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"noiDung\":\"AB\",\"token\":10} {\"noiDung\":\"ABC\",\"token\":15}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`AB`/`10` hay `ABC`/`15` — CÙNG ba mảnh, CÙNG thứ tự, khác Ở cách NHÓM
trước khi gấp. Ràng buộc ngân sách LÀ thứ phá vỡ tính kết hợp thuần
tuý mà một Monoid lẽ ra PHẢI có. BOSS của quest này ráp lại toàn bộ
năm bài — đo trên MỘT phiên hội thoại nhiều lượt thật sự, không phải
một ví dụ tĩnh.
::::

::::reflect{#nghi-lai}
`ganMonoid` VÀ `NGU_CANH_RONG` LÀ một Monoid THẬT SỰ — kết hợp được,
có đơn vị, đúng y hệt Monoid `number`/`string`/`array` Ở
`chapter-29-monoids-algebra`. Nhưng NGAY khi thêm MỘT ràng buộc thực
tế (ngân sách token hữu hạn) vào phép gấp, tính chất đẹp đẽ nhất của
Monoid — kết hợp theo cách nào cũng ra cùng kết quả — biến mất. Đây
không phải LÀ một lỗi thiết kế của `ganNhieuManhCoNganSach` — đây LÀ
đúng bản chất của "lắp ngữ cảnh có ngân sách" trong hệ thống thật: nó
là một phép gấp CÓ TRẠNG THÁI (biết được đã gộp bao nhiêu token), và
mọi phép gấp có trạng thái phụ thuộc THỨ TỰ xử lý từng phần tử — không
còn LÀ một Monoid thuần tuý nữa, dù được xây trên một Monoid thuần tuý
Ở bên trong.
::::

::::checkpoint{mastery=0.85}
::::
