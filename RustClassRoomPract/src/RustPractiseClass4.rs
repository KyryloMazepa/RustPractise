const SIZE: usize = 5; // Розмір ромба

fn main() {
    
    let mut diamond = String::new();

    // Верхня частина ромба
    for i in 0..SIZE {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "*".repeat(2 * i + 1);
        diamond.push_str(&format!("{}{}\n", spaces, stars));
    }

    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "*".repeat(2 * i + 1);
        diamond.push_str(&format!("{}{}\n", spaces, stars));
    }

    
    print!("{}", diamond);
}