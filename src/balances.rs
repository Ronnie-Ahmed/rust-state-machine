use std::collections::BTreeMap;

type AccountId=String;
type Balance=u128;



#[derive(Debug)]
pub struct Pallet{
    balances:BTreeMap<AccountId,Balance>,
}

impl Pallet{
    pub fn new()->Self{
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self,account:AccountId,amount:Balance){
        self.balances.insert(account, amount);
    }

    pub fn see_balance(&self,account:&AccountId)->u128{
        *self.balances.get(account).unwrap_or(&0)        
    }

    pub fn transfer(&mut self,from:&AccountId,to:&AccountId,amount:Balance)->Result<(),&'static str>{
        // self.set_balance(from.to_string(), 100);
        // self.set_balance(to.to_string(), 100);
        assert!( amount>0,"Amount should be more than 0");
        assert!(self.see_balance(&from)>=amount,"From account should have enough Money");
        self.set_balance(from.to_string(), self.see_balance(&from)-amount);
        self.set_balance(to.to_string(), self.see_balance(&to)+amount);
        Ok(())
    }
}

#[cfg(test)]
mod test{
    use super::Pallet;

    #[test]
    fn init_balances(){
        let mut bal=Pallet::new();
        assert_eq!(bal.see_balance(&"Ronnie".to_string()),0);
        bal.set_balance("Ronnie".to_string(), 10);
        assert_eq!(bal.see_balance(&"Ronnie".to_string()),10);
    }

    #[test]
    fn transfer_balance(){
        let mut bal=Pallet::new();
        bal.transfer(&"Ronnie".to_string(), &"Rony".to_string(), 10);
    }
}
