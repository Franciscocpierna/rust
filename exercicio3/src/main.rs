fn dobro(num: i32) -> i32{
    return num*2;
}

fn maior(a: i32, b: i32) -> i32{
    if a >= b {
        return a;
    }else{
        return  b;
    }
}
fn main() {
   /* let mut a=15;
    let mut b=40;
    while b != 0 {
        let temp = b;
        b = a%b;
        println!("b = {} ",b);
        a=temp;
    }
    println!("Maior divisor comum : {} ",a); */
    println!("o dobro de 5 é {}", dobro(5)); 
    println!("o dobro de 4 e 5 é {}", maior(4,5)); 
}
