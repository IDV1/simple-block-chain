use serde::Serialize;
use crate::errors::er::Error;
#[derive(Debug,Serialize)]
pub enum CityName {
    NewYork,
    Washington,
    Fuzhou,
    Shanghai,
    Beijing,
    Shenzhen
}

#[derive(Debug,Serialize)]
pub enum Country {
    American(CityName),
    Canada(CityName),
    Japan(CityName),
    Australia(CityName),
    China(CityName),
    Error(Error),
}

// 生成国家和城市名
pub fn from(country_num: usize) -> Country{
    match country_num{
        1 => Country::American(CityName::NewYork),
        2 => Country::China(CityName::Shanghai),
        _ => Country::Error(Error::NotTheChose(format!("{} is not a valid country code", country_num)))
    }
}