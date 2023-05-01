// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     // println!("Secret number is: {secret_number}");
    
//     loop {
        
//         println!("Please input your guess.");
//         let mut guess = String::new();
        
//         io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
        
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

        
//         println!("You guessed: {guess}");
        
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
    
// }
//CHAPTER 3===========================================================================
use std::io;
fn main() {
    convertion();
    christmass_carrol();
}
fn christmass_carrol() {
    let mut counter = 0;
    while counter < 12 {
        println!("On the {counter} day of christmas");
        counter += 1;
    }
}


fn convertion() {
    println!("Give me a celsius");
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("NaN, failed to read");

    let celsius: f32 = celsius
        .trim()
        .parse()
        .expect("NaN, failed to convert");
    
    let converted = convert_temperature(celsius);
    println!("The value of celsius: {celsius} is in fahrenheit {converted}");
}
fn convert_temperature(x: f32) -> f32 {
    (x*1.8) + 32.0
} 





