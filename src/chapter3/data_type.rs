fn data_type() {
    /*
    0. Scalar Type : 정수, 부동 소수점 숫자, 불린, 문자 4가지 타입을 가지고 있다.
    1. 정수형 (Integer)
    부호 있음 : i, 부호 없음 : u
    */
    let signed8 :i8 = 1; let signed16:i16 = 2; let signed32:i32 = 4; let signed64:i64 = 8; let signed128:i128 = 16; let signed_arch:isize = 1;
    let unsigned8:u8 = 1; let unsigned16:u16 = 2; let unsigned32:u32 = 4; let unsigned64:u64 = 8; let unsigned128:u128 = 16; let unsigned_arch:usize = 1;

    /*
    Decimal / Hex / Octal / Binary / Byte(u8 Only)
    https://doc.rust-kr.org/ch03-02-data-types.html
    */
    let decimal :u32 = 10_000;
    let hex :u32 = 0xff;
    let octal :u32 = 0o77;
    let binary:u32 = 0b10_0000;
    let byte:u8 = b'A';

    /*
    3. 부동 소수점 타임 (float) f32 / f64
     */
    let x = 2.0; let y: f32 = 3.0;

    // 4. 연산
    // 덧셈
    let sum = 5 + 10;

    // 뺄셈
    let difference = 95.5 - 4.3;

    // 곱셈
    let product = 4 * 30;

    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결괏값은 -1입니다

    // 나머지 연산
    let remainder = 43 % 5;

    // 5. 불린
    let t = true;

    let f: bool = false;

    // 6. 문자 타입
    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("index 0 : {}, index 1: {}, index 2: {}", tup.0, tup.1, tup.2);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

}