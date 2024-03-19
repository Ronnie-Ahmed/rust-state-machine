use std::collections::BTreeMap;
pub struct Pallet{
    block_number:u32,
    nonce:BTreeMap<String,u32>,
}


impl Pallet{
    pub fn new()->Self{
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn get_block_number(&self)->u32{
        self.block_number
    }

    pub fn inc_block_number(&mut self){
        self.block_number=self.block_number+1

    }
}