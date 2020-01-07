fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    s
}
