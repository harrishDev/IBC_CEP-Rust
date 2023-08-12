fn main() {
    println!("The Table of 9");
    table9();
}

fn table9(){
    for i in 1..=12 {
        println!("9*{}={}",i,9*i);
    }
}