---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.luu-tru-khu-trung-theo-hash
title: "Khử trùng theo hash: một khối, nhiều chủ"
summary: "luuKhoi(kho, noiDung) bam NOI DUNG khoi thanh mot hash -- neu hash DA co trong kho (noi dung TRUNG voi khoi khac) thi CHI tang soThamChieu, KHONG luu ban thu hai; hash MOI thi tao muc moi voi soThamChieu=1. Hai file cung noi dung 'phan-mo-dau-chung' cho ra CUNG mot hash, kho.khoi.size van la 1 du soThamChieu=2. xoaThamChieuKhoi giam dan -- khoi CHI bi xoa THAT su khoi kho khi soThamChieu ve 0 (tat ca file tham chieu deu da xoa)."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.luu-tru-khu-trung-theo-hash]
requires: [sd.luu-tru-theo-khoi]
concepts: [sd.luu-tru-khu-trung-theo-hash]
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
Khối tới không theo thứ tự đã được theo dõi đúng. Nhưng nhiều người
dùng khác nhau upload cùng một file (hoặc hai file chỉ khác VÀI byte
đầu) thì phần LỚN các khối bên trong LÀ giống hệt nhau. Lưu MỖI khối
đó lặp lại cho từng file LÀ lãng phí đĩa RÕ ràng — vậy phải nhận RA sự
trùng lặp đó bằng cách nào?
::::

