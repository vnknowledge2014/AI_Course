---
id: thiet-ke-he-thong.trien-khai-an-toan.ba-chien-luoc-trien-khai
title: "Ba chiến lược triển khai: rolling, blue-green, canary"
summary: "moPhongRolling(tongSo, soMoiLanThay) mo phong thay TUNG instance mot -- 10 instance, moi lan thay 2, can 6 buoc (buoc 0..5), CU+MOI CHAY SONG SONG suot qua trinh (vi du buoc 3: cu=4 moi=6); moPhongBlueGreen(10) chi 3 buoc: 10/0 -> 10/10 (GAP DOI tai nguyen, dinh cao 20) -> 0/10; moPhongCanary(10,[10,50,100]) tang dan 9/1 -> 5/5 -> 0/10. taiNguyenDinhCao() do bang so: rolling va canary khong bao gio vuot tong so instance (dinh 10), blue-green LUON gap doi (dinh 20) trong luc chuyen doi."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.ba-chien-luoc-trien-khai]
requires: [sd.fp.boss-streaming-va-hieu-ung-o-bien]
concepts: [sd.ba-chien-luoc-trien-khai]
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
Ba track trước của T7.3 dạy hệ thống PHẢI đúng đắn ra sao khi đã chạy
— log bất biến, CRDT, streaming. Track CUỐI của Realm 7 hỏi một câu
khác hẳn: làm sao ĐƯA một thay đổi vào production mà không làm sập
thứ đang chạy tốt? Bắt đầu bằng câu hỏi nền tảng nhất — có BAO NHIÊU
cách đưa code mới vào, và mỗi cách đánh đổi CÁI gì?
::::

