// https://blog.thoughtram.io/string-vs-str-in-rust/
#[derive(Debug)] // debug structure
struct SelectBox<'a> {
  case1: i8,
  case2: &'a str,
  case3: f32,
  case4: bool,
  case5: [NestedArray<'a>;2]
}

#[derive(Debug)]
pub struct NestedArray<'a> {
  cas1: &'a str,
  cas2: &'a str,
  cas3: bool,
  cas4: &'a str,
}

#[derive(Debug)]
pub struct UnsizedArr<'a> {
  pub components: Vec<NestedArray<'a>>,
}

fn main(){
  print_beach("toto","toto2");
  format_beach("tata");
  epr_beach("Saucisse");

  // Debug
  const ARRAY_PASS : [i8;4] = [1,2,3,4];
  debug_moi(ARRAY_PASS);

  const ARRAY_FLOAT : [f32;4] = [1.0,2.0,3.0,4.0];
  debug_moi_virgule(ARRAY_FLOAT);

  const TRAIT_NESTED:NestedArray = NestedArray{
    cas1: "ouir",
    cas2: "&'a str",
    cas3: true,
    cas4: "&'a str"
  };

  const TRAIT_DIFF:SelectBox  = SelectBox{
    case1: 1,
    case2: "2.0",
    case3: 3.0,
    case4: true,
    case5: [TRAIT_NESTED, TRAIT_NESTED]
  };

  debug_moi_trait(TRAIT_DIFF);

  debug_moi_imbriqtrait(TRAIT_NESTED);

  let mut xs = vec![1, 2, 3];
  xs.push(4);
  xs.pop();
  debug_moi_vec(xs);

  let mut xsy : Vec<SelectBox> = vec![TRAIT_DIFF, TRAIT_DIFF];
  xsy.push(TRAIT_DIFF);
  xsy.pop();
  debug_moi_vec_trait(xsy);
}
///  text is printed to the console (io::stdout).
fn print_beach(text: &str, text2: &str){
  print!("hello {0} & {1} sans saut de ligne",text, text2);
  println!("hello {} avec saut de ligne",text);
  println!("On est pas colle {prenom}",prenom="text remp");
}

/// write formatted text to String
fn format_beach(text: &str) -> String{
  format!("hella {}", text)
}

/// same as format! but the text is printed to the standard error (io::stderr).
fn epr_beach(text: &str){
  eprint!("Bonjour : {} & ", text);
  eprint!("encore {} colle", text);
  eprintln!("Saut de ligne : {}", text);
}

fn debug_moi(tab: [i8;4]){
  println!("{:?}", tab)
}

fn debug_moi_virgule(tab: [f32;4]){
  println!("{:?}", tab)
}

fn debug_moi_trait(tab: SelectBox ){
  println!("{:#?}", tab)
}

fn debug_moi_imbriqtrait(tab: NestedArray ){
  println!("{:?}", tab)
}

fn debug_moi_vec(resizable: Vec<i8> ){
  println!("{:?}", resizable)
}

fn debug_moi_vec_trait(resizable: Vec<SelectBox> ){
  println!("{:?}", resizable)
}