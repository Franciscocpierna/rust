fn compress_string(original_str: &str)->String{
   
   let mut original_modificada = String::new();
   let mut soma = 0;
   let mut guarda='0';
   let mut guardaanterior='0'; 
   let total= original_str.len();
   match original_str.chars().nth(0) {
    Some(c) => guarda=c,
    None=> {
     println!("não tem index ")
 
    }
   }
   guardaanterior=guarda;
   for n in 0..total{ 
     
       if original_str.chars().nth(n)==Some(guarda){
            soma = soma +1;
       }
        
      
       if original_str.chars().nth(n)!=Some(guarda){
         if soma > 1{
            
            original_modificada.push_str(&soma.to_string());
            original_modificada.push(guardaanterior);
            
            match original_str.chars().nth(n) {
                Some(c) => guarda=c,
                None=> {
                 println!("não tem index ")
             
                }      
              }
              guardaanterior=guarda;  
         }else{
            original_modificada.push(guardaanterior);
            match original_str.chars().nth(n) {
                Some(c) => guarda=c,
                None=> {
                 println!("não tem index ")
             
                }   
              }  
            }      
         guardaanterior=guarda;
         soma=1;
         }
         
         
       }
     
       if soma > 1{
        original_modificada.push_str(&soma.to_string());
        original_modificada.push(guardaanterior);

       }else {
        original_modificada.push(guardaanterior);
       }     
     
    
     return original_modificada;
        }
    





fn main() {
    let original_str = "aabccccccaaa";
    let compressed_str = compress_string(original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);

    let other_str = "abcdefgh";
    let compressed_other = compress_string(other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}


