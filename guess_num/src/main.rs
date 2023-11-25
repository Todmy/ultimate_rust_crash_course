
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let rand_num = rng.gen_range(1..=10);

    println!("Guess a number between 1 and 10");

    let mut guess = std::string::String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");

    match guess.trim().parse::<u32>() {
        Ok(num) => {
            println!("You guessed: {}, but the number was: {}", num, rand_num);
            if num == rand_num {
                println!("You guessed correctly!");
            } else {
                println!("You guessed incorrectly!");
                main();
            }
        },
        Err(_) => {
            println!("Please type a number!");
            main();
        }
    }
}
