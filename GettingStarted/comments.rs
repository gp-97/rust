/*
    Line Comments:
    // -> Single line comments
    /**/ -> Multi line comments
    
    Doc comments: 
    /// -> Outer doc comments (Supports markdown) (Single line)
    //! -> Inner doc comments (Single line)

*/

/// This is a Doc comment outside the function
/// Generate docs for the following item.
/// This shows my code outside a module or a function
fn main() {
    //! This a doc comment that is inside the function   
    //! This comment shows my code inside a module or a function  
    //! Generate docs for the enclosing item
    println!("{} can support {} notation","Doc comment","markdown");
}