/*enum Direction{
    Up,
    Down,
    Left,
    Right
}
 
 
fn main() {
 
    let player: Direction = Direction::Right;
    match player{
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Right => println!("O jogador foi para direita"),
        Direction::Left => println!("O jogador foi para esquerda"),
    }
    
}*/
//#[derive(Debug)]
/*enum Gender{
    Male, Female
}
 
 
fn main() {
 
    let player_male: Gender = Gender::Male;
    let player_female: Gender = Gender::Female;
    
    println!("{:?}", player_male);
    println!("{:?}", player_female);
    
}
*/
#[derive(Debug)]
enum CarType{
    Fiat,
    Ford,
    Renault,
}
 
fn nacionalidade_carro(car: CarType){
 
    match car{
        CarType::Fiat => println!("O carro eh italiano"),
        CarType::Ford => println!("O carro eh americano"),
        CarType::Renault => println!("O carro eh frances"),
    }
 
}
 
 
fn main() {
 
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renault);
    
}