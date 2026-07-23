fn main() {
    let arr: [&str; 5] = ["There", "is", "a", "some", "vector"];
    let five_strings: Vec<String> = arr.map(|e| e.to_string()).to_vec();

    print_len(five_strings.clone());

    for s in five_strings {
        println!("{s}");
    }

    // println!("{}", vec.len()); // Ошибка владения!

    let a = String::from("Some");
    let b = String::from("thing");

    let concat = concatenate(a, b);

    // println!("{a} + {b} = {concat}"); // a и b не доступны!
    println!("{concat}");

    let some_strings = vec!["Some".to_string(), "string".to_string()];
    let some_strings = add_item(some_strings, String::from("yay"));
    let some_strings = add_item(some_strings, String::from("yay"));

    for s in some_strings {
        println!("{s}");
    }

    let num = 3;
    let boolean = false;

    let tup = (num, boolean);

    let _tup2 = tup;

    println!("{}", tup.0);
    println!("{}", tup.1);

    let nums = vec![num, num * 2, num * 3];

    let _another_nums = nums;

    // for num in v { // Ошибка!
    //     println!("v: {num}");
    // }
}

fn print_len(v: Vec<String>) {
    println!("{}", v.len());
}

fn concatenate(a: String, b: String) -> String {
    format!("{a}{b}")
}

fn add_item(mut v: Vec<String>, item: String) -> Vec<String> {
    v.push(item);
    v
}
