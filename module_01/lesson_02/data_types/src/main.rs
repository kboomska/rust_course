fn main() {
    let temperature: f64 = 36.6;
    println!("Температура: {temperature}");

    let mut counter: i32 = 0;
    for _ in 0..3 {
        counter += 1;
    }
    println!("Счетчик: {counter}");

    let input = "42";
    let input = input.parse::<i32>().unwrap();
    println!("Число из строки: {input}");

    let tup = ("Алексей", 33, 1.76);
    println!("Имя: {}, возраст: {}, рост: {}", tup.0, tup.1, tup.2);

    let list = [2; 5];
    println!("Первая оценка: {}, последняя: {}", list[0], list[4]);
}
