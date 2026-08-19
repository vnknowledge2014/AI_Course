fn main() {
    let result = 2 + 3;
    assert_eq!(result, 5);  // ✅ Pass: Mọi thứ êm đẹp, chương trình tiếp tục chạy

    // assert_eq!(result, 99); // ❌ Lỗi! Chương trình sẽ hoảng loạn (panic) và dừng lại kèm thông báo lỗi
    println!("All assertions passed!");
}
