

// Randomg number generator 
// Using the rand crate to generate numbers
use rand::Rng;
// user input
use std::io;






fn main() {
    println!("\n\nWELCOME TO YODA ADVICE GENERATOR! ");
    println!("Yoda was a legendary member of the Jedi Council.");
    println!("And a member of a mysterious species capable of living for centuries.");
    println!("He had a strong connection to the mystical field of energy the Force!");
    println!("Even after he died, he found a way to attain immortality through the Force.");
    println!("So call on him to seek his celestial guidance!");

    loop {
        println!("\n\nChoose an emotion!");
        println!("1 > Sad");
        println!("2 > Happy");
        println!("3 > Excited");
        println!("4 > Exit");

        let choice = get_user_input();
        match choice.trim(){
            "1" => generate_advice("sad"), 
            "2" => generate_advice("happy"),
            "3" => generate_advice("excited"),
            "4" => { 
                println!("YOU ARE ONE WITH THE FORCE!");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option. "),
        }
 
    }
   

}


// A function to get user input
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input
}


fn generate_advice(emotion:&str){
    let advice = match emotion{
        "sad" => {
            let sad_advice = [
                "Patience you must have, my young Padawan.",
                "You must unlearn what you have learned. Yeessssss.",
                "A Jedis strength flows from the Force.",
                "Do or Do not, there is no try. Yeesssss"
            ];
            sad_advice
        }

        "happy" => {
            let happy_advice = [
                "Impossible, nothing is. Yeesssss.", 
                "You changed your thoughts, changed your world it has.",
                "Feel the Force, you must. Yeessss.",
                "Pass on what you have learned. Hmmmm.",

            ];
            happy_advice
        }

        "excited" => {
            let excited_advice = [
            "May the Force be with you!!", 
            "For your ally is the force, and a powerful ally it is. Hmmmm",
            "Difficult to see. Always in motion is the future.", 
            "Secret, shall I tell you? Grand Master of Jedi Order am I.",
            ];
            excited_advice
        }
        _ => {
            println!("Invalid emotion.");
            return;
        }
    };

let random_index = rand::random::<usize>() % advice.len();
println!("Yoda's Advice for being {}: {}", emotion,advice[random_index]);

}
