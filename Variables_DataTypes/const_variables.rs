/*
    ->  Const variables act as 'final' keyword in java.
        These cannot be changed after being declared and initialized once

    ->  Const varaibles can never be made mutable.

    ->  Const variables when defined, need to be provided types 
        exlicitly.
*/

#[allow(unused_variables, unused_mut)]

fn main() {
    const PI: f64 = 355.0 / 113.0;
    println!("PI = {}", PI);
}