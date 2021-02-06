fn main() {
    greeting("Koudjo");
}

fn greeting(prenom: &str){
    let mut greeting = String::from("Bonjour, ");
    greeting.push_str(prenom);
    println!("{}", greeting);
}