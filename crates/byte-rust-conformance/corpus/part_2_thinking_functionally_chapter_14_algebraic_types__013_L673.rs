enum A { X, Y, Z }              // ? states
struct B { flag: bool, choice: A } // ? states
enum C { P(bool), Q(A) }           // ? states

fn main() {}
