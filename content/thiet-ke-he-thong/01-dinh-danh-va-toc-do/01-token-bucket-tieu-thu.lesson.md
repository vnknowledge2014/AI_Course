---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.token-bucket-tieu-thu
title: "Token bucket: tiêu thụ và từ chối"
summary: "ThungToken{soTokenHienTai,soTokenToiDa} bắt đầu ĐẦY. tieuThuToken trừ đúng 1 token mỗi request, cho qua (true) nếu còn ≥1, từ chối (false) nếu hết — không bao giờ để soTokenHienTai âm. Bucket dung lượng 5: 8 request liên tiếp (cùng lúc) chỉ 5 được cho qua (burst dùng hết token tích luỹ), 3 request sau bị từ chối; napDayLai nạp lại thủ công về đầy."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.token-bucket-tieu-thu]
requires: [sd.boss-do-truoc-khi-thiet-ke]
concepts: [sd.token-bucket-tieu-thu]
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
Realm 6 đo quy mô, cân tải, cache dữ liệu. Realm 7 giờ hỏi câu khác:
một client CÓ được phép gửi request NÀY không, hay nó đã gửi QUÁ
nhiều rồi? Đây LÀ giới hạn tốc độ (rate limiting) — bắt đầu VỚI thuật
toán đơn giản nhất: token bucket.
::::

