# Hotter Cooler

A number guessing game. Given your previous guess, you are told how closer to the secret number your current guess is compared to the previous one. This variant is somewhat harder that the classic less/bigger game.

### Gameplay example
```
Guess the number.
Your guess: 50
My number is different.
Your guess: 25
Cooler!
Your guess: 75
Hotter!
Your guess: 66
Cooler!
Your guess: 80
Hotter!
Your guess: 76
Cooler!
Your guess: 85
Hotter!
Your guess: 83
You win!
8 moves taken.
```

### How to build

Make sure you have rust compiler and cargo installed on your machine. Then in the project root run
```
$ cargo run --release
```

### Why

It is written as a rust exercise.