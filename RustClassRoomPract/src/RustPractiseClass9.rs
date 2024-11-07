fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false; // Числа менші або рівні 1 не є простими
    }
    if *n == 2 {
        return true; // 2 — єдине парне просте число
    }
    if *n % 2 == 0 {
        return false; // парні числа більші за 2 не є простими
    }

    // Перевірка дільників від 3 до √n
    let limit = (*n as f64).sqrt() as u32;
    for i in 3..=limit {
        if *n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data
            .iter()
            .for_each(|(n, prime)| {
                assert_eq!(is_prime(n), *prime);
            });
    }
}
