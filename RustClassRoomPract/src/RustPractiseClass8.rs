fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else {
                c.to_lowercase().next().unwrap()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_the_case() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter().for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        });
    }
}

fn main() {
    let example = "Hello Привет".to_string();
    println!("{}", invert_the_case(example));
}