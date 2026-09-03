---
id: co-so-du-lieu.thac-du-lieu.skip-list-la-gi
title: Skip list là gì
summary: "Một danh sách liên kết CÓ NHIỀU tầng — tầng đáy giữ MỌI phần tử, mỗi tầng cao hơn giữ MỘT PHẦN thưa dần của tầng dưới. Tìm kiếm bắt đầu Ở tầng cao NHẤT, nhảy nhanh qua nhiều phần tử, rồi tụt XUỐNG tầng thấp hơn khi cần chính xác — ít bước hơn hẳn quét tuyến tính trên tầng đáy."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.skip-list-idea]
requires: [db.memtable-sorted-insert]
concepts: [db.skip-list-idea]
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
`chenMemtable` (bài TRƯỚC) dùng `splice` trên một mảng — MỖI lần
chèn giữa mảng phải DỊCH chuyển mọi phần tử phía SAU. Cấu trúc THẬT
LSM dùng thay mảng trần tên LÀ gì?
::::

::::explain{#nhieu-tang}
Một skip list LÀ một danh sách liên kết CÓ NHIỀU tầng — tầng ĐÁY
(tầng `0`) giữ MỌI phần tử, mỗi tầng CAO hơn giữ một PHẦN thưa dần
của tầng dưới. Tìm kiếm bắt đầu TỪ tầng cao nhất, nhảy nhanh, rồi
TỤT xuống khi cần CHÍNH xác:

```typescript title=readonly
// Tầng 0: mọi phần tử. Tầng 1: mỗi 2 phần tử. Tầng 2: mỗi 4 phần tử.
const tang0 = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
const tang1 = [1, 5, 9, 13, 17];
const tang2 = [1, 9, 17];

function demBuocQuetTuyenTinh(mucTieu: number): number {
  let buoc = 0;
  for (const x of tang0) {
    buoc++;
    if (x === mucTieu) break;
  }
  return buoc;
}

console.log(demBuocQuetTuyenTinh(19));
```

```text title=readonly
10
```

Tìm `19` (phần tử CUỐI) bằng quét tuyến tính TRÊN tầng đáy tốn ĐỦ
`10` bước — phải đi QUA từng phần tử một, không BỎ SÓT phần tử nào.
::::

::::example{#nhay-qua-cac-tang}
Cùng tìm `19`, nhưng bắt đầu TỪ tầng cao nhất (`tầng 2`), chỉ TỤT
xuống tầng dưới khi CẦN:

```typescript title=readonly
function demBuocQuaCacTang(mucTieu: number): number {
  let buoc = 0;
  let viTriTang1BatDau = 0;
  for (let i = 0; i < tang2.length; i++) {
    buoc++;
    const gt = tang2[i];
    if (gt !== undefined && gt <= mucTieu) {
      viTriTang1BatDau = tang1.indexOf(gt);
    } else {
      break;
    }
  }
  let viTriTang0BatDau = 0;
  for (let i = viTriTang1BatDau; i < tang1.length; i++) {
    buoc++;
    const gt = tang1[i];
    if (gt !== undefined && gt <= mucTieu) {
      viTriTang0BatDau = tang0.indexOf(gt);
    } else {
      break;
    }
  }
  for (let i = viTriTang0BatDau; i < tang0.length; i++) {
    buoc++;
    if (tang0[i] === mucTieu) break;
  }
  return buoc;
}

console.log(demBuocQuaCacTang(19));
```

```text title=readonly
6
```

Cùng TÌM `19`, chỉ TỐN `6` bước thay VÌ `10` — Đi QUA tầng `2`
(`3` bước, dừng Ở `17` — phần tử CUỐI cùng của tầng 2 KHÔNG lớn hơn
`19`), rồi tầng `1` bắt đầu TỪ `17` (thêm bước, tụt XUỐNG khi `17`
đã LÀ điểm gần nhất), rồi mới quét NỐT tầng `0` từ đó. Tầng cao
giúp BỎ QUA hẳn những phần tử KHÔNG cần xét TỚI.
::::

::::predict{#doan-so-buoc-tim-17 commitOnce}
Byte tìm `17` (khoá gần CUỐI, đã có mặt Ở CẢ ba tầng) BẰNG cả hai
cách:

```typescript
console.log(demBuocQuetTuyenTinh(17));
console.log(demBuocQuaCacTang(17));
```

Hai dòng in ra gì?

:::opt{correct}
`9` rồi `5`
:::

:::opt
`9` rồi `9` — vì `17` nằm Ở CẢ ba tầng, nên đi qua tầng CAO không
tiết kiệm được bước nào SO với quét thẳng tầng đáy
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`demBuocQuetTuyenTinh(17)
= 9` — quét TỚI vị trí thứ 9 trong tầng đáy để gặp `17`), một phép
đếm chính xác.

Chỗ lệch: `17` xuất hiện Ở CẢ ba tầng ĐÚNG, nhưng đó chính LÀ lý do
`demBuocQuaCacTang` tiết kiệm bước — nó gặp `17` NGAY Ở tầng cao
nhất (`tầng 2`, chỉ `3` phần tử) MÀ không cần quét hết tầng đáy.
Đã chạy thật: `demBuocQuaCacTang(17) = 5`, ÍT hơn `9`.
::
:::

:::opt
`5` rồi `9` — vì quét tuyến tính LUÔN nhanh hơn khi khoá NẰM gần
đầu danh sách, còn đi QUA nhiều tầng chỉ nhanh hơn với khoá Ở CUỐI
::why
Gần đúng ở việc bạn nghĩ TỚI vị trí của khoá ẢNH hưởng tới chi phí
— MỘT trực giác đúng cho CẢ hai cách tìm.

Chỗ lệch: bạn ĐẢO ngược hai con số — quét TUYẾN tính trên tầng đáy
(không đi qua tầng NÀO) LUÔN tốn từ đầu ĐẾN vị trí khoá, ra `9` (số
LỚN), còn đi qua các tầng (BẮT đầu từ tầng cao, nhảy nhanh) ra `5`
(số NHỎ hơn) — đã chạy thật xác NHẬN đúng thứ tự này.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhiều tầng, nhảy qua NHIỀU phần tử cùng lúc — ít bước hơn hẳn quét
tuyến tính. Nhưng skip list THẬT không cố định số tầng CHO mỗi
phần tử như ví dụ này — nó tung ĐỒNG xu để quyết định, mỗi lần
chèn.
::::

::::reflect{#nghi-lai}
Ba tầng cố định Ở đây CHỈ để thấy Ý tưởng — skip list THẬT (dùng
trong LevelDB, RocksDB) quyết định độ CAO của mỗi phần tử bằng một
lần TUNG đồng xu ngẫu nhiên lúc chèn: phần LỚN dừng Ở tầng `0`, một
số ÍT vươn lên tầng `1`, càng ÍT hơn vươn lên tầng `2`, và CỨ thế —
kết quả kỳ VỌNG là `O(log n)` bước cho CẢ chèn lẫn tìm, không cần
dịch chuyển phần tử như `splice` trên mảng. Vì độ cao NGẪU nhiên,
bài học SAU sẽ tiếp tục dùng mảng đã sắp xếp (`chenMemtable`,
`timMemtable`) LÀM đại diện học được — cùng Ý tưởng "cấu trúc trong
bộ nhớ giữ thứ tự", chỉ khác cách CÀI đặt bên dưới. Memtable đầy —
bước tiếp THEO là gì?
::::

::::checkpoint{mastery=0.8}
::::
