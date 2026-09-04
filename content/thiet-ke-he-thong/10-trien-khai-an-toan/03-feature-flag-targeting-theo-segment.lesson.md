---
id: thiet-ke-he-thong.trien-khai-an-toan.feature-flag-targeting-theo-segment
title: "Feature flag: bật/tắt độc lập với deploy, targeting theo segment"
summary: "kiemTraBat(flag, userId) tra ve true theo THU TU uu tien: batToanBo=true bat cho MOI nguoi; danhSachUserBat.has(userId) bat cho user CU THE; con lai dung hashOnDinh(userId) (tong ma ky tu mod 100, KHONG Math.random) so voi tyLePhanTramBat -- hash('an')=7 duoi 10% duoc bat, hash('binh')=17 tren 10% bi tat, CUNG userId luon ra CUNG ket qua qua nhieu lan goi. Bat/tat CHI doi field tren object, khong dong lai code."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.feature-flag-targeting-theo-segment]
requires: [sd.canary-tang-dan-theo-nguong-loi]
concepts: [sd.feature-flag-targeting-theo-segment]
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
Canary kiểm soát BAO NHIÊU traffic chạm code MỚI — nhưng đó LÀ kiểm
soát Ở cấp INSTANCE. Đôi khi cần kiểm soát Ở cấp TÍNH NĂNG: cùng một
bản deploy, một nhóm người dùng thấy nút MỚI, số còn lại thì KHÔNG —
VÀ đảo ngược quyết định đó không cần deploy lại BẤT CỨ dòng code nào.
::::

