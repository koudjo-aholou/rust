// io (input/output) library into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Deviner le numero!");

    let secret_number = rand::thread_rng().gen_range(1.. 101);

    println!("Le numero secret est : {}", secret_number);
loop{
    println!("Svp saisissez un numéro.");
    // The :: syntax in the ::new line indicates that new is an associated function of the String type.
    // associated function is implemented on a type. Create new empty string  static method.

    let mut guess = String::new(); // mutable variable that is currently bound to a new, empty instance of a String.
    
    //call the stdin function from the io module
    io::stdin()
    .read_line(&mut guess) //  take whatever the user types into standard input and place that into a string
    // & indicates that this argument is a reference,  references are immutable by default
    .expect("Echec, impossible de lire la ligne");

    let guess: u32 =  match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    }; // Rust allows us to shadow the previous value of guess 

    println!("Votre supposition : {}", guess);
    // The cmp method compares two values and can be called on anything that can be compared
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Trop petit!"),
        Ordering::Greater => println!("Trop Grand!"),
        Ordering::Equal => {
            println!("Vous avez gagne!");
            break;
        } 
    }
}

}
