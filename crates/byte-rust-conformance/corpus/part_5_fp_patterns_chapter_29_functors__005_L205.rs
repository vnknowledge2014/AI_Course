// filename: src/main.rs

#[derive(Debug, Clone)]
enum Tree<T> {
    Leaf(T),
    Branch { left: Box<Tree<T>>, right: Box<Tree<T>> },
}

impl<T> Tree<T> {
    fn leaf(value: T) -> Self { Tree::Leaf(value) }

    fn branch(left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Branch { left: Box::new(left), right: Box::new(right) }
    }

    // Functor map: transform ALL values, keep tree structure
    fn map<U, F: Fn(&T) -> U>(&self, f: &F) -> Tree<U> {
        match self {
            Tree::Leaf(val) => Tree::Leaf(f(val)),
            Tree::Branch { left, right } => Tree::Branch {
                left: Box::new(left.map(f)),
                right: Box::new(right.map(f)),
            },
        }
    }

    fn to_vec(&self) -> Vec<&T> {
        match self {
            Tree::Leaf(val) => vec![val],
            Tree::Branch { left, right } => {
                let mut v = left.to_vec();
                v.extend(right.to_vec());
                v
            }
        }
    }
}

fn main() {
    //       Branch
    //      /      \
    //   Branch    Leaf(4)
    //   /    \
    // Leaf(1) Leaf(2)
    let tree = Tree::branch(
        Tree::branch(Tree::leaf(1), Tree::leaf(2)),
        Tree::leaf(4),
    );

    println!("Original: {:?}", tree.to_vec());

    let doubled = tree.map(&|x| x * 2);
    println!("Doubled: {:?}", doubled.to_vec());

    let strings = tree.map(&|x| format!("#{}", x));
    println!("Strings: {:?}", strings.to_vec());
}