::::explain{#feature-flag-uu-tien}
`kiemTraBat` quyết định MỘT user cụ thể có thấy tính năng hay không,
theo đúng THỨ tự ưu tiên: `batToanBo=true` thắng TUYỆT đối; nếu
KHÔNG, user CÓ trong `danhSachUserBat` thì được bật; nếu KHÔNG NỮA,
dùng một hash ổn định của `userId` so với `tyLePhanTramBat`. Hash tự
viết — cộng dồn mã ký tự RỒI lấy dư cho `100` — KHÔNG dùng
`Math.random()`, vì phân bổ PHẢI lặp lại được y hệt mỗi lần gọi:

```typescript title=readonly
interface FeatureFlag {
  ten: string;
  batToanBo: boolean;
  danhSachUserBat: Set<string>;
  tyLePhanTramBat: number;
}

function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}

function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

const flag: FeatureFlag = {
  ten: "giao-dien-moi",
  batToanBo: false,
  danhSachUserBat: new Set(["vip-1"]),
  tyLePhanTramBat: 10,
};

console.log("vip-1 (trong danh sach rieng):", kiemTraBat(flag, "vip-1"));
console.log("an (hash 7, duoi 10%):", kiemTraBat(flag, "an"));
console.log("binh (hash 17, tren 10%):", kiemTraBat(flag, "binh"));
console.log("an lan 2 (goi lai, van la 'an'):", kiemTraBat(flag, "an"));
```

```text title=readonly
vip-1 (trong danh sach rieng): true
an (hash 7, duoi 10%): true
binh (hash 17, tren 10%): false
an lan 2 (goi lai, van la 'an'): true
```

`"vip-1"` được bật vì có TÊN trong `danhSachUserBat` — hash của nó
không hề được tính tới. `"an"` VÀ `"binh"` không nằm trong danh sách
riêng nào, nên rơi vào nhánh phần trăm: hash của `"an"` LÀ `7`, dưới
`tyLePhanTramBat=10`, được bật; hash của `"binh"` LÀ `17`, trên `10`,
bị tắt. Gọi lại `"an"` lần nữa cho CÙNG kết quả — không có gì ngẫu
nhiên trong hàm này.
::::

::::example{#bat-toan-bo-va-danh-sach-rong}
`batToanBo=true` bỏ QUA hoàn toàn cả danh sách riêng LẪN tỷ lệ phần
trăm — nó LÀ công tắc khẩn cấp. Ngược lại, `tyLePhanTramBat=0` nghĩa
LÀ KHÔNG ai lọt qua nhánh hash, TRỪ những user có TÊN trong danh sách
riêng:

```typescript title=readonly
interface FeatureFlag {
  ten: string;
  batToanBo: boolean;
  danhSachUserBat: Set<string>;
  tyLePhanTramBat: number;
}

function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}

function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

// batToanBo=true bo qua CA danh sach rieng lan ty le phan tram
const flagBatHet: FeatureFlag = {
  ten: "sua-loi-khan-cap",
  batToanBo: true,
  danhSachUserBat: new Set(),
  tyLePhanTramBat: 0,
};
console.log("binh, flag BAT TOAN BO (tyLePhanTramBat=0):", kiemTraBat(flagBatHet, "binh"));

// ty le phan tram bat = 0 nghia la KHONG ai duoc bat qua hash, tru danh sach rieng
const flagTat: FeatureFlag = {
  ten: "thu-nghiem-A",
  batToanBo: false,
  danhSachUserBat: new Set(),
  tyLePhanTramBat: 0,
};
console.log("an, flag ty le 0% (hash 7, khong con nao lot duoc):", kiemTraBat(flagTat, "an"));

// so sanh hai user voi CUNG mot flag ty le 15%: chi (hash 8) lot, duc (hash 16) khong
const flag15: FeatureFlag = {
  ten: "thu-nghiem-B",
  batToanBo: false,
  danhSachUserBat: new Set(),
  tyLePhanTramBat: 15,
};
console.log("chi (hash 8, duoi 15%):", kiemTraBat(flag15, "chi"));
console.log("duc (hash 16, tren 15%):", kiemTraBat(flag15, "duc"));
```

```text title=readonly
binh, flag BAT TOAN BO (tyLePhanTramBat=0): true
an, flag ty le 0% (hash 7, khong con nao lot duoc): false
chi (hash 8, duoi 15%): true
duc (hash 16, tren 15%): false
```

`flagBatHet` bật cho `"binh"` dù `tyLePhanTramBat` LÀ `0` — nhánh
`batToanBo` chặn NGAY từ dòng đầu, hai điều kiện phía sau không hề
được xét TỚI. `"an"` (hash `7`) bị tắt khi `tyLePhanTramBat=0` vì `7 <
0` LÀ `false` — không CÓ ngưỡng nào để lọt qua. Nâng ngưỡng lên `15`
cho `flag15`, `"chi"` (hash `8`) lọt qua, `"duc"` (hash `16`) thì
không — CÙNG cơ chế, khác NGƯỠNG, khác kết quả.
::::

::::predict{#doan-bien-hash commitOnce}
Một flag có `tyLePhanTramBat=17`. Hash của `"binh"` ĐÃ biết LÀ `17`
(từ ví dụ trước). Gọi `kiemTraBat(flag, "binh")` VỚI flag NÀY — kết
quả LÀ gì?

:::opt{correct}
`false` — điều kiện LÀ `hashOnDinh(userId) < flag.tyLePhanTramBat`
(nghiêm ngặt); `17 < 17` LÀ `false`, nên `"binh"` VẪN chưa lọt qua dù
hash trùng KHỚP với ngưỡng
:::
:::opt
`true` — hash của `"binh"` chạm ĐÚNG ngưỡng `17`, VÀ "chạm ngưỡng"
nên được tính LÀ "đủ điều kiện lọt qua"
::why
Nhầm "bằng ngưỡng" VỚI "dưới ngưỡng" — nhưng `kiemTraBat` dùng `<`
(nghiêm ngặt), KHÔNG phải `<=`.

Chỗ lệch: dòng cuối LÀ `return hashOnDinh(userId) < flag.tyLePhanTramBat;`.
Với `hashOnDinh("binh") = 17` VÀ `tyLePhanTramBat = 17`, biểu thức
LÀ `17 < 17`, cho `false`. Muốn `"binh"` lọt qua Ở đúng ngưỡng NÀY,
`tyLePhanTramBat` phải LỚN hơn `17` (ví dụ `18`), không phải bằng.
::
:::
::::

::::code{#viet_kiem_tra_bat}
Hoàn thiện `kiemTraBat` — theo ĐÚNG thứ tự ưu tiên: `batToanBo` trước,
`danhSachUserBat` kế tiếp, cuối cùng mới so hash với `tyLePhanTramBat`.

```typescript title=starter
interface FeatureFlag {
  ten: string;
  batToanBo: boolean;
  danhSachUserBat: Set<string>;
  tyLePhanTramBat: number;
}

function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}

function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  ___
}

const flagX: FeatureFlag = { ten: "x", batToanBo: false, danhSachUserBat: new Set(["vip-1"]), tyLePhanTramBat: 10 };
console.log(kiemTraBat(flagX, "vip-1"), kiemTraBat(flagX, "an"), kiemTraBat(flagX, "binh"));
```

```typescript title=solution
interface FeatureFlag {
  ten: string;
  batToanBo: boolean;
  danhSachUserBat: Set<string>;
  tyLePhanTramBat: number;
}

function hashOnDinh(chuoi: string): number {
  let tong = 0;
  for (let i = 0; i < chuoi.length; i++) tong += chuoi.charCodeAt(i);
  return tong % 100;
}

function kiemTraBat(flag: FeatureFlag, userId: string): boolean {
  if (flag.batToanBo) return true;
  if (flag.danhSachUserBat.has(userId)) return true;
  return hashOnDinh(userId) < flag.tyLePhanTramBat;
}

const flagX: FeatureFlag = { ten: "x", batToanBo: false, danhSachUserBat: new Set(["vip-1"]), tyLePhanTramBat: 10 };
console.log(kiemTraBat(flagX, "vip-1"), kiemTraBat(flagX, "an"), kiemTraBat(flagX, "binh"));
```

```typescript title=test
const flagBatHetT: FeatureFlag = { ten: "t1", batToanBo: true, danhSachUserBat: new Set(), tyLePhanTramBat: 0 };
if (kiemTraBat(flagBatHetT, "bat-ky-ai") !== true) throw new Error("batToanBo=true phai bat cho MOI userId, bat ke danh sach hay ty le");

const flagDanhSachT: FeatureFlag = { ten: "t2", batToanBo: false, danhSachUserBat: new Set(["nguoi-dac-biet"]), tyLePhanTramBat: 0 };
if (kiemTraBat(flagDanhSachT, "nguoi-dac-biet") !== true) throw new Error("user trong danhSachUserBat phai duoc bat, du ty le phan tram la 0");
if (kiemTraBat(flagDanhSachT, "nguoi-khac") !== false) throw new Error("user KHONG trong danh sach, ty le 0%, phai bi tat");

const flagTyLeT: FeatureFlag = { ten: "t3", batToanBo: false, danhSachUserBat: new Set(), tyLePhanTramBat: 10 };
if (kiemTraBat(flagTyLeT, "an") !== true) throw new Error("hash cua 'an' la 7, duoi nguong 10%, phai duoc bat");
if (kiemTraBat(flagTyLeT, "binh") !== false) throw new Error("hash cua 'binh' la 17, tren nguong 10%, phai bi tat");

const goiLan1T = kiemTraBat(flagTyLeT, "an");
const goiLan2T = kiemTraBat(flagTyLeT, "an");
if (goiLan1T !== goiLan2T) throw new Error("cung mot flag, cung mot userId phai luon ra CUNG mot ket qua (on dinh)");

const flagBienT: FeatureFlag = { ten: "t4", batToanBo: false, danhSachUserBat: new Set(), tyLePhanTramBat: 17 };
if (kiemTraBat(flagBienT, "binh") !== false) throw new Error("hash cua 'binh' DUNG BANG nguong (17 = 17) khong duoc tinh la lot qua -- dieu kien phai la < khong phai <=");
```

:::hints
- kind: attention
  body: "Ba dieu kien theo dung thu tu: neu flag.batToanBo thi return true; neu flag.danhSachUserBat.has(userId) thi return true; nguoc lai return hashOnDinh(userId) < flag.tyLePhanTramBat."
- kind: strategy
  body: "if (flag.batToanBo) return true; if (flag.danhSachUserBat.has(userId)) return true; return hashOnDinh(userId) < flag.tyLePhanTramBat;"
- kind: one-line
  body: "if (flag.batToanBo) return true; if (flag.danhSachUserBat.has(userId)) return true; return hashOnDinh(userId) < flag.tyLePhanTramBat;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bật/tắt tức thì, không cần deploy — VÀ ổn định cho từng user. Nhưng
feature flag giả định traffic ĐÃ tới được instance đó. Nếu instance
CHƯA sẵn sàng — mới khởi động, cache còn RỖNG — thì dù flag bật cỡ
nào, request vẫn không nên rơi vào nó.
::::

::::reflect{#nghi-lai}
`kiemTraBat` không hề gọi `Math.random()` — MỌI quyết định đều suy ra
từ dữ liệu ĐÃ có: cờ toàn cục, một `Set`, VÀ một hash tất định của
chính `userId`. Chính vì tất định, CÙNG user luôn nhận CÙNG câu trả
lời dù gọi bao nhiêu lần, trên bao nhiêu request khác nhau — đây LÀ
điều kiện bắt buộc để một feature flag không LÀM trải nghiệm của một
người dùng nhấp NHÁY giữa "thấy" VÀ "không thấy" tính năng.
::::

::::checkpoint{mastery=0.75}
::::
