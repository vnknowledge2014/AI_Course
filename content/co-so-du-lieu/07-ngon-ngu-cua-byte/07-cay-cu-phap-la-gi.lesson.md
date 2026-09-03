---
id: co-so-du-lieu.ngon-ngu-cua-byte.cay-cu-phap-la-gi
title: "Cây cú pháp là gì: arena thay vì Box"
summary: "Cây cú pháp trừu tượng (AST) biểu diễn cấu trúc câu truy vấn. CayAst { nut: Vec<NoAst> } — mỗi NoAst chứa CHỈ SỐ (usize) trỏ tới nút con TRONG CHÍNH Vec đó, không con trỏ thật — vì Box<Self> chưa dùng được ở đây. Đây là arena, một kỹ thuật Rust thật, nhiều parser production dùng để né vấn đề mượn/vòng đời của cây đệ quy."
locale: vi
track: co-so-du-lieu
module: ngon-ngu-cua-byte
order: 7
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 12
teaches: [db.ast-arena]
requires: [db.lexer-hoan-chinh]
concepts: [db.ast-arena]
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
`Vec<Token>` phẳng (bài trước) CHƯA nói lên cấu TRÚC. `tuoi > 18` VÀ
`diem < 100` nối bằng `AND` cần một hình dạng KHÁC — một CÂY. Cây
đó lưu Ở đâu?
::::

::::explain{#arena-thay-box}
Cách quen thuộc để biểu diễn cây LÀ enum đệ quy với `Box<Self>` —
`enum NoAst { ..., Con(Box<NoAst>) }`. Cách ĐÓ **không dùng được**
trong track NÀY (`Box::new` chưa cài). Thay VÀO đó, dùng **arena**:
một `Vec` PHẲNG chứa MỌI nút, con trỏ LÀ CHỈ SỐ (`usize`) trỏ VÀO
chính `Vec` đó:

```rust title=readonly
enum NoAst {
    SoSanh(Vec<char>, char, i64),
    VaNut(usize, usize),
    HoacNut(usize, usize),
}

struct CayAst {
    nut: Vec<NoAst>,
}

fn main() {
    let mut cay = CayAst { nut: Vec::new() };

    let tuoi: Vec<char> = vec!['t','u','o','i'];
    cay.nut.push(NoAst::SoSanh(tuoi, '>', 18));

    let diem: Vec<char> = vec!['d','i','e','m'];
    cay.nut.push(NoAst::SoSanh(diem, '<', 100));

    cay.nut.push(NoAst::VaNut(0, 1));

    println!("{}", cay.nut.len());
}
```

```text title=readonly
3
```

Ba nút: chỉ số `0` LÀ `tuoi > 18`, chỉ số `1` LÀ `diem < 100`, chỉ
số `2` LÀ `VaNut(0, 1)` — nút GỘP, trỏ TỚI hai nút con bằng CHÍNH
chỉ số của chúng trong `nut`, không phải con trỏ bộ nhớ THẬT.
`NoAst::SoSanh` dùng dạng TUPLE-variant (`(Vec<char>, char, i64)`),
không phải struct-variant (`{ truong: ..., }`) — engine track này
CHỈ dựng được biến thể enum theo VỊ trí, không theo tên trường. Tên
`VaNut`/`HoacNut` (không phải `Va`/`Hoac` trần) LÀ CÓ CHỦ đích:
track NÀY sắp có token `Token::Va`/`Token::Hoac` (đã dạy Ở bài 4) —
đặt TRÙNG tên biến thể GIỮA hai enum khác nhau khiến engine chấm
NHẦM nút thuộc enum này LÀ enum kia lúc `match`.
::::

::::example{#doc-lai-nut-goc}
Đọc LẠI nút gốc (chỉ số `2`) — `match` trên tham chiếu tới phần tử
`Vec`, lấy RA hai chỉ số con:

```rust title=readonly
match &cay.nut[2] {
    NoAst::VaNut(trai, phai) => println!("{} {}", trai, phai),
    _ => println!("?"),
}
```

```text title=readonly
0 1
```

`NoAst::VaNut(0, 1)` KHÔNG "chứa" hai nút con — nó chỉ chứa HAI SỐ.
Muốn ĐỌC nút con thật SỰ, phải INDEX lại vào `cay.nut[0]` VÀ
`cay.nut[1]` — một bước gián tiếp THÊM, đổi lại: không cần `Box`,
không cần lo mượn/vòng đời của con trỏ đệ quy (điều mà cây `Box`
kiểu cũ luôn gặp phải trong Rust thật) — đây LÀ đánh đổi CÓ chủ
đích của kỹ thuật arena, không phải một hạn chế PHẢI né tránh.
::::

::::predict{#doan-them-nut-sau commitOnce}
Sau khi cây ĐÃ có ba nút (`0`, `1`, `2` như TRÊN), thêm một nút MỚI
(`tên = ...`) vào CUỐI, RỒI đọc lại nút `VaNut` Ở chỉ số `2`:

```rust
let ten: Vec<char> = vec!['t','e','n'];
cay.nut.push(NoAst::SoSanh(ten, '=', 0));
println!("{}", cay.nut.len());
match &cay.nut[2] {
    NoAst::VaNut(trai, phai) => println!("{} {}", trai, phai),
    _ => println!("?"),
}
```

Dòng CUỐI in ra gì?

:::opt{correct}
`4` rồi `0 1`
:::

:::opt
`4` rồi `0 1` bị lệch thành một cặp SỐ khác — vì thêm nút MỚI làm
xáo trộn chỉ số của các nút CŨ
::why
Gần đúng ở việc bạn thận trọng VỀ việc "thêm phần tử có thể làm
lệch mọi thứ" — một lo ngại hợp lý khi làm việc VỚI cấu trúc dữ
liệu có chỉ số.

Chỗ lệch: `.push(...)` LUÔN thêm phần tử vào CUỐI `Vec`, KHÔNG BAO
GIỜ chèn vào giữa hay dịch chuyển các phần tử ĐÃ có — chỉ số `0`,
`1`, `2` (đã gán TỪ trước) giữ NGUYÊN ý nghĩa mãi mãi, chừng nào
không có thao tác XOÁ/chèn-giữa nào xảy ra (track NÀY không dùng
thao tác đó). `NoAst::VaNut(0, 1)` vẫn trỏ ĐÚNG hai nút cũ, không hề
đổi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Arena dựng xong — cây đọc/ghi được qua chỉ số. Nhưng `"a > 1 OR b >
2 AND c > 3"` phải NHÓM đúng thứ tự — AND VÀ OR ai bó chặt hơn?
::::

::::reflect{#nghi-lai}
`CayAst { nut: Vec<NoAst> }` LÀ nền cho MỌI bài còn lại của quest —
mỗi hàm "thêm nút" chỉ LÀ một lệnh `.push()`, mỗi lần "đi xuống con"
chỉ LÀ một lệnh INDEX vào `nut`. Arena thay Box KHÔNG chỉ LÀ một
cách né hạn chế của track — nó phản ánh một lựa chọn kiến trúc CÓ
thật trong Rust sản xuất, đặc biệt Ở những nơi cây thay đổi NHIỀU
hoặc cần duyệt qua lại (một nút CÓ thể được nhiều nút KHÁC tham
chiếu tới, điều `Box` sở hữu-duy-nhất không cho phép dễ dàng). Cây
đã có CHỖ chứa — bước tiếp LÀ quyết định THỨ TỰ ghép các nút SO
SÁNH lại với nhau: `AND` VÀ `OR`, ai bó chặt hơn?
::::

::::checkpoint{mastery=0.8}
::::
