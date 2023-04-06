fn main() {
    get_string();
}

fn get_string<'a>() -> &'a str {
    "aa"
}