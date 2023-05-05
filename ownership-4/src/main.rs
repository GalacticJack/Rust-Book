fn main() {
    let mut s = String::from("hello");
    let s2 = s.clone();
    s.push_str(", world!");

    let s3 = gives_ownership();
    let s4 = String::from("String 4");
    let s4 = takes_and_gives_back(s4);

    println!("s = {}, s2 = {}, s3 = {}, s4 = {}", s, s2, s3, s4);

    let calc_string = String::from("Calc this MF!!!!!");
    let (calc_string, calc_string_len) = calc_length(calc_string);

    println!("The length of '{}' is {}.", calc_string, calc_string_len);

    println!(
        "The length of '{}' by reference is {}.",
        calc_string,
        calc_length_ref(&calc_string)
    );

    let mut change_me = String::from("We is da");
    change_barrow(&mut change_me);
    println!("{change_me}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calc_length_ref(s: &String) -> usize {
    s.len()
}

fn change_barrow(s: &mut String) {
    s.push_str(", world!");
}

fn crappy_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
