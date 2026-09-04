---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.snowflake-dong-goi-bit
title: "Snowflake ID: đóng gói bit"
summary: "ghepSnowflakeId ghép timestamp(41 bit, dịch 22)+datacenterId(5 bit, dịch 17)+machineId(5 bit, dịch 12)+sequence(12 bit) thành một bigint 64-bit (bit dấu luôn 0 ngầm định). Với ms=1.700.000.000.000 (thực tế, còn trong 41 bit) VÀ sequence=777: ID kết quả (7130316800000430857) VƯỢT Number.MAX_SAFE_INTEGER -- ép về number rồi ép lại bigint làm SAI LỆCH đúng 247 đơn vị, chứng minh bigint là bắt buộc chứ không phải tuỳ chọn phong cách."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.snowflake-dong-goi-bit]
requires: [sd.sliding-window-counter]
concepts: [sd.snowflake-dong-goi-bit]
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
Rate limiting xong — bốn thuật toán, một mục ĐÍCH: quyết định request
NÀO được QUA. Nửa sau quest chuyển sang bài toán khác hẳn: sinh một
ĐỊNH DANH duy nhất cho MỖI request được qua, không TRÙNG, không cần
hỏi máy khác.
::::

::::explain{#dong-goi-bit}
Snowflake ID (kiểu Twitter) nhét BỐN mảnh thông tin VÀO đúng một số
`64-bit`: bit dấu (luôn `0`, ngầm định — không CẦN gán tay), timestamp
(`41` bit), datacenter ID (`5` bit, tối đa `32` giá trị), machine ID
(`5` bit, tối đa `32`), sequence (`12` bit, tối đa `4096`). Ghép bằng
DỊCH bit (`<<`) rồi HOẶC bit (`|`) — PHẢI dùng `bigint`, vì kết quả VƯỢT
xa những gì `number` biểu diễn CHÍNH xác được:

```typescript title=readonly
const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;

function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

const id1 = ghepSnowflakeId(123456789n, 3n, 7n, 0n);
console.log("id1 (ms=123456789, dc=3, machine=7, seq=0):", String(id1));
console.log("id1 dang nhi phan (64 bit):", id1.toString(2).padStart(64, "0"));

const id2 = ghepSnowflakeId(123456789n, 3n, 7n, 1n);
console.log("id2 (chi doi seq: 0 -> 1):", String(id2));
console.log("id2 - id1 (chenh lech dung bang 1, vi seq la 12 bit thap nhat):", String(id2 - id1));

const id3 = ghepSnowflakeId(123456790n, 3n, 7n, 0n);
console.log("id3 (ms tang 1, seq ve 0):", String(id3));
console.log("id3 lon hon id1 (ID tang theo thoi gian):", id3 > id1);
```

```text title=readonly
id1 (ms=123456789, dc=3, machine=7, seq=0): 517815304351744
id1 dang nhi phan (64 bit): 0000000000000001110101101111001101000101010001100111000000000000
id2 (chi doi seq: 0 -> 1): 517815304351745
id2 - id1 (chenh lech dung bang 1, vi seq la 12 bit thap nhat): 1
id3 (ms tang 1, seq ve 0): 517815308546048
id3 lon hon id1 (ID tang theo thoi gian): true
```

`sequence` chiếm `12` bit THẤP nhất — đổi CHỈ `sequence` (từ `0` lên
`1`) làm ID tăng ĐÚNG `1`, không hề đụng chạm tới CÁC phần khác. Tăng
`msTuEpoch` (timestamp) LUÔN cho ID LỚN hơn, dù `datacenterId` VÀ
`machineId` giữ nguyên — VÌ timestamp nằm Ở nhóm bit CAO nhất (dịch
`22`), chi phối GIÁ trị tổng nhiều hơn HẲN mọi phần khác cộng lại.
::::

::::example{#vi-sao-can-bigint}
Với `msTuEpoch` NHỎ (như `123456789` ở TRÊN), kết quả VẪN còn trong
tầm `Number.MAX_SAFE_INTEGER`. Nhưng VỚI mốc thời gian THỰC tế (hàng
nghìn tỷ mili-giây), ID vượt xa giới HẠN đó — dùng `number` thay VÌ
`bigint` gây MẤT chính xác THẬT SỰ, không phải lý thuyết SUÔNG:

```typescript title=readonly
const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;

function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

// mot moc thoi gian THUC TE (hang ty mili-giay tu mot epoch tuy chon) -- khong phai so nho nhu vi du truoc
const msThucTe = 1700000000000n;
const idThuc = ghepSnowflakeId(msThucTe, 3n, 9n, 777n);
console.log("id voi ms thuc te lon:", String(idThuc));
console.log("Number.MAX_SAFE_INTEGER:", Number.MAX_SAFE_INTEGER);
console.log("id nay VUOT QUA MAX_SAFE_INTEGER:", idThuc > BigInt(Number.MAX_SAFE_INTEGER));

// minh chung mat chinh xac neu lo dung 'number' thay vi 'bigint'
const idBangNumber = Number(idThuc);
const idBangNumberVeLaiBigint = BigInt(idBangNumber);
console.log("id goc (bigint):", String(idThuc));
console.log("id sau khi ep ve number roi ep lai bigint:", String(idBangNumberVeLaiBigint));
console.log("hai gia tri co con GIONG NHAU khong (co mat chinh xac khong):", idThuc === idBangNumberVeLaiBigint);
console.log("chenh lech (id goc - id qua number):", String(idThuc - idBangNumberVeLaiBigint));
```

```text title=readonly
id voi ms thuc te lon: 7130316800000430857
Number.MAX_SAFE_INTEGER: 9007199254740991
id nay VUOT QUA MAX_SAFE_INTEGER: true
id goc (bigint): 7130316800000430857
id sau khi ep ve number roi ep lai bigint: 7130316800000431104
hai gia tri co con GIONG NHAU khong (co mat chinh xac khong): false
chenh lech (id goc - id qua number): -247
```

ID thật (`7130316800000430857`) LỚN hơn HẲN `Number.MAX_SAFE_INTEGER`
(`9007199254740991`) — chênh nhau CẢ nghìn lần. Ép nó VỀ `number` rồi
ép LẠI `bigint` cho ra một GIÁ trị KHÁC hẳn (`...431104` thay VÌ
`...430857`) — sai lệch ĐÚNG `247` đơn vị. Đây KHÔNG phải rủi ro lý
thuyết: bất KỲ hệ thống nào lỡ dùng `number` cho Snowflake ID SẼ tạo
ra ID trùng LẶP hoặc sai lệch NGAY trong sản xuất thật.
::::

::::predict{#doan-doi-datacenter-id commitOnce}
Hai lệnh gọi `ghepSnowflakeId` giống HỆT nhau về `msTuEpoch`,
`machineId`, VÀ `sequence`, chỉ khác `datacenterId` (một LÀ `0`, một
LÀ `1`). Sự khác biệt GIỮA hai ID kết quả LÀ bao nhiêu?

:::opt{correct}
`131072` (`= 2^17`) — `datacenterId` được dịch TRÁI đúng `17` bit
(`SO_BIT_MACHINE + SO_BIT_SEQUENCE = 5+12`), nên đổi nó TỪ 0 lên 1
cộng thêm đúng `2^17` vào giá trị TỔNG
:::
:::opt
`1` — VÌ chỉ đổi đúng MỘT tham số (`datacenterId` từ `0` lên `1`),
kết quả cũng CHỈ lệch đúng `1` đơn vị, giống hệt trường HỢP đổi
sequence Ở phần readonly TRÊN
::why
Nhầm "đổi giá trị THAM SỐ đi đúng 1" VỚI "đổi giá trị KẾT QUẢ đi đúng
1" — nhưng MỖI tham số nằm Ở một VỊ TRÍ bit khác nhau trong số `64-bit`
cuối cùng, không phải CỘNG trực tiếp vào tổng.

Chỗ lệch: `sequence` nằm Ở `12` bit THẤP nhất (dịch `0`), nên đổi nó
đi `1` LÀM tổng đổi đi đúng `1`. Nhưng `datacenterId` bị dịch TRÁI
`DICH_DATACENTER = 17` bit TRƯỚC khi OR VÀO — đổi nó từ `0` lên `1`
tương đương CỘNG thêm `1 << 17 = 131072` vào tổng, không phải `1`.
::
:::
::::

::::code{#viet_ghep_snowflake_id}
Hoàn thiện `ghepSnowflakeId` — dịch trái MỖI phần TỚI đúng vị trí bit
của nó (`DICH_TIMESTAMP`, `DICH_DATACENTER`, `DICH_MACHINE`), rồi HOẶC
(`|`) tất cả LẠI với `sequence` (đã Ở đúng vị trí thấp nhất).

```typescript title=starter
const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;

function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  ___
}

console.log(String(ghepSnowflakeId(1n, 0n, 0n, 0n)));
```

```typescript title=solution
const SO_BIT_SEQUENCE = 12n;
const SO_BIT_MACHINE = 5n;
const SO_BIT_DATACENTER = 5n;
const DICH_MACHINE = SO_BIT_SEQUENCE;
const DICH_DATACENTER = SO_BIT_MACHINE + SO_BIT_SEQUENCE;
const DICH_TIMESTAMP = SO_BIT_DATACENTER + SO_BIT_MACHINE + SO_BIT_SEQUENCE;

function ghepSnowflakeId(msTuEpoch: bigint, datacenterId: bigint, machineId: bigint, sequence: bigint): bigint {
  return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;
}

console.log(String(ghepSnowflakeId(1n, 0n, 0n, 0n)));
```

```typescript title=test
if (ghepSnowflakeId(0n, 0n, 0n, 0n) !== 0n) throw new Error("tat ca bang 0 phai cho ID = 0n");
if (ghepSnowflakeId(1n, 0n, 0n, 0n) !== 4194304n) throw new Error("ms=1, con lai 0: ID phai la 4194304n (1 << 22)");
if (ghepSnowflakeId(0n, 1n, 0n, 0n) !== 131072n) throw new Error("datacenterId=1, con lai 0: ID phai la 131072n (1 << 17)");
if (ghepSnowflakeId(0n, 0n, 1n, 0n) !== 4096n) throw new Error("machineId=1, con lai 0: ID phai la 4096n (1 << 12)");
if (ghepSnowflakeId(0n, 0n, 0n, 1n) !== 1n) throw new Error("sequence=1, con lai 0: ID phai la 1n (bit thap nhat)");
if (ghepSnowflakeId(123456789n, 3n, 7n, 777n) !== 517815304352521n) throw new Error("to hop day du (ms=123456789, dc=3, machine=7, seq=777) phai dung la 517815304352521n");
if (ghepSnowflakeId(0n, 31n, 0n, 0n) !== 4063232n) throw new Error("datacenterId toi da (31, 5 bit) phai dung la 4063232n");
if (ghepSnowflakeId(0n, 0n, 0n, 4095n) !== 4095n) throw new Error("sequence toi da (4095, 12 bit) phai dung la 4095n");

const idA = ghepSnowflakeId(999n, 2n, 4n, 100n);
const idB = ghepSnowflakeId(999n, 2n, 4n, 105n);
if (idB - idA !== 5n) throw new Error("chi doi sequence tu 100 len 105: chenh lech ID phai dung bang 5n");
```

:::hints
- kind: attention
  body: "Dich trai timestamp/datacenter/machine toi dung vi tri, OR tat ca voi sequence -- mot dong."
- kind: strategy
  body: "return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;"
- kind: one-line
  body: "return (msTuEpoch << DICH_TIMESTAMP) | (datacenterId << DICH_DATACENTER) | (machineId << DICH_MACHINE) | sequence;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "4194304"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bit đã đóng gói đúng chỗ. Nhưng NẾU hai request đến trong ĐÚNG cùng
một mili-giây, TRÊN cùng một máy — `sequence` phải làm GÌ để chúng
không trùng ID?
::::

::::reflect{#nghi-lai}
`ghepSnowflakeId` chỉ LÀ dịch bit VÀ OR bit — nhưng lựa chọn `bigint`
thay VÌ `number` không phải "cẩn thận CHO chắc", nó LÀ điều kiện BẮT
buộc: đã đo được SAI lệch `247` đơn vị NGAY trên một VÍ dụ đơn giản.
Với ID thật, sai lệch NÀY nghĩa LÀ hai request khác nhau có THỂ nhận
CÙNG một ID.
::::

::::checkpoint{mastery=0.75}
::::
