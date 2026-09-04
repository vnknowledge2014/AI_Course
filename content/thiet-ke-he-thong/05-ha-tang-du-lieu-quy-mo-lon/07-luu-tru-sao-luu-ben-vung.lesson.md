---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.luu-tru-sao-luu-ben-vung
title: "Sao lưu bền vững: một bản sống là đủ"
summary: "docKhoi(khoi, tt) tra ve true NEU CO IT NHAT mot dia trong dsDia (danh sach dia chua ban sao cua khoi) CHUA bi danh dau hong -- khong can TAT CA con nguyen. Khoi sao thanh 3 ban tren dia-1/2/3: dia-1 hong van doc duoc (2 ban con), dia-1 VA dia-2 CUNG hong van doc duoc (1 ban con) -- CHI khi CA BA dia deu hong thi docKhoi moi tra ve false, dung luc do moi la MAT du lieu that su."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.luu-tru-sao-luu-ben-vung]
requires: [sd.luu-tru-khu-trung-theo-hash]
concepts: [sd.luu-tru-sao-luu-ben-vung]
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
Khối trùng nhau giờ chỉ chiếm một chỗ trên đĩa — nhưng "một chỗ" cũng
là "một điểm hỏng duy nhất". Một đĩa cháy, một ổ cứng chết — vậy khối
đó mất VĨNH viễn? Không, NẾU nó không chỉ nằm trên đúng một đĩa.
::::

