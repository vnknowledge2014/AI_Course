---
id: thiet-ke-he-thong.crdt-va-hop-nhat.lww-register
title: "LWW-Register: giá trị đơn, timestamp lớn hơn thắng, hoà thì so replicaId"
summary: "interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string } -- dung DongHoMoPhong/tienThoiGian nhu cac bai T7.1/T7.2; hopNhat(a,b) so a.thoiDiem voi b.thoiDiem, thoiDiem LON hon thang; NEU BANG nhau, so a.replicaId voi b.replicaId (chuoi lon hon thang) de dam bao KET QUA GIONG HET nhau du goi hopNhat(a,b) hay hopNhat(b,a) -- neu chi 'ai den truoc thi thang' se KHONG doi xung, pha vo tinh giao hoan."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.lww-register]
requires: [sd.fp.or-set-quan-sat-xoa]
concepts: [sd.fp.lww-register]
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
Bốn CRDT vừa học đều GIỮ LẠI mọi phiên bản khi hợp nhất — max từng
ô, union, tag kèm tombstone. Nhưng đôi khi chỉ cần MỘT giá trị đơn —
tên hiển thị của một hồ sơ, trạng thái online cuối cùng — và hợp lý
khi chấp nhận "bản MỚI NHẤT thắng". Câu hỏi: MỚI hơn theo cái gì, và
nếu hai bản ghi CÙNG lúc thì sao?
::::

