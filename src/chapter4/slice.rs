fn slice() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 값 5를 받습니다

    s.clear();

    println!("{}", word);

    // 문자열 슬라이스
    let s = String::from("hello world");
    let hello = &s[0..5]; // string slice
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello world");
    let word = first_word_v2(&s);
    println!("word={}", word);
    println!("s={}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}