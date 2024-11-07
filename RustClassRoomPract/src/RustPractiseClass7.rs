fn draw_tree(triangles: usize) {
    let mut width = 1; // Початкова ширина першого трикутника

    for i in 1..=triangles {
        // Кількість рядків у кожному трикутнику
        let height = i;

        // Малюємо кожен рядок трикутника
        (0..height).for_each(|j| {
            let padding = " ".repeat(triangles + height - j - 1); // Відступ зліва для центрування
            let stars = "*".repeat(width + 2 * j); // Зірочки для кожного рядка трикутника
            println!("{}{}", padding, stars);
        });

        // Збільшуємо початкову ширину для наступного трикутника
        width += 2;
    }
}

fn main() {
    let triangles = 5; // Задайте бажану кількість трикутників
    draw_tree(triangles);
}
