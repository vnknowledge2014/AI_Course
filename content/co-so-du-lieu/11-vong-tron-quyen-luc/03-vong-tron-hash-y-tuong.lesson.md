---
id: co-so-du-lieu.vong-tron-quyen-luc.vong-tron-hash-y-tuong
title: "Vòng tròn hash — ý tưởng consistent hashing"
summary: "Đặt CẢ node lẫn khoá lên MỘT vòng tròn (0..999, theo giá trị bam của tên/khoá) — mỗi khoá thuộc VỀ node đầu tiên gặp được khi đi THEO chiều tăng dần từ vị trí khoá, quấn về đầu vòng nếu vượt quá vị trí lớn nhất. Bốn node alpha/beta/gamma/delta nằm rải rác trên vòng ở các vị trí 785/375/210/66 — hoàn toàn không liên quan tới SỐ LƯỢNG node, khác hẳn % soLuongNode của bài trước."
locale: vi
track: co-so-du-lieu
module: vong-tron-quyen-luc
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.vong-tron-hash-y-tuong]
requires: [db.tham-hoa-rehash]
concepts: [db.vong-tron-hash-y-tuong]
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
`% soLuongNode` xáo trộn hầu như MỌI khoá khi số node đổi — VÌ kết
quả phụ thuộc TRỰC tiếp vào chính CON số `soLuongNode`. Bỏ hẳn phép
CHIA lấy dư, dùng cách KHÁC — trông ra sao?
::::

::::explain{#dat-len-vong-tron}
`bam` (bài 1) đã trả VỀ một số trong `[0, 999]` — coi con SỐ đó LÀ
một VỊ trí trên một VÒNG tròn (`0` nối tiếp NGAY sau `999`). Đặt CẢ
tên node LẪN khoá lên vòng NÀY, THEO đúng giá trị `bam` của CHÚNG:

```typescript title=readonly
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}

for (const ten of ["alpha", "beta", "gamma", "delta"]) {
  console.log(ten, bam(ten));
}
```

```text title=readonly
alpha 785
beta 375
gamma 210
delta 66
```

BỐN node nằm RẢI rác trên vòng, Ở các VỊ trí `785, 375, 210, 66` —
sắp XẾP lại theo thứ tự TĂNG dần: `delta(66) → gamma(210) → beta
(375) → alpha(785)`, RỒI quấn VỀ `delta` (VÌ sau `999` LÀ `0` rồi
lại tới `66`). Quy tắc gán khoá: MỘT khoá thuộc VỀ node ĐẦU tiên gặp
được khi đi THEO chiều tăng dần (kim đồng HỒ) TỪ vị trí `bam(khoa)`.
::::

::::example{#khong-lien-quan-so-luong-node}
Vị trí CỦA `alpha, beta, gamma, delta` trên vòng CHỈ phụ thuộc VÀO
TÊN của CHÚNG (`bam("alpha")`, v.v.) — KHÔNG hề phụ THUỘC "hiện đang
CÓ bao nhiêu node" NHƯ `% soLuongNode`. THÊM một node THỨ năm không
hề DỊCH chuyển vị trí của bốn node CŨ — nó chỉ chèn THÊM một điểm
mới VÀO giữa các điểm đã CÓ. Đây chính LÀ khác biệt cốt LÕI so với
`chonNodeNgayTho` (bài 1-2): vị trí trên VÒNG LÀ một tính chất RIÊNG
của TỪNG node, không phải một hàm CỦA "tổng số node hiện TẠI".
::::

::::predict{#doan-khoa-tai-vi-tri-node commitOnce}
Một khoá CÓ `bam(khoa) = 210` — TRÙNG chính xác vị trí CỦA `gamma`.
Áp quy tắc "node ĐẦU tiên gặp được ĐI theo chiều tăng dần TỪ vị trí
khoá" (KỂ cả vị trí trùng khớp CHÍNH xác được tính LÀ "gặp"), khoá
NÀY thuộc VỀ node nào?

:::opt{correct}
`gamma` — vị trí trùng khớp CHÍNH xác vẫn tính LÀ "gặp được", không
cần đi TIẾP
:::

:::opt
`beta` — vị trí trùng khớp CHÍNH xác với `gamma` bị coi LÀ "ĐÃ đi
qua" `gamma`, phải tính node kế TIẾP
::why
Gần đúng ở việc bạn nghĩ TỚI ranh giới "gặp = đi QUA rồi mới tính" —
MỘT quy ước hợp lý CHO một số hệ thống xếp hàng (VÍ dụ hàng đợi
"tới lượt SAU khi gọi tên").

Chỗ lệch: quy tắc `diem.viTri >= viTriKhoa` (SO sánh KHÔNG nghiêm
ngặt, `>=` chứ không phải `>`) coi VỊ trí BẰNG nhau LÀ đã "gặp" —
khoá NẰM ĐÚNG tại vị trí của MỘT node thì thuộc VỀ CHÍNH node đó,
không cần đi tiếp tới node kế. Đây LÀ chi tiết NHỎ nhưng quan trọng
— dùng `>` (nghiêm ngặt) thay VÌ `>=` sẽ khiến khoá TRÙNG vị trí bị
đẩy SANG node kế tiếp một cách SAI.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ý tưởng đã RÕ — giờ viết hàm THẬT tìm node chịu trách nhiệm CHO một
khoá bất kỳ trên vòng.
::::

::::reflect{#nghi-lai}
Đặt CẢ node lẫn khoá LÊN cùng một trục toạ độ (vòng `[0,999]`) LÀ ý
tưởng cốt LÕI của consistent hashing: quan hệ "khoá THUỘC về node
nào" trở THÀNH một câu hỏi HÌNH học thuần tuý ("node gần nhất THEO
một chiều"), không CÒN phụ thuộc và tổng SỐ node hiện có NHƯ phép
chia lấy dư. Viết hàm tìm node chịu TRÁCH nhiệm — CÓ xử lý đúng
trường hợp "quấn VÒNG" khi khoá nằm SAU node cuối cùng — trông RA
sao?
::::

::::checkpoint{mastery=0.75}
::::
