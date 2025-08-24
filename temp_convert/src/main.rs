fn main() {
    let temperature_celsius = 45.0;

    let temperature_fahrenheit = temperature_celsius * (9.0 / 5.0) + 32.0;

    println!("{temperature_celsius}C is {temperature_fahrenheit}F");

    let temperature_celsius_again = (temperature_fahrenheit - 32.0) / (9.0 / 5.0);

    println!("{temperature_fahrenheit}F is {temperature_celsius_again}C");
}
