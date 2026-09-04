---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.truy-van-xep-hang
title: "Truy vấn xếp hạng: top N VÀ hạng của một người"
summary: "layTopN doc O(N) tu DAU mang da sap xep (bxh.slice(0,n)), timHangCuaNguoiChoi tim VI TRI trong mang roi cong 1. Ca hai deu tan dung tinh chat 'LUON sap xep san' tu bai truoc -- KHONG can sap lai. Hai nguoi CHOI diem BANG nhau (p1=70 roi p2=70, p2 chen truoc) nhan HAI hang KHAC nhau (p2=1, p1=2) -- ham chi dem VI TRI, khong dong hang theo diem."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.truy-van-xep-hang]
requires: [sd.bang-xep-hang-thoi-gian-thuc]
concepts: [sd.truy-van-xep-hang]
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
`capNhatDiem` giữ mảng LUÔN sắp xếp — bài trước làm đúng một việc, làm
kỹ. Giờ tận dụng nó: hai câu hỏi người chơi hỏi NHIỀU nhất, "top mấy
người dẫn đầu" VÀ "tôi đang đứng hạng bao nhiêu", đều trả lời được mà
không cần TÍNH toán gì thêm.
::::

::::explain{#top-n-va-hang-ca-nhan}
Vì `BangXepHang` LUÔN Ở trạng thái sắp xếp GIẢM dần (bất biến bài trước
giữ), "top N" chỉ LÀ đọc N phần tử ĐẦU tiên — VÀ "hạng của một người" chỉ
LÀ tìm VỊ TRÍ của họ trong mảng rồi cộng `1` (vì mảng đánh chỉ số từ
`0`, còn hạng bắt đầu từ `1`):

```typescript title=readonly
interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];

function taoBangXepHang(): BangXepHang { return []; }

function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const phanTuGiua = bxh[giua];
    if (phanTuGiua !== undefined && phanTuGiua.diem > diem) {
      trai = giua + 1;
    } else {
      phai = giua;
    }
  }
  return trai;
}

function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

function layTopN(bxh: BangXepHang, n: number): NguoiChoi[] {
  return bxh.slice(0, n);
}

function timHangCuaNguoiChoi(bxh: BangXepHang, ten: string): number | undefined {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  if (viTri === -1) return undefined;
  return viTri + 1;
}

const bxh = taoBangXepHang();
capNhatDiem(bxh, "an", 95);
capNhatDiem(bxh, "binh", 80);
capNhatDiem(bxh, "chi", 70);
capNhatDiem(bxh, "dung", 60);
capNhatDiem(bxh, "em", 40);

console.log("top 3:", layTopN(bxh, 3).map((ng) => ng.ten));
console.log("hang cua dung:", timHangCuaNguoiChoi(bxh, "dung"));
console.log("hang cua nguoi khong ton tai:", timHangCuaNguoiChoi(bxh, "khong-ton-tai"));
```

```text title=readonly
top 3: [ 'an', 'binh', 'chi' ]
hang cua dung: 4
hang cua nguoi khong ton tai: undefined
```

`layTopN` chỉ LÀ `bxh.slice(0, n)` — không quét gì thêm, không so sánh
điểm số, vì mảng ĐÃ sắp xếp sẵn từ trước. `timHangCuaNguoiChoi` dùng
`findIndex` để tìm vị trí `"dung"` (chỉ số `3`, tính từ `0`), rồi cộng
`1` ra hạng `4`. Người không tồn tại trong bảng khiến `findIndex` trả về
`-1`, VÀ hàm trả `undefined` thay vì một con số vô nghĩa.
::::

::::example{#top-vuot-so-luong-va-hang-cuoi}
Xin nhiều hơn số người chơi thực TẾ không hề gây lỗi — VÀ hạng của người
đứng đầu LUÔN LÀ `1`, người đứng cuối LUÔN LÀ đúng số người chơi:

```typescript title=readonly
interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];

function taoBangXepHang(): BangXepHang { return []; }

function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const phanTuGiua = bxh[giua];
    if (phanTuGiua !== undefined && phanTuGiua.diem > diem) {
      trai = giua + 1;
    } else {
      phai = giua;
    }
  }
  return trai;
}

function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

function layTopN(bxh: BangXepHang, n: number): NguoiChoi[] {
  return bxh.slice(0, n);
}

function timHangCuaNguoiChoi(bxh: BangXepHang, ten: string): number | undefined {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  if (viTri === -1) return undefined;
  return viTri + 1;
}

// tai lap dung trang thai: bang xep hang 2 nguoi choi
const bxh = taoBangXepHang();
capNhatDiem(bxh, "giang", 55);
capNhatDiem(bxh, "hoa", 30);

console.log("top 10 (nhieu hon so nguoi choi thuc te):", layTopN(bxh, 10).map((ng) => ng.ten));
console.log("hang cua giang (dung dau):", timHangCuaNguoiChoi(bxh, "giang"));
console.log("hang cua hoa (cuoi bang):", timHangCuaNguoiChoi(bxh, "hoa"));
```

```text title=readonly
top 10 (nhieu hon so nguoi choi thuc te): [ 'giang', 'hoa' ]
hang cua giang (dung dau): 1
hang cua hoa (cuoi bang): 2
```

`layTopN(bxh, 10)` xin `10` người trong khi bảng chỉ có `2` — `slice`
tự động trả về TOÀN bộ những gì có, không hề ném lỗi vì "thiếu" phần tử.
`"giang"` đứng đầu nhận hạng `1`; `"hoa"` đứng cuối nhận đúng hạng `2` —
bằng SỐ người chơi hiện có.
::::

::::predict{#doan-diem-bang-nhau-khong-dong-hang commitOnce}
`"p1"` ghi `70` điểm trước, RỒI `"p2"` ghi ĐÚNG `70` điểm (bằng hệt) sau
đó — mảng trở thành `["p2", "p1"]` (như bài trước đã chỉ ra: điểm bằng
nhau thì người ĐẾN sau chen NGAY trước người cũ). `timHangCuaNguoiChoi`
trả về hạng của `"p1"` VÀ `"p2"` LÀ bao nhiêu?

:::opt{correct}
`"p2"` hạng `1`, `"p1"` hạng `2` — dù điểm số BẰNG nhau, hai người vẫn
nhận HAI hạng khác nhau, vì `timHangCuaNguoiChoi` chỉ đếm VỊ TRÍ trong
mảng (`findIndex` cộng `1`), không hề so sánh điểm SỐ để gộp hạng
:::
:::opt
Cả hai đều hạng `1` (đồng hạng) — vì điểm số bằng nhau, hệ thống PHẢI xếp
họ ngang hàng nhau, giống cách các bảng xếp hạng thể thao vẫn làm
::why
Nhầm "điểm bằng nhau" VỚI "đồng hạng" — nhưng `timHangCuaNguoiChoi` không
hề CÓ bước nào so sánh điểm của người này với người khác để gộp hạng.

Chỗ lệch: hàm chỉ làm ĐÚNG một việc — `bxh.findIndex((ng) => ng.ten ===
ten)` tìm VỊ TRÍ, rồi `+ 1`. `"p2"` đứng Ở chỉ số `0` (hạng `1`), `"p1"`
đứng Ở chỉ số `1` (hạng `2`) — đơn thuần LÀ vị trí trong mảng, hoàn toàn
không đọc tới trường `diem` của bất kỳ ai để quyết định đồng hạng hay
không.
::
:::
::::

::::code{#viet_tim_hang_cua_nguoi_choi}
Hoàn thiện `timHangCuaNguoiChoi` — TÌM vị trí người chơi bằng
`findIndex`; nếu không tồn tại (`-1`) trả `undefined`, ngược lại trả về
vị trí CỘNG thêm `1`.

```typescript title=starter
interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];

function taoBangXepHang(): BangXepHang { return []; }

function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const phanTuGiua = bxh[giua];
    if (phanTuGiua !== undefined && phanTuGiua.diem > diem) {
      trai = giua + 1;
    } else {
      phai = giua;
    }
  }
  return trai;
}

function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

function layTopN(bxh: BangXepHang, n: number): NguoiChoi[] {
  return bxh.slice(0, n);
}

function timHangCuaNguoiChoi(bxh: BangXepHang, ten: string): number | undefined {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  ___
}

const bxhX = taoBangXepHang();
capNhatDiem(bxhX, "x1", 10);
console.log(timHangCuaNguoiChoi(bxhX, "x1"));
```

```typescript title=solution
interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[];

function taoBangXepHang(): BangXepHang { return []; }

function timViTriChenNhiPhan(bxh: BangXepHang, diem: number): number {
  let trai = 0;
  let phai = bxh.length;
  while (trai < phai) {
    const giua = Math.floor((trai + phai) / 2);
    const phanTuGiua = bxh[giua];
    if (phanTuGiua !== undefined && phanTuGiua.diem > diem) {
      trai = giua + 1;
    } else {
      phai = giua;
    }
  }
  return trai;
}

function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

function layTopN(bxh: BangXepHang, n: number): NguoiChoi[] {
  return bxh.slice(0, n);
}

function timHangCuaNguoiChoi(bxh: BangXepHang, ten: string): number | undefined {
  const viTri = bxh.findIndex((ng) => ng.ten === ten);
  if (viTri === -1) return undefined;
  return viTri + 1;
}

const bxhX = taoBangXepHang();
capNhatDiem(bxhX, "x1", 10);
console.log(timHangCuaNguoiChoi(bxhX, "x1"));
```

```typescript title=test
const bxh = taoBangXepHang();
capNhatDiem(bxh, "an", 95);
capNhatDiem(bxh, "binh", 80);
capNhatDiem(bxh, "chi", 70);
capNhatDiem(bxh, "dung", 60);
capNhatDiem(bxh, "em", 40);

if (timHangCuaNguoiChoi(bxh, "an") !== 1) throw new Error("nguoi dung dau bang phai co hang 1");
if (timHangCuaNguoiChoi(bxh, "dung") !== 4) throw new Error("dung phai co hang 4 (vi tri thu 4 trong bang)");
if (timHangCuaNguoiChoi(bxh, "em") !== 5) throw new Error("em (thap nhat) phai co hang 5");
if (timHangCuaNguoiChoi(bxh, "khong-ton-tai") !== undefined) throw new Error("nguoi choi khong ton tai phai tra ve undefined, khong duoc tra ve 0 hay -1");

if (layTopN(bxh, 3).map((ng) => ng.ten).join(",") !== "an,binh,chi") throw new Error("top 3 phai la 3 nguoi dan dau, dung thu tu");
if (layTopN(bxh, 100).length !== 5) throw new Error("xin top nhieu hon so nguoi choi thuc te phai tra ve TOAN BO, khong duoc nem loi");

capNhatDiem(bxh, "chi", 96); // chi vuot len dau
if (timHangCuaNguoiChoi(bxh, "chi") !== 1) throw new Error("sau khi chi cap nhat diem vuot len, hang phai phan anh trang thai MOI NHAT (khong bi cache cu)");
if (timHangCuaNguoiChoi(bxh, "an") !== 2) throw new Error("an bi chi vuot qua, hang cua an phai lui xuong 2");
```

:::hints
- kind: attention
  body: "Sau khi tim viTri, phai kiem tra truong hop viTri === -1 (khong tim thay) va tra ve undefined; nguoc lai tra ve viTri + 1 (hang bat dau tu 1, khong phai 0)."
- kind: strategy
  body: "if (viTri === -1) return undefined; return viTri + 1; -- dung hai cau lenh rieng, khong gop chung thanh mot bieu thuc."
- kind: one-line
  body: "if (viTri === -1) return undefined; return viTri + 1;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chín mảnh đã xong: thứ tự tin nhắn, online, đã đọc cả nhóm, đa kênh, gộp
thông báo, mã ô, lân cận, chèn nhị phân, top-N và hạng. BOSS ráp TẤT cả
vào một trận đấu trực tiếp.
::::

::::reflect{#nghi-lai}
`layTopN` VÀ `timHangCuaNguoiChoi` không hề PHÁT minh thuật toán mới — cả
hai chỉ đơn thuần ĐỌC từ một cấu trúc đã được GIỮ bất biến (luôn sắp
xếp) từ bài trước. Bất biến đúng LÀ thứ khiến câu hỏi khó trở nên rẻ.
::::

::::checkpoint{mastery=0.82}
::::
