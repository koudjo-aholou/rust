fn main(){
  let mut salutation = "Bonjour, ".to_string();
  const PRENOM : &str = "Koudjo";
  salutation.push_str(PRENOM);
  println!(" {}", salutation);
}
