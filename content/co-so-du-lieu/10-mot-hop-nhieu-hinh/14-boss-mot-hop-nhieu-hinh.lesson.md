---
id: co-so-du-lieu.mot-hop-nhieu-hinh.boss-mot-hop-nhieu-hinh
title: "BOSS — Một hộp, nhiều hình"
summary: "Ráp TRỌN q10: banBeTrongKhoangTuoi lọc người theo khoảng tuổi (bài 2, 13), RỒI với MỖI người lọc được, tra cạnh 'đi' của đồ thị bạn bè (bài 5-6) để lấy tên bạn — pipeline TypeScript khớp CHÍNH XÁC câu SELECT ->biet->nguoi.ten FROM (SELECT * FROM nguoi WHERE tuoi > 18 AND tuoi < 27) chạy thật trên SurrealDB: cả hai đều trả về ['Em', 'Em'] trên cùng dữ liệu."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.doi-chieu-pipeline]
concepts: [db.boss-q10]
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
Chín quest so sánh — SELECT, mutation, RELATE, duyệt đồ thị, index,
transaction, kiểu dữ liệu, aggregation, AND/OR, pipeline. Ráp TRỌN
lại thành một BOSS cuối cùng, chạy VÀ khớp SurrealDB thật, trông ra
sao?
::::

