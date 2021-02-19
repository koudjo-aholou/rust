// io (input/output) library into scope
use std::io;
use rand::Rng;

fn main() {
    println!("Deviner le numero!");

    let secret_number = rand::thread_rng().gen_range(1.. 101);

    println!("The secret number is: {}", secret_number);

    println!("Svp saisissez un numero.");
    // The :: syntax in the ::new line indicates that new is an associated function of the String type.
    // associated function is implemented on a type. Create new empty string  static method.

    let mut guess = String::new(); // mutable variable that is currently bound to a new, empty instance of a String.
    
    //call the stdin function from the io module
    io::stdin()
    .read_line(&mut guess) //  take whatever the user types into standard input and place that into a string
    // & indicates that this argument is a reference,  references are immutable by default
    .expect("Echec, impossible de lire la ligne");

    println!("Votre supposition : {}", guess);
}
