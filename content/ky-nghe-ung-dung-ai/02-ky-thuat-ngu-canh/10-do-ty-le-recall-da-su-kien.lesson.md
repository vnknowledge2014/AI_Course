---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.do-ty-le-recall-da-su-kien
title: "Đo tỉ lệ recall trên nhiều sự kiện, không phải một boolean"
summary: "coConNhoSuKien (bài 3, q9.2a) đo ĐÚNG MỘT cụm từ, trả về boolean nhị phân. tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number đếm bao nhiêu cụm trong cacCumTuCanKiemTra (N cụm, không phải một) CÓ MẶT trong soGhiNho (so khớp qua .cumTu), trả về tỉ lệ soCoMat/N — một số LIÊN TỤC từ 0 đến 1. Với sổ ghi nhớ chỉ ghi được 3/5 sự kiện cần kiểm: tinhTyLeRecall trả về 0.6, không phải true/false. Trùng lặp mục trong soGhiNho (hai mục cùng cumTu) KHÔNG làm tỉ lệ vượt 1 — .some() chỉ hỏi CÓ MẶT hay không, không đếm số lần xuất hiện."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [kna.do-ty-le-recall-da-su-kien]
requires: [kna.so-ghi-nho-dai-han]
concepts: [kna.do-ty-le-recall-da-su-kien]
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
`coConNhoSuKien` (bài `3`, q9.2a) trả lời ĐÚNG MỘT câu hỏi nhị phân:
"cụm từ NÀY có còn không?" — `true` hoặc `false`, không có mức độ Ở
giữa. Nhưng một hệ thống thật thường cần theo dõi NHIỀU sự kiện CÙNG
lúc — mã đơn hàng, chính sách đổi trả, chương trình khuyến mãi — VÀ câu
hỏi thật sự LÀ "bao nhiêu PHẦN TRĂM trong số đó còn được nhớ?", không
phải "có nhớ MỘT cái cụ thể hay không". Bài này đo đúng con số đó.
::::

::::explain{#ty-le-recall}
`tinhTyLeRecall` nhận `soGhiNho` (bài trước) VÀ một danh sách
`cacCumTuCanKiemTra` — CÓ THỂ nhiều hơn MỘT cụm. Với MỖI cụm cần kiểm,
nó hỏi: sổ ghi nhớ có MỤC nào (`.cumTu`) khớp ĐÚNG cụm đó không? Đếm số
cụm khớp được, chia cho TỔNG số cụm cần kiểm:

```typescript title=readonly
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}
```

So khớp qua `.cumTu` — KHÔNG phải qua `.noiDung` (nội dung message gốc
có thể khác nhau dù cùng LOẠI sự kiện). `.some()` chỉ trả lời CÓ hay
KHÔNG — nếu `soGhiNho` chứa HAI mục cùng `cumTu` (như bài trước, message
lặp lại `"don hang"` Ở hai message khác nhau), `.some()` VẪN chỉ đếm
cụm đó LÀ có mặt ĐÚNG MỘT LẦN trong `soCoMat` — không nhân đôi.
::::

::::example{#do-tren-so-ghi-nho-thieu}
Sổ ghi nhớ (bài trước, sau khi xử lý `LICH_SU_HOI_THOAI` VỚI ba từ khoá
`"XY789"`/`"don hang"`/`"30 ngay"`) CHỈ ghi được BA loại sự kiện — kiểm
trên NĂM cụm, hai cụm (`"khuyen mai"`, `"giam gia"`) chưa từng được
`trichXuatGhiNho` nào bắt được:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}

function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}

function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}

