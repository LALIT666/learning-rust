// Borrowing x mutably and WARNING: THERE IS NO TRANSFER OF OWNERSHIP
fn main(){

    let mut x: String = "Hello".into();
    println!("{}",x);
    {
        let  z = &mut x;

      

        
        // Overall x should not be accessed while z has mutable reference and we must wait until z is out of scope.
        // Comments are ERRORS 
        // println!("{}",x);
        // x.push_str("_Hi");
        // ERROR: x is already mutably borrowed by z
        // println!("Access by x: {}",x);
        
        z.push_str("_World");
    
        // Same as it is still under mutable borrow
        // println!("Access By x: {}",x);
        println!("Access By z: {}",z);
        /* The latter part of the code should also be invalid but 
        Modern Rust uses Non-Lexical Lifetimes (NLL).
        After the last use of z, Rust notices that nobody uses z anymore.Therefore Rust ends the borrow early.*/
        println!("Access By x: {}",x);
        x.push_str("_My");
    
        println!("Access By x:{}",x);
        // Now the thing below will make Rust realize that z is still used as I said wait until z is out of scope
        // println!("{}",z);
}



    println!("{}",x);
// This line below showed error as it is out of scope
    // println!("{}",z);
}


/*
Reader(Immutable Reference) and writer (Mutable Reference) at the same time is not allowed
let z = &x; // Immutable Reference
let y = &mut x; //Mutable reference // ERROR 
*/ 
/*
Immutable references can coexist with any number of other immutable references.
Mutable references cannot coexist with other mutable as well as immutable references in the same scope.
*/