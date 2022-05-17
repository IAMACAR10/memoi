use rand::Rng;
use text_io::read;

fn main() {
    println!("Welcome! Please select an excercise: \n1. Multiplication");
    let test:u8 = read!();
    match test{
        1 => multiply(),
        _ => println!("Please select an excercise :)"),
    }
}

fn multiply() {
    let mut alive = true;
    let mut combo = 0;
    while alive{
        let mut rng = rand::thread_rng();
        let a:u32 = rng.gen_range(1..=12);
        let b:u32 = rng.gen_range(1..=12);
        let num:u32 = a * b;
        println!("Multiply {} with {}", a, b);
        let answer:u32 = read!();
        if answer == num{
            combo += 1;
        } else{
            println!("You got a score of {}", combo);
            alive = false;
        }
    }
}
