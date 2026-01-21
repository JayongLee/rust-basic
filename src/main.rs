use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");



    // let apple = 5; // immutable
    // let mut banana = 3; // mutable

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // let : 변수 설정 (불변), mut : 변수의 값을 가변으로 설정 (mutable)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Result Type (잠재적 실패) : 에러 처리용 정보 담기

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
