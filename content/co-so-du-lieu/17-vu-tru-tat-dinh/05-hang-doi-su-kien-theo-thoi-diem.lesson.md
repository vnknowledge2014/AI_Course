---
id: co-so-du-lieu.vu-tru-tat-dinh.hang-doi-su-kien-theo-thoi-diem
title: "Hàng đợi sự kiện theo thời điểm"
summary: "layTiepTheo lấy ra sự kiện có thoiDiem NHỎ NHẤT trong hàng đợi (quét tuyến tính, không phải thứ tự chèn vào). Ba sự kiện chèn theo thứ tự c(300), a(100), b(200) -- layTiepTheo gọi ba lần liên tiếp trả về đúng a, b, c (tăng dần theo thoiDiem), không phải thứ tự chèn c, a, b. Hàng đợi rỗng thì layTiepTheo trả về undefined, không lỗi."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.hang-doi-su-kien-theo-thoi-diem]
requires: [db.dong-ho-ao-khong-goi-datenow]
concepts: [db.hang-doi-su-kien-theo-thoi-diem]
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
Đồng hồ ảo (bài trước) BIẾT "bây giờ" LÀ mấy — nhưng một mô phỏng
CÓ nhiều sự kiện xảy ra Ở NHIỀU thời điểm khác nhau. Xử LÝ theo thứ
tự NÀO — thứ tự chúng được TẠO ra, hay thứ tự thời gian?
::::

