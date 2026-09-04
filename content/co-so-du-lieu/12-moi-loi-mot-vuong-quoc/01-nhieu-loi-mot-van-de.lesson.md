---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.nhieu-loi-mot-van-de
title: "Nhiều lõi, một vấn đề"
summary: "Nhiều lõi CPU CÙNG đọc/ghi một Map chung có thể làm MẤT một lần cập nhật: lõi A đọc giá trị cũ (0), lõi B cũng đọc giá trị cũ (0) TRƯỚC khi A kịp ghi lại — cả hai đều ghi lại 0+1=1, kết quả cuối là 1 thay vì 2. Đây là 'lost update' — cái giá của việc chia SẺ một cấu trúc dữ liệu giữa nhiều lõi mà không đồng bộ."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.nhieu-loi-mot-van-de]
requires: [db.vnode-nhieu-diem-ao]
concepts: [db.nhieu-loi-mot-van-de]
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
q11 chia khoá RA nhiều MÁY. Một máy THẬT lại CÓ nhiều lõi CPU (VÍ dụ
`8` lõi) — nhiều lõi CÙNG đọc/ghi một cấu trúc dữ liệu CHUNG thì SAO?
::::

::::explain{#chia-se-mot-kho}
Hai lõi (`A` VÀ `B`) CÙNG tăng một bộ đếm `kho.get("x")` LÊN `1`. MỖI
lõi ĐỌC giá trị hiện tại RỒI ghi LẠI giá trị đã tăng. Nếu `B` đọc
TRƯỚC khi `A` kịp ghi LẠI (hai lõi XEN kẽ nhau), chuyện GÌ xảy ra:

```typescript title=readonly
const kho = new Map<string, number>();
kho.set("x", 0);

const docA = kho.get("x")!;
const docB = kho.get("x")!;
kho.set("x", docA + 1);
kho.set("x", docB + 1);

console.log(kho.get("x"));
console.log(docA, docB);
```

```text title=readonly
1
0 0
```

CẢ `A` LẪN `B` đều ĐỌC giá trị `0` (TRƯỚC khi bên KIA kịp ghi lại) —
CẢ hai đều ghi `0 + 1 = 1`. Lần ghi THỨ hai (CỦA `B`) GHI ĐÈ lên lần
ghi thứ NHẤT (của `A`), không hề "CỘNG dồn". Kết quả CUỐI LÀ `1`,
KHÔNG phải `2` — MỘT lần tăng đã bị MẤT.
::::

::::example{#neu-tuan-tu}
NẾU `A` đọc RỒI ghi XONG hẳn TRƯỚC khi `B` bắt đầu đọc (KHÔNG xen
kẽ), kết quả sẽ ĐÚNG:

```typescript title=readonly
const kho2 = new Map<string, number>();
kho2.set("x", 0);

const docA2 = kho2.get("x")!;
kho2.set("x", docA2 + 1);
const docB2 = kho2.get("x")!;
kho2.set("x", docB2 + 1);

console.log(kho2.get("x"));
```

```text title=readonly
2
```

Đây chính LÀ việc một "khoá" (lock/mutex) LÀM: ép `B` phải CHỜ tới
khi `A` xong HẲN (đọc VÀ ghi), không cho ĐAN xen. Khoá SỬA đúng lỗi
"lost update" — nhưng ĐỔI lại, `B` phải CHỜ, VÀ khi CÓ nhiều lõi cùng
tranh MỘT khoá dưới tải cao, thời gian CHỜ cộng dồn LÀM chậm cả hệ
thống.
::::

::::predict{#doan-ba-loi commitOnce}
BA lõi (`A`, `B`, `C`) CÙNG đọc giá trị `0` (giống hệt kịch bản đầu
bài — KHÔNG lõi nào chờ lõi nào), rồi CẢ ba đều ghi lại giá trị đã
tăng CỦA riêng mình. Giá trị CUỐI cùng CỦA `kho.get("x")` LÀ bao
nhiêu?

:::opt{correct}
`1` — CẢ ba đều đọc `0`, CẢ ba đều ghi `0+1=1`, lần ghi CUỐI cùng
(bất kể LÀ của lõi nào) LÀ giá trị SỐNG sót — vẫn CHỈ có một lần tăng
được GHI nhận, dù ba lõi CÙNG cố gắng tăng
:::

:::opt
`3` — BA lõi CÙNG tăng thì kết quả PHẢI phản ánh đủ BA lần tăng, dù
CÓ xen kẽ hay không
::why
Gần đúng ở việc bạn kỳ vọng "BA lần tăng THÌ phải CỘNG dồn thành BA"
— MỘT kỳ vọng đúng NẾU mỗi lõi CHỜ lõi trước xong HẲN rồi mới đọc
(giống ví dụ TUẦN tự Ở trên).

Chỗ lệch: CẢ ba lõi Ở kịch bản NÀY đều đọc giá trị `0` TRƯỚC khi BẤT
kỳ lõi nào kịp ghi LẠI — không lõi nào "THẤY" phần việc của hai lõi
kia. CẢ ba đều ghi ĐÚNG `1` (giá trị chúng tính được TỪ giá trị ĐàđỌC,
`0+1`), lần ghi CUỐI cùng đơn giản GHI ĐÈ lên hai lần TRƯỚC — kết quả
VẪN LÀ `1`, dù SỐ lõi tham gia LÀ hai HAY ba HAY nhiều hơn nữa.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chia SẺ một cấu trúc dữ liệu GIỮA nhiều lõi cần khoá — VÀ khoá CÓ
giá của nó. Đo cái GIÁ đó trông ra sao?
::::

::::reflect{#nghi-lai}
"Lost update" Ở đây LÀ một PHIÊN bản thu NHỎ của đúng vấn đề q11 GIẢI
quyết Ở quy mô NHIỀU máy: khi nhiều "người thực THI" (lõi, HAY máy)
cùng đụng và MỘT tài nguyên chung mà KHÔNG có cơ chế phối HỢP, kết
quả có thể SAI theo những CÁCH tinh vi, khó phát HIỆN (chương trình
KHÔNG hề báo lỗi — nó chỉ ÂM thầm cho ra một con SỐ sai). Khoá LÀ một
cách SỬA — buộc các lõi PHẢI xếp hàng CHỜ nhau. Cái GIÁ của việc CHỜ
đó LÀ bao nhiêu?
::::

::::checkpoint{mastery=0.75}
::::
</content>
