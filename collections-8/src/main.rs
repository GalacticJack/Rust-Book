#![allow(unused_variables)]
fn main() {
    vectors();
}

fn vectors() {
    // Explicit type decleration
    let v: Vec<i32> = Vec::new();
    // Implicit type decleration
    let v_macro = vec![1, 2, 3];
    // Implicit type decleration from insert
    let mut v_mut = Vec::new();
    v_mut.push(5);

    let v_access = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v_access[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v_foreach = vec![100, 32, 57];
    for x in &mut v_foreach {
        *x += 50;
    }
    for x in &v_foreach {
        println!("{x}")
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
