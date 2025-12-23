#[derive(Debug)]
pub struct BankAccount{
    id: u32,
    balance: i32,
    holder: String,
}


/// #Bank Account
/// 
impl BankAccount{
    pub fn new(id: u32, holder: String) -> Self
    {
        BankAccount 
        {
            id,
            holder,
            balance: 0
        }
    }
}