// filename: src/main.rs

#[derive(Debug)]
enum Permission {
    Read,
    Write,
    Execute,
    Admin,
}

// Thêm variant mới? Compiler báo MỌI CHỖ cần sửa!
// Thử uncomment Delete:
// #[derive(Debug)]
// enum Permission {
//     Read, Write, Execute, Admin, Delete,  // ← MỚI
// }
// → Compiler error ở MỌI match block thiếu Delete!

fn can_modify(perm: &Permission) -> bool {
    match perm {
        Permission::Write | Permission::Admin => true,
        Permission::Read | Permission::Execute => false,
        // Nếu thêm Delete → compiler bắt buộc xử lý ở đây!
    }
}

fn permission_level(perm: &Permission) -> u8 {
    match perm {
        Permission::Read => 1,
        Permission::Write => 2,
        Permission::Execute => 3,
        Permission::Admin => 4,
        // Thêm variant → BAT BUOC thêm arm ở đây!
    }
}

fn main() {
    let perms = vec![Permission::Read, Permission::Write, Permission::Admin];
    for p in &perms {
        println!("{:?}: level={}, can_modify={}", p, permission_level(p), can_modify(p));
    }
}
