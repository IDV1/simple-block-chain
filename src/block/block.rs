use std::thread::sleep;
use std::time::{Duration, Instant};
use serde::Serialize;
use time::OffsetDateTime;
use crate::hash::hash_comp::{hash_str, serialize};
use crate::header::block::Header;
use crate::block::localtion::from;

// 整个区块
#[derive(Serialize, Debug)]
pub struct Block{
    // 头部 pre_hash + trs_hash + time
    pub header: Header,
    // 所有交易
    pub all_trs: String,
    // hash计算
    pub hash: String,
}

impl Block{
    pub(crate) fn new(tsx: String, per_hash: String) -> Self{
        println!("Start mining ....");
        sleep(Duration::from_secs(3));
        // 计算现在时间
        let time_start = OffsetDateTime::now_utc().unix_timestamp();
        // 序列化
        let serialize = serialize(&tsx).unwrap();
        // 计算hash
        let hash_comp = hash_str(&serialize).unwrap();
        let mut block = Block{
            header: Header{
                pre_hash: per_hash,
                tx_hash: hash_comp,
                location: "".to_string(),
                tx_id: Default::default(),
                time: time_start,
            },
            all_trs: tsx,
            hash: "".to_string(),
        };
        block.produce_location(2);
        block.set_hash();
        println!("one has been done !");
        block
    }

    // block 的hash设置
    pub fn set_hash(&mut self){
        let serialize = serialize(&self.header).unwrap();
        self.hash = hash_str(&serialize).unwrap()
    }

    // 生成国家和地点
    pub fn produce_location(&mut self,num: usize){
        let country = from(num);
        // 使用hash验证加密
        let serialize = serialize(&country).unwrap();
        let hash_comp = hash_str(&serialize).unwrap();

        self.header.location = hash_comp;
    }
}