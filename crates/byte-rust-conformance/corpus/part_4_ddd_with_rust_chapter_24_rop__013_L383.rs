// filename: src/main.rs

// 1. Types
struct RegistrationForm { name: String, email: String }
struct RegisteredUser { id: u64, name: String, email: String }

// 2. Định nghĩa Validated gọn nhẹ
enum V<T> { Ok(T), Errs(Vec<String>) }

fn combine2<A,B,R>(a: V<A>, b: V<B>, f: impl FnOnce(A,B)->R) -> V<R> {
    let mut errs = vec![];
    let val_a = match a { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    let val_b = match b { V::Ok(v) => Some(v), V::Errs(e) => { errs.extend(e); None } };
    
    if errs.is_empty() { V::Ok(f(val_a.unwrap(), val_b.unwrap())) }
    else { V::Errs(errs) }
}

// 3. Validators
fn v_name(name: &str) -> V<String> {
    if name.len() < 2 { V::Errs(vec!["Name: Tối thiểu 2 ký tự".into()]) } else { V::Ok(name.into()) }
}
fn v_email(email: &str) -> V<String> {
    if !email.contains('@') { V::Errs(vec!["Email: Phải có chữ @".into()]) } else { V::Ok(email.into()) }
}

// 4. Hàm Chốt (Đóng gói 2 Giai đoạn)
fn register(form: RegistrationForm) -> Result<RegisteredUser, Vec<String>> {
    
    // Giai đoạn 1: Gom mọi lỗi chính tả (Validate)
    let validated = combine2(v_name(&form.name), v_email(&form.email), |n, e| (n, e));

    match validated {
        V::Errs(errors) => Err(errors), // Trả 1 rổ lỗi cho Frontend
        V::Ok((name, email)) => {
            // Giai đoạn 2: Lỗi nghiệp vụ (Business Rules - Fail Fast)
            if email == "admin@co.com" { 
                return Err(vec!["Không được phép đăng ký email này".into()]); 
            }
            Ok(RegisteredUser { id: 99, name, email })
        }
    }
}

fn main() {}
