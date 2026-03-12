use crate::block::block::Block;

// 第一个区块hash
const PRE_HASH: &'static str = "020100202030419495321823";

// 链
pub struct Chain{
    // 存放多个块
    pub blocks: Vec<Block>
}

impl Chain {
    pub fn new() -> Self{
        Self{
            blocks: vec![Self::dadly_block()]
        }
    }

    // 创建第一个爹块
    pub fn dadly_block() -> Block {
        Block::new("dadly".to_string(),PRE_HASH.to_string())
    }

    // 添加区块
    pub fn add_block(&mut self, trs_data: String){
        // 前一个区块
        let get_pre_block = &self.blocks[self.blocks.len() - 1];
        // 拿到前一个区块的hash
        let get_pre_hash = get_pre_block.hash.clone();
        // 构建一个
        let new_block = Block::new(trs_data, get_pre_hash);
        self.blocks.push(new_block);
    }

    // 输出区块信息
    pub fn print_info(&self){
        self.blocks
            .iter()
            .for_each(|b| println!("{:#?}", b));
    }
}