::::explain{#hang-doi-theo-thoi-diem}
`layTiepTheo` lấy RA sự kiện CÓ `thoiDiem` NHỎ NHẤT trong hàng đợi
(quét TUYẾN tính toàn bộ mảng, KHÔNG phải thứ tự CHÈN vào):

```typescript title=readonly
interface SuKien { thoiDiem: number; nhan: string; }

function themSuKien(hangDoi: SuKien[], sk: SuKien): void {
  hangDoi.push(sk);
}

function layTiepTheo(hangDoi: SuKien[]): SuKien | undefined {
  if (hangDoi.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hangDoi.length; i++) {
    if (hangDoi[i]!.thoiDiem < hangDoi[idxNhoNhat]!.thoiDiem) idxNhoNhat = i;
  }
  return hangDoi.splice(idxNhoNhat, 1)[0];
}

const hd: SuKien[] = [];
themSuKien(hd, { thoiDiem: 300, nhan: "c" });
themSuKien(hd, { thoiDiem: 100, nhan: "a" });
themSuKien(hd, { thoiDiem: 200, nhan: "b" });

console.log("thu tu chen:", hd.map((sk) => sk.nhan).join(","));
console.log("lay 1:", layTiepTheo(hd)?.nhan);
console.log("lay 2:", layTiepTheo(hd)?.nhan);
console.log("lay 3:", layTiepTheo(hd)?.nhan);
console.log("hang doi rong, lay tiep:", String(layTiepTheo(hd)));
```

```text title=readonly
thu tu chen: c,a,b
lay 1: a
lay 2: b
lay 3: c
hang doi rong, lay tiep: undefined
```

Ba sự kiện được CHÈN theo thứ tự `c, a, b` (`thoiDiem` `300, 100,
200`) — NHƯNG `layTiepTheo` LUÔN trả về sự kiện CÓ `thoiDiem` nhỏ
NHẤT còn LẠI, nên thứ tự LẤY ra LÀ `a, b, c` (tăng DẦN theo thời
điểm), hoàn toàn KHÁC thứ tự chèn. Hàng đợi RỖNG thì trả VỀ
`undefined` — dùng `String(...)` để in ra literal `"undefined"`,
KHÔNG phải chuỗi rỗng.
::::

::::example{#tai-sao-khong-phai-fifo}
Một hàng đợi THƯỜNG (FIFO, first-in-first-out) LẤY ra ĐÚNG theo thứ
tự chèn — nhưng mô PHỎNG cần thứ tự THỜI GIAN, không phải thứ tự
"ai được TẠO trước". Một sự kiện "gửi tin NHẮN lúc t=500" được LÊN
lịch TRƯỚC một sự kiện "hết giờ CHỜ lúc t=100" (do lập trình VIÊN
gọi hai dòng code THEO thứ tự đó) — nhưng vũ trụ mô phỏng PHẢI xử lý
sự kiện `t=100` TRƯỚC, đúng thứ tự THỜI gian, không phải thứ tự mã
nguồn.
::::

::::predict{#doan-chen-them-giua-chung commitOnce}
Sau khi ĐÃ lấy ra `a` (bài readonly TRÊN, hàng đợi giờ CHỈ còn `b`,
`c`), CHÈN thêm một sự kiện MỚI `{thoiDiem: 150, nhan: "d"}`. Gọi
`layTiepTheo` LẦN tiếp theo trả VỀ nhãn NÀO?
:::opt{correct}
`"d"` — `thoiDiem=150` nhỏ hơn CẢ `b` (`200`) LẪN `c` (`300`) đang
còn TRONG hàng đợi; `layTiepTheo` LUÔN quét lại TOÀN bộ hàng đợi HIỆN
tại, không hề "nhớ" thứ tự ĐàXỬ lý trước đó
:::
:::opt
`"b"` — `b` đã "xếp hàng" TRƯỚC `d` (được chèn SỚM hơn), nên PHẢI
được xử lý trước
::why
Trực giác NÀY áp DỤNG đúng cho hàng đợi FIFO — nhưng `layTiepTheo`
KHÔNG hề quan tâm thứ tự CHÈN, chỉ quan tâm `thoiDiem`.

Chỗ lệch: MỖI lần gọi, `layTiepTheo` quét LẠI từ đầu, so sánh
`thoiDiem` của MỌI phần tử ĐANG có trong `hangDoi` — không hề CÓ khái
niệm "đã ở hàng đợi LÂU hơn". `d` (`150`) nhỏ hơn CẢ `b` (`200`),
nên `d` được LẤY ra trước, DÙ nó vừa được chèn SAU CÙNG.
::
:::
::::

::::code{#viet_lay_tiep_theo}
Hoàn thiện `layTiepTheo` — so sánh `thoiDiem`, cập nhật `idxNhoNhat`
khi tìm thấy một phần TỬ nhỏ hơn.

```typescript title=starter
interface SuKien { thoiDiem: number; nhan: string; }

function themSuKien(hangDoi: SuKien[], sk: SuKien): void {
  hangDoi.push(sk);
}

function layTiepTheo(hangDoi: SuKien[]): SuKien | undefined {
  if (hangDoi.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hangDoi.length; i++) {
    ___
  }
  return hangDoi.splice(idxNhoNhat, 1)[0];
}

const hd: SuKien[] = [];
themSuKien(hd, { thoiDiem: 300, nhan: "c" });
themSuKien(hd, { thoiDiem: 100, nhan: "a" });
themSuKien(hd, { thoiDiem: 200, nhan: "b" });
console.log(layTiepTheo(hd)?.nhan);
```

```typescript title=solution
interface SuKien { thoiDiem: number; nhan: string; }

function themSuKien(hangDoi: SuKien[], sk: SuKien): void {
  hangDoi.push(sk);
}

function layTiepTheo(hangDoi: SuKien[]): SuKien | undefined {
  if (hangDoi.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hangDoi.length; i++) {
    if (hangDoi[i]!.thoiDiem < hangDoi[idxNhoNhat]!.thoiDiem) idxNhoNhat = i;
  }
  return hangDoi.splice(idxNhoNhat, 1)[0];
}

const hd: SuKien[] = [];
themSuKien(hd, { thoiDiem: 300, nhan: "c" });
themSuKien(hd, { thoiDiem: 100, nhan: "a" });
themSuKien(hd, { thoiDiem: 200, nhan: "b" });
console.log(layTiepTheo(hd)?.nhan);
```

```typescript title=test
const hd2: SuKien[] = [];
themSuKien(hd2, { thoiDiem: 300, nhan: "c" });
themSuKien(hd2, { thoiDiem: 100, nhan: "a" });
themSuKien(hd2, { thoiDiem: 200, nhan: "b" });

const l1 = layTiepTheo(hd2);
if (l1?.nhan !== "a") throw new Error("lay dau tien phai la 'a' (thoiDiem 100, nho nhat)");
const l2 = layTiepTheo(hd2);
if (l2?.nhan !== "b") throw new Error("lay thu hai phai la 'b' (thoiDiem 200)");
const l3 = layTiepTheo(hd2);
if (l3?.nhan !== "c") throw new Error("lay thu ba phai la 'c' (thoiDiem 300)");
const l4 = layTiepTheo(hd2);
if (l4 !== undefined) throw new Error("hang doi rong phai tra ve undefined");

const hd3: SuKien[] = [];
themSuKien(hd3, { thoiDiem: 100, nhan: "a" });
themSuKien(hd3, { thoiDiem: 200, nhan: "b" });
themSuKien(hd3, { thoiDiem: 300, nhan: "c" });
const laDau = layTiepTheo(hd3);
if (laDau?.nhan !== "a") throw new Error("da theo dung thu tu chen thi van phai lay dung thoiDiem nho nhat");
themSuKien(hd3, { thoiDiem: 150, nhan: "d" });
const laTiep = layTiepTheo(hd3);
if (laTiep?.nhan !== "d") throw new Error("them 'd' (150) vao GIUA khi 'b'(200) va 'c'(300) con trong hang doi -- phai lay 'd' truoc");

const hdMot: SuKien[] = [];
themSuKien(hdMot, { thoiDiem: 5, nhan: "mot" });
const layMot = layTiepTheo(hdMot);
if (layMot?.nhan !== "mot") throw new Error("hang doi 1 phan tu phai lay dung phan tu do");
const layRong = layTiepTheo(hdMot);
if (layRong !== undefined) throw new Error("sau khi lay het, hang doi rong phai tra ve undefined");

const hd4: SuKien[] = [];
themSuKien(hd4, { thoiDiem: 50, nhan: "p" });
themSuKien(hd4, { thoiDiem: 50, nhan: "q" });
const layHoa = layTiepTheo(hd4);
if (layHoa?.nhan !== "p") throw new Error("hoa thoiDiem: phan tu DUNG TRUOC trong mang ('p', chi so 0) phai thang, khong phai 'q'");
```

:::hints
- kind: attention
  body: "Neu thoiDiem cua phan tu i nho hon phan tu idxNhoNhat thi cap nhat idxNhoNhat = i -- mot dong."
- kind: strategy
  body: "if (hangDoi[i]!.thoiDiem < hangDoi[idxNhoNhat]!.thoiDiem) idxNhoNhat = i;"
- kind: one-line
  body: "if (hangDoi[i]!.thoiDiem < hangDoi[idxNhoNhat]!.thoiDiem) idxNhoNhat = i;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "a"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thứ tự thời gian đã ĐÚNG — nhưng nếu HAI sự kiện có ĐÚNG cùng một
`thoiDiem`, `layTiepTheo` chọn CÁI nào? Câu trả lời hiện tại có TẤT
định không?
::::

::::reflect{#nghi-lai}
`layTiepTheo` giải quyết đúng bài TOÁN "ai xảy ra TRƯỚC" — nhưng nó
CHƯA hề nói RÕ chuyện gì xảy ra khi HAI sự kiện hoà `thoiDiem`. Vòng
`for` dùng `<` (nghiêm ngặt), nên khi HOÀ, phần tử ĐỨNG trước trong
mảng "thắng" — nhưng đó LÀ một tác dụng PHỤ ngẫu nhiên của cách CÀI
đặt (quét tuyến tính), không phải một LUẬT tường minh. Bước tiếp
theo: biến "thắng do TÌNH cờ" thành một LUẬT rõ ràng.
::::

::::checkpoint{mastery=0.85}
::::
