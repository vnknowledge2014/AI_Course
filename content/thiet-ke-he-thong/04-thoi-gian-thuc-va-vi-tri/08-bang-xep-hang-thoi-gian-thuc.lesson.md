---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.bang-xep-hang-thoi-gian-thuc
title: "Bảng xếp hạng: chèn nhị phân, LUÔN giữ thứ tự"
summary: "capNhatDiem tu VIET binary insertion (khong dung .sort() cua thu vien) de mang BangXepHang LUON sap xep GIAM dan theo diem NGAY sau moi lan cap nhat -- an tu 50 len 90 chuyen tu cuoi bang LEN dau chi qua mot lan xoa+chen, khong sap lai toan bo mang. Diem tut manh (100 xuong 10) day nguoi choi do ROI xuong CUOI, van giu thu tu dung cho ca nhung nguoi con lai."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.bang-xep-hang-thoi-gian-thuc]
requires: [sd.tim-ban-gan]
concepts: [sd.bang-xep-hang-thoi-gian-thuc]
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
Ô lưới VÀ lân cận đóng phần Proximity. Mảnh CUỐI — Gaming Leaderboard —
đổi hẳn dữ liệu: không còn toạ độ, mà LÀ điểm số THAY đổi liên tục, VÀ
màn hình LUÔN phải hiện đúng thứ hạng, NGAY sau mỗi lần ai đó ghi điểm.
::::

