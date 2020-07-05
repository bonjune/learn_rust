fn main() {
    let mut s = String::from("Hello World");
    let word = fst_wrd(&s); // borrowed as immutable
    s.clear(); // borrowed as mutable -> error!
    println!("{}", word);
}

// we don't want a owenership since we are going to manipulate the original string
fn fst_wrd(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// but what if s is dropped? the value of fst_wrd will be remained undropped
// therefore, the program will potentially fail