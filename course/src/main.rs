mod Entities;

use Entities::Bank::*;
use Entities::Account::*;

fn main() 
{
    println!("Hello, world!");

    let bank = Bank::new(); // Call the associated function new()
    println!("{:?}", bank);
}
