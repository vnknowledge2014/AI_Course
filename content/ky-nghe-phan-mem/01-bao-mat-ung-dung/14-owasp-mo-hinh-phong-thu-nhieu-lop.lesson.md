---
id: ky-nghe-phan-mem.bao-mat-ung-dung.owasp-mo-hinh-phong-thu-nhieu-lop
title: "Phòng thủ nhiều lớp — lỗ hổng THƯỜNG là bug code, không phải vận hành"
summary: "Ẩn dụ lâu đài: một bức tường không đủ, nhiều lớp ĐỘC LẬP giúp MỘT lớp thất bại không sập cả hệ thống. Cụm này dạy một phần đại diện của OWASP Top 10 (SQLi, XSS, CSRF + rate limiting/headers/secrets cụm sau), không phải toàn bộ 10 mục."
locale: vi
track: ky-nghe-phan-mem
module: bao-mat-ung-dung
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [bmud.defense-in-depth]
requires: [bmud.auth-pipeline-result]
concepts: [bmud.defense-in-depth]
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
Track chuyển hướng. Từ "AI được làm gì" (cụm 1-2) sang "code có LỖ
HỔNG gì" — khi dữ liệu người dùng nhập trở thành LỆNH thực thi?
::::

::::explain{#lau-dai-phong-thu-nhieu-lop}
Ẩn dụ **LÂU ĐÀI**: MỘT bức tường KHÔNG đủ — hào nước, tường thành,
lính gác, cửa TRONG là NHIỀU lớp phòng thủ **ĐỘC LẬP**, MỘT lớp thất
bại thì lớp SAU vẫn CHẶN được. Mỗi kỹ thuật (bài 15-26) là MỘT LỚP,
BẢO VỆ một LOẠI tấn công CỤ THỂ:

```typescript
type LopPhongThu = { ten: string; chanDuoc: string };

const cacLop: LopPhongThu[] = [
  { ten: "tham số hoá truy vấn", chanDuoc: "SQL Injection" },
  { ten: "escape đầu ra HTML", chanDuoc: "XSS" },
  { ten: "CSRF token", chanDuoc: "CSRF" },
];

for (const lop of cacLop) {
  console.log(`${lop.ten} -- chặn ${lop.chanDuoc}`);
}
```

```text
tham số hoá truy vấn -- chặn SQL Injection
escape đầu ra HTML -- chặn XSS
CSRF token -- chặn CSRF
```

MỖI lớp bảo vệ chống LẠI một LOẠI tấn công **KHÁC NHau, ĐỘC LẬP** —
"phòng thủ nhiều lớp" Ở CẤP CODE nghĩa là: MỘT bug (VD quên escape)
CHỈ để lộ **ĐÚNG** loại tấn công lớp ĐÓ bảo vệ (XSS) — **KHÔNG** kéo
sập luôn các lớp CÒN LẠI (tham số hoá truy vấn VẪN chặn SQL Injection
BÌNH THƯỜNG, dù escape có bị quên ở đâu đó).
::::

::::example{#khong-toan-bo-owasp}
Khung `OWASP Top 10` liệt kê **MƯỜI** loại lỗ hổng phổ biến nhất —
track NÀY dạy **MỘT PHẦN ĐẠI DIỆN**, KHÔNG PHẢI toàn bộ mười mục:

```typescript title=readonly
type MucOwasp = { ten: string; trongTrackNay: boolean };

const owaspTop10Rutgon: MucOwasp[] = [
  { ten: "Injection (SQL Injection)", trongTrackNay: true },
  { ten: "Cross-Site Scripting (XSS)", trongTrackNay: true },
  { ten: "Cross-Site Request Forgery (CSRF)", trongTrackNay: true },
  { ten: "Security Misconfiguration (headers)", trongTrackNay: true },
  { ten: "Broken Access Control", trongTrackNay: false },
  { ten: "Cryptographic Failures", trongTrackNay: false },
];

const soMucDuocDay = owaspTop10Rutgon.filter((m) => m.trongTrackNay).length;
console.log(soMucDuocDay);
console.log(owaspTop10Rutgon.length);
```

```text title=readonly
4
6
```

Track dạy MỘT PHẦN ĐẠI DIỆN của OWASP (SQL Injection, XSS, CSRF ở cụm
NÀY + rate limiting/headers/secrets ở cụm sau) — mục tiêu KHÔNG PHẢI
"phủ hết mười mục", mà là thấy được **PATTERN CHUNG**: PHẦN LỚN lỗ
hổng bảo mật ứng dụng đến từ **BUG TRONG CODE** người viết (tin dữ
liệu người dùng, quên escape, quên giới hạn) — **KHÔNG PHẢI** lỗi hạ
tầng/vận hành.
::::

::::predict{#doan-mot-lop-thieu-khong-sap-het commitOnce}
```typescript
type LopPhongThu = { ten: string; chanDuoc: string };
const cacLop: LopPhongThu[] = [
  { ten: "tham số hoá truy vấn", chanDuoc: "SQL Injection" },
  { ten: "escape đầu ra HTML", chanDuoc: "XSS" },
  { ten: "CSRF token", chanDuoc: "CSRF" },
];

function timLopChan(loaiTanCong: string, lopHienCo: LopPhongThu[]): string | null {
  const lop = lopHienCo.find((l) => l.chanDuoc === loaiTanCong);
  return lop ? lop.ten : null;
}

// LỚP "escape đầu ra HTML" bị BỎ SÓT (bug quên escape một chỗ)
const lopConLai = cacLop.filter((l) => l.ten !== "escape đầu ra HTML");

console.log(timLopChan("XSS", lopConLai));
console.log(timLopChan("SQL Injection", lopConLai));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`null` rồi `tham số hoá truy vấn`
:::

:::opt
`null` rồi `null` — vì MỘT lớp bị thiếu (do bug), TOÀN BỘ hệ thống
phòng thủ SỤP ĐỔ, không còn lớp nào bảo vệ được gì nữa
::why
Gần đúng ở việc bạn nhớ ĐÚNG `timLopChan("XSS", lopConLai)` ra `null`
— lớp "escape đầu ra HTML" (bảo vệ XSS) THẬT SỰ đã bị lọc BỎ khỏi
`lopConLai` — quan sát ĐÓ đúng.

Chỗ lệch: `lopConLai` KHÔNG "sập TOÀN BỘ" — nó CHỈ mất **ĐÚNG MỘT**
phần tử (lớp escape), HAI lớp CÒN LẠI (`"tham số hoá truy vấn"`,
`"CSRF token"`) VẪN NGUYÊN trong mảng, KHÔNG bị ảnh hưởng gì.
`timLopChan("SQL Injection", lopConLai)` tìm thấy lớp `"tham số hoá
truy vấn"` (VẪN CÒN trong `lopConLai`) — trả về ĐÚNG tên của nó,
KHÔNG PHẢI `null`. Đây CHÍNH LÀ điểm cốt lõi của phòng thủ nhiều lớp
Ở CẤP CODE: MỘT bug CHỈ để lộ ĐÚNG loại tấn công lớp ĐÓ bảo vệ, các
lớp KHÁC (bảo vệ loại tấn công KHÁC) tiếp tục hoạt động ĐỘC LẬP.
::
:::

:::opt
Máy báo lỗi biên dịch — `cacLop.filter(...)` không hợp lệ vì
`LopPhongThu[]` là một mảng CỐ ĐỊNH kích thước (readonly tuple), lọc
bớt phần tử làm thay đổi độ dài không được phép
::why
Gần đúng ở việc bạn nghĩ tới việc MỘT SỐ mảng trong TypeScript CÓ
THỂ bị khai kiểu CỐ ĐỊNH kích thước (tuple, `readonly [A, B, C]`) — một
lo ngại HỢP LÝ nếu `cacLop` được khai theo dạng ĐÓ.

Chỗ lệch: `cacLop: LopPhongThu[]` là kiểu **MẢNG THƯỜNG** (không phải
tuple, không `readonly`) — hoàn toàn CHO PHÉP MỌI thao tác biến đổi
độ dài, bao gồm `.filter(...)` (tạo mảng MỚI, độ dài KHÁC). Biên dịch
sạch, không có ràng buộc kích thước nào.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Phòng thủ nhiều lớp: mỗi kỹ thuật chặn MỘT loại tấn công, độc lập với
các lớp khác. Bước tiếp theo: lớp ĐẦU TIÊN — khi input người dùng trở
thành lệnh SQL.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`` `SELECT * FROM users WHERE email = '${email}'` `` — nối chuỗi
TRỰC TIẾP dữ liệu người dùng vào câu lệnh SQL. Nếu `email` chứa ký
tự đặc biệt của SQL, chuyện gì xảy ra?
::::

::::checkpoint{mastery=0.8}
::::
