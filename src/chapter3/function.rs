fn function() {
    println!("Hello, world!");

    another_function();
}

fn another_function(x: i32) {
    println!("Hello, another function! {} ", x);
}