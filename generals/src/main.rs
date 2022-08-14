static GLOBAL_INTEGER: i32 = 10;
fn main() {

    let s = String::from("howdy");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    tries_context();

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(raw_string: String) {
    return raw_string;
}

fn takes_ownership(string_name: String) {
    println!("{}", string_name  );
}

fn makes_copy(integer_name: i32) {
    println!("{}", integer_name);
}

fn tries_context() {
    println!("{}", GLOBAL_INTEGER);
}