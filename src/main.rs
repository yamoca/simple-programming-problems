use std::io;

fn main() {
    println!("{}", one_to_n(4));
}

fn one_to_n(number: i32) -> i32 {
    let mut answer: i32 = 0;

    for n in 1..=number {
        answer += n;
    }
    return answer;
}

fn greet_alice_bob() {
    println!("Enter your name: ");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read name");
    name = name.to_string();

    match name.trim().to_lowercase().as_str() {
        "alice" => println!("Hello alice!"),
        "bob" => println!("Hello bob!"),
        _ => return,
    }
}

fn greet() {
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read name");

    name = name.to_string();
    println!("Hello {name}!");
}
