---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.can-bang-tai-xoay-vong
title: "Cân bằng tải: xoay vòng (round-robin)"
summary: "CanBangTai giữ danh sách server + chiSoTiep. chonServerTiepTheo trả về server tại chiSoTiep RỒI tăng chiSoTiep lên 1, cuộn vòng bằng % cacServer.length -- tất định, không phụ thuộc tải hiện tại. Với 3 server [s1,s2,s3], 5 lần gọi liên tiếp ra đúng s1,s2,s3,s1,s2 -- chiSoTiep sau đó là 2, không phải 0 hay 5 (đã cuộn vòng qua modulo)."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 8
teaches: [sd.can-bang-tai-xoay-vong]
requires: [sd.qps-va-luu-tru]
concepts: [sd.can-bang-tai-xoay-vong]
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
Bài 4 tính ra: đỉnh tải `111` request/giây. MỘT máy chủ chịu được
`111` request/giây không? Có LẼ không — cần NHIỀU máy. Nhưng request
MỚI tới, đi VÀO máy nào?
::::

::::explain{#xoay-vong}
Cách ĐƠN giản nhất để rải request RA nhiều server: xoay VÒNG
(round-robin) — request ĐẦU tiên đi tới server `0`, request THỨ hai
tới server `1`, ..., HẾT danh sách thì quay LẠI server `0`:

```typescript title=readonly
interface CanBangTai { cacServer: string[]; chiSoTiep: number; }
function taoCanBangTai(cacServer: string[]): CanBangTai {
  return { cacServer, chiSoTiep: 0 };
}

function chonServerTiepTheo(cbt: CanBangTai): string {
  const server = cbt.cacServer[cbt.chiSoTiep]!;
  cbt.chiSoTiep = (cbt.chiSoTiep + 1) % cbt.cacServer.length;
  return server;
}

const cbt = taoCanBangTai(["s1", "s2", "s3"]);
for (let i = 0; i < 5; i++) console.log(chonServerTiepTheo(cbt));
console.log("chiSoTiep sau 5 lan goi:", cbt.chiSoTiep);
```

```text title=readonly
s1
s2
s3
s1
s2
chiSoTiep sau 5 lan goi: 2
```

Với `3` server, gọi `5` lần cho ra `s1,s2,s3,s1,s2` — SAU khi hết
danh sách (`s3`), `%` (chia LẤY dư) đưa `chiSoTiep` VỀ `0`, tiếp tục
`s1` RỒI `s2`. `chiSoTiep` cuối LÀ `2` (không phải `5`) — VÌ `5 % 3 =
2`, "cuộn vòng" đúng nghĩa.
::::

::::example{#tat-dinh-khong-nhin-tai}
`chonServerTiepTheo` KHÔNG hề nhìn TỚI server nào đang bận HAY rảnh —
nó CHỈ đếm tuần TỰ. Với đúng MỘT server trong danh sách, kết quả LUÔN
LÀ chính nó:

```typescript title=readonly
interface CanBangTai { cacServer: string[]; chiSoTiep: number; }
function taoCanBangTai(cacServer: string[]): CanBangTai {
  return { cacServer, chiSoTiep: 0 };
}
function chonServerTiepTheo(cbt: CanBangTai): string {
  const server = cbt.cacServer[cbt.chiSoTiep]!;
  cbt.chiSoTiep = (cbt.chiSoTiep + 1) % cbt.cacServer.length;
  return server;
}

const cbtMot = taoCanBangTai(["chi-mot"]);
console.log(chonServerTiepTheo(cbtMot), chonServerTiepTheo(cbtMot), cbtMot.chiSoTiep);
```

```text title=readonly
chi-mot chi-mot 0
```

`chiSoTiep` LUÔN LÀ `0` (`(0+1) % 1 = 0` MỖI lần) — round-robin VỚI
một server duy nhất tương đương "luôn CHỌN đúng nó", một trường hợp
BIÊN hợp lệ chứ không phải lỗi.
::::

::::predict{#doan-goi-du-vong commitOnce}
Với `4` server (`["a","b","c","d"]`), gọi `chonServerTiepTheo` đúng
`4` LẦN liên tiếp (từ đầu, `chiSoTiep` khởi tạo `0`). Bốn server CÓ
được chọn đủ, MỖI server đúng một lần, đúng không?

:::opt{correct}
ĐÚNG — `4` lần gọi VỚI `4` server đi đúng MỘT vòng tròn tròn: `a, b,
c, d`, mỗi server xuất hiện đúng một LẦN, không thiếu không thừa
:::
:::opt
KHÔNG chắc — round-robin không đảm bảo "công bằng" tuyệt đối, VÌ mỗi
lần gọi CHỌN ngẫu nhiên trong số server CHƯA được chọn gần đây
::why
Nhầm round-robin VỚI một chiến lược NGẪU nhiên có trọng số (kiểu
"tránh lặp gần đây" nhưng vẫn random).

Chỗ lệch: `chonServerTiepTheo` HOÀN toàn tất định — `chiSoTiep` tăng
đúng `1` MỖI lần gọi (cuộn vòng qua `%`), không CÓ yếu tố ngẫu nhiên
NÀO. Đúng `N` lần gọi VỚI `N` server LUÔN đi đúng một vòng tròn tròn,
mỗi server đúng MỘT lần — kết quả có thể DỰ đoán trước, không phải
"công bằng trung bình VỀ lâu dài" như random.
::
:::
::::

::::code{#viet_chon_server_tiep_theo}
Hoàn thiện `chonServerTiepTheo` — lấy server TẠI `chiSoTiep`, RỒI
tăng `chiSoTiep` lên `1`, cuộn vòng VỀ `0` khi VƯỢT quá số server
(dùng `%`).

```typescript title=starter
interface CanBangTai { cacServer: string[]; chiSoTiep: number; }
function taoCanBangTai(cacServer: string[]): CanBangTai {
  return { cacServer, chiSoTiep: 0 };
}

function chonServerTiepTheo(cbt: CanBangTai): string {
  const server = cbt.cacServer[cbt.chiSoTiep]!;
  ___
  return server;
}

const cbt = taoCanBangTai(["s1", "s2", "s3"]);
console.log(chonServerTiepTheo(cbt));
```

```typescript title=solution
interface CanBangTai { cacServer: string[]; chiSoTiep: number; }
function taoCanBangTai(cacServer: string[]): CanBangTai {
  return { cacServer, chiSoTiep: 0 };
}

function chonServerTiepTheo(cbt: CanBangTai): string {
  const server = cbt.cacServer[cbt.chiSoTiep]!;
  cbt.chiSoTiep = (cbt.chiSoTiep + 1) % cbt.cacServer.length;
  return server;
}

const cbt = taoCanBangTai(["s1", "s2", "s3"]);
console.log(chonServerTiepTheo(cbt));
```

```typescript title=test
const cbtT = taoCanBangTai(["s1", "s2", "s3"]);
const ketQua: string[] = [];
for (let i = 0; i < 5; i++) ketQua.push(chonServerTiepTheo(cbtT));
if (ketQua.join(",") !== "s1,s2,s3,s1,s2") throw new Error("5 lan goi voi 3 server phai ra dung s1,s2,s3,s1,s2");
if (cbtT.chiSoTiep !== 2) throw new Error("chiSoTiep sau 5 lan goi phai la 2 (5 % 3), khong phai 5 hay 0");

const cbtMotT = taoCanBangTai(["chi-mot"]);
chonServerTiepTheo(cbtMotT);
chonServerTiepTheo(cbtMotT);
if (cbtMotT.chiSoTiep !== 0) throw new Error("voi 1 server, chiSoTiep phai luon quay ve 0");

const cbt4 = taoCanBangTai(["a", "b", "c", "d"]);
const bonLan = [chonServerTiepTheo(cbt4), chonServerTiepTheo(cbt4), chonServerTiepTheo(cbt4), chonServerTiepTheo(cbt4)];
if (new Set(bonLan).size !== 4) throw new Error("4 lan goi voi 4 server phai cho 4 server KHAC nhau (dung mot vong)");
if (cbt4.chiSoTiep !== 0) throw new Error("dung 4 lan goi voi 4 server phai cuon het mot vong, chiSoTiep quay ve 0");
```

:::hints
- kind: attention
  body: "Cap nhat cbt.chiSoTiep = (chiSoTiep + 1) % so luong server -- mot dong."
- kind: strategy
  body: "cbt.chiSoTiep = (cbt.chiSoTiep + 1) % cbt.cacServer.length;"
- kind: one-line
  body: "cbt.chiSoTiep = (cbt.chiSoTiep + 1) % cbt.cacServer.length;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "s1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xoay vòng công bằng — NHƯNG nó "mù": không biết server NÀO đang gánh
nhiều request hơn, hay server nào ĐÃ sập.
::::

::::reflect{#nghi-lai}
`chonServerTiepTheo` chỉ cần đúng MỘT trường trạng thái (`chiSoTiep`)
VÀ một phép `%` — nhưng "tất định, không nhìn tải" chính LÀ điểm
YẾU của nó: NẾU một server chậm hơn hẳn (hoặc đã sập), round-robin
vẫn CỨ gửi request TỚI nó đúng như MỌI server khác.
::::

::::checkpoint{mastery=0.75}
::::
