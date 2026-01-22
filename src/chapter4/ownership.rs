fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1; // s1이 s2로 이동하여 기존 s1 변수는 무효화
    //
    // println!("{}, world!", s1); // 무효화 된 s1을 출력시도하니 오류 발생

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("check {}", s); // takes_ownership 함수를 마치는 과정에서 메모리가 해제되었기 때문에 오류 발생

    let s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로 이동시킵니다

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데, 이 함수 또한 자신의 반환 값을 s3로 이동시킵니다
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 여기서 some_string이 drop 메서드에 의해 메모리 해제

fn gives_ownership() -> String { // gives_ownership은 자신의 반환 값을 자신의 호출자 함수로 이동시킬 것입니다

    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다

    some_string // some_string이 반환되고 호출자 함수 쪽으로 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어옵니다
    a_string // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}
