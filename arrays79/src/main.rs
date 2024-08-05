fn rotacionar(k: usize, numeros: &mut [i32]) {
   
    let mut tam=numeros.len()-1;
    let mut i=0;
    let mut pega=0;
    let mut valork = k;
    let mut valortam=tam;
    numeros[tam]=pega;
    println!(" em rotacionar {}",numeros[0]);
    //for  (g,h) in t.chars().enumerate()
    while tam>=0 && valork>0{
        numeros[i]= numeros[tam]; //não esta
        
    
       tam=tam-1; 
       i=i+1;
    }
  

}
 



fn main() {
    let mut numeros: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    let k=4;
    numeros[0]=20;
   
    println!("vetor origianal{:?}",numeros); 
       
    rotacionar(k,&mut numeros);
    
    
    println!("vetor retornado{:?}",numeros); 
     

    
  
}

/*
fn rotacionar_array(nums: &mut [i32; 7], k: usize) {
    let n = nums.len();

    if n == 0 {
        return; // Array vazio, não há nada para rotacionar
    }

    let k = k % n; // Lidar com casos em que k é maior que n

    nums.reverse(); // Inverter todo o array
    nums[0..k].reverse(); // Inverter os primeiros k elementos
    nums[k..].reverse(); // Inverter os elementos restantes
}

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7];
    let k = 1;

    println!("Array original: {:?}", array);

    rotacionar_array(&mut array, k);

    println!("Array rotacionado: {:?}", array);
}
*/