::::explain{#thung-token}
Tưởng tượng một CÁI xô chứa TỐI ĐA `soTokenToiDa` token, bắt ĐẦU đầy.
MỖI request hợp lệ tiêu thụ đúng MỘT token. Còn token thì CHO qua (và
trừ), HẾT token thì từ chối — KHÔNG hề để số token xuống ÂM:

```typescript title=readonly
interface ThungToken { soTokenHienTai: number; soTokenToiDa: number; }
function taoThungToken(soTokenToiDa: number): ThungToken {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa };
}
function tieuThuToken(thung: ThungToken): boolean {
  if (thung.soTokenHienTai <= 0) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const thung = taoThungToken(3);
console.log("token con lai luc dau:", thung.soTokenHienTai);
console.log("request 1:", tieuThuToken(thung), "-- con lai:", thung.soTokenHienTai);
console.log("request 2:", tieuThuToken(thung), "-- con lai:", thung.soTokenHienTai);
console.log("request 3:", tieuThuToken(thung), "-- con lai:", thung.soTokenHienTai);
console.log("request 4:", tieuThuToken(thung), "-- con lai:", thung.soTokenHienTai);
```

```text title=readonly
token con lai luc dau: 3
request 1: true -- con lai: 2
request 2: true -- con lai: 1
request 3: true -- con lai: 0
request 4: false -- con lai: 0
```

Bucket `3` token: BA request đầu đều `true` (tiêu thụ dần TỪ `3`
xuống `0`), request thứ TƯ gặp `soTokenHienTai <= 0` NÊN bị từ chối
`false` — VÀ số token KHÔNG hề tụt xuống `-1`, nó dừng ĐÚNG tại `0`.
::::

::::example{#burst-va-nap-lai}
Vì token tích LUỸ sẵn trong bucket, MỘT client có thể gửi CẢ một
"đợt" (burst) request LIÊN tiếp — miễn còn token — RỒI mới bị chặn.
Đây LÀ điểm khác biệt LỚN nhất so với "mỗi giây chỉ 1 request": token
bucket cho phép DỒN dùng:

```typescript title=readonly
interface ThungToken { soTokenHienTai: number; soTokenToiDa: number; }
function taoThungToken(soTokenToiDa: number): ThungToken {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa };
}
function tieuThuToken(thung: ThungToken): boolean {
  if (thung.soTokenHienTai <= 0) return false;
  thung.soTokenHienTai -= 1;
  return true;
}
function napDayLai(thung: ThungToken): void {
  thung.soTokenHienTai = thung.soTokenToiDa;
}

const thung = taoThungToken(5);
const ketQua: boolean[] = [];
for (let i = 0; i < 8; i++) ketQua.push(tieuThuToken(thung));
console.log("8 request lien tiep (cung mot luc):", ketQua.join(","));
console.log("so request duoc cho qua:", ketQua.filter((k) => k).length);
console.log("token con lai:", thung.soTokenHienTai);

napDayLai(thung);
console.log("sau napDayLai, token con lai:", thung.soTokenHienTai);
console.log("request tiep theo:", tieuThuToken(thung));
```

```text title=readonly
8 request lien tiep (cung mot luc): true,true,true,true,true,false,false,false
so request duoc cho qua: 5
token con lai: 0
sau napDayLai, token con lai: 5
request tiep theo: true
```

Dung lượng `5`, gửi liền `8` request: đúng NĂM request đầu qua (dùng
HẾT vốn token tích luỹ), BA request cuối bị chặn. `napDayLai` (nạp
thủ công) đưa bucket VỀ đầy — nhưng đây chỉ LÀ mô phỏng tạm; bài SAU
sẽ thay nó bằng một cơ chế nạp TỰ ĐỘNG theo thời gian trôi qua, không
cần ai gọi tay.
::::

::::predict{#doan-tieu-thu-het-token commitOnce}
Một bucket dung lượng `1` (bắt đầu đầy, `soTokenHienTai = 1`). Gọi
`tieuThuToken` liên tiếp đúng `3` LẦN, không hề gọi `napDayLai` ở
giữa. Kết quả BA lần gọi (VÀ `soTokenHienTai` cuối cùng) là gì?

:::opt{correct}
`true, false, false` — VÀ `soTokenHienTai` cuối LÀ `0`: chỉ lần gọi
ĐẦU tiêu thụ được token DUY nhất, hai lần sau đều gặp bucket rỗng
:::
:::opt
`true, false, false` — nhưng `soTokenHienTai` cuối LÀ `-2`: mỗi lần
gọi `tieuThuToken` đều trừ `1`, kể cả khi ĐÃ hết, nên số token tiếp
tục ÂM dần
::why
Nhầm "hàm ĐƯỢC gọi" VỚI "hàm THỰC SỰ trừ token" — nhưng `tieuThuToken`
kiểm tra ĐIỀU kiện `soTokenHienTai <= 0` TRƯỚC khi trừ bất cứ gì.

Chỗ lệch: dòng đầu tiên trong `tieuThuToken` LÀ `if
(thung.soTokenHienTai <= 0) return false;` — hàm THOÁT ngay, KHÔNG hề
chạm tới dòng trừ `-= 1` khi bucket ĐÃ rỗng. `soTokenHienTai` dừng
ĐÚNG tại `0`, không bao giờ đi xuống ÂM dù gọi bao nhiêu lần đi nữa.
::
:::
::::

::::code{#viet_tieu_thu_token}
Hoàn thiện `tieuThuToken` — sau khi đã xác nhận CÒN token (điều kiện
`<= 0` đã kiểm ở trên), trừ đúng MỘT token rồi cho qua.

```typescript title=starter
interface ThungToken { soTokenHienTai: number; soTokenToiDa: number; }
function taoThungToken(soTokenToiDa: number): ThungToken {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa };
}
function tieuThuToken(thung: ThungToken): boolean {
  if (thung.soTokenHienTai <= 0) return false;
  ___
  return true;
}

const thung = taoThungToken(2);
console.log(tieuThuToken(thung), thung.soTokenHienTai);
```

```typescript title=solution
interface ThungToken { soTokenHienTai: number; soTokenToiDa: number; }
function taoThungToken(soTokenToiDa: number): ThungToken {
  return { soTokenHienTai: soTokenToiDa, soTokenToiDa };
}
function tieuThuToken(thung: ThungToken): boolean {
  if (thung.soTokenHienTai <= 0) return false;
  thung.soTokenHienTai -= 1;
  return true;
}

const thung = taoThungToken(2);
console.log(tieuThuToken(thung), thung.soTokenHienTai);
```

```typescript title=test
function laySoTokenConLai(t: ThungToken): number { return t.soTokenHienTai; }

const thungT = taoThungToken(2);
const kqT: boolean[] = [];
for (let i = 0; i < 4; i++) kqT.push(tieuThuToken(thungT));
if (kqT.join(",") !== "true,true,false,false") throw new Error("2 token: 2 request dau duoc, 2 request sau bi tu choi");
if (laySoTokenConLai(thungT) !== 0) throw new Error("token con lai phai la 0, khong duoc am");

const thungT2 = taoThungToken(1);
tieuThuToken(thungT2);
if (laySoTokenConLai(thungT2) !== 0) throw new Error("1 token, tieu thu 1 lan, con lai phai la 0");
if (tieuThuToken(thungT2) !== false) throw new Error("het token phai tu choi (false)");
if (laySoTokenConLai(thungT2) !== 0) throw new Error("tu choi khong duoc lam token am");
```

:::hints
- kind: attention
  body: "Da xac nhan con token (dieu kien tren da loai truong hop het) -- gio tru 1 token, mot dong."
- kind: strategy
  body: "thung.soTokenHienTai -= 1;"
- kind: one-line
  body: "thung.soTokenHienTai -= 1;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bucket tiêu thụ đúng, không âm. Nhưng MỘT bucket chỉ tiêu thụ mà
không BAO giờ nạp lại thì vô dụng cho rate limiting THẬT — hết token
LÀ hết vĩnh viễn.
::::

::::reflect{#nghi-lai}
`tieuThuToken` chỉ CÓ đúng một điều kiện VÀ một phép trừ — nhưng thứ
tự kiểm tra TRƯỚC khi trừ chính LÀ điều giữ cho `soTokenHienTai` không
bao giờ âm. Bài NÀY cố tình bỏ qua chuyện "nạp lại token theo thời
gian" (dùng `napDayLai` thủ công thay THẾ) để tách RIÊNG hai mối
quan tâm: tiêu thụ trước, nạp lại tự động sau.
::::

::::checkpoint{mastery=0.75}
::::
