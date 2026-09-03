---
id: co-so-du-lieu.bo-nao-cua-truy-van.vi-sao-loc-truoc-chon-cot
title: "Vì sao phải lọc trước khi chọn cột"
summary: "Nếu ChonCot chạy TRƯỚC Loc mà WHERE cần một cột KHÔNG nằm trong SELECT, Loc sẽ không tìm thấy cột đó nữa — tra_cuu trả về 0, so sánh sai lệch, hàng lẽ ra PHẢI qua bị loại (hoặc ngược lại). Đây LÀ lý do BẮT BUỘC (không chỉ tối ưu) để pushdown lọc TRƯỚC chọn cột trong ke_hoach."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 7
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.pushdown-idea]
requires: [db.plan-builder]
concepts: [db.pushdown-idea]
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
Kế hoạch (bài trước) LUÔN xây `Quet → Loc → ChonCot`. Đảo THỨ tự —
`ChonCot` TRƯỚC `Loc` — có THỰC sự thành vấn đề không?
::::

::::explain{#ly-do-thu-tu}
`SELECT ten FROM nguoi WHERE tuoi > 18` — `SELECT` CHỈ giữ cột
`ten`, nhưng `WHERE` CẦN cột `tuoi`. Nếu `ChonCot` chạy TRƯỚC `Loc`,
cột `tuoi` ĐÃ bị bỏ đi TRƯỚC khi `Loc` kịp dùng NÓ:

```rust title=readonly
let mut cay = CayAst { nut: Vec::new() };
let tuoi: Vec<char> = vec!['t','u','o','i'];
cay.nut.push(NoAst::SoSanh(tuoi, '>', 18));
let goc = (cay.nut.len() - 1) as usize;

let hang: Vec<Truong> = vec![
    Truong{ten:vec!['t','u','o','i'],gia_tri:20},
    Truong{ten:vec!['t','e','n'],gia_tri:1},
];

// Loc TRƯỚC (đúng thứ tự): đánh giá trên hàng GỐC, còn đủ cột tuoi
println!("{}", danh_gia(&cay, goc, &hang));
```

```text title=readonly
true
```

`hang` gốc CÓ đủ cả `tuoi` VÀ `ten` — `danh_gia` tìm thấy `tuoi=20`,
`20 > 18` LÀ `true`. Hàng NÀY LẼ RA phải được GIỮ lại.
::::

::::example{#chon-cot-truoc-sai}
Đảo NGƯỢC thứ tự — `ChonCot` TRƯỚC — cùng một hàng, nhưng NAY đánh
giá TRÊN kết quả ĐÃ bị bỏ bớt cột:

```rust title=readonly
let cot: Vec<Vec<char> > = vec![vec!['t','e','n']];
let hang_sau_chon = chon_cot(&hang, &cot);
println!("{}", danh_gia(&cay, goc, &hang_sau_chon));
```

```text title=readonly
false
```

`hang_sau_chon` CHỈ còn cột `ten` — `tuoi` đã BIẾN mất. `danh_gia`
gọi `tra_cuu(hang_sau_chon, "tuoi")`, KHÔNG tìm thấy, trả VỀ `0`
(bài 1) — `0 > 18` LÀ `false`. CÙNG một hàng, CÙNG một điều kiện,
nhưng kết QUẢ ĐẢO ngược HOÀN toàn CHỈ vì đổi THỨ tự hai bước. Đây
KHÔNG phải một vấn đề TỐI ưu (nhanh/chậm) — đây LÀ một lỗi ĐÚNG/SAI.
::::

::::predict{#doan-chon-du-cot commitOnce}
Nếu `SELECT` LIỆT kê CẢ `ten` LẪN `tuoi` (không bỏ SÓT cột nào
`WHERE` cần), đảo thứ tự `ChonCot`/`Loc` CÓ còn gây sai KHÔNG?

:::opt{correct}
Không — vì `tuoi` VẪN còn trong hàng SAU khi chọn cột, `Loc` (dù
chạy SAU) vẫn tìm THẤY nó bình thường
:::

:::opt
Có — vì thứ tự `Quet → Loc → ChonCot` LÀ MỘT quy tắc CỐ định của
mọi kế hoạch, đảo NGƯỢC LUÔN sai bất kể `SELECT` liệt kê CỘT nào
::why
Gần đúng ở việc bạn coi thứ tự "Loc TRƯỚC ChonCot" LÀ một QUY tắc
CHUNG nên tuân theo — một thói quen AN toàn hợp lý (VÀ ĐÚNG LÀ quy
ước `xay_ke_hoach` bài trước LUÔN dùng).

Chỗ lệch: bản THÂN vấn đề CHỈ xảy ra khi `ChonCot` LOẠI BỎ một cột
mà `WHERE` cần TỚI. Nếu `SELECT ten, tuoi FROM nguoi WHERE tuoi >
18` (CẢ hai cột đều được GIỮ), đảo thứ tự KHÔNG còn gây sai — `tuoi`
vẫn CÓ mặt Ở hàng ĐÃ chọn cột, `tra_cuu` vẫn TÌM thấy nó bình
thường. LUÔN đặt `Loc` TRƯỚC LÀ an toàn TUYỆT đối (không phụ thuộc
`SELECT` liệt kê gì) — nhưng bản chất VẤN đề CHỈ xảy ra Ở trường
hợp cụ thể (cột `WHERE` cần bị `SELECT` bỏ SÓT), không phải MỌI
trường hợp.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lọc PHẢI đứng trước chọn cột — không phải TUỲ ý. Bây giờ CHẠY kế
hoạch NÀY trên MỘT bảng — mô hình Volcano "kéo TỪNG hàng một" LÀ gì?
::::

::::reflect{#nghi-lai}
Pushdown ("đẩy" `WHERE` xuống CÀNG sớm càng TỐT trong kế hoạch) THƯỜNG
được dạy như một TỐI ưu hiệu năng (lọc SỚM giảm số hàng phải XỬ lý
tiếp) — nhưng VÍ dụ NÀY cho thấy nó CÒN LÀ một RÀNG buộc ĐÚNG/SAI:
nếu `SELECT` không liệt KÊ hết mọi cột `WHERE` cần, chọn cột TRƯỚC
lọc cho ra KẾT quả SAI, không chỉ CHẬM hơn. `xay_ke_hoach` (bài
trước) LUÔN đặt `Loc` TRƯỚC `ChonCot` — CHÍNH vì lý do NÀY. Kế
hoạch ĐÃ đúng thứ tự — CHẠY nó thật SỰ, từng HÀNG một, LÀ bước tiếp
theo. Mô hình "kéo TỪNG hàng một" gọi LÀ Volcano — nó hoạt động thế
nào?
::::

::::checkpoint{mastery=0.8}
::::
