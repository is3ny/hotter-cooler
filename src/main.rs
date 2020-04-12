use std::{env, io::{self, Write}, cmp::Ordering};
use rand::Rng;

const DEFAULT_MAX_SECRET: i32 = 100; 
const DEFAULT_MIN_SECRET: i32 = 0;

fn get_secret_bounds(args: &Vec<String>) -> (i32, i32) {
    let mut secret_min = DEFAULT_MIN_SECRET;
    let mut secret_max = DEFAULT_MAX_SECRET;

    if args.len() == 3 {
        secret_min = match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse <min>. Defaulting to {}.", secret_min);
                secret_min
            },
        };

        secret_max = match args[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse <max>. Defaulting to {}.", secret_max);
                secret_max
            },
        };
    }
    
    (secret_min, secret_max)
}

fn main() {
    println!("Guess the number.");

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 || args.len() > 3 {
        println!("Usage:\n\thc\t-- run with default bounds [{}..{}]\n\thc <min> <max>\n",
                 DEFAULT_MIN_SECRET, DEFAULT_MAX_SECRET);
        return;
    }

    let (secret_min, secret_max) = get_secret_bounds(&args);
    let secret_number = rand::thread_rng().gen_range(secret_min, secret_max);
    
    println!("The number is in range [{}..{}].", secret_min, secret_max);

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
