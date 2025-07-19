struct SymmetricMatrix{
    _data: Vec<bool>,
    _n: usize,
}

impl SymmetricMatrix {
    fn new(n:usize) -> Self {
        return SymmetricMatrix{ _data: vec![false;(n*n-1)/2], _n: (n) }
    }
}


fn main() {
    println!("Hello, world!");
    
}
