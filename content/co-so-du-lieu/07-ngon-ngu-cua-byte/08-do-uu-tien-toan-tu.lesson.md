---
id: co-so-du-lieu.ngon-ngu-cua-byte.do-uu-tien-toan-tu
title: "Độ ưu tiên: AND bó chặt hơn OR"
summary: "'a>1 OR b>2 AND c>3' phải đọc thành 'a>1 OR (b>2 AND c>3)' — AND có độ ưu tiên CAO hơn OR, giống && / || hầu hết ngôn ngữ. do_uu_tien(t: &Token) -> i64 trả về một con số (Hoac=1, Va=2, còn lại=3) — số CÀNG lớn càng bó CHẶT hơn."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 8
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 11
teaches: [db.parser-precedence]
requires: [db.ast-arena]
concepts: [db.parser-precedence]
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
Cây đã có chỗ chứa (bài TRƯỚC). Nhưng `"a>1 OR b>2 AND c>3"` gộp
THEO thứ tự nào — `(a>1 OR b>2) AND c>3`, hay `a>1 OR (b>2 AND
c>3)`? Kết quả HOÀN toàn khác nhau.
::::

::::explain{#do-uu-tien}
`AND` có độ ưu tiên CAO hơn `OR` — giống `&&`/`||` Ở hầu hết ngôn
ngữ. `do_uu_tien` gán một con SỐ cho mỗi loại token: số CÀNG lớn
càng bó CHẶT hơn:

```rust title=readonly
enum Token {
    So(i64),
    DinhDanh(Vec<char>),
    Chon,
    Tu,
    ODau,
    Va,
    Hoac,
    Phay,
    Bang,
    LonHon,
    NhoHon,
    KetThuc,
}

fn do_uu_tien(t: &Token) -> i64 {
    match t {
        Token::Hoac => 1,
        Token::Va => 2,
        _ => 3,
    }
}

fn main() {
    println!("{}", do_uu_tien(&Token::Hoac));
    println!("{}", do_uu_tien(&Token::Va));
}
```

```text title=readonly
1
2
```

`Token::Hoac` (OR) LÀ `1` — độ ưu tiên THẤP nhất. `Token::Va` (AND)
LÀ `2` — CAO hơn OR. Nhánh `_` (mọi token KHÁC — dấu so sánh, số,
định danh...) trả VỀ `3`, cao NHẤT: chúng không phải toán tử GHÉP
bieu_thuc, nên "bó chặt" hơn CẢ AND lẫn OR theo quy ước NÀY.
::::

::::example{#so-sanh-hai-do-uu-tien}
So sánh trực tiếp hai độ ưu tiên bằng phép toán SO sánh SỐ bình
thường:

```rust title=readonly
println!("{}", do_uu_tien(&Token::Va) > do_uu_tien(&Token::Hoac));
```

```text title=readonly
true
```

`do_uu_tien(&Token::Va)` LÀ `2`, `do_uu_tien(&Token::Hoac)` LÀ `1`
— `2 > 1` LÀ `true`. Đây CHÍNH LÀ điều Ý nghĩa nhất của cả bài học:
"AND bó chặt hơn OR" không phải một QUY tắc trừu tượng, nó LÀ một
phép so sánh SỐ đơn giản, tính được NGAY bằng `>`.
::::

::::predict{#doan-so-sanh-uu-tien-cao-nhat commitOnce}
So sánh độ ưu tiên của một dấu SO sánh (`Token::LonHon`, dấu `>`)
VỚI `Token::Va` (AND):

```rust
println!("{}", do_uu_tien(&Token::LonHon) > do_uu_tien(&Token::Va));
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì `LonHon` (dấu `>`) không phải MỘT trong hai toán tử
ghép chính (`AND`/`OR`), nên KHÔNG có độ ưu tiên nào cả
::why
Gần đúng ở việc bạn nghĩ TỚI "chỉ AND/OR mới CÓ độ ưu tiên" như một
giới hạn hợp lý — CHỈ hai toán tử ĐÓ mới xuất hiện Ở "tầng" bieu_
thuc trong ngữ pháp track NÀY.

Chỗ lệch: `do_uu_tien` LÀ một hàm TOÀN ánh — nó gán một con SỐ cho
MỌI biến thể `Token`, không CHỈ hai cái. `Token::LonHon` rơi vào
nhánh `_`, nhận `3` — một con SỐ hợp lệ, dùng ĐƯỢC trong phép so
sánh `>` y hệt mọi con số khác. `3 > 2` LÀ `true`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Độ ưu tiên đã có con SỐ đo được. Bước tiếp: parse MỘT phép so sánh
đơn — `tuoi > 18` — thành một nút TRONG cây.
::::

::::reflect{#nghi-lai}
`do_uu_tien` biến một Ý niệm ngôn NGỮ học ("AND bó chặt hơn OR")
thành một PHÉP so sánh số học ĐƠN giản — đây chính LÀ "trick" cốt
lõi của thuật toán Pratt/precedence-climbing: KHÔNG cần một bảng
luật phức tạp, chỉ cần MỘT hàm ánh xạ token → số, RỒI so sánh SỐ.
Bài SAU dùng con số NÀY để quyết định: gặp `AND` ngay sau một phép
so sánh, GỘP liền hay chờ? Nhưng TRƯỚC hết, cần parse ĐƯỢC một phép
so sánh ĐƠN — `tuoi > 18` — thành một nút trong cây. Trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
