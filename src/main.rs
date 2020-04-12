use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(0, 101);

    let mut old_dist = 0;
    let mut first_guess = true;
    let mut moves_taken = 0;

    loop {
        print!("Your guess: ");
        io::stdout().flush()
            .expect("Could not flush stdout.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let current_dist = (secret_number - guess).abs();

        if first_guess {
            old_dist = current_dist;
        }

        moves_taken += 1;

        match current_dist.cmp(&old_dist) {
            Ordering::Less => match current_dist {
                0 => { 
                    println!("You win!");
                    break;
                },
                _ => println!("Hotter!")
            },
            Ordering::Greater => println!("Cooler!"),
            Ordering::Equal => 
                if first_guess {
                    if guess != secret_number {
                        println!("My number is different.");
                    } else {
                        println!("First try!?");
                        println!("You win!");
                        break;
                    }
                    first_guess = false;
                } else {
                    println!("Same!");
                }
        }
        old_dist = current_dist;
    }

    println!("{} moves taken.", moves_taken);
}
