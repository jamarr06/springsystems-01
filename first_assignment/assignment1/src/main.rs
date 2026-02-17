// freezing point of water
const FREEZING_F: f64 = 32.0;

// function to change Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// function to change Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    // starting temperature (can change)
    let mut temp = FREEZING_F;

    // convert starting value
    let c = fahrenheit_to_celsius(temp);
    println!("{} F = {:.2} C", temp, c);

    // print next 5 temperatures
    for _ in 0..5 {
        temp = temp + 1.0;
        let c = fahrenheit_to_celsius(temp);
        println!("{} F = {:.2} C", temp, c);
    }
}