::::explain{#lww-timestamp-lon-hon-thang}
`LWWRegister` giữ MỘT `giaTri`, kèm `thoiDiem` (dùng
`DongHoMoPhong`/`tienThoiGian` như các bài đo thời gian Ở T7.1/T7.2)
VÀ `replicaId` của bên đã ghi. `hopNhat` so `thoiDiem`: ai LỚN hơn
thắng:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }
function ghiLWW(giaTriMoi: string, thoiDiem: number, replicaId: string): LWWRegister {
  return { giaTri: giaTriMoi, thoiDiem, replicaId };
}
function hopNhat(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

const dh = taoDongHoMoPhong();
tienThoiGian(dh, 100);
const ghiA = ghiLWW("Bao", dh.thoiGianHienTai, "server-a");
tienThoiGian(dh, 50);
const ghiB = ghiLWW("Nam", dh.thoiGianHienTai, "server-b");
console.log("ghiA:", JSON.stringify(ghiA));
console.log("ghiB:", JSON.stringify(ghiB));
console.log("hopNhat(ghiA,ghiB):", JSON.stringify(hopNhat(ghiA, ghiB)));
console.log("hopNhat(ghiB,ghiA):", JSON.stringify(hopNhat(ghiB, ghiA)));
```

```text title=readonly
ghiA: {"giaTri":"Bao","thoiDiem":100,"replicaId":"server-a"}
ghiB: {"giaTri":"Nam","thoiDiem":150,"replicaId":"server-b"}
hopNhat(ghiA,ghiB): {"giaTri":"Nam","thoiDiem":150,"replicaId":"server-b"}
hopNhat(ghiB,ghiA): {"giaTri":"Nam","thoiDiem":150,"replicaId":"server-b"}
```

`ghiB` ghi Ở `thoiDiem: 150`, sau `ghiA` (`thoiDiem: 100`) — `hopNhat`
chọn `ghiB` bất kể thứ tự truyền tham số: `hopNhat(ghiA,ghiB)` VÀ
`hopNhat(ghiB,ghiA)` đều ra ĐÚNG `ghiB`. Đây là điểm khác biệt LỚN
nhất so với bốn CRDT trước: không có gì được "gộp" lại — `ghiA` bị bỏ
hoàn toàn, không còn một dấu vết nào của `"Bao"` trong kết quả.
::::

::::example{#tie-break-bang-replicaid-de-giao-hoan}
Nếu hai bản ghi có CÙNG `thoiDiem` (đồng hồ mô phỏng không phân biệt
được ai "trước" ai) — cần một luật tie-break RÕ RÀNG, nếu không
`hopNhat` sẽ phụ thuộc thứ tự gọi, phá vỡ tính giao hoán đã học Ở
bài 2. So `replicaId`: chuỗi LỚN hơn thắng:

```typescript title=readonly
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }
function ghiLWW(giaTriMoi: string, thoiDiem: number, replicaId: string): LWWRegister {
  return { giaTri: giaTriMoi, thoiDiem, replicaId };
}
function hopNhat(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

const ghiC = ghiLWW("Chau", 200, "server-a");
const ghiD = ghiLWW("Duy", 200, "server-b");
console.log("ghiC:", JSON.stringify(ghiC));
console.log("ghiD:", JSON.stringify(ghiD));
console.log("hopNhat(ghiC,ghiD):", JSON.stringify(hopNhat(ghiC, ghiD)));
console.log("hopNhat(ghiD,ghiC):", JSON.stringify(hopNhat(ghiD, ghiC)));
console.log("hai chieu GIONG HET nhau (doi xung, dam bao giao hoan)?", JSON.stringify(hopNhat(ghiC, ghiD)) === JSON.stringify(hopNhat(ghiD, ghiC)));
```

```text title=readonly
ghiC: {"giaTri":"Chau","thoiDiem":200,"replicaId":"server-a"}
ghiD: {"giaTri":"Duy","thoiDiem":200,"replicaId":"server-b"}
hopNhat(ghiC,ghiD): {"giaTri":"Duy","thoiDiem":200,"replicaId":"server-b"}
hopNhat(ghiD,ghiC): {"giaTri":"Duy","thoiDiem":200,"replicaId":"server-b"}
hai chieu GIONG HET nhau (doi xung, dam bao giao hoan)? true
```

`ghiC` VÀ `ghiD` cùng `thoiDiem: 200` — nhánh đầu (`a.thoiDiem !==
b.thoiDiem`) không kích hoạt, rơi xuống so `replicaId`: chuỗi
`"server-b"` lớn hơn `"server-a"` (so theo thứ tự ký tự), nên `ghiD`
thắng. Quan trọng hơn cả kết quả: `hopNhat(ghiC,ghiD)` VÀ
`hopNhat(ghiD,ghiC)` ra ĐÚNG CÙNG một kết quả — nếu tie-break chỉ đơn
giản là "tham số ĐẦU tiên thắng khi hoà", hai lời gọi đó sẽ cho hai
kết quả KHÁC nhau, và `hopNhat` không còn giao hoán nữa.
::::

::::predict{#doan-thoi-diem-lon-luon-thang commitOnce}
`ghiE = ghiLWW("Em", 500, "server-z")` VÀ `ghiF = ghiLWW("Phu", 300,
"server-a")`. So sánh CHỈ dựa vào `replicaId` theo thứ tự ký tự,
`"server-a"` ĐỨNG TRƯỚC `"server-z"`. `hopNhat(ghiE, ghiF)` trả về
bản ghi nào?

:::opt{correct}
`ghiE` (`"Em"`, `thoiDiem: 500`) — nhánh ĐẦU của `hopNhat` so
`thoiDiem` TRƯỚC (`500 !== 300`), VÀ `500 > 300`, nên hàm return
NGAY Ở đó; nhánh so `replicaId` không bao giờ được chạm tới trong
trường hợp này
:::
:::opt
`ghiF` (`"Phu"`, `replicaId: "server-a"`) — vì `"server-a"` đứng
TRƯỚC `"server-z"` theo thứ tự ký tự, VÀ tie-break dùng `replicaId`
để quyết định
::why
Áp dụng nhánh tie-break (so `replicaId`) trong khi tình huống này
KHÔNG hề chạm tới nhánh đó — hai `thoiDiem` (`500` VÀ `300`) khác
nhau hoàn toàn.

Chỗ lệch: dòng ĐẦU tiên của `hopNhat` là `if (a.thoiDiem !==
b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;` — với `ghiE.
thoiDiem = 500` VÀ `ghiF.thoiDiem = 300`, điều kiện `500 !== 300` là
`true`, hàm return NGAY từ nhánh này, không bao giờ chạy tới dòng so
`replicaId`. Tie-break bằng `replicaId` CHỈ áp dụng khi hai
`thoiDiem` HOÀN TOÀN bằng nhau — nó không phải luật "so mọi lúc",
nó là luật DỰ PHÒNG cho đúng MỘT trường hợp: hoà thời gian.
::
:::
::::

::::code{#viet_hop_nhat_lww}
Hoàn thiện `hopNhat` cho `LWWRegister` — nếu `a.thoiDiem` khác
`b.thoiDiem`, trả về bản ghi có `thoiDiem` LỚN hơn. Nếu BẰNG nhau,
trả về bản ghi có `replicaId` LỚN hơn (so chuỗi).

```typescript title=starter
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }

function hopNhat(a: LWWRegister, b: LWWRegister): LWWRegister {
  ___
}

const gT1: LWWRegister = { giaTri: "cu", thoiDiem: 10, replicaId: "r1" };
const gT2: LWWRegister = { giaTri: "moi", thoiDiem: 20, replicaId: "r2" };
const kqT = hopNhat(gT1, gT2);
console.log(kqT.giaTri, kqT.thoiDiem);
```

```typescript title=solution
interface LWWRegister { giaTri: string; thoiDiem: number; replicaId: string; }

function hopNhat(a: LWWRegister, b: LWWRegister): LWWRegister {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

const gT1: LWWRegister = { giaTri: "cu", thoiDiem: 10, replicaId: "r1" };
const gT2: LWWRegister = { giaTri: "moi", thoiDiem: 20, replicaId: "r2" };
const kqT = hopNhat(gT1, gT2);
console.log(kqT.giaTri, kqT.thoiDiem);
```

```typescript title=test
const w1: LWWRegister = { giaTri: "cu", thoiDiem: 5, replicaId: "za" };
const w2: LWWRegister = { giaTri: "moi", thoiDiem: 9, replicaId: "ab" };
const kqW = hopNhat(w1, w2);
if (kqW.giaTri !== "moi") throw new Error("thoiDiem lon hon (9 > 5) phai thang, bat ke replicaId");

const w3: LWWRegister = { giaTri: "hoa-a", thoiDiem: 7, replicaId: "server-a" };
const w4: LWWRegister = { giaTri: "hoa-b", thoiDiem: 7, replicaId: "server-b" };
const kqHoa1 = hopNhat(w3, w4);
const kqHoa2 = hopNhat(w4, w3);
if (kqHoa1.giaTri !== "hoa-b") throw new Error("hoa thoiDiem phai tie-break bang replicaId lon hon (server-b > server-a)");
if (kqHoa1.giaTri !== kqHoa2.giaTri) throw new Error("hopNhat phai giao hoan -- hopNhat(w3,w4) va hopNhat(w4,w3) phai ra CUNG ket qua");

const truoc1 = JSON.stringify(w1);
const truoc2 = JSON.stringify(w2);
hopNhat(w1, w2);
if (JSON.stringify(w1) !== truoc1) throw new Error("KHONG duoc mutate a truyen vao");
if (JSON.stringify(w2) !== truoc2) throw new Error("KHONG duoc mutate b truyen vao");
```

:::hints
- kind: attention
  body: "Neu a.thoiDiem khac b.thoiDiem, tra ve ben co thoiDiem LON HON. Chi khi HAI thoiDiem BANG NHAU moi so tiep a.replicaId va b.replicaId (so chuoi bang >), tra ve ben co replicaId lon hon."
- kind: strategy
  body: "if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b; return a.replicaId > b.replicaId ? a : b;"
- kind: one-line
  body: "if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b; return a.replicaId > b.replicaId ? a : b;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "moi 20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một giá trị đơn, timestamp lớn hơn thắng, tie-break đảm bảo giao
hoán thật sự. Nhưng khi `ghiA` ("Bao") thua trong ví dụ Ở trên, nó
biến mất HOÀN TOÀN — không như `GCounter` hay `OR-Set`, nơi mọi đóng
góp đều được GIỮ LẠI theo cách nào đó. Đây là một đánh đổi lớn, đáng
nhìn thẳng vào.
::::

::::reflect{#nghi-lai}
`LWWRegister` là CRDT đơn giản nhất trong quest này — không `Set`
gộp lại, không mảng tag, chỉ MỘT giá trị VÀ một phép so sánh. Nhưng
sự đơn giản đó có giá: hợp nhất `LWWRegister` LÀ phép CHỌN, không
phải phép GỘP — kết quả luôn LÀ một trong hai bản ghi đầu vào, không
bao giờ là một giá trị pha trộn cả hai. Bốn CRDT trước giữ MỌI đóng
góp; `LWWRegister` chỉ giữ đúng MỘT. Bài sau đặt hai cách tiếp cận
này cạnh nhau để thấy rõ cái giá phải trả.
::::

::::checkpoint{mastery=0.81}
::::
