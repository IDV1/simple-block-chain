use serde::Serialize;

// 头区块
#[derive(Debug,Serialize)]
pub struct Header{
    // 前hash
    pub pre_hash: String,
    // 交易hash
    pub tx_hash: String,
    // 地理位置
    pub location: String,
    // 交易id
    pub tx_id: usize,
    // 交易时间
    pub time: i64,
}