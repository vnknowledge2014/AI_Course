// ❌ Không thể viết:
// fn double_inside<F: Functor>(container: F<i32>) -> F<i32> {
//     container.map(|x| x * 2)
// }
// double_inside(Some(5))    → Some(10)
// double_inside(vec![1,2])  → vec![2,4]
// double_inside(Ok(3))      → Ok(6)

// ✅ Phải viết riêng cho mỗi type:
fn double_option(x: Option<i32>) -> Option<i32> { x.map(|n| n * 2) }
fn double_vec(x: Vec<i32>) -> Vec<i32> { x.iter().map(|n| n * 2).collect() }
fn double_result(x: Result<i32, String>) -> Result<i32, String> { x.map(|n| n * 2) }

fn main() {}
