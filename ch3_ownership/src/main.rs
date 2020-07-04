
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // s should be declared as mutable
    println!("{}", s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();
}

fn takes_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn makes_copy(c: i32) {
    println!("{}", c);
}