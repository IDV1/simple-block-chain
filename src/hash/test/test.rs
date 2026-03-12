use crate::hash::hash_comp::{hash_str, serialize};
use crate::block::block::Block;

#[test]
fn test_block(){
    let data = "hello world";
    let ser = serialize(data).unwrap();
    println!("{:?}", ser);
    let result = hash_str(&ser).unwrap();
    println!("hash_str result: {:?}", result);

    let block = Block::new("20354".to_string(),result);
    println!("{:?}", block);
}