use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1: &str = "Hello World!";
    let s2: String = String::from("Hello World!");
    let s3: String = "Hello World!".to_string();
    let s4: String = "Hello World!".to_owned();
    let s5: &str = &s4[..];

    let mut s: String = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.replace_range(.., "baz");
    println!("{}", s);

    let s1: String = String::from("Hello ");
    let s2: String = String::from("World!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, "toe");
    println!("{}", s4);

    let s1 = ["Hello ", "World!"].concat();
    let s2 = format!("{}{}", "Hello ", "World!");
    let s3 = concat!("Hello ", "World!");
    let s4 = String::from("Hello ");
    let s5 = s4 + "World!";

    let s1 = "🦀-🦀-🦀-🦀";
    let s2 = &s1[0..4];
    println!("{}", s2);

    for b in "hello".bytes() {
        println!("{}", b);
    }

    for c in "hello".chars() {
        println!("{}", c);
    }

    for g in "hello".graphemes(true) {
        println!("{}", g);
    }

    let s1: &str = "Hello World!";
    let s2: String = String::from("Hello World!");

    my_function(s1);
    my_function(&s2);
}

fn my_function(s: &str) -> String {
    return format!("{}", s);
}