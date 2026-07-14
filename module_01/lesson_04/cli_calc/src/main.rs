use std::io;
use std::io::Write;

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Деление на ноль!"))
            } else {
                Ok(a / b)
            }
        }
        "%" => {
            if b == 0.0 {
                Err(String::from("Деление на ноль!"))
            } else {
                Ok(a % b)
            }
        }
        _ => Err(format!("Неизвестная операция: {}", op)),
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");
    input.trim().to_string()
}

fn main() {
    println!("=== Калькулятор ===");
    println!("Введите выражение в формате: число операция число");
    println!("Операции: + - * / %");
    println!("Для выхода введите: выход\n");

    loop {
        let input = read_input(">>> ");

        if input == "выход" || input == "exit" {
            println!("До встречи!");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!(
                "Ошибка: введите выражение в формате: число операция число"
            );
            continue;
        }

        let a: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Ошибка: '{}' - не число", parts[0]);
                continue;
            }
        };

        let op = parts[1];

        let b: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Ошибка: '{}' - не число", parts[2]);
                continue;
            }
        };

        match calculate(a, op, b) {
            Ok(result) => {
                println!("Результат: {} {} {} = {}\n", a, op, b, result)
            }
            Err(e) => println!("Ошибка: {}\n", e),
        }
    }
}
