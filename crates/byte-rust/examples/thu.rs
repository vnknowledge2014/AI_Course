use byte_rust::interp::chay;
use byte_rust::span::SourceMap;
fn main() {
    let src = r#"
fn main() {
    let v = vec![1, 2, 3, 4];
    let t: i64 = v.iter().map(|x| x * 2).filter(|x| *x > 4).sum();
    println!("{}", t);
}"#;
    let (out, d) = chay(src);
    println!("XUẤT: {out:?}");
    if !d.is_empty() { println!("{}", d.render(&SourceMap::new(src))); }
}
