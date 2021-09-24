const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
    
    // READY_AMOUNT = 1;
    // ------------ ^
    //  cannot assign to this expression (declared as constant)

    // "mut" keyword will allow the variable to be changed after being initialized
    // let mut missiles:i32 = STARTING_MISSILES;
    // let ready:i32 = READY_AMOUNT;
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT); 

    println!("Firing {} of my {} missiles...", ready, missiles);

    //  E0384: Immutability
    //================================================================================ 
    //
    //    An immutable variable was reassigned.
    //
    //    Erroneous code example:
    //
    //    ```
    //    fn main() {
    //        let x = 3;
    //        x = 5; // error, reassignment of immutable variable
    //    }
    //    ```
    //
    //    By default, variables in Rust are immutable. To fix this error, add the keyword
    //    `mut` after the keyword `let` when declaring the variable. For example:
    //
    //    ```
    //    fn main() {
    //        let mut x = 3;
    //        x = 5;
    //    }

    // missiles =  missiles - ready;

    println!("{} missiles left", missiles - ready);
}
