fn main() {
    let hello = String::from("Hello, world!");
    let some = String::from("Some");

    println!("{}", first_word(&hello));
    println!("{}", first_word(&some));

    let long_string = String::from("Some long string");
    let word_count = count_words(&long_string);

    println!("{word_count}");

    let mut lowercase_string = String::from("привет");
    capitalize_first(&mut lowercase_string);

    println!("{lowercase_string}");

    let numbers = [-2, 3, 23, 6, 0, 10];
    let max = max_in_slice(&numbers);

    match max {
        Some(value) => println!("{value}"),
        None => println!("Список пуст"),
    }

    let mut mutable_string = String::from("Изменяемая строка");

    let non_mutable_link = &mutable_string;
    println!("{non_mutable_link}");

    let mutable_link = &mut mutable_string;
    mutable_link.push('!');
    println!("{mutable_link}");
}

fn first_word(s: &str) -> &str {
    if let Some(index) = s.find(' ') {
        &s[..index]
    } else {
        s
    }
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn capitalize_first(s: &mut String) {
    if s.is_empty() {
        return;
    }

    let first = s.remove(0);
    s.insert_str(0, &first.to_uppercase().collect::<String>());
}

fn max_in_slice(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        let mut max = numbers[0];

        for &num in numbers {
            if num > max {
                max = num;
            }
        }

        Some(max)
    }
}
