use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("guess the number..");
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        println!("please input the number");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) =>{
                println!("enter a valid number");
                continue;
            },
        };
        
        println!("you guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too less"),
            Ordering::Equal => {
                println!("you won");
            break;           },
            Ordering::Greater => println!("too high"),
        }
    
    }
}
