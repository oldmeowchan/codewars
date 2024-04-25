
mod cw_5264d2b162488dc400000001;
mod cw_5502c9e7b3216ec63c0001aa;

fn main() {
    let words = "Hey fellow warriors" ;
 
 let s: String = words.split(" ")
    .map(|i| {
        if i.len() > 4 {
            i.chars().rev().collect::<String>()
        } else {
            i.to_owned()
        }
    })
    .collect::<Vec<String>>()
    .join(" ");


    println!("{:?}", s);
}
