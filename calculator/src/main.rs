use std::io;

use colored::*;

// Hesap makinesi uygulamas. Bu uygulama 4 işlem yapabilmektedir.
// Toplama, çıkarma, çarpma ve bölme işlemleri yapabilmektedir.
// Bu uygulama ile Rust dilinde fonksiyon tanımlamaları, kullanımı ve
// parametrelerin nasıl kullanılacağı öğrenilebilir.
fn main() {
    println!("{}", "Hesap Makinesi".bold().blue());
    println!("{}", "Bir işlem girin (örnek: 5 + 3):".bold().green());
    let mut input = String::new();
    let operator: &str;

    io::stdin().read_line(&mut input).expect("Başarısız okuma");
    operator = get_operator(&input);

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    let num1: i32 = tokens[0].parse().expect("Geçersiz sayı");
    let num2: i32 = tokens[2].parse().expect("Geçersiz sayı");

    match calculate(num1, num2, operator) {
        Ok(result) => println!("Sonuç: {}", result),
        Err(e) => println!("Hata: {}", e),
    };
}

fn get_operator(input: &String) -> &str {
    if input.contains("+") {
        return "+";
    }

    if input.contains("-") {
        return "-";
    }

    if input.contains("*") {
        return "*";
    }

    if input.contains("/") {
        return "/";
    }

    panic!("Geçersiz işlem");
}

fn calculate(num1: i32, num2: i32, operator: &str) -> Result<i32, &str> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => Ok(num1 / num2),
        _ => Err("Geçersiz işlem"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(5, 3, "+").unwrap(), 8);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(5, 3, "-").unwrap(), 2);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(5, 3, "*").unwrap(), 15);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(6, 3, "/").unwrap(), 2);
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(calculate(5, 3, "^").unwrap_err(), "Geçersiz işlem");
    }
}
