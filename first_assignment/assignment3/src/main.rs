// returns:
// 0 = correct
// 1 = too high
// -1 = too low
fn check_guess(guess: i32, hidden: i32) -> i32 {
    if guess == hidden {
        0
    } else if guess > hidden {
        1
    } else {
        -1
    }
}

fn main() {
    let hidden = 7;

    // simulated guesses
    let guesses = [5, 9, 6, 7];

    let mut index = 0;
    let mut attempts = 0;

    loop {
        let guess = guesses[index];
        attempts += 1;

        let result = check_guess(guess, hidden);

        if result == 0 {
            println!("{} is correct!", guess);
            break;
        } else if result == 1 {
            println!("{} is too high", guess);
        } else {
            println!("{} is too low", guess);
        }

        index += 1;
    }

    println!("Attempts: {}", attempts);
}
