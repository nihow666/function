fn main() {
    println!("Hello, world!");
    another_function(444, 666);
    statment_function();
}

fn another_function(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}

fn statment_function() {
    // unused variable
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("y 的數值為：{}", y);
}