::::explain{#chen-nhi-phan-giu-thu-tu}
Gọi lại `.sort()` sau MỖI lần một người chơi ghi điểm thì tốn công duyệt
LẠI toàn bộ mảng — trong khi mảng gần như đã sắp xếp sẵn, chỉ MỘT người
vừa đổi vị trí. Cách rẻ hơn: tự VIẾT một phép chèn nhị phân — tìm đúng vị
trí người chơi ĐÓ nên đứng, rồi chèn thẳng vào đó, giữ toàn bộ mảng vẫn
sắp xếp GIẢM dần theo điểm:

```typescript title=readonly
interface NguoiChoi { ten: string; diem: number; }
type BangXepHang = NguoiChoi[]; // luon sap xep GIAM dan theo diem

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

function tenTheoThuTu(bxh: BangXepHang): string[] { return bxh.map((ng) => ng.ten); }

const bxh = taoBangXepHang();
capNhatDiem(bxh, "an", 50);
capNhatDiem(bxh, "binh", 80);
capNhatDiem(bxh, "chi", 65);
console.log("sau khi them 3 nguoi choi (an=50,binh=80,chi=65):", tenTheoThuTu(bxh));

capNhatDiem(bxh, "an", 90);
console.log("sau khi an cap nhat diem len 90:", tenTheoThuTu(bxh));
```

```text title=readonly
sau khi them 3 nguoi choi (an=50,binh=80,chi=65): [ 'binh', 'chi', 'an' ]
sau khi an cap nhat diem len 90: [ 'an', 'binh', 'chi' ]
```

`capNhatDiem` LUÔN làm đúng hai bước: xoá bản GHI cũ (nếu người chơi đã
tồn tại), rồi CHÈN lại vào đúng vị trí mới bằng `timViTriChenNhiPhan` —
một phép tìm kiếm NHỊ PHÂN, không quét tuyến tính. Khi `"an"` ghi thêm
điểm LÊN `90`, nó bị xoá khỏi vị trí CUỐI rồi chèn NGAY lên đầu — mảng
vẫn giữ nguyên tính chất "sắp xếp giảm dần" SAU đúng một lần cập nhật.
::::

::::example{#diem-tut-manh-roi-xuong-cuoi}
Chèn nhị phân hoạt động GIỐNG hệt khi điểm số ĐI XUỐNG — người chơi tụt
điểm mạnh sẽ bị đẩy về CUỐI bảng, ngay lập tức, không cần thao tác thêm:

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

function tenTheoThuTu(bxh: BangXepHang): string[] { return bxh.map((ng) => ng.ten); }

// tai lap dung trang thai: 3 nguoi choi da co san
const bxh = taoBangXepHang();
capNhatDiem(bxh, "dung", 100);
capNhatDiem(bxh, "em", 80);
capNhatDiem(bxh, "giang", 60);
console.log("truoc khi dung tut diem:", tenTheoThuTu(bxh));

capNhatDiem(bxh, "dung", 10); // diem TUT manh, phai roi xuong CUOI
console.log("sau khi dung tut diem xuong 10:", tenTheoThuTu(bxh));
```

```text title=readonly
truoc khi dung tut diem: [ 'dung', 'em', 'giang' ]
sau khi dung tut diem xuong 10: [ 'em', 'giang', 'dung' ]
```

`"dung"` đứng ĐẦU bảng với điểm `100`, RỒI tụt xuống `10` — thấp hơn CẢ
`"giang"` (điểm `60`, đang Ở cuối bảng). `capNhatDiem` xoá `"dung"` khỏi
vị trí đầu, RỒI `timViTriChenNhiPhan` tìm đúng vị trí MỚI (cuối cùng) cho
điểm `10` — kết quả LÀ `"dung"` rơi thẳng xuống CUỐI, `"em"` VÀ `"giang"`
tự động LÊN hạng mà không cần cập nhật riêng.
::::

::::predict{#doan-diem-bang-nhau-chen-truoc commitOnce}
`"binh"` đang Ở bảng MỘT MÌNH với điểm `70`. `"chi"` VỪA tham gia, cũng
ghi ĐÚNG `70` điểm — bằng hệt `"binh"`. Sau khi `capNhatDiem(bxh, "chi",
70)` chạy xong, `"chi"` đứng Ở vị trí NÀO so với `"binh"`?

:::opt{correct}
`"chi"` đứng NGAY TRƯỚC `"binh"` — `timViTriChenNhiPhan` chỉ dịch vị trí
chèn sang phải khi phần tử Ở giữa có điểm LỚN HƠN (`phanTuGiua.diem >
diem`, so sánh nghiêm ngặt); điểm bằng nhau KHÔNG thoả điều kiện đó, nên
vị trí chèn dừng lại NGAY trước `"binh"`
:::
:::opt
`"chi"` đứng NGAY SAU `"binh"` — người đến TRƯỚC giữ vị trí cao hơn khi
điểm số bằng nhau, giống quy tắc "ai đến trước xếp trước"
::why
Nhầm quy tắc "đến trước xếp trước" VỚI cách `timViTriChenNhiPhan` THẬT
sự hoạt động — hàm này không hề biết ai gia nhập bảng TRƯỚC, nó chỉ so
sánh điểm SỐ.

Chỗ lệch: điều kiện `phanTuGiua.diem > diem` dùng dấu `>` NGHIÊM ngặt —
khi `phanTuGiua.diem === diem` (cả hai đều LÀ `70`), điều kiện SAI, nhánh
`else` chạy, đẩy `phai = giua`. Vòng lặp kết thúc VỚI vị trí chèn LÀ
`0` — đứng TRƯỚC `"binh"`, không phải sau.
::
:::
::::

::::code{#viet_tim_vi_tri_chen_nhi_phan}
Hoàn thiện `timViTriChenNhiPhan` — vòng `while` đã CÓ sẵn khung; viết
điều kiện quyết định thu hẹp về BÊN nào (`trai` hay `phai`) dựa vào điểm
Ở giữa mảng.

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
    ___
  }
  return trai;
}

function capNhatDiem(bxh: BangXepHang, ten: string, diemMoi: number): void {
  const viTriCu = bxh.findIndex((ng) => ng.ten === ten);
  if (viTriCu !== -1) bxh.splice(viTriCu, 1);
  const viTriMoi = timViTriChenNhiPhan(bxh, diemMoi);
  bxh.splice(viTriMoi, 0, { ten, diem: diemMoi });
}

const bxhX = taoBangXepHang();
capNhatDiem(bxhX, "x1", 10);
capNhatDiem(bxhX, "x2", 30);
console.log(bxhX.map((ng) => ng.ten));
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

const bxhX = taoBangXepHang();
capNhatDiem(bxhX, "x1", 10);
capNhatDiem(bxhX, "x2", 30);
console.log(bxhX.map((ng) => ng.ten));
```

```typescript title=test
const bxh = taoBangXepHang();
capNhatDiem(bxh, "an", 50);
capNhatDiem(bxh, "binh", 80);
capNhatDiem(bxh, "chi", 65);
if (bxh.map((ng) => ng.ten).join(",") !== "binh,chi,an") throw new Error("sau 3 lan them, thu tu phai la binh(80),chi(65),an(50)");

capNhatDiem(bxh, "an", 90);
if (bxh.map((ng) => ng.ten).join(",") !== "an,binh,chi") throw new Error("an cap nhat len 90 phai LEN dau bang");
if (bxh.length !== 3) throw new Error("cap nhat KHONG duoc tao them ban ghi trung, van phai la 3 nguoi choi");

capNhatDiem(bxh, "binh", 5);
if (bxh.map((ng) => ng.ten).join(",") !== "an,chi,binh") throw new Error("binh tut xuong 5 phai ROI xuong CUOI bang");

const bxhTie = taoBangXepHang();
capNhatDiem(bxhTie, "p1", 70);
capNhatDiem(bxhTie, "p2", 70);
if (bxhTie.map((ng) => ng.ten).join(",") !== "p2,p1") throw new Error("diem bang nhau: nguoi den SAU phai duoc chen NGAY TRUOC nguoi cu (dung != strict greater-than)");

const bxhRong = taoBangXepHang();
if (timViTriChenNhiPhan(bxhRong, 100) !== 0) throw new Error("bang rong, vi tri chen phai la 0");
```

:::hints
- kind: attention
  body: "phanTuGiua co the la undefined (do noUncheckedIndexedAccess) -- phai kiem tra 'phanTuGiua !== undefined' TRUOC khi so sanh phanTuGiua.diem."
- kind: strategy
  body: "Neu phanTuGiua ton tai VA phanTuGiua.diem > diem (diem giua LON HON diem dang chen) thi doi tuong dang chen phai dung SAU no -- trai = giua + 1. Nguoc lai (nho hon hoac BANG) thi phai = giua."
- kind: one-line
  body: "if (phanTuGiua !== undefined && phanTuGiua.diem > diem) { trai = giua + 1; } else { phai = giua; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "x2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng LUÔN sắp xếp, ngay sau mỗi lần cập nhật. Bài cuối cùng trước BOSS:
tận dụng tính chất đó để trả lời "top mấy" VÀ "hạng bao nhiêu" thật rẻ.
::::

::::reflect{#nghi-lai}
`timViTriChenNhiPhan` không hề PHỨC tạp hơn một phép tìm kiếm nhị phân
thông thường — điều đáng nhớ LÀ nó biến `capNhatDiem` từ "sắp xếp LẠI
toàn bộ" thành "chèn đúng MỘT chỗ". Giữ bất biến "LUÔN sắp xếp sẵn" chính
LÀ thứ khiến các câu hỏi Ở bài sau trở nên rẻ.
::::

::::checkpoint{mastery=0.80}
::::
