---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.can-bang-tai-it-ket-noi-nhat
title: "Cân bằng tải: ít kết nối nhất + loại server chết"
summary: "chonServerItKetNoiNhat bỏ qua server conSong=false (health check), rồi chọn server có soKetNoi nhỏ nhất trong số còn lại. ganYeuCau gọi hàm này rồi tăng soKetNoi của server được chọn. Với s1(3 kết nối,sống)/s2(1,sống)/s3(0,CHẾT): lần chọn đầu ra s2 (không phải s3 dù s3 có 0 kết nối -- s3 đã chết). Bốn yêu cầu liên tiếp -> s2,s2,s1,s2, kết thúc s1=4/s2=4/s3=0 (s3 không đổi vì health check loại nó khỏi MỌI lượt chọn)."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.can-bang-tai-it-ket-noi-nhat]
requires: [sd.can-bang-tai-xoay-vong]
concepts: [sd.can-bang-tai-it-ket-noi-nhat]
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
Xoay vòng (bài trước) "mù" — không nhìn tải, không nhìn server SỐNG
hay chết. Hai vấn đề CẦN giải quyết CÙNG lúc: chọn server RẢNH nhất,
VÀ đừng bao giờ gửi request tới server ĐÃ chết.
::::

::::explain{#it-ket-noi-nhat}
Mỗi server giữ MỘT bộ đếm `soKetNoi` (số kết nối đang xử LÝ) VÀ một
cờ `conSong` (health check — server CÓ đang phản hồi hay không).
`chonServerItKetNoiNhat` bỏ QUA mọi server đã chết, RỒI chọn server
CÓ `soKetNoi` nhỏ nhất trong số CÒN lại:

```typescript title=readonly
interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }

function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}

const cacServer: ServerInfo[] = [
  { ten: "s1", soKetNoi: 3, conSong: true },
  { ten: "s2", soKetNoi: 1, conSong: true },
  { ten: "s3", soKetNoi: 0, conSong: false },
];

console.log("chon lan 1:", chonServerItKetNoiNhat(cacServer));
```

```text title=readonly
chon lan 1: s2
```

`s3` CÓ `soKetNoi = 0` — Ít NHẤT trong CẢ ba — nhưng `conSong = false`
NÊN bị `continue` bỏ qua NGAY từ đầu vòng lặp, KHÔNG bao giờ được
xét TỚI. Trong hai server còn LẠI (`s1` VÀ `s2`), `s2` (`1` kết nối)
Ít hơn `s1` (`3` kết nối) — `s2` thắng.
::::

::::example{#gan-yeu-cau-hoi-tu}
`ganYeuCau` gọi `chonServerItKetNoiNhat` RỒI tăng `soKetNoi` CỦA
server được chọn — mô phỏng "một request MỚI vừa được giao":

```typescript title=readonly
interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }
function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}
const cacServer: ServerInfo[] = [
  { ten: "s1", soKetNoi: 3, conSong: true },
  { ten: "s2", soKetNoi: 1, conSong: true },
  { ten: "s3", soKetNoi: 0, conSong: false },
];

function ganYeuCau(cacServer: ServerInfo[]): string | undefined {
  const ten = chonServerItKetNoiNhat(cacServer);
  if (ten === undefined) return undefined;
  cacServer.find((s) => s.ten === ten)!.soKetNoi += 1;
  return ten;
}

for (let i = 0; i < 4; i++) console.log("gan yeu cau", i, "->", ganYeuCau(cacServer));
console.log(JSON.stringify(cacServer));
```

```text title=readonly
gan yeu cau 0 -> s2
gan yeu cau 1 -> s2
gan yeu cau 2 -> s1
gan yeu cau 3 -> s2
[{"ten":"s1","soKetNoi":4,"conSong":true},{"ten":"s2","soKetNoi":4,"conSong":true},{"ten":"s3","soKetNoi":0,"conSong":false}]
```

`s2` được chọn LIÊN tiếp (`1→2→3`) VÌ mỗi lần nó vẫn Ít kết nối HƠN
`s1` (bắt đầu Ở `3`) — CHO tới khi `s2` đạt `3`, NGANG `s1`, thì
LƯỢT sau `s1` (`3`) thắng vì so sánh dùng `<` nghiêm ngặt (không
phải `<=`, NÊN khi bằng nhau, server ĐẦU tiên trong vòng lặp — ở đây
LÀ `s1` — giữ NGUYÊN kết quả). Sau `4` yêu cầu, `s1` VÀ `s2` cân bằng
Ở `4` — `s3` (chết) vẫn `0`, KHÔNG hề nhận request NÀO.
::::

::::predict{#doan-tat-ca-chet commitOnce}
Nếu TẤT CẢ server trong danh sách đều `conSong: false` (cả cụm đang
"down"), `chonServerItKetNoiNhat` trả VỀ gì?

:::opt{correct}
`undefined` — vòng lặp `continue` VỚI mọi phần tử (không server nào
qua được kiểm tra `conSong`), `ketQua` không BAO giờ được gán, kết
quả CUỐI LÀ `undefined`
:::
:::opt
Trả VỀ server ĐẦU tiên trong danh sách — health check chỉ LÀ một gợi
Ý ưu tiên, không phải điều kiện BẮT buộc
::why
Nhầm `continue` (bỏ qua HOÀN toàn, không xét gì thêm VỀ phần tử đó)
VỚI "hạ độ ưu tiên" (vẫn xét nhưng ít khả năng được chọn hơn).

Chỗ lệch: `if (!s.conSong) continue;` nhảy THẲNG sang vòng lặp kế
tiếp — server chết KHÔNG BAO giờ được so sánh VỚI `ketQua`, dù danh
sách CÓ mười server chết VÀ không server nào sống. `ketQua` giữ
NGUYÊN `undefined` suốt vòng LẶP, VÀ `ketQua?.ten` trên `undefined`
LÀ `undefined` — không CÓ "server dự phòng" nào được trả VỀ.
::
:::
::::

::::code{#viet_chon_server_it_ket_noi_nhat}
Hoàn thiện `chonServerItKetNoiNhat` — SO sánh `soKetNoi` để giữ LẠI
server ít kết nối NHẤT trong số CÒN sống (`conSong` đã được lọc TRƯỚC
bằng `continue`).

```typescript title=starter
interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }

function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    ___
  }
  return ketQua?.ten;
}

const cacServer: ServerInfo[] = [
  { ten: "s1", soKetNoi: 3, conSong: true },
  { ten: "s2", soKetNoi: 1, conSong: true },
  { ten: "s3", soKetNoi: 0, conSong: false },
];
console.log(chonServerItKetNoiNhat(cacServer));
```

```typescript title=solution
interface ServerInfo { ten: string; soKetNoi: number; conSong: boolean; }

function chonServerItKetNoiNhat(cacServer: ServerInfo[]): string | undefined {
  let ketQua: ServerInfo | undefined;
  for (const s of cacServer) {
    if (!s.conSong) continue;
    if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;
  }
  return ketQua?.ten;
}

const cacServer: ServerInfo[] = [
  { ten: "s1", soKetNoi: 3, conSong: true },
  { ten: "s2", soKetNoi: 1, conSong: true },
  { ten: "s3", soKetNoi: 0, conSong: false },
];
console.log(chonServerItKetNoiNhat(cacServer));
```

```typescript title=test
function ganYeuCau(cacServer: ServerInfo[]): string | undefined {
  const ten = chonServerItKetNoiNhat(cacServer);
  if (ten === undefined) return undefined;
  cacServer.find((s) => s.ten === ten)!.soKetNoi += 1;
  return ten;
}

if (chonServerItKetNoiNhat(cacServer) !== "s2") throw new Error("lan chon dau phai la s2 (it ket noi nhat TRONG SO server con song)");

const cacGoi: (string | undefined)[] = [];
for (let i = 0; i < 4; i++) cacGoi.push(ganYeuCau(cacServer));
if (cacGoi.join(",") !== "s2,s2,s1,s2") throw new Error("4 lan gan yeu cau phai theo dung thu tu s2,s2,s1,s2");

const s1 = cacServer.find((s) => s.ten === "s1")!;
const s2 = cacServer.find((s) => s.ten === "s2")!;
const s3 = cacServer.find((s) => s.ten === "s3")!;
if (s1.soKetNoi !== 4) throw new Error("s1.soKetNoi phai la 4 sau 4 lan gan yeu cau");
if (s2.soKetNoi !== 4) throw new Error("s2.soKetNoi phai la 4 sau 4 lan gan yeu cau");
if (s3.soKetNoi !== 0) throw new Error("s3 (chet) khong duoc nhan bat ky yeu cau nao, soKetNoi phai giu nguyen 0");

const tatCaChet: ServerInfo[] = [{ ten: "x", soKetNoi: 0, conSong: false }];
if (chonServerItKetNoiNhat(tatCaChet) !== undefined) throw new Error("neu tat ca server deu chet, phai tra ve undefined");
```

:::hints
- kind: attention
  body: "Neu ketQua chua co HOAC s.soKetNoi nho hon ketQua.soKetNoi thi cap nhat ketQua = s -- mot dong."
- kind: strategy
  body: "if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;"
- kind: one-line
  body: "if (!ketQua || s.soKetNoi < ketQua.soKetNoi) ketQua = s;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "s2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Load balancer BIẾT phân bổ request VÀ tránh server chết. Nhưng MỖI
request tới đều đụng thẳng NGUỒN dữ liệu (database) — cứ THẾ mãi thì
nguồn nào chịu NỔI?
::::

::::reflect{#nghi-lai}
`chonServerItKetNoiNhat` khác `chonServerTiepTheo` (bài trước) Ở đúng
MỘT điểm cốt lõi: nó NHÌN vào trạng thái THẬT (số kết nối, còn sống
hay không) thay VÌ đếm mù. Cái GIÁ phải trả: mỗi lần chọn giờ tốn
`O(n)` để duyệt VÀ so sánh, thay VÌ `O(1)` của round-robin — đánh đổi
kinh điển GIỮA "chất lượng quyết định" VÀ "chi phí ra quyết định".
::::

::::checkpoint{mastery=0.8}
::::
