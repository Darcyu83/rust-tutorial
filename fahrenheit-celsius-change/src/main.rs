fn main() {
    let fahrenheit: f64 = 98.6;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F = {celsius:.1}°C");

    let celsius: f64 = 37.0;
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{celsius}°C = {fahrenheit:.1}°F");
}
