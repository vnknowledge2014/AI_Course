---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.khoa-ghep-cam-bay-noi-chuoi
title: "Khoá ghép — cạm bẫy nối chuỗi ngây thơ"
summary: "noi_ngay_tho(a, b) nối trực tiếp hai Vec<char>. Hai CẶP trường khác nhau — ('ab','c') và ('a','bc') — cho CÙNG một khoá ['a','b','c'] sau khi nối, dù ý nghĩa hoàn toàn khác nhau. Nối ngây thơ hai trường làm khoá ghép có thể ĐỤNG ĐỘ (hai khoá logic khác nhau trở thành một khoá vật lý giống hệt)."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 3
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 9
teaches: [db.composite-key-pitfall]
requires: [db.int-key-encoding]
concepts: [db.composite-key-pitfall]
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
Khoá từ MỘT trường (số) đã mã hoá xong. Nhiều chỉ mục THẬT cần khoá
GHÉP từ HAI trường trở lên (VÍ dụ: tên bảng + tên cột). Nối trực
tiếp hai chuỗi lại — cách ĐƠN giản nhất nghĩ tới ĐẦU tiên — có ổn
không?
::::

::::explain{#noi-truc-tiep}
`noi_ngay_tho` nối chuỗi `a` RỒI nối tiếp chuỗi `b` NGAY sau, không
CÓ gì phân TÁCH giữa hai phần:

```rust title=readonly
fn noi_ngay_tho(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut ra: Vec<char> = Vec::new();
    let mut i = 0;
    while i < a.len() {
        ra.push(a[i]);
        i = 1 + i;
    }
    let mut j = 0;
    while j < b.len() {
        ra.push(b[j]);
        j = 1 + j;
    }
    ra
}

fn main() {
    let a1: Vec<char> = vec!['a','b'];
    let b1: Vec<char> = vec!['c'];
    let a2: Vec<char> = vec!['a'];
    let b2: Vec<char> = vec!['b','c'];
    let k1 = noi_ngay_tho(&a1, &b1);
    let k2 = noi_ngay_tho(&a2, &b2);
    println!("{:?}", k1);
    println!("{:?}", k2);
    println!("{}", k1 == k2);
}
```

```text title=readonly
['a', 'b', 'c']
['a', 'b', 'c']
true
```

`("ab", "c")` VÀ `("a", "bc")` LÀ hai CẶP trường khác nhau — ý nghĩa
logic hoàn toàn KHÁC (VÍ dụ: bảng "ab" cột "c", so với bảng "a" cột
"bc"). NHƯNG sau khi nối ngây thơ, CẢ hai đều cho ĐÚNG cùng một
`Vec<char>`: `['a', 'b', 'c']`. Khoá vật LÝ giống hệt nhau, dù khoá
LOGIC khác nhau — đây LÀ một ĐỤNG độ (collision).
::::

::::example{#hau-qua-cua-dung-do}
Hậu quả: một chỉ mục dùng `noi_ngay_tho` làm khoá sẽ COI hai mục
khác nhau (bảng "ab" cột "c" VÀ bảng "a" cột "bc") LÀ MỘT mục DUY
nhất. Tra cứu khoá `['a','b','c']` sẽ trả VỀ lẫn LỘN dữ liệu của CẢ
hai — không CÁCH nào phân biệt LẠI được, vì thông tin "chỗ nào LÀ
ranh giới giữa hai trường" ĐÃ mất NGAY từ lúc mã hoá. Đây KHÔNG phải
lỗi HIẾM — bất kỳ CẶP trường nào mà một trường có thể "cho MƯỢN" vài
ký tự CUỐI sang trường kia (VÀ vẫn tạo ra chuỗi tổng giống hệt) đều
đụng độ theo ĐÚNG cách này.
::::

::::predict{#doan-cap-khac commitOnce}
CẶP trường `("x", "yz")` VÀ CẶP `("xy", "z")` — nối bằng
`noi_ngay_tho`. Hai khoá kết quả CÓ đụng độ (bằng nhau) không?

:::opt{correct}
CÓ — cả hai đều cho `['x', 'y', 'z']`
:::

:::opt
KHÔNG — vì `"x"` VÀ `"xy"` có ĐỘ dài khác nhau, `noi_ngay_tho` sẽ
tạo ra hai `Vec<char>` khác ĐỘ dài
::why
Gần đúng ở việc bạn để Ý đúng rằng ĐỘ dài của trường ĐẦU (`a`) khác
nhau giữa hai CẶP (`1` so VỚI `2`) — một quan sát THẬT về đầu vào.

Chỗ lệch: `noi_ngay_tho` không hề GIỮ lại thông tin "trường `a` dài
BAO nhiêu" trong kết QUẢ — nó chỉ nối THẲNG các ký tự lại, không
chèn gì đánh dấu ranh GIỚI. Kết quả CUỐI của CẢ hai phép nối đều LÀ
`['x', 'y', 'z']` — ba ký tự GIỐNG hệt nhau theo ĐÚNG thứ tự, dù
ĐỘ dài từng trường gốc khác nhau. Đụng ĐỘ vẫn xảy ra.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đụng độ ĐẾN từ việc mất thông tin ranh GIỚI. Thêm lại thông tin đó —
bằng cách NÀO — sẽ giải quyết được vấn đề?
::::

::::reflect{#nghi-lai}
Cạm bẫy CỐT lõi: nối chuỗi KHÔNG phải một phép mã hoá "MỘT-một"
(injective) TRỪ khi có thêm thông tin đánh dấu RANH giới giữa các
phần — thiếu nó, nhiều CẶP đầu vào LOGIC khác nhau có thể ánh xạ tới
CÙNG một chuỗi kết quả. Đây LÀ đúng LOẠI lỗi mà một chỉ mục THẬT
KHÔNG được phép mắc: hai khoá LOGIC khác nhau ĐỤNG thành một khoá
vật LÝ nghĩa LÀ dữ liệu bị LẪN, không CÒN cách phân biệt lại. Cách
sửa: chèn thêm một TÍN hiệu đánh dấu ranh giới VÀO giữa hai trường
lúc mã hoá — trông ra sao?
::::

::::checkpoint{mastery=0.75}
::::
