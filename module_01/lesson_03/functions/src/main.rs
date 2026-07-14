fn main() {
    // Проверка на четность.

    let even = 10;
    println!("Число {even} четное: {}", is_even(even));

    let odd = 7;
    println!("Число {odd} четное: {}", is_even(odd));

    // FizzBuzz.

    for i in 1..=20 {
        println!("{}", fizzbuzz(i));
    }

    // Проверка на кратность 3 и 7 одновременно.

    let mut counter = 1;

    loop {
        if counter % 3 == 0 && counter % 7 == 0 {
            println!("Число кратное 3 и 7 это {counter}");
            break;
        }

        counter += 1;
    }
}

/// Функция проверки целого числа на четность.
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Функция возвращает строку "Fizz", "Buzz" или "FizzBuzz" в зависимости от
/// переданного целого числа.
fn fizzbuzz(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}
