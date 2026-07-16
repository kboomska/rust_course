use std::io::{self, Write};

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
        "^" => {
            if a == 0.0 && b < 0.0 {
                Err(String::from("Деление на ноль!"))
            } else {
                Ok(a.powf(b))
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

fn parse_operand(value: &str, last: Option<f64>) -> Result<f64, String> {
    if value == "last" {
        match last {
            Some(last) => Ok(last),
            None => Err(String::from("Нет предыдущих результатов")),
        }
    } else {
        value
            .parse::<f64>()
            .map_err(|_| format!("'{}' - не является числом", value))
    }
}

fn main() {
    let mut last: Option<f64> = None;
    let mut operations: u32 = 0;

    println!("=== Расширенный калькулятор ===");
    println!("Введите выражение в формате: число операция число");
    println!("Операции: + - * / % ^");
    println!("Для справки введите: помощь\n");
    println!("Для выхода введите: выход\n");

    loop {
        let input = read_input(">>> ");

        if input == "выход" || input == "exit" {
            println!("Выполнено вычислений: {operations}");
            println!("До встречи!");
            break;
        }

        if input == "помощь" || input == "help" {
            println!("Доступные операции: + - * / % ^");
            println!(
                "Используйте 'last' для подстановки предыдущего результата."
            );
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!(
                "Ошибка! Введите выражение в формате: число операция число"
            );
            continue;
        }

        let a: f64 = match parse_operand(parts[0], last) {
            Ok(value) => value,
            Err(e) => {
                println!("Ошибка: {e}");
                continue;
            }
        };

        let op = parts[1];

        let b: f64 = match parse_operand(parts[2], last) {
            Ok(value) => value,
            Err(e) => {
                println!("Ошибка: {e}");
                continue;
            }
        };

        match calculate(a, op, b) {
            Ok(result) => {
                last = Option::Some(result);
                operations += 1;

                println!("Результат: {} {} {} = {}\n", a, op, b, result)
            }
            Err(e) => println!("Ошибка: {}\n", e),
        }
    }
}
