---
id: co-so-du-lieu.bo-nao-cua-truy-van.mo-hinh-volcano
title: "Mô hình Volcano: kéo từng hàng một"
summary: "Volcano (Iterator model) THỰC thi một hàng MỖI lần gọi — không tính hết toàn bộ kết quả một lượt rồi trả về. Một hàm tiep(...) -> Option<Hang> trả về Some(hang) khi CÒN kết quả, None khi HẾT — gọi lặp lại tiep() nhiều lần LÀ cách lấy TOÀN bộ kết quả, mỗi lần CHỈ giữ đúng MỘT hàng trong bộ nhớ."
locale: vi
track: co-so-du-lieu
module: bo-nao-cua-truy-van
order: 8
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 11
teaches: [db.volcano-idea]
requires: [db.pushdown-idea]
concepts: [db.volcano-idea]
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
Kế hoạch đúng thứ tự (bài trước). CHẠY nó — LẤY hết mọi hàng kết
quả CÙNG một lượt, hay TỪNG hàng một?
::::

::::explain{#keo-tung-hang}
Mô hình VOLCANO (còn gọi Iterator model): mỗi lần GỌI hàm THỰC thi,
chỉ LẤY đúng MỘT hàng — KHÔNG tính hết toàn bộ RỒI trả về CÙNG lúc:

```rust title=readonly
fn tiep_tho(bang: &Vec<Vec<Truong> >, vi_tri: &mut usize) -> Option<Vec<Truong> > {
    if *vi_tri >= bang.len() {
        return None;
    }
    let hang = &bang[*vi_tri];
    let mut sao_chep: Vec<Truong> = Vec::new();
    let mut i = 0;
    while i < hang.len() {
        sao_chep.push(Truong { ten: hang[i].ten.clone(), gia_tri: hang[i].gia_tri });
        i = 1 + i;
    };
    *vi_tri = 1 + *vi_tri;
    Some(sao_chep)
}

fn main() {
    let bang: Vec<Vec<Truong> > = vec![
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:20}],
        vec![Truong{ten:vec!['t','u','o','i'],gia_tri:15}],
    ];
    let mut vt: usize = 0;
    let r1 = tiep_tho(&bang, &mut vt);
    match &r1 {
        Some(h) => println!("{}", h[0].gia_tri),
        None => println!("het"),
    }
}
```

```text title=readonly
20
```

`tiep_tho` nhận `vi_tri: &mut usize` — MỘT con TRỎ nhớ ĐANG Ở đâu
trong bảng — trả VỀ ĐÚNG một hàng (`Some(...)`), TỰ tăng `vi_tri`
lên MỘT, rồi TRẢ quyền điều khiển VỀ cho người GỌI NGAY. Không hề
CÓ vòng lặp "chạy hết bảng RỒI trả về danh SÁCH" Ở đây.
::::

::::example{#goi-lap-lai-lay-het}
Gọi LẶP LẠI `tiep_tho` NHIỀU lần LÀ cách lấy TOÀN bộ kết quả — mỗi
lần chỉ giữ ĐÚNG một hàng trong bộ NHỚ:

```rust title=readonly
let r2 = tiep_tho(&bang, &mut vt);
match &r2 {
    Some(h) => println!("{}", h[0].gia_tri),
    None => println!("het"),
}
let r3 = tiep_tho(&bang, &mut vt);
match &r3 {
    Some(h) => println!("{}", h[0].gia_tri),
    None => println!("het"),
}
```

```text title=readonly
15
het
```

Lần GỌI thứ hai LẤY hàng thứ hai (`15`). Lần THỨ ba: `vi_tri` ĐÃ
vượt quá `bang.len()` — trả VỀ `None`, báo "HẾT kết quả". Tên
"Volcano" (núi LỬA) ám chỉ hình ẢNH: mỗi lần "GỌI" LÀ một lần
"phun" ra ĐÚNG một hàng, không PHUN cả núi CÙNG lúc.
::::

::::predict{#doan-vi-sao-tung-hang commitOnce}
So VỚI cách "tính HẾT một lượt, trả VỀ cả `Vec<Hang>`" (gọi LÀ mô
hình "vectorized" — trái ngược VOLCANO), mô hình VOLCANO ("từng
hàng một") có ưu điểm GÌ RÕ RỆT nhất VỀ bộ NHỚ khi bảng CÓ hàng
TRIỆU hàng?

:::opt{correct}
Chỉ CẦN giữ đúng MỘT hàng trong bộ nhớ tại MỘT thời điểm — không
cần cấp PHÁT một `Vec` khổng lồ chứa TOÀN bộ kết quả CÙNG lúc
:::

:::opt
Volcano LUÔN chạy nhanh hơn hẳn cách tính hết một LƯỢT, bất kể kích
thước bảng LÀ bao nhiêu
::why
Gần đúng ở việc bạn liên tưởng "từng bước NHỎ" VỚI "nhanh hơn" —
một trực giác dễ CÓ khi so hai cách tiếp cận KHÁC nhau.

Chỗ lệch: Volcano KHÔNG hề nhanh HƠN VỀ tổng THỜI gian xử lý MỌI
hàng (VẪN phải duyệt qua TỪNG hàng y hệt) — ưu điểm CHÍNH của nó
LÀ BỘ NHỚ (chỉ giữ MỘT hàng tại một THỜI điểm, không cần cấp phát
MỘT cấu trúc khổng lồ chứa TOÀN bộ kết quả), VÀ khả năng "dừng SỚM"
(nếu người GỌI chỉ cần MỘT vài hàng đầu, KHÔNG cần chạy hết TOÀN
bộ bảng). Tốc độ tổng KHÔNG phải điểm khác biệt CHÍNH.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kéo từng hàng một — ý tưởng đã rõ. Thực thi bước Quét (Scan) đầu
tiên trong kế hoạch, dùng chính con trỏ vị trí này, trông ra sao?
::::

::::reflect{#nghi-lai}
Mô hình VOLCANO thay đổi CÂU hỏi TỪ "kết quả LÀ gì" (một `Vec` đầy
đủ) sang "làm sao LẤY hàng TIẾP theo" (một hàm `tiep`, gọi LẶP lại
tới khi `None`). `vi_tri: &mut usize` LÀ trạng THÁI duy nhất CẦN
giữ Ở NGOÀI — mỗi lần GỌI `tiep` tự TĂNG nó lên, KHÔNG cần một cấu
trúc phức TẠP hơn. `tiep_tho` (bài NÀY) mới CHỈ LÀ bước `Quet` —
LẤY từng hàng, CHƯA lọc, CHƯA chọn cột. Ghép THÊM `Loc` VÀO ngay
TRONG `tiep`, trông ra sao?
::::

::::checkpoint{mastery=0.8}
::::
