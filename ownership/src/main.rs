
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

fn calc_len(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope.
// But s does not have onwership, nothing happends(e.g. drop does not happen)

fn change(s: &mut String) -> () { // By default, references are immutable
    s.push_str(", which has been changed") // thus, `mut` keyword is needed
}

// Erroneous rust code
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
// s dropped here
// &s may reference to the dropped value: dangling!

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
} // the ownership of s is moved to other value
// no dangling occurs