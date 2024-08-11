use std::{fs, io, num, path::PathBuf};

#[derive(Debug)]
pub enum W1Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    BadSerialConnection,
    UnsupportedId,
}

impl From<io::Error> for W1Error {
    fn from(err: io::Error) -> W1Error {
        W1Error::Io(err)
    }
}

impl From<num::ParseIntError> for W1Error {
    fn from(err: num::ParseIntError) -> W1Error {
        W1Error::Parse(err)
    }
}

const W1_SYS_FS_PATH: &str = "/sys/bus/w1/devices";
const W1_SLAVE_SYS_SUB_PATH: &str = "w1_slave";
const DS18B20_IDENTIFIER: &str = "28-";

pub struct Temperature(u32);

impl Temperature {
    pub fn to_celsius(self) -> f64 {
        (self.0 as f64) / 1000.0
    }

    pub fn to_fahrenheit(self) -> f64 {
        (self.0 as f64) / 1000.0 / 5.0 * 9.0 + 32.0
    }
}

pub struct Ds18b20 {
    identifier: String,
    pub name: String,
}

impl Ds18b20 {
    pub fn new(id: String, name: String) -> Result<Self, W1Error> {
        if !id.starts_with(DS18B20_IDENTIFIER) {
            return Err(W1Error::UnsupportedId);
        }
        Ok(Ds18b20 {
            identifier: id,
            name,
        })
    }

    pub fn read_raw(&self) -> io::Result<String> {
        let mut path = PathBuf::from(W1_SYS_FS_PATH);
        path.push(&self.identifier);
        path.push(W1_SLAVE_SYS_SUB_PATH);
        fs::read_to_string(path)
    }

    fn parse_temp(data: String) -> Result<u32, num::ParseIntError> {
        let (_, temp_str) = data.split_at(data.find("t=").unwrap() + 2);
        temp_str.trim().parse::<u32>()
    }

    pub fn read_temp(&self) -> Result<Temperature, W1Error> {
        let temp_data = self.read_raw()?;
        if !temp_data.contains("YES") {
            return Err(W1Error::BadSerialConnection);
        }
        Ok(Temperature(Ds18b20::parse_temp(temp_data)?))
    }
}
