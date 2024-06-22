fn main() {
    let mut a=15;
    let mut b=40;
    while b != 0 {
        let temp = b;
        b = a%b;
        println!("b = {} ",b);
        a=temp;
    }
    println!("Maior divisor comum : {} ",a);
}
