use crate::Entities::Account::BankAccount;

#[derive(Debug)]
pub struct Bank{
    accounts: Vec<BankAccount>,

}

/**
 * // Assiociated function called new() same as facoty in Java
 * like static method that we call the function, not  an instance
 * @return BankAccount
 */
 impl Bank{
    pub fn new() -> Self
    {
         Bank {accounts: vec![]}
    }
}

