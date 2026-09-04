---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.nen-chon-theo-kich-co-stcs
title: "Nén chọn theo kích cỡ — STCS"
summary: "moPhongSTCS mô phỏng Size-Tiered Compaction: mỗi flush tạo một SSTable kích cỡ 1; khi đủ nguongMin bảng CÙNG kích cỡ tích luỹ, chúng gộp lại thành một bảng lớn hơn (kích cỡ nhân nguongMin). Sau 16 lần flush với nguongMin=4, số bảng dao động 1-6 rồi CUỐI cùng gộp về đúng 1 -- so với 16 bảng riêng lẻ nếu không hề nén."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.nen-chon-theo-kich-co-stcs]
requires: [db.tombstone-tich-luy-mai]
concepts: [db.nen-chon-theo-kich-co-stcs]
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
q04 bài 12 gộp SSTable BẰNG một lần `gopSSTable` duy nhất — "gộp
TOÀN bộ". Nhưng trong THỰC tế, flush diễn ra LIÊN tục — gộp TOÀN bộ
mỗi lần LÀ quá tốn. Chọn BẢNG nào để gộp TRƯỚC?
::::

::::explain{#mo-phong-stcs}
`moPhongSTCS` mô phỏng "Size-Tiered Compaction Strategy" (STCS): mỗi
lần flush TẠO một SSTable kích cỡ `1`. Sau MỖI flush, nếu CÓ ít nhất
`nguongMin` bảng CÙNG kích cỡ, chúng được gộp NGAY thành MỘT bảng lớn
hơn (`kichCo * nguongMin`) — LẶP lại cho tới khi KHÔNG còn nhóm nào
đủ ngưỡng:

```typescript title=readonly
function moPhongSTCS(soLanFlush: number, nguongMin: number): number[] {
  const cacBang: number[] = [];
  const dauSoBang: number[] = [];
  for (let lan = 0; lan < soLanFlush; lan++) {
    cacBang.push(1);
    let doiMoi = true;
    while (doiMoi) {
      doiMoi = false;
      const theoKichCo = new Map<number, number[]>();
      cacBang.forEach((kc, i) => {
        const ds = theoKichCo.get(kc) ?? [];
        ds.push(i);
        theoKichCo.set(kc, ds);
      });
      for (const [kc, ds] of theoKichCo) {
        if (ds.length >= nguongMin) {
          const idsGop = new Set(ds.slice(0, nguongMin));
          const conLai = cacBang.filter((_, i) => !idsGop.has(i));
          conLai.push(kc * nguongMin);
          cacBang.length = 0;
          cacBang.push(...conLai);
          doiMoi = true;
          break;
        }
      }
    }
    dauSoBang.push(cacBang.length);
  }
  return dauSoBang;
}

console.log("nguongMin=4, so bang sau MOI lan flush (16 lan):");
console.log(moPhongSTCS(16, 4));
```

```text title=readonly
nguongMin=4, so bang sau MOI lan flush (16 lan):
[
  1, 2, 3, 1, 2, 3,
  4, 2, 3, 4, 5, 3,
  4, 5, 6, 1
]
```

Sau flush THỨ tư (`4` bảng kích cỡ `1`), CẢ bốn gộp NGAY thành MỘT
bảng kích cỡ `4` — số bảng "quay VỀ" `1`. Số bảng KHÔNG BAO giờ vượt
quá `6` xuyên suốt cả `16` lần flush — VÀ tại lần flush thứ `16`
(`4×4=16` khoá gộp XONG), TOÀN bộ dữ liệu gộp lại thành đúng MỘT
bảng.
::::

::::example{#khong-nen-tang-tuyen-tinh}
So sánh: nếu KHÔNG hề nén, `16` lần flush cho ĐÚNG `16` SSTable riêng
lẻ — tăng TUYẾN tính, không giới hạn. STCS giữ số bảng LUÔN Ở mức
thấp (tối đa `6` trong ví DỤ trên) bằng cách gộp SỚM những bảng "cùng
tầng kích cỡ" LẠI với nhau, thay VÌ để chúng tích luỹ.
::::

::::predict{#doan-nguong-lon-hon-so-flush commitOnce}
Gọi `moPhongSTCS(4, 10)` — chỉ `4` lần flush, NHƯNG `nguongMin=10`
(lớn hơn CẢ số lần flush). Giá trị CUỐI cùng của mảng trả VỀ LÀ gì?

:::opt{correct}
`4` — không lần NÀO đủ `10` bảng CÙNG kích cỡ để gộp, NÊN mỗi lần
flush chỉ đơn giản CỘNG thêm một bảng MỚI, không hề gộp
:::

:::opt
`1` — STCS LUÔN gộp hết mọi bảng LẠI cuối cùng, bất kể `nguongMin`
LÀ bao nhiêu
::why
Trực giác NÀY nhầm STCS VỚI "gộp toàn bộ" (`gopSSTable`, q04 bài 12)
— MỘT chiến lược HOÀN toàn khác, luôn gộp TẤT cả bất kể kích cỡ.

Chỗ lệch: STCS chỉ gộp KHI có đủ `nguongMin` bảng CÙNG kích cỡ —
điều kiện `ds.length >= nguongMin` LÀ MỘT ngưỡng THẬT, không phải
hình thức. VỚI `nguongMin=10` VÀ chỉ `4` lần flush, KHÔNG nhóm kích
cỡ nào từng đạt `10` phần tử — vòng `while` không BAO giờ kích hoạt,
`cacBang` giữ nguyên `4` bảng kích cỡ `1` riêng biệt.
::
:::
::::

::::code{#viet_mo_phong_stcs}
Hoàn thiện `moPhongSTCS` — VỚI mỗi nhóm kích cỡ đã tích luỹ ĐỦ
`nguongMin` bảng, gộp CHÚNG lại.

```typescript title=starter
function moPhongSTCS(soLanFlush: number, nguongMin: number): number[] {
  const cacBang: number[] = [];
  const dauSoBang: number[] = [];
  for (let lan = 0; lan < soLanFlush; lan++) {
    cacBang.push(1);
    let doiMoi = true;
    while (doiMoi) {
      doiMoi = false;
      const theoKichCo = new Map<number, number[]>();
      cacBang.forEach((kc, i) => {
        const ds = theoKichCo.get(kc) ?? [];
        ds.push(i);
        theoKichCo.set(kc, ds);
      });
      for (const [kc, ds] of theoKichCo) {
        if (___) {
          const idsGop = new Set(ds.slice(0, nguongMin));
          const conLai = cacBang.filter((_, i) => !idsGop.has(i));
          conLai.push(kc * nguongMin);
          cacBang.length = 0;
          cacBang.push(...conLai);
          doiMoi = true;
          break;
        }
      }
    }
    dauSoBang.push(cacBang.length);
  }
  return dauSoBang;
}

console.log(moPhongSTCS(16, 4).at(-1));
```

```typescript title=solution
function moPhongSTCS(soLanFlush: number, nguongMin: number): number[] {
  const cacBang: number[] = [];
  const dauSoBang: number[] = [];
  for (let lan = 0; lan < soLanFlush; lan++) {
    cacBang.push(1);
    let doiMoi = true;
    while (doiMoi) {
      doiMoi = false;
      const theoKichCo = new Map<number, number[]>();
      cacBang.forEach((kc, i) => {
        const ds = theoKichCo.get(kc) ?? [];
        ds.push(i);
        theoKichCo.set(kc, ds);
      });
      for (const [kc, ds] of theoKichCo) {
        if (ds.length >= nguongMin) {
          const idsGop = new Set(ds.slice(0, nguongMin));
          const conLai = cacBang.filter((_, i) => !idsGop.has(i));
          conLai.push(kc * nguongMin);
          cacBang.length = 0;
          cacBang.push(...conLai);
          doiMoi = true;
          break;
        }
      }
    }
    dauSoBang.push(cacBang.length);
  }
  return dauSoBang;
}

console.log(moPhongSTCS(16, 4).at(-1));
```

```typescript title=test
const ketQua16 = moPhongSTCS(16, 4);
if (ketQua16.length !== 16) throw new Error("phai co dung 16 phan tu, moi phan tu ung voi 1 lan flush");
if (ketQua16.at(-1) !== 1) throw new Error("sau 16 lan flush voi nguongMin=4, moi thu phai gop ve dung 1 bang");
if (Math.max(...ketQua16) !== 6) throw new Error("so bang toi da xuyen suot qua trinh phai la 6, khong duoc vuot qua");

const khongNen = moPhongSTCS(4, 10);
if (khongNen.at(-1) !== 4) throw new Error("nguongMin=10 lon hon 4 lan flush -- khong bao gio du de gop, phai giu nguyen 4 bang");

const gopDuNgay = moPhongSTCS(4, 4);
if (gopDuNgay.at(-1) !== 1) throw new Error("dung 4 lan flush voi nguongMin=4 -- gop het thanh 1 bang duy nhat");
```

:::hints
- kind: attention
  body: "Dieu kien kich hoat gop: nhom kich co da co it nhat nguongMin bang -- mot bieu thuc so sanh."
- kind: strategy
  body: "if (ds.length >= nguongMin) {"
- kind: one-line
  body: "if (ds.length >= nguongMin) {"
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
STCS giữ số bảng THẤP bằng cách gộp theo KÍCH cỡ. Nhưng "cùng kích
cỡ" KHÔNG có nghĩa LÀ "cùng phạm vi khoá" — điều đó ảnh hưởng tới ĐỌC
thế NÀO?
::::

::::reflect{#nghi-lai}
STCS giải quyết đúng vấn ĐỀ bài 4 nêu ra: bia mộ (VÀ dữ liệu nói
CHUNG) không tự dọn — cần một tiến TRÌNH chủ động. STCS chọn "gộp
những bảng CÙNG tầng kích cỡ" LÀ MỘT chiến lược hợp LÝ (ít viết lại
dữ liệu CŨ nhiều lần, phù hợp workload GHI nhiều), nhưng nó KHÔNG hề
quan TÂM tới phạm vi KHOÁ của mỗi bảng — hai bảng CÙNG kích cỡ có thể
chứa những khoá HOÀN toàn chồng lấn nhau. Điều đó nghĩa LÀ gì cho một
lần ĐỌC?
::::

::::checkpoint{mastery=0.85}
::::
