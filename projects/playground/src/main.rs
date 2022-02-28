fn main() {
    let my_string = String::from("hello world");
    first_word(&my_string[0..6]);
    first_word(&my_string[..]);
    first_word(&my_string);

    let my_string_literal = "hello world";
    first_word(&my_string_literal[0..6]);
    first_word(&my_string_literal[..]);
    first_word(my_string_literal);
}

fn first_word(s: &str) -> String {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return s[..i].trim().to_string()
        }
    }

    s[..].trim().to_string()
}
