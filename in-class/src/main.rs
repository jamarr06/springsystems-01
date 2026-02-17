

   //let mut result : f32 = 0.0; //int
   //let x:i32 = 5; //float
   //result = result + x as f32; //no implicit conversion

   //println!("{}", result); 


   ///fn main() {
   /// Shadowing
   /// let x = 5;
    ///let x = x + 1;  // Creates a new variable
    
    // Mutation
    ///let mut y = 5;
    ///y = y + 1;  // Modifies the existing variable
    
    ///println!("x: {}, y: {}", x, y);
///}
///


////fn main() {
    // Shadowing
  ////  let x = 5;
 ////{   
    ////let x = x + 10;  // Creates a new variable
    ////println!("x: {}", x);

////}

    ////println!("x: {}", x);
////}

/////fn say_hi(x:i32){
    /////println!("Hi John! My fav nume {}", x);
/////}
/////fn main(){
    /////say_hi(5);
/////}



fn double (x:i32) -> i32 {
    //return x*2
    {
        let y=10;
        x*2*y
    }
}
fn main(){
    println!("Double {} equeals to {}", 5, double(5));
}