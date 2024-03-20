use std::collections::BTreeMap;

type AccountId=String;
type Nonce=u32;
type BlockNumber=u32;
type Balance=u128;


#[derive(Debug)]
pub struct Pallet{
    block_number:BlockNumber,
    nonce:BTreeMap<AccountId,Nonce>,
}

impl Pallet{
    pub fn new()->Self{
        Self {
            block_number:0,
            nonce:BTreeMap::new()
        }
    }
    pub fn get_block_number(&self)->BlockNumber{
        self.block_number
    }
    pub fn inc_block_number(&mut self){
        self.block_number=self.block_number+1
    }

    pub fn inc_nonce(&mut self,from:&AccountId){
        let  prev_nonce=*self.nonce.get(from).unwrap_or(&0);
    
        self.nonce.insert(from.to_string(), prev_nonce+1);
    }

    pub fn get_current_nonce(&self,account:&AccountId)->u32{
        *self.nonce.get(account).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test{
    use std::{assert_eq, string::ToString};

    use super::Pallet;

    #[test]
    fn init_system(){
        let mut sys=Pallet::new();
        assert_eq!(sys.get_block_number(),0);
        sys.inc_block_number();
        assert_eq!(sys.get_block_number(),1);
        let account="Ronnie";
        sys.inc_nonce(&account.to_string());
        assert_eq!(sys.get_current_nonce(&account.to_string()),1);
    }
}