---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.vi-sao-can-khoa-co-thu-tu
title: "Vì sao cần khoá có thứ tự"
summary: "So sánh SỐ (9 < 10) đúng, nhưng biến số thành CHUỖI chữ số rồi so sánh TỪ điển (Vec<char>) lại SAI — '9' > '1' nên so_thanh_chu_so(9) > so_thanh_chu_so(10), NGƯỢC với 9 < 10. Một hệ khoá-giá trị chỉ so sánh được BYTE trên khoá — muốn tra cứu theo khoảng mà KHÔNG giải mã lại, khoá phải được MÃ hoá sao cho so sánh byte PHẢN ánh đúng so sánh số."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 1
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 9
teaches: [db.ordered-key-why]
requires: [db.executor-project-step]
concepts: [db.ordered-key-why]
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
q08 xây xong bộ não thực thi — CHẠY được câu truy vấn. Nhưng `tra_cuu`
(bài 1, q08) quét TUYẾN tính, từng trường một. Nếu khoá được lưu
theo một thứ TỰ nào đó, tra cứu có thể nhanh hơn nhiều — nhưng thứ
TỰ đó phải đến từ ĐÂU?
::::

::::explain{#so-sanh-byte-khong-phai-so-sanh-so}
Nhiều hệ lưu trữ khoá-giá trị KHÔNG hiểu "khoá này LÀ một con số" —
chúng chỉ so sánh khoá THEO byte/ký tự, giống hệt so sánh TỪ điển
hai chuỗi. Nếu biến số nguyên thành DÃY chữ số thập phân rồi lưu
làm khoá, so sánh byte có PHẢN ánh đúng so sánh số không?

```rust title=readonly
fn so_thanh_chu_so(n: i64) -> Vec<char> {
    if n < 10 {
        let c = (('0' as u8) + (n as u8)) as char;
        vec![c]
    } else {
        let chuc = n / 10;
        let don_vi = n % 10;
        let c1 = (('0' as u8) + (chuc as u8)) as char;
        let c2 = (('0' as u8) + (don_vi as u8)) as char;
        vec![c1, c2]
    }
}

fn main() {
    let a = so_thanh_chu_so(9);
    let b = so_thanh_chu_so(10);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a < b);
    println!("{}", 9 < 10);
}
```

```text title=readonly
['9']
['1', '0']
false
true
```

`9 < 10` (so sánh SỐ) LÀ `true` — không có gì bàn cãi. Nhưng
`so_thanh_chu_so(9) < so_thanh_chu_so(10)` (so sánh TỪ điển trên
`Vec<char>`) lại LÀ `false`, vì ký tự `'9'` đứng SAU `'1'` trong
bảng mã. Hai phép so sánh trả về hai kết quả TRÁI ngược nhau — biểu
diễn "chuỗi chữ số" này KHÔNG giữ đúng thứ tự số.
::::

::::example{#lech-cang-ro-o-do-dai-khac-nhau}
Độ LỆCH càng rõ khi hai số có ĐỘ dài chữ số khác nhau:

```rust title=readonly
let c = so_thanh_chu_so(5);
let d = so_thanh_chu_so(12);
println!("{:?}", c);
println!("{:?}", d);
println!("{}", c < d);
println!("{}", 5 < 12);
```

```text title=readonly
['5']
['1', '2']
false
true
```

`5 < 12` LÀ `true`, nhưng `['5'] < ['1', '2']` LÀ `false` — ký tự
ĐẦU tiên `'5'` lớn hơn `'1'`, so sánh TỪ điển dừng NGAY ở đó, không
hề "nhìn" tới việc `12` có HAI chữ số còn `5` chỉ có MỘT. Biểu diễn
NGÂY thơ này hỏng theo đúng CÙNG một cách ở CẢ hai ví dụ: chữ số ĐẦU
tiên (hàng cao nhất) quyết định so sánh TỪ điển, nhưng độ DÀI chuỗi
lại không phản ánh ĐỘ lớn của số.
::::

::::predict{#doan-99-vs-100 commitOnce}
`so_thanh_chu_so(99)` cho `['9', '9']`, `so_thanh_chu_so(100)` cho
`['1', '0', '0']` (mở rộng hàm ở trên để chấp nhận số có BA chữ số).
So sánh `so_thanh_chu_so(99) < so_thanh_chu_so(100)` cho kết quả gì?

:::opt
`true` — giống hệt `99 < 100`
::why
Gần đúng ở việc bạn kỳ vọng biểu diễn "chuỗi chữ số" PHẢI phản ánh
đúng quan hệ số — một kỳ vọng hợp lý cho MỌI cách mã hoá "TỐT".

Chỗ lệch: biểu diễn NÀY không hề tốt theo nghĩa đó — nó chỉ LÀ dãy
ký tự thập phân KHÔNG có độ dài cố định. Ký tự ĐẦU của `99` LÀ
`'9'`, ký tự ĐẦU của `100` LÀ `'1'` — so sánh TỪ điển dừng ở vị trí
ĐẦU tiên lệch nhau, thấy `'9' > '1'` rồi kết luận `99 > 100`. Kết
quả THẬT LÀ `false`, đúng CÙNG lỗi đã thấy ở `9` với `10`.
::
:::

:::opt{correct}
`false` — NGƯỢC với `99 < 100`
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Vấn đề đã RÕ: mã hoá "chuỗi chữ số" không giữ thứ tự. Một cách mã
hoá KHÁC — dùng byte thay vì ký tự thập phân — có giải quyết được
không?
::::

::::reflect{#nghi-lai}
Bài học CỐT lõi: một hệ chỉ so sánh được BYTE trên khoá KHÔNG tự
động "hiểu" khoá đó LÀ số — nó chỉ thấy một DÃY byte, so TỪNG vị
trí một, giống hệt so hai TỪ trong từ điển. Muốn tra cứu "MỌI khoá
lớn hơn X" hay "MỌI khoá trong khoảng [A, B]" mà KHÔNG cần giải mã
ngược lại từng khoá về số để so sánh, chính bản THÂN cách MÃ hoá
phải đảm bảo: so sánh byte cho ĐÚNG kết quả như so sánh số. Biểu
diễn "chuỗi chữ số" ở bài NÀY không có tính chất đó. Một cách mã
hoá KHÁC — độ RỘNG cố định, dùng `u8` thay vì ký tự — trông ra sao?
::::

::::checkpoint{mastery=0.75}
::::
