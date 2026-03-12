use serde::Serialize;
use sha3::Digest;
use crate::errors::er::Error;
use hex;
use hex::encode;

// 序列化
pub fn serialize<T: ?Sized + Serialize>(data: &T) -> Result<Vec<u8>, Error> {
    match bcs::to_bytes(data) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(Error::SerializationError(format!("BCS serialization failed: {}", e))),
    }
}

// hash 计算
pub fn hash_str(value: &[u8]) -> Result<String, Error> {
    let sha3_256 = sha3::Sha3_256::digest(value);
    let string = encode(sha3_256);
    Ok(string)
}