// function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // array of 10 numbers
    let numbers = [4, 9, 10, 15, 7, 3, 20, 8, 6, 11];

    // -------- FOR LOOP (Fizz / Buzz / Even / Odd) --------
    for num in numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} -> FizzBuzz", num);
        } 
        else if num % 3 == 0 {
            println!("{} -> Fizz", num);
        } 
        else if num % 5 == 0 {
            println!("{} -> Buzz", num);
        } 
        else {
            if is_even(num) {
                println!("{} -> Even", num);
            } else {
                println!("{} -> Odd", num);
            }
        }
    }

    // -------- WHILE LOOP (sum of array) --------
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum = sum + numbers[index];
        index = index + 1;
    }

    println!("Sum = {}", sum);

    // -------- LOOP (find largest number) --------
    let mut i = 0;
    let mut largest = numbers[0];

    loop {
        if numbers[i] > largest {
            largest = numbers[i];
        }

        i = i + 1;

        if i == numbers.len() {
            break;
        }
    }

    println!("Largest = {}", largest);
}
