use std::io;
mod games;

fn main() {
    println!("Welcome to Russian Roulette!");
    let mut choice = 0;
    while choice == 0 {
        let mut choice_str: String = String::new();
        let wincon = games::russianroulette();
        match wincon {
            0 => println!("you win!"),
            1 => println!("you lose!"),
            _ => todo!(),
        }
        loop {
            println!("\nWould you like to play again? [y/n]");
            io::stdin()
                .read_line(&mut choice_str)
                .expect("that isn't right...");

            match choice_str.as_str().to_lowercase().trim() {
                "y" | "yes" => {
                    choice = 0;
                    break;
                }
                "n" | "no" => {
                    choice = 1;
                    break;
                }
                _ => {
                    println!("that isn't right...");
                    choice_str.clear();
                }
            }
        }
    }
}