::::explain{#boss-that}
Năm người, hai người CÓ bạn (`Binh` VÀ `Chi` cùng biết `Em`). Lọc
tuổi TRONG khoảng `(18, 27)` — CHỈ `Binh` (`20`) VÀ `Chi` (`25`)
qua được — RỒI lấy bạn CỦA họ:

```text title=readonly
CREATE nguoi:a SET ten = 'An', tuoi = 15;
CREATE nguoi:b SET ten = 'Binh', tuoi = 20;
CREATE nguoi:c SET ten = 'Chi', tuoi = 25;
CREATE nguoi:d SET ten = 'Dung', tuoi = 30;
CREATE nguoi:e SET ten = 'Em', tuoi = 40;
RELATE nguoi:b->biet->nguoi:e;
RELATE nguoi:c->biet->nguoi:e;
SELECT ->biet->nguoi.ten AS ban_be FROM
  (SELECT * FROM nguoi WHERE tuoi > 18 AND tuoi < 27);
```

```text title=readonly
[{ ban_be: ["Em"] }, { ban_be: ["Em"] }]
```

HAI hàng kết quả — MỘT cho `Binh`, MỘT cho `Chi` — CẢ hai cùng có
`ban_be: ["Em"]`, vì CẢ hai cùng CÓ đúng một người bạn CHUNG. `An`
(`15`) VÀ `Dung` (`30`) bị LOẠI ngay TỪ bước lọc tuổi, không hề xuất
hiện Ở kết quả cuối — GIỐNG hệt cách `ban_be_trong_khoang_tuoi` (q09
BOSS) trả về `[4, 4]` khi hai người CÙNG lọt khoảng tuổi CÙNG chia
sẻ một bạn.
::::

::::example{#tai-sao-khong-thay-an-va-dung}
`An` (`15`) không thoả `tuoi > 18` — bị loại NGAY bước ĐẦU, TRƯỚC
khi truy vấn hề "biết" `An` có bạn HAY không. `Dung` (`30`) thoả
điều kiện lọc TUỔI Ở MỘT vế (`< 27` SAI, `30` không < `27`) — cũng
bị loại. `Dung` KHÔNG hề CÓ cạnh `biet` nào (không được `RELATE`)
— nhưng ĐIỀU đó không quan TRỌNG, vì `Dung` đã bị loại TỪ bước lọc
tuổi rồi, không bao GIỜ tới lượt tra bạn.
::::

::::predict{#doan-doi-khoang-tuoi commitOnce}
CÙNG dữ liệu Ở trên, đổi khoảng tuổi THÀNH `(28, 45)` (chỉ khớp
`Dung` VÀ `Em`). `Dung` KHÔNG có cạnh `biet` nào. `Em` (`40`) CŨNG
không có cạnh `biet` XUẤT phát (nó chỉ LÀ đích của hai cạnh, không
phải NGUỒN). Truy vấn tương ứng trả về gì?

:::opt{correct}
`[{ ban_be: [] }, { ban_be: [] }]` — HAI hàng (một CHO `Dung`, một
CHO `Em`, vì cả hai đều thoả khoảng TUỔI), MỖI hàng mang mảng bạn
RỖNG
:::

:::opt
`[]` — mảng rỗng HOÀN toàn, vì KHÔNG ai trong khoảng tuổi CÓ bạn
::why
Gần đúng ở việc bạn nhận RA cả `Dung` lẫn `Em` đều KHÔNG có cạnh
`biet` xuất phát — MỘT quan sát đúng VỀ dữ liệu đồ thị.

Chỗ lệch: "không CÓ bạn" LÀ một tính CHẤT của từng người, KHÔNG
phải lý do để LOẠI người đó khỏi kết QUẢ lọc tuổi. `Dung` VÀ `Em`
đều thoả `tuoi` trong khoảng `(28, 45)` — CẢ hai vẫn TẠO ra một
hàng kết quả (bước LỌC tuổi hoàn toàn ĐỘC lập với việc CÓ bạn hay
không) — CHỈ LÀ trường `ban_be` của mỗi hàng LÀ mảng rỗng, không
phải cả hàng biến MẤT.
::
:::
::::

::::code{#viet_boss_q10}
Hoàn thiện `banBeTrongKhoangTuoi` — với MỖI người lọc được TRONG
khoảng tuổi, tra `hangXomDi` để lấy TÊN bạn của họ.

```typescript title=starter
interface Nguoi {
  ten: string;
  tuoi: number;
}

interface Canh {
  vao: string;
  ra: string;
}

function hangXomDi(canh: Canh[], tuId: string): string[] {
  const ketQua: string[] = [];
  for (const c of canh) {
    if (c.vao === tuId) {
      ketQua.push(c.ra);
    }
  }
  return ketQua;
}

function banBeTrongKhoangTuoi(
  bang: Map<string, Nguoi>,
  canh: Canh[],
  nguongDuoi: number,
  nguongTren: number
): string[] {
  const ketQua: string[] = [];
  for (const [id, nguoi] of bang) {
    if (nguoi.tuoi > nguongDuoi && nguoi.tuoi < nguongTren) {
      const banBe = hangXomDi(canh, id);
      for (const idBanBe of banBe) {
        ___
      }
    }
  }
  return ketQua;
}

const bang = new Map<string, Nguoi>([
  ["nguoi:a", { ten: "An", tuoi: 15 }],
  ["nguoi:b", { ten: "Binh", tuoi: 20 }],
  ["nguoi:c", { ten: "Chi", tuoi: 25 }],
  ["nguoi:d", { ten: "Dung", tuoi: 30 }],
  ["nguoi:e", { ten: "Em", tuoi: 40 }],
]);
const canh: Canh[] = [
  { vao: "nguoi:b", ra: "nguoi:e" },
  { vao: "nguoi:c", ra: "nguoi:e" },
];
const ketQua = banBeTrongKhoangTuoi(bang, canh, 18, 27);
console.log(ketQua.join(","));
```

```typescript title=solution
interface Nguoi {
  ten: string;
  tuoi: number;
}

interface Canh {
  vao: string;
  ra: string;
}

function hangXomDi(canh: Canh[], tuId: string): string[] {
  const ketQua: string[] = [];
  for (const c of canh) {
    if (c.vao === tuId) {
      ketQua.push(c.ra);
    }
  }
  return ketQua;
}

function banBeTrongKhoangTuoi(
  bang: Map<string, Nguoi>,
  canh: Canh[],
  nguongDuoi: number,
  nguongTren: number
): string[] {
  const ketQua: string[] = [];
  for (const [id, nguoi] of bang) {
    if (nguoi.tuoi > nguongDuoi && nguoi.tuoi < nguongTren) {
      const banBe = hangXomDi(canh, id);
      for (const idBanBe of banBe) {
        const nguoiBanBe = bang.get(idBanBe);
        if (nguoiBanBe !== undefined) {
          ketQua.push(nguoiBanBe.ten);
        }
      }
    }
  }
  return ketQua;
}

const bang = new Map<string, Nguoi>([
  ["nguoi:a", { ten: "An", tuoi: 15 }],
  ["nguoi:b", { ten: "Binh", tuoi: 20 }],
  ["nguoi:c", { ten: "Chi", tuoi: 25 }],
  ["nguoi:d", { ten: "Dung", tuoi: 30 }],
  ["nguoi:e", { ten: "Em", tuoi: 40 }],
]);
const canh: Canh[] = [
  { vao: "nguoi:b", ra: "nguoi:e" },
  { vao: "nguoi:c", ra: "nguoi:e" },
];
const ketQua = banBeTrongKhoangTuoi(bang, canh, 18, 27);
console.log(ketQua.join(","));
```

```typescript title=test
const bang2 = new Map<string, Nguoi>([
  ["nguoi:a", { ten: "An", tuoi: 15 }],
  ["nguoi:b", { ten: "Binh", tuoi: 20 }],
  ["nguoi:c", { ten: "Chi", tuoi: 25 }],
  ["nguoi:d", { ten: "Dung", tuoi: 30 }],
  ["nguoi:e", { ten: "Em", tuoi: 40 }],
]);
const canh2: Canh[] = [
  { vao: "nguoi:b", ra: "nguoi:e" },
  { vao: "nguoi:c", ra: "nguoi:e" },
];
const ketQua2 = banBeTrongKhoangTuoi(bang2, canh2, 18, 27);
console.log(ketQua2.join(","));
if (ketQua2.length !== 2) throw new Error("phai co dung 2 ban be tim duoc (Binh va Chi cung co ban la Em)");
if (ketQua2[0] !== "Em" || ketQua2[1] !== "Em") throw new Error("ca hai ban be phai la Em, khop SurrealDB that");

const ketQuaKhoangKhac = banBeTrongKhoangTuoi(bang2, canh2, 28, 35);
if (ketQuaKhoangKhac.length !== 0) throw new Error("khoang 28..35 chi khop Dung, nhung Dung khong co canh ban be nao");

const canhRong: Canh[] = [];
if (banBeTrongKhoangTuoi(bang2, canhRong, 18, 27).length !== 0) throw new Error("khong co canh nao thi khong co ban be nao");
```

:::hints
- kind: attention
  body: "Voi moi idBanBe, tra ten cua ho tu bang (Map.get), roi day vao ketQua neu tim thay -- ba dong."
- kind: strategy
  body: "const nguoiBanBe = bang.get(idBanBe); if (nguoiBanBe !== undefined) { ketQua.push(nguoiBanBe.ten); }"
- kind: one-line
  body: "const nguoiBanBe = bang.get(idBanBe); if (nguoiBanBe !== undefined) ketQua.push(nguoiBanBe.ten);"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "Em,Em"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lọc, đếm, đồ thị, AND/OR — ráp thành một pipeline khớp SurrealDB
thật. q10 "Một hộp, nhiều hình" khép LẠI — vòng q07-q10 hoàn tất.
::::

::::reflect{#nghi-lai}
`banBeTrongKhoangTuoi` (bài NÀY) LÀ `ban_be_trong_khoang_tuoi` (q09
BOSS) viết LẠI bằng TypeScript, VỚI đúng một khác biệt: q09 tự XÂY
`MucChiMuc`/`xay_chi_muc`/`trong_khoang` để tra cứu NHANH THEO khoá
đã mã hoá, còn Ở đây `Map<string, Nguoi>` (tra cứu O(1) SẴN có
trong TypeScript) đóng vai trò TƯƠNG đương — bài học Ở q09 (tại SAO
tra cứu qua chỉ mục nhanh hơn quét) vẫn đúng, chỉ LÀ ngôn ngữ NÀY
đã có sẵn công cụ. Kết quả khớp SurrealDB thật (`["Em", "Em"]`)
đóng vòng q07-q10: lexer/parser (q07) → executor (q08) → chỉ mục/
đồ thị (q09) → đối chiếu VỚI một cơ sở dữ liệu THẬT (q10) — TỪ đầu
tới cuối, mini-engine tự viết nắm ĐÚNG cơ chế mà một hệ thống THẬT,
production, đang chạy.
::::

::::checkpoint{mastery=0.9}
::::
