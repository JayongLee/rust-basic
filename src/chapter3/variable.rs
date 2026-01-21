fn variable() {
    // 1. 변수와 가변성
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // 2. 상수
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 상수는 항상 불변 & 데이터 타입을 반드시 명시해줘야 함

    // 3. 섀도잉 (새 변수를 이전 변수명으로 선언, 덮어씌움) shadowed
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {}", y);
    }
    println!("The value of y is {}", y);
}