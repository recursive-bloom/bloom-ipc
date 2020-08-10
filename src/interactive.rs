
use common_types::block::Block;
use common_types::header::Header;
use ethereum_types::{ H256, H160, Address, U256, BigEndianHash };

fn get_latest_blocks(count : u32) -> Vec<Block> {
    let ret = vec![];
    ret
}

fn get_blocks_after_number(number: u128) -> Vec<Block> {
    let ret = vec![];
    ret
}

fn get_accounts_info(address_list : Vec<Address>) -> Vec<(U256, U256)> {
    let nonce = U256::from(0);
    let balance : U256 = U256::from(0);
    let info_list : Vec<(U256, U256)> = vec![];
    info_list.push((nonce, balance));
    info_list
}


