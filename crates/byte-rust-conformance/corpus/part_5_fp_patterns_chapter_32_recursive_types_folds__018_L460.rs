enum Html {
    Text(String),
    Element { tag: String, attrs: Vec<(String, String)>, children: Vec<Html> },
}

fn main() {}