const LICH_SU_HOI_THOAI: ChatMessage[] = [
  { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
  { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." },
  { role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
  { role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." },
];

let soGhiNho: MucGhiNho[] = [];
for (const tin of LICH_SU_HOI_THOAI) {
  soGhiNho = capNhatSoGhiNho(soGhiNho, tin, ["XY789", "don hang", "30 ngay"]);
}

const canKiemTra5 = ["XY789", "don hang", "30 ngay", "khuyen mai", "giam gia"];
console.log("ty le recall (5 cum can kiem):", tinhTyLeRecall(soGhiNho, canKiemTra5));
console.log("ty le recall (chi 1 cum, co 2 muc trung 'don hang'):", tinhTyLeRecall(soGhiNho, ["don hang"]));
```

```text title=readonly
ty le recall (5 cum can kiem): 0.6
ty le recall (chi 1 cum, co 2 muc trung 'don hang'): 1
```

`0.6` — ĐÚNG `3` trong `5` cụm CÓ mặt (`"XY789"`, `"don hang"`,
`"30 ngay"`); HAI cụm còn lại (`"khuyen mai"`, `"giam gia"`) không hề
xuất hiện trong `soGhiNho`, vì trước đó KHÔNG có từ khoá nào trong
`cacTuKhoaCanTrich` (bài trước) khớp được với chúng. VÀ dù `"don hang"`
có TỚI HAI mục trùng trong `soGhiNho`, kiểm riêng CHỈ MỘT cụm đó vẫn ra
`1` (`100%`), không phải `2` — `.some()` không đếm SỐ LẦN, chỉ hỏi
CÓ MẶT hay không.
::::

::::predict{#doan-ty-le-thieu commitOnce}
Với ĐÚNG kịch bản trên (`3` trong `5` cụm có mặt), `tinhTyLeRecall`
trả về `0.6`. Nếu có người tính NGƯỢC — đếm số cụm KHÔNG có mặt
(`"khuyen mai"`, `"giam gia"`) chia cho tổng — con số ĐÓ SẼ LÀ bao
nhiêu, VÀ nó có PHẢI LÀ giá trị `tinhTyLeRecall` thật sự trả về không?

:::opt{correct}
`0.4` (`2/5`) — nhưng đó LÀ tỉ lệ THIẾU, KHÔNG PHẢI tỉ lệ recall;
`tinhTyLeRecall` đếm cụm CÓ MẶT (`soCoMat`), không đếm cụm vắng mặt, nên
giá trị THẬT SỰ nó trả về vẫn LÀ `0.6`, không phải `0.4`
:::
:::opt
`0.4`, VÀ đó CHÍNH LÀ giá trị `tinhTyLeRecall` trả về — "tỉ lệ" trong
tên hàm không phân biệt đếm CÓ hay đếm KHÔNG, miễn LÀ MỘT tỉ lệ hợp lệ
::why
Nhầm "một con số LÀ tỉ lệ hợp lệ (giữa 0 và 1)" VỚI "con số ĐÓ chính LÀ
cái hàm `tinhTyLeRecall` tính" — `0.4` VÀ `0.6` đều LÀ tỉ lệ hợp lệ,
nhưng chúng đo HAI điều NGƯỢC nhau (recall vs. thiếu sót).

Chỗ lệch: dòng `const soCoMat = cacCumTuCanKiemTra.filter((cum) =>
soGhiNho.some((m) => m.cumTu === cum)).length;` đếm cụm CÓ MẶT — filter
GIỮ LẠI những cụm mà `.some()` LÀ `true`, không phải những cụm `.some()`
LÀ `false`.
::
:::
:::opt
Không tính được — `tinhTyLeRecall` chỉ nhận `cacCumTuCanKiemTra` LÀM
tham số DUY NHẤT, không có cách nào biết được cụm nào bị THIẾU
::why
Gần đúng Ở việc bạn để Ý đến chữ ký hàm — quan sát về THAM SỐ đúng.

Chỗ lệch: "biết cụm nào bị thiếu" hoàn toàn tính được TỪ CHÍNH những
tham số đã có — bất kỳ cụm nào trong `cacCumTuCanKiemTra` mà
`soGhiNho.some(...)` trả về `false` chính LÀ một cụm bị thiếu; hàm
KHÔNG cần thêm dữ liệu nào để biết điều đó, nó chỉ đơn giản không TÍNH
theo hướng đó.
::
:::
::::

::::code{#viet_tinh_ty_le_recall}
Hoàn thiện `tinhTyLeRecall` — đếm SỐ CỤM trong `cacCumTuCanKiemTra` có
MẶT trong `soGhiNho` (so khớp qua `.cumTu`), rồi chia cho TỔNG số cụm
cần kiểm.

```typescript title=starter
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = ___;
  ___
}

const SO_GHI_NHO_VI_DU: MucGhiNho[] = [
  { cumTu: "XY789", noiDung: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { cumTu: "don hang", noiDung: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { cumTu: "don hang", noiDung: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { cumTu: "30 ngay", noiDung: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
];

const ketQua5Cum = tinhTyLeRecall(SO_GHI_NHO_VI_DU, ["XY789", "don hang", "30 ngay", "khuyen mai", "giam gia"]);
console.log(ketQua5Cum);
```

```typescript title=solution
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}

const SO_GHI_NHO_VI_DU: MucGhiNho[] = [
  { cumTu: "XY789", noiDung: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { cumTu: "don hang", noiDung: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  { cumTu: "don hang", noiDung: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." },
  { cumTu: "30 ngay", noiDung: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." },
];

const ketQua5Cum = tinhTyLeRecall(SO_GHI_NHO_VI_DU, ["XY789", "don hang", "30 ngay", "khuyen mai", "giam gia"]);
console.log(ketQua5Cum);
```

```typescript title=test
if (Math.abs(ketQua5Cum - 0.6) > 1e-9) throw new Error("5 cum can kiem, dung 3 cum co mat -- ty le phai la 0.6");

if (Math.abs(tinhTyLeRecall(SO_GHI_NHO_VI_DU, ["don hang"]) - 1) > 1e-9) {
  throw new Error("kiem rieng 1 cum 'don hang' (co 2 muc trung) van phai la 1, khong phai 2 -- .some() khong dem so lan");
}
if (tinhTyLeRecall(SO_GHI_NHO_VI_DU, ["khong ton tai cum nay"]) !== 0) {
  throw new Error("cum khong ton tai trong so ghi nho phai cho ty le 0");
}
if (Math.abs(tinhTyLeRecall(SO_GHI_NHO_VI_DU, ["XY789", "don hang", "30 ngay"]) - 1) > 1e-9) {
  throw new Error("kiem dung 3 cum DEU co mat phai cho ty le 1 (100%)");
}
if (tinhTyLeRecall([], ["XY789", "don hang"]) !== 0) {
  throw new Error("so ghi nho RONG thi khong cum nao co mat, ty le phai la 0");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau: soCoMat = cacCumTuCanKiemTra.filter dung mot dieu kien soGhiNho.some((m) => m.cumTu === cum), roi lay .length. Cho hai: return soCoMat / cacCumTuCanKiemTra.length (chia cho TONG so cum can kiem, khong phai so muc trong soGhiNho)."
- kind: strategy
  body: "Cho dau: const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length; Cho hai: return soCoMat / cacCumTuCanKiemTra.length;"
- kind: one-line
  body: "Sao chep dung hai dong o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "0.6"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`0.6` — một con số LIÊN TỤC, không phải `true`/`false`. Nhưng CẢ hai
bài về sổ ghi nhớ (bài trước VÀ bài này) vẫn đo trên một sổ ghi nhớ
TĨNH, dựng sẵn. Bài sau ráp nó vào một PHIÊN nhiều lượt THẬT — VÀ so
sánh trực tiếp VỚI cửa sổ hiển thị (bị cắt bởi `catCuaSoTruot`, q9.2a)
— để chứng minh: sổ ghi nhớ VÀ cửa sổ LÀ HAI THỨ KHÁC NHAU.
::::

::::reflect{#nghi-lai}
`coConNhoSuKien` (bài `3`, q9.2a) trả lời cho ĐÚNG MỘT sự kiện —
`tinhTyLeRecall` mở rộng câu hỏi đó sang NHIỀU sự kiện CÙNG lúc, VÀ đổi
kiểu câu trả lời từ boolean sang MỘT tỉ lệ. Đây không phải chỉ LÀ một
phép tính trung bình đơn giản — nó LÀ một sự thay đổi về LOẠI CÂU HỎI
mà một hệ thống đo lường có thể trả lời: "còn nhớ KHÔNG" (nhị phân, phù
hợp khi CHỈ có một sự kiện quan trọng) sẽ luôn kém thông tin hơn "còn
nhớ được BAO NHIÊU PHẦN TRĂM" (liên tục, phù hợp khi có nhiều sự kiện
cần theo dõi cùng lúc — đúng tình huống của một hệ thống production
thật, nơi hiếm khi chỉ có MỘT sự kiện đáng nhớ).
::::

::::checkpoint{mastery=0.82}
::::
