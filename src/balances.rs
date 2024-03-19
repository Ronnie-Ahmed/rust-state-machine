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
}