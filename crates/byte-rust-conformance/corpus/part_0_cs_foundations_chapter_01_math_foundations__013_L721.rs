// Cũ: 2 × 2 × 2 = 8 trạng thái, hợp lệ chỉ ~4
// → 4 trạng thái thừa = bugs tiềm ẩn

// ✅ Mới: chính xác 4 trạng thái, tất cả hợp lệ
enum AccountStatus {
    Registered,                     // chưa xác minh
    Verified,                       // đã xác minh email
    Active,                         // tài khoản đang hoạt động
    Suspended { reason: String },   // bị tạm khóa (kèm lý do)
}

fn main() {}
