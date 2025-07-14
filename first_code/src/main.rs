fn main() {
    let mut var: u16 = 0;

    let max_value = loop {
        match var.overflowing_add(1) {
            (_, false) => var+=1,
            (_, true) => break var,
        }
    };

    println!("O valor máximo de u16 é: {max_value}");
}
