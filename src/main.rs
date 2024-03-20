use std::{println, result::Result::{self, Ok}};



mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime{
    system:system::Pallet,
    balances:balances::Pallet
}

impl Runtime{
    pub fn new()->Self{
        Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
    }
}



fn main()->Result<(),&'static str> {
 // Convert the string literal to a String
 /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    /* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */

    let mut runtime=Runtime::new();
    runtime.system.inc_block_number();

    let blocknumber=runtime.system.get_block_number();
    assert_eq!(blocknumber,1);

    let alice=String::from("Alice");
    let bob=String::from("Bob");
    let charlie=String::from("charlie");
    let amount=50;

    runtime.balances.set_balance(alice.clone(), amount);
    runtime.balances.set_balance(bob.clone(), amount);
    runtime.balances.set_balance(charlie.clone(), amount);
    
    runtime.system.inc_nonce(&alice);
    let mut _res=runtime.balances.transfer(&alice, &bob, 30);

    runtime.system.inc_nonce(&alice);
    let mut _res=runtime.balances.transfer(&alice, &charlie, 20);
    println!("{:#?}",runtime);

    


    Ok(())
    
    
}