fn main() {
    let s = String::from("some value");
    println!("{s}");

    let slice = first_word(s.clone());
    println!("{s}");
    println!("{slice}");

    let (slice, s) = first_word_2(s);
    println!("{s}");
    println!("{slice}");

    let a = String::from("first string");
    let b = String::from("second string");
    let c = String::from("третья строка");

    let longest = longest_of_three(a.clone(), b.clone(), c.clone());

    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("Longest string (bytes) is: {longest}");

    let s = String::from("world");
    let s = greet(uppercase(exclamation(s)));
    println!("{s}");
}

fn first_word(s: String) -> String {
    let index = s.find(' ').unwrap_or(s.len());
    s[..index].to_string()
}

fn first_word_2(s: String) -> (String, String) {
    let index = s.find(' ').unwrap_or(s.len());
    let slice = s[..index].to_string();
    (slice, s)
}

fn longest_of_three(a: String, b: String, c: String) -> String {
    if a.len() >= b.len() && a.len() >= c.len() {
        a
    } else if b.len() >= c.len() {
        b
    } else {
        c
    }
}

fn exclamation(mut s: String) -> String {
    s.push('!');
    s
}

fn uppercase(s: String) -> String {
    s.to_uppercase()
}

fn greet(s: String) -> String {
    format!("Привет, {s}")
}