::::explain{#khu-trung-theo-hash}
`luuKhoi` không định danh một khối bằng "nó thuộc file NÀO" — nó định
danh bằng chính NỘI dung của khối đó, qua một hàm băm. Nếu hash ĐÃ tồn
tại trong kho (nghĩa LÀ có khối khác trùng NỘI dung), nó chỉ tăng
`soThamChieu` — KHÔNG lưu thêm một bản SAO nào:

```typescript title=readonly
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
interface KhoiLuuTru { noiDung: string; soThamChieu: number; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1 });
  }
  return hash;
}
function xoaThamChieuKhoi(kho: KhoLuuTruKhoi, hash: string): void {
  const k = kho.khoi.get(hash);
  if (k === undefined) return;
  k.soThamChieu -= 1;
  if (k.soThamChieu <= 0) kho.khoi.delete(hash);
}

const kho = taoKhoLuuTruKhoi();
const hashA = luuKhoi(kho, "phan-mo-dau-chung");
console.log("file A luu khoi, hash:", hashA, "so tham chieu:", kho.khoi.get(hashA)?.soThamChieu);
const hashB = luuKhoi(kho, "phan-mo-dau-chung");
console.log("file B luu khoi TRUNG NOI DUNG, hash:", hashB, "so tham chieu:", kho.khoi.get(hashB)?.soThamChieu);
console.log("hash A === hash B:", hashA === hashB);
console.log("so khoi THAT SU luu trong kho:", kho.khoi.size);
```

```text title=readonly
file A luu khoi, hash: h1f2d6f27 so tham chieu: 1
file B luu khoi TRUNG NOI DUNG, hash: h1f2d6f27 so tham chieu: 2
hash A === hash B: true
so khoi THAT SU luu trong kho: 1
```

File A VÀ file B đều lưu khối MANG nội dung `"phan-mo-dau-chung"` —
`bamNoiDung` cho ra ĐÚNG cùng một `hash`, NÊN `luuKhoi` không tạo mục
mới cho lần gọi THỨ hai, nó chỉ tăng `soThamChieu` LÊN `2`. Dù đã gọi
`luuKhoi` hai lần, `kho.khoi.size` vẫn LÀ `1` — chỉ một bản NỘI dung
thật sự nằm trên đĩa.
::::

::::example{#xoa-tham-chieu-khong-xoa-khoi-neu-con-dung}
Xoá một file KHÔNG xoá ngay khối nó tham chiếu — `xoaThamChieuKhoi`
chỉ giảm bộ đếm; khối chỉ THẬT sự biến mất khỏi kho khi bộ đếm CHẠM
`0`, nghĩa LÀ không còn file NÀO cần tới nó nữa:

```typescript title=readonly
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
interface KhoiLuuTru { noiDung: string; soThamChieu: number; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1 });
  }
  return hash;
}
function xoaThamChieuKhoi(kho: KhoLuuTruKhoi, hash: string): void {
  const k = kho.khoi.get(hash);
  if (k === undefined) return;
  k.soThamChieu -= 1;
  if (k.soThamChieu <= 0) kho.khoi.delete(hash);
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: file A va B cung luu
// khoi "phan-mo-dau-chung", soThamChieu = 2
const kho = taoKhoLuuTruKhoi();
const hashA = luuKhoi(kho, "phan-mo-dau-chung");
const hashB = luuKhoi(kho, "phan-mo-dau-chung");

xoaThamChieuKhoi(kho, hashA); // file A bi xoa, gia bo goi ham nay
console.log("sau khi file A xoa tham chieu -- khoi con ton tai?", kho.khoi.has(hashA), "so tham chieu con lai:", kho.khoi.get(hashA)?.soThamChieu);

xoaThamChieuKhoi(kho, hashB); // file B cung xoa tham chieu
console.log("sau khi file B CUNG xoa tham chieu -- khoi con ton tai?", kho.khoi.has(hashB));
console.log("so khoi con lai trong kho:", kho.khoi.size);
```

```text title=readonly
sau khi file A xoa tham chieu -- khoi con ton tai? true so tham chieu con lai: 1
sau khi file B CUNG xoa tham chieu -- khoi con ton tai? false
so khoi con lai trong kho: 0
```

Sau khi file A xoá tham chiếu, khối VẪN còn TRONG kho (`has` trả về
`true`) — vì file B vẫn ĐANG dùng nó, `soThamChieu` chỉ giảm còn `1`.
Chỉ khi file B CŨNG xoá tham chiếu, `soThamChieu` chạm `0` VÀ
`xoaThamChieuKhoi` mới thật sự `delete` khối đó khỏi `kho.khoi`.
::::

::::predict{#doan-xoa-tham-chieu-hai-lan commitOnce}
Một khối chỉ có ĐÚNG một file tham chiếu (`soThamChieu = 1`).
`xoaThamChieuKhoi` được gọi, khối bị xoá thật khỏi kho (`soThamChieu`
chạm `0`). NGAY sau đó, cùng hàm được gọi LẦN nữa với đúng `hash` đó
(ví dụ do một lần thử LẠI (retry) gửi trùng yêu cầu xoá). Lần gọi THỨ
hai này gây ra điều gì?

:::opt{correct}
Không gì cả — dòng đầu tiên `const k = kho.khoi.get(hash)` trả về
`undefined` (khối đã bị xoá RỒI), NÊN hàm `return` ngay LẬP tức; không
có `soThamChieu` âm, không có lỗi nào xảy ra
:::
:::opt
`soThamChieu` bị trừ xuống `-1` — hàm vẫn thực hiện `k.soThamChieu -=
1` như bình thường, chỉ LÀ giá trị lúc này ÂM thay vì dương
::why
Nhầm "khối đã bị xoá khỏi Map" VỚI "khối vẫn còn Ở đó với số tham
chiếu bằng 0" — nhưng `delete` trong `xoaThamChieuKhoi` XOÁ hẳn mục
đó khỏi `kho.khoi`, không phải chỉ đặt `soThamChieu` bằng `0`.

Chỗ lệch: dòng `if (k.soThamChieu <= 0) kho.khoi.delete(hash);` Ở lần
gọi ĐẦU đã xoá hẳn mục khỏi `Map`. Lần gọi THỨ hai, `kho.khoi.get(hash)`
trả về `undefined` NGAY từ đầu — dòng `if (k === undefined) return;`
chặn hàm lại TRƯỚC khi kịp chạm tới `k.soThamChieu -= 1`. Không có
phép trừ NÀO xảy ra Ở lần gọi thứ hai.
::
:::
::::

::::code{#viet_luu_khoi}
Hoàn thiện `luuKhoi` — `hash` của nội dung đã tính. Còn thiếu: kiểm
tra khối VỚI hash đó đã tồn tại trong kho CHƯA — nếu RỒI thì chỉ tăng
`soThamChieu`, nếu CHƯA thì tạo mục mới với `soThamChieu = 1` — RỒI
trả về `hash`.

```typescript title=starter
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
interface KhoiLuuTru { noiDung: string; soThamChieu: number; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  ___
}
function xoaThamChieuKhoi(kho: KhoLuuTruKhoi, hash: string): void {
  const k = kho.khoi.get(hash);
  if (k === undefined) return;
  k.soThamChieu -= 1;
  if (k.soThamChieu <= 0) kho.khoi.delete(hash);
}

const khoX = taoKhoLuuTruKhoi();
luuKhoi(khoX, "abc");
const h2 = luuKhoi(khoX, "abc");
console.log(khoX.khoi.get(h2)?.soThamChieu, khoX.khoi.size);
```

```typescript title=solution
function bamNoiDung(noiDung: string): string {
  let h = 0;
  for (let i = 0; i < noiDung.length; i++) {
    h = (h * 31 + noiDung.charCodeAt(i)) % 1000000007;
  }
  return "h" + h.toString(16);
}
interface KhoiLuuTru { noiDung: string; soThamChieu: number; }
interface KhoLuuTruKhoi { khoi: Map<string, KhoiLuuTru>; }
function taoKhoLuuTruKhoi(): KhoLuuTruKhoi { return { khoi: new Map() }; }
function luuKhoi(kho: KhoLuuTruKhoi, noiDung: string): string {
  const hash = bamNoiDung(noiDung);
  const hienCo = kho.khoi.get(hash);
  if (hienCo !== undefined) {
    hienCo.soThamChieu += 1;
  } else {
    kho.khoi.set(hash, { noiDung, soThamChieu: 1 });
  }
  return hash;
}
function xoaThamChieuKhoi(kho: KhoLuuTruKhoi, hash: string): void {
  const k = kho.khoi.get(hash);
  if (k === undefined) return;
  k.soThamChieu -= 1;
  if (k.soThamChieu <= 0) kho.khoi.delete(hash);
}

const khoX = taoKhoLuuTruKhoi();
luuKhoi(khoX, "abc");
const h2 = luuKhoi(khoX, "abc");
console.log(khoX.khoi.get(h2)?.soThamChieu, khoX.khoi.size);
```

```typescript title=test
const khoT = taoKhoLuuTruKhoi();
const hashA = luuKhoi(khoT, "phan-mo-dau-chung");
const hashB = luuKhoi(khoT, "phan-mo-dau-chung");
if (hashA !== hashB) throw new Error("noi dung giong het nhau phai cho ra CUNG mot hash");
const soKhoiSauTrungLap = khoT.khoi.size;
if (soKhoiSauTrungLap !== 1) throw new Error("noi dung trung lap chi duoc luu MOT lan trong kho");
if (khoT.khoi.get(hashA)?.soThamChieu !== 2) throw new Error("hai file cung tham chieu phai co soThamChieu = 2");

const hashC = luuKhoi(khoT, "noi-dung-khac-han");
if (hashC === hashA) throw new Error("noi dung khac nhau khong duoc trung hash");
const soKhoiSauNoiDungMoi = khoT.khoi.size;
if (soKhoiSauNoiDungMoi !== 2) throw new Error("noi dung moi phai them mot muc moi vao kho");

xoaThamChieuKhoi(khoT, hashA);
if (khoT.khoi.has(hashA) !== true) throw new Error("con 1 tham chieu thi khoi PHAI con ton tai");
if (khoT.khoi.get(hashA)?.soThamChieu !== 1) throw new Error("sau 1 lan xoa tham chieu, soThamChieu phai giam con 1");

xoaThamChieuKhoi(khoT, hashA);
if (khoT.khoi.has(hashA) !== false) throw new Error("het tham chieu (0) thi khoi phai bi xoa that su khoi kho");

xoaThamChieuKhoi(khoT, hashA); // goi lai lan nua, khong duoc loi
if (khoT.khoi.has(hashA) !== false) throw new Error("goi xoa tham chieu tren khoi da xoa khong duoc lam gi them");
```

:::hints
- kind: attention
  body: "Con thieu: kiem tra kho.khoi.get(hash) da co chua. Neu co (khac undefined) thi tang soThamChieu cua muc do. Neu chua co thi kho.khoi.set(hash, { noiDung, soThamChieu: 1 }). Cuoi cung return hash."
- kind: strategy
  body: "const hienCo = kho.khoi.get(hash); if (hienCo !== undefined) { hienCo.soThamChieu += 1; } else { kho.khoi.set(hash, { noiDung, soThamChieu: 1 }); } return hash;"
- kind: one-line
  body: "const hienCo = kho.khoi.get(hash); if (hienCo !== undefined) { hienCo.soThamChieu += 1; } else { kho.khoi.set(hash, { noiDung, soThamChieu: 1 }); } return hash;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nội dung trùng nhau giờ chỉ chiếm một chỗ trên đĩa. Nhưng "một chỗ" lại
là một điểm YẾU — đĩa đó hỏng thì SAO?
::::

::::reflect{#nghi-lai}
`luuKhoi` đổi câu hỏi "khối này thuộc VỀ ai?" thành "khối này CHỨA gì?"
— danh tính của một khối LÀ chính nội dung của nó, không phải file nào
đã tạo RA nó. Đếm tham chiếu LÀ cách duy nhất biết KHI nào an toàn để
xoá THẬT: khi không còn AI cần tới nó nữa.
::::

::::checkpoint{mastery=0.76}
::::