::::explain{#ba-chien-luoc}
`moPhongRolling` thay TỪNG instance một, theo từng đợt nhỏ — mỗi bước
chuyển một số instance CŨ thành MỚI, cho tới khi hết. Trong SUỐT quá
trình đó, `soCu` và `soMoi` đều lớn hơn `0` CÙNG một lúc — hai phiên
bản đang PHỤC vụ traffic song song, dù downtime bằng `0`:

```typescript title=readonly
interface TrangThaiTrienKhai { buoc: number; soCu: number; soMoi: number; }

function moPhongRolling(tongSo: number, soMoiLanThay: number): TrangThaiTrienKhai[] {
  const cacBuoc: TrangThaiTrienKhai[] = [];
  let soMoi = 0;
  let buoc = 0;
  cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  while (soMoi < tongSo) {
    soMoi = Math.min(tongSo, soMoi + soMoiLanThay);
    buoc += 1;
    cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  }
  return cacBuoc;
}

const ketQuaRolling = moPhongRolling(10, 2);
console.log("rolling, tong 10 instance, moi lan thay 2:");
for (const b of ketQuaRolling) {
  console.log(`  buoc ${b.buoc}: cu=${b.soCu} moi=${b.soMoi}`);
}
console.log("so buoc rolling can:", ketQuaRolling.length - 1);
```

```text title=readonly
rolling, tong 10 instance, moi lan thay 2:
  buoc 0: cu=10 moi=0
  buoc 1: cu=8 moi=2
  buoc 2: cu=6 moi=4
  buoc 3: cu=4 moi=6
  buoc 4: cu=2 moi=8
  buoc 5: cu=0 moi=10
so buoc rolling can: 5
```

Từ buốc `1` tới buốc `4`, cả `soCu` LẪN `soMoi` đều khác `0` — đây
CHÍNH LÀ cái giá của rolling: không tốn thêm tài nguyên (`soCu +
soMoi` luôn bằng `10`), nhưng có một CỬA sổ thời gian mà request có
thể rơi vào phiên bản CŨ hoặc MỚI tuỳ ngẫu nhiên, tuỳ instance nào
nhận request đó.
::::

::::example{#blue-green-va-canary}
`moPhongBlueGreen` dựng nguyên một môi trường MỚI cạnh môi trường CŨ,
rồi chuyển traffic tức thì — không có bước trung gian. `moPhongCanary`
đưa MỘT tỷ lệ traffic nhỏ trước, tăng dần theo danh sách mốc. Hàm
`taiNguyenDinhCao` đo số instance CHẠY ĐỒNG THỜI (cộng cả cũ lẫn mới)
tại thời điểm cao nhất của cả quá trình:

```typescript title=readonly
interface TrangThaiTrienKhai { buoc: number; soCu: number; soMoi: number; }

function moPhongRolling(tongSo: number, soMoiLanThay: number): TrangThaiTrienKhai[] {
  const cacBuoc: TrangThaiTrienKhai[] = [];
  let soMoi = 0;
  let buoc = 0;
  cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  while (soMoi < tongSo) {
    soMoi = Math.min(tongSo, soMoi + soMoiLanThay);
    buoc += 1;
    cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  }
  return cacBuoc;
}

function moPhongBlueGreen(tongSo: number): TrangThaiTrienKhai[] {
  return [
    { buoc: 0, soCu: tongSo, soMoi: 0 },
    { buoc: 1, soCu: tongSo, soMoi: tongSo },
    { buoc: 2, soCu: 0, soMoi: tongSo },
  ];
}

function moPhongCanary(tongSo: number, cacTyLePhanTram: number[]): TrangThaiTrienKhai[] {
  return cacTyLePhanTram.map((tyLe, i) => {
    const soMoi = Math.round((tongSo * tyLe) / 100);
    return { buoc: i, soCu: tongSo - soMoi, soMoi };
  });
}

function taiNguyenDinhCao(cacBuoc: TrangThaiTrienKhai[]): number {
  return Math.max(...cacBuoc.map((b) => b.soCu + b.soMoi));
}

const rolling = moPhongRolling(10, 2);
const blueGreen = moPhongBlueGreen(10);
const canary = moPhongCanary(10, [10, 50, 100]);

console.log("blue-green, tong 10 instance:");
for (const b of blueGreen) {
  console.log(`  buoc ${b.buoc}: cu=${b.soCu} moi=${b.soMoi} tong-dang-chay=${b.soCu + b.soMoi}`);
}

console.log("canary, tong 10 instance, ty le 10% -> 50% -> 100%:");
for (const b of canary) {
  console.log(`  buoc ${b.buoc}: cu=${b.soCu} moi=${b.soMoi}`);
}

console.log("tai nguyen dinh cao (so instance CHAY DONG THOI, cong ca cu lan moi):");
console.log("  rolling:", taiNguyenDinhCao(rolling));
console.log("  blue-green:", taiNguyenDinhCao(blueGreen));
console.log("  canary:", taiNguyenDinhCao(canary));
```

```text title=readonly
blue-green, tong 10 instance:
  buoc 0: cu=10 moi=0 tong-dang-chay=10
  buoc 1: cu=10 moi=10 tong-dang-chay=20
  buoc 2: cu=0 moi=10 tong-dang-chay=10
canary, tong 10 instance, ty le 10% -> 50% -> 100%:
  buoc 0: cu=9 moi=1
  buoc 1: cu=5 moi=5
  buoc 2: cu=0 moi=10
tai nguyen dinh cao (so instance CHAY DONG THOI, cong ca cu lan moi):
  rolling: 10
  blue-green: 20
  canary: 10
```

Ở buốc `1` của blue-green, CẢ môi trường cũ LẪN môi trường mới đều
chạy ĐỦ `10` instance cùng lúc — tổng `20`, ĐÚNG gấp đôi. Đó LÀ cái
giá của "switch tức thì": phải có sẵn TOÀN bộ hạ tầng mới TRƯỚC khi
chuyển, dù chỉ dùng trong một khoảnh khắc. Rolling VÀ canary không hề
có bước nào vượt quá tổng `10` — chúng THAY THẾ dần dần, không XÂY
DỰNG song song một bản sao toàn bộ.
::::

::::predict{#doan-rolling-buoc-giua commitOnce}
Gọi `moPhongRolling(10, 3)` — tổng `10` instance, mỗi lần thay `3`.
Tại bước NGAY TRƯỚC bước cuối cùng (bước mà `soMoi` vừa đạt `9`, chưa
tới `10`) — `soCu` tại bước ĐÓ là bao nhiêu?

:::opt{correct}
`1` — vòng lặp cộng dồn `soMoi` bằng bội số của `3`: `0 -> 3 -> 6 ->
9 -> 10` (bước cuối bị `Math.min` chặn lại ở `10`); tại `soMoi = 9`,
`soCu = 10 - 9 = 1`
:::
:::opt
`0` — mỗi lần thay `3` instance, sau `3` lần thay (`3 x 3 = 9`) coi
như GẦN xong, VÀ hệ thống nên làm tròn để không còn instance cũ nào
sót lại giữa chừng
::why
Nhầm "gần đạt tổng số" VỚI "đã đạt tổng số" — nhưng `moPhongRolling`
không hề làm tròn hay dồn bước cuối SỚM hơn thực tế.

Chỗ lệch: vòng lặp cộng CHÍNH XÁC `soMoiLanThay` (`3`) mỗi lần, không
quan tâm phần dư còn lại. Sau ba lần cộng, `soMoi = 0 + 3 + 3 + 3 =
9`, CÒN CÁCH `10` đúng `1` instance — `soCu` tại bước đó VẪN là `10 -
9 = 1`, không phải `0`. Chỉ ở bước KẾ tiếp, `Math.min(10, 9 + 3)` mới
chặn `soMoi` dừng đúng ở `10`.
::
:::
::::

::::code{#viet_mo_phong_rolling}
Hoàn thiện `moPhongRolling` — vòng lặp `while` còn thiếu: mỗi lần
lặp, tăng `soMoi` thêm `soMoiLanThay` (dùng `Math.min` để không vượt
quá `tongSo`), tăng `buoc` thêm `1`, RỒI đẩy trạng thái mới vào
`cacBuoc`. Vòng lặp dừng khi `soMoi` đã bằng `tongSo`.

```typescript title=starter
interface TrangThaiTrienKhai { buoc: number; soCu: number; soMoi: number; }

function moPhongRolling(tongSo: number, soMoiLanThay: number): TrangThaiTrienKhai[] {
  const cacBuoc: TrangThaiTrienKhai[] = [];
  let soMoi = 0;
  let buoc = 0;
  cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  while (soMoi < tongSo) {
    ___
  }
  return cacBuoc;
}

const ketQuaX = moPhongRolling(6, 4);
const cuoiX = ketQuaX[ketQuaX.length - 1]!;
console.log(ketQuaX.length, cuoiX.soCu, cuoiX.soMoi);
```

```typescript title=solution
interface TrangThaiTrienKhai { buoc: number; soCu: number; soMoi: number; }

function moPhongRolling(tongSo: number, soMoiLanThay: number): TrangThaiTrienKhai[] {
  const cacBuoc: TrangThaiTrienKhai[] = [];
  let soMoi = 0;
  let buoc = 0;
  cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  while (soMoi < tongSo) {
    soMoi = Math.min(tongSo, soMoi + soMoiLanThay);
    buoc += 1;
    cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });
  }
  return cacBuoc;
}

const ketQuaX = moPhongRolling(6, 4);
const cuoiX = ketQuaX[ketQuaX.length - 1]!;
console.log(ketQuaX.length, cuoiX.soCu, cuoiX.soMoi);
```

```typescript title=test
const ketQuaT = moPhongRolling(8, 2);
if (ketQuaT.length !== 5) throw new Error("tong 8, moi lan thay 2, phai co DUNG 5 buoc (ke ca buoc 0)");
const buocDauT = ketQuaT[0];
if (buocDauT === undefined || buocDauT.soCu !== 8 || buocDauT.soMoi !== 0) throw new Error("buoc 0 phai la toan bo CU, chua co MOI nao");
const buocCuoiT = ketQuaT[ketQuaT.length - 1];
if (buocCuoiT === undefined || buocCuoiT.soCu !== 0 || buocCuoiT.soMoi !== 8) throw new Error("buoc cuoi phai la toan bo MOI, khong con CU");
const buocGiuaT = ketQuaT[2];
if (buocGiuaT === undefined || buocGiuaT.soCu !== 4 || buocGiuaT.soMoi !== 4) throw new Error("buoc 2 (sau 2 lan thay 2) phai la cu=4 moi=4 -- CHAY SONG SONG hai phien ban");

const ketQuaLeT = moPhongRolling(5, 2);
if (ketQuaLeT.length !== 4) throw new Error("tong 5 (le), moi lan thay 2, phai co DUNG 4 buoc: 0, 2, 4, roi Math.min chan o 5");
const buocCuoiLeT = ketQuaLeT[ketQuaLeT.length - 1];
if (buocCuoiLeT === undefined || buocCuoiLeT.soMoi !== 5) throw new Error("buoc cuoi phai dat DUNG tongSo, khong vuot qua (Math.min chan lai)");
```

:::hints
- kind: attention
  body: "Trong than while, can dung 3 dong: soMoi = Math.min(tongSo, soMoi + soMoiLanThay); buoc += 1; cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });"
- kind: strategy
  body: "soMoi = Math.min(tongSo, soMoi + soMoiLanThay); buoc += 1; cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });"
- kind: one-line
  body: "soMoi = Math.min(tongSo, soMoi + soMoiLanThay); buoc += 1; cacBuoc.push({ buoc, soCu: tongSo - soMoi, soMoi });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "3 0 6"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba con số cụ thể cho ba chiến lược — không còn LÀ ba đoạn văn mô tả
mơ hồ NỮA. Nhưng dù chọn chiến lược nào, một câu hỏi vẫn CÒN đó: LÀM
SAO tăng dần tỷ lệ traffic một cách AN TOÀN, VÀ dừng lại NGAY khi có
dấu hiệu hỏng?
::::

::::reflect{#nghi-lai}
Ba hàm mô phỏng không hề PHÁN xét chiến lược nào "tốt hơn" — chúng chỉ
tính ra ĐÚNG con số mà mỗi chiến lược tạo ra tại MỖI bước. Đánh đổi
không nằm Ở lời mô tả ("blue-green tốn tài nguyên hơn") mà nằm Ở
`taiNguyenDinhCao` trả về `20` thay vì `10` — một sự thật đo được,
không phải một nhận xét chung chung. Từ đây, MỖI bài trong track sẽ
đào sâu MỘT mảnh của quy trình triển khai, bắt đầu từ chính chiến
lược đo được nhiều nhất Ở đây: canary.
::::

::::checkpoint{mastery=0.72}
::::
