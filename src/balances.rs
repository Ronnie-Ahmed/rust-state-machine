use std::collections::BTreeMap;
pub struct Pallet{
    balances:BTreeMap<String,u128>,
}

impl Pallet{
    pub fn new()->Self{
        Self { balances: BTreeMap::new() }
    }

    pub fn set_balance(&mut self,account:String,amount:u128){
        self.balances.insert(account, amount);
    }

    pub fn see_balance(&self,account:&String)->u128{
        *self.balances.get(account).unwrap_or(&0)        
    }

    pub fn transfer(&mut self,from:&String,to:&String,amount:u128)->Result<(),&'static str>{
        self.set_balance(from.to_string(), 100);
        self.set_balance(to.to_string(), 100);
        assert!( amount>0,"Amount should be more than 0");
        assert!(self.see_balance(&from)>amount,"From account should have enough Money");
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