#[derive(Debug)]
struct Power<'a>(bool, &'a str, i32, f64, u8, [u8;4], Vec<SpreadsheetCell>);

#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

#[derive(Debug)]
struct TabTab<'a> {
  cas1 : Vec<SpreadsheetCell>,
  cas2: Vec<Power<'a>>,
  cas3: [&'a i8; 3]
}

fn main(){
  let data : Power = Power(true,"gaara", 80, 0.99, 1, [1,2,3,4], vec![SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),]);
  shikaku(data);
}

fn shikaku(power: Power) -> TabTab{
  
  let mut xs = vec![SpreadsheetCell::Text(String::from("blue"))];
  let mut xy = vec![Power(true,"gaara", 80, 0.99, 1, [1,2,3,4], vec![SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),])];
  xs.push(SpreadsheetCell::Text(String::from("red")));
  xy.push(power);

  let mut newdata : TabTab = TabTab{cas1:xs, cas2:xy, cas3: [&42,&5,&9]};
  newdata.cas1.push(SpreadsheetCell::Text(String::from("Purple Haze")));
  newdata.cas3[1] = &55;
  newdata.cas1[0] = SpreadsheetCell::Text(String::from("Foxy Lady"));
  let length_beach1 = newdata.cas1.len();
  let length_beach2 = newdata.cas2.len();
  let length_beach3 = newdata.cas3.len();
  println!("{:?} {:?} {:?} {:?}", newdata, length_beach1, length_beach2, length_beach3);
  return newdata;
}