::::explain{#doc-can-mot-ban-song}
Mỗi khối được sao RA nhiều bản, mỗi bản nằm trên một đĩa RIÊNG
(`dsDia`). `docKhoi` không đòi hỏi TẤT cả các đĩa đó phải còn nguyên —
nó chỉ cần tìm ĐƯỢC MỘT đĩa CHƯA bị đánh dấu hỏng LÀ đủ để đọc thành
công:

```typescript title=readonly
interface KhoiSaoLuu { idKhoi: string; dsDia: string[]; }
interface TrangThaiDia { diaHong: Set<string>; }
function taoTrangThaiDia(): TrangThaiDia { return { diaHong: new Set() }; }
function danhDauDiaHong(tt: TrangThaiDia, idDia: string): void { tt.diaHong.add(idDia); }
function docKhoi(khoi: KhoiSaoLuu, tt: TrangThaiDia): boolean {
  return khoi.dsDia.some((idDia) => !tt.diaHong.has(idDia));
}
function soBanSaoConSong(khoi: KhoiSaoLuu, tt: TrangThaiDia): number {
  return khoi.dsDia.filter((idDia) => !tt.diaHong.has(idDia)).length;
}

const khoi: KhoiSaoLuu = { idKhoi: "khoi-01", dsDia: ["dia-1", "dia-2", "dia-3"] };
const tt = taoTrangThaiDia();
console.log("chua dia nao hong -- doc duoc?", docKhoi(khoi, tt), "so ban sao con song:", soBanSaoConSong(khoi, tt));

danhDauDiaHong(tt, "dia-1");
console.log("dia-1 hong -- doc duoc?", docKhoi(khoi, tt), "so ban sao con song:", soBanSaoConSong(khoi, tt));
```

```text title=readonly
chua dia nao hong -- doc duoc? true so ban sao con song: 3
dia-1 hong -- doc duoc? true so ban sao con song: 2
```

Khối `"khoi-01"` có `3` bản sao, mỗi bản nằm trên một đĩa RIÊNG. Ngay
cả khi `"dia-1"` hỏng, `docKhoi` vẫn trả về `true` — `2` bản còn LẠI
(`"dia-2"`, `"dia-3"`) LÀ đủ. `some` chỉ cần TÌM đúng một phần tử thoả
điều kiện, không cần TẤT cả.
::::

::::example{#mat-du-lieu-that-su-khi-ca-ba-hong}
Chỉ khi TOÀN bộ các đĩa mang bản sao đều hỏng, `docKhoi` mới trả về
`false` — VÀ đó chính LÀ định nghĩa của "mất dữ liệu THẬT sự":

```typescript title=readonly
interface KhoiSaoLuu { idKhoi: string; dsDia: string[]; }
interface TrangThaiDia { diaHong: Set<string>; }
function taoTrangThaiDia(): TrangThaiDia { return { diaHong: new Set() }; }
function danhDauDiaHong(tt: TrangThaiDia, idDia: string): void { tt.diaHong.add(idDia); }
function docKhoi(khoi: KhoiSaoLuu, tt: TrangThaiDia): boolean {
  return khoi.dsDia.some((idDia) => !tt.diaHong.has(idDia));
}
function soBanSaoConSong(khoi: KhoiSaoLuu, tt: TrangThaiDia): number {
  return khoi.dsDia.filter((idDia) => !tt.diaHong.has(idDia)).length;
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: khoi-01 co 3 ban sao
// tren dia-1/2/3, dia-1 da hong
const khoi: KhoiSaoLuu = { idKhoi: "khoi-01", dsDia: ["dia-1", "dia-2", "dia-3"] };
const tt = taoTrangThaiDia();
danhDauDiaHong(tt, "dia-1");

danhDauDiaHong(tt, "dia-2");
console.log("dia-1 VA dia-2 hong -- doc duoc?", docKhoi(khoi, tt), "so ban sao con song:", soBanSaoConSong(khoi, tt));

danhDauDiaHong(tt, "dia-3");
console.log("CA BA dia deu hong -- doc duoc?", docKhoi(khoi, tt), "so ban sao con song:", soBanSaoConSong(khoi, tt));
```

```text title=readonly
dia-1 VA dia-2 hong -- doc duoc? true so ban sao con song: 1
CA BA dia deu hong -- doc duoc? false so ban sao con song: 0
```

Dù `2` trong `3` đĩa đã hỏng, `docKhoi` vẫn trả về `true` — chỉ CẦN
`"dia-3"` còn nguyên. Chỉ khi `"dia-3"` CŨNG hỏng, `soBanSaoConSong`
mới chạm `0` VÀ `docKhoi` mới thật sự trả về `false`.
::::

::::predict{#doan-dung-chung-dia-hong-voi-khoi-khac commitOnce}
Tiếp tục trạng thái Ở trên: `"dia-1"`, `"dia-2"`, `"dia-3"` đều ĐÃ bị
đánh dấu hỏng. Một khối HOÀN TOÀN khác, `"khoi-02"`, có `dsDia =
["dia-3", "dia-9"]` — `"dia-9"` CHƯA từng bị đánh dấu hỏng. Gọi
`docKhoi(khoi-02, tt)` — trả về gì?

:::opt{correct}
`true` — `"dia-9"` vẫn còn nguyên, VÀ chỉ cần một bản sao SỐNG LÀ đủ;
việc `"dia-3"` đã hỏng (do đứng chung trạng thái với khối kia) không hề
kéo `"khoi-02"` xuống theo
:::
:::opt
`false` — `"dia-3"` đã bị đánh dấu hỏng RỒI, nên MỌI khối nào có nhắc
tới `"dia-3"` trong `dsDia` của nó đều phải coi LÀ mất
::why
Nhầm "một đĩa đã hỏng" VỚI "mọi khối tham chiếu đĩa đó đều mất" —
nhưng `docKhoi` xét TOÀN bộ `dsDia` của TỪNG khối, không dừng lại ngay
khi gặp MỘT đĩa hỏng.

Chỗ lệch: `khoi.dsDia.some((idDia) => !tt.diaHong.has(idDia))` — `some`
duyệt HẾT mảng (`["dia-3", "dia-9"]`) tìm phần tử THOẢ điều kiện.
`"dia-3"` hỏng NÊN không thoả, nhưng `"dia-9"` chưa hỏng NÊN thoả —
`some` trả về `true` ngay khi gặp `"dia-9"`. Trạng thái đĩa hỏng LÀ
dùng CHUNG, nhưng mỗi khối vẫn tự quyết định LÀ đọc được hay không
DỰA trên chính danh sách đĩa của NÓ.
::
:::
::::

::::code{#viet_doc_khoi}
Viết `docKhoi` — trả về `true` khi VÀ chỉ khi còn ÍT NHẤT một đĩa
trong `dsDia` của khối chưa bị đánh dấu hỏng.

```typescript title=starter
interface KhoiSaoLuu { idKhoi: string; dsDia: string[]; }
interface TrangThaiDia { diaHong: Set<string>; }
function taoTrangThaiDia(): TrangThaiDia { return { diaHong: new Set() }; }
function danhDauDiaHong(tt: TrangThaiDia, idDia: string): void { tt.diaHong.add(idDia); }
function docKhoi(khoi: KhoiSaoLuu, tt: TrangThaiDia): boolean {
  ___
}

const ttX = taoTrangThaiDia();
const khoiX: KhoiSaoLuu = { idKhoi: "kx", dsDia: ["a", "b"] };
danhDauDiaHong(ttX, "a");
console.log(docKhoi(khoiX, ttX));
```

```typescript title=solution
interface KhoiSaoLuu { idKhoi: string; dsDia: string[]; }
interface TrangThaiDia { diaHong: Set<string>; }
function taoTrangThaiDia(): TrangThaiDia { return { diaHong: new Set() }; }
function danhDauDiaHong(tt: TrangThaiDia, idDia: string): void { tt.diaHong.add(idDia); }
function docKhoi(khoi: KhoiSaoLuu, tt: TrangThaiDia): boolean {
  return khoi.dsDia.some((idDia) => !tt.diaHong.has(idDia));
}

const ttX = taoTrangThaiDia();
const khoiX: KhoiSaoLuu = { idKhoi: "kx", dsDia: ["a", "b"] };
danhDauDiaHong(ttX, "a");
console.log(docKhoi(khoiX, ttX));
```

```typescript title=test
const ttT = taoTrangThaiDia();
const khoi01: KhoiSaoLuu = { idKhoi: "khoi-01", dsDia: ["dia-1", "dia-2", "dia-3"] };
if (docKhoi(khoi01, ttT) !== true) throw new Error("chua co dia nao hong thi phai doc duoc");

danhDauDiaHong(ttT, "dia-1");
if (docKhoi(khoi01, ttT) !== true) throw new Error("1/3 dia hong, van con 2 ban sao song, phai doc duoc");

danhDauDiaHong(ttT, "dia-2");
if (docKhoi(khoi01, ttT) !== true) throw new Error("2/3 dia hong, con 1 ban sao song, VAN phai doc duoc");

danhDauDiaHong(ttT, "dia-3");
if (docKhoi(khoi01, ttT) !== false) throw new Error("CA 3 dia deu hong -- mat du lieu that su, phai la false");

const khoi02: KhoiSaoLuu = { idKhoi: "khoi-02", dsDia: ["dia-3", "dia-9"] };
if (docKhoi(khoi02, ttT) !== true) throw new Error("khoi-02 co dia-9 con song (chua bao gio hong) -- phai doc duoc du dung chung dia-3 da hong voi khoi khac");
```

:::hints
- kind: attention
  body: "Dung phuong thuc .some tren mang khoi.dsDia -- chi can MOT phan tu thoa dieu kien 'chua bi hong' la du."
- kind: strategy
  body: "return khoi.dsDia.some((idDia) => !tt.diaHong.has(idDia));"
- kind: one-line
  body: "return khoi.dsDia.some((idDia) => !tt.diaHong.has(idDia));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba mảnh về file — chia khối, khử trùng, sao lưu — đã xong. Track rẽ
sang một hệ thống hoàn toàn khác: hộp thư.
::::

::::reflect{#nghi-lai}
`docKhoi` không hỏi "đĩa nào ĐANG khoẻ" — nó chỉ hỏi "còn ÍT nhất một
đĩa khoẻ hay không". Cách đặt câu hỏi ĐÓ chính LÀ điều làm sao lưu bội
số phát huy tác dụng: hệ thống không cần MỌI bản sao còn nguyên, nó chỉ
cần KHÔNG để tất cả bản sao hỏng CÙNG lúc.
::::

::::checkpoint{mastery=0.78}
::::
