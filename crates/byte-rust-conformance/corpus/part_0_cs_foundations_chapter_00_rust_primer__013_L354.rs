fn main() {
    let name = "Rust";
    let version = 1.84;

    // Sử dụng {} làm "chỗ trống", giá trị sẽ được điền vào theo thứ tự
    println!("Hello, {}!", name);
    println!("{} version {}", name, version);

    // Dùng {:?} để in cấu trúc dữ liệu theo định dạng Debug (rất tiện để soi nội dung struct/enum/mảng)
    let nums = vec![1, 2, 3];
    println!("nums = {:?}", nums);

    // Định dạng số thực hiển thị đúng 2 chữ số thập phân với {:.2}
    println!("Pi ≈ {:.2}", std::f64::consts::PI);
}
