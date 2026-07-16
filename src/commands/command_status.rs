#[derive(Debug, PartialEq)]
pub enum CommandStatus {
    Inserted = 1,
    Deleted = 2,
    RateLimited = 3,
    Ok = 4,
    NotFound = 5,
    Found = 6,
    ConfigNotFound = 7,
    KeyMoved = 8,
    Booting = 9,
    Queued = 10,
    Error = 255,
}

impl From<u8> for CommandStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => CommandStatus::Inserted,
            2 => CommandStatus::Deleted,
            3 => CommandStatus::RateLimited,
            4 => CommandStatus::Ok,
            5 => CommandStatus::NotFound,
            6 => CommandStatus::Found,
            7 => CommandStatus::ConfigNotFound,
            8 => CommandStatus::KeyMoved,
            9 => CommandStatus::Booting,
            10 => CommandStatus::Queued,
            _ => CommandStatus::Error,
        }
    }
}

impl From<&CommandStatus> for u8 {
    fn from(value: &CommandStatus) -> Self {
        match value {
            CommandStatus::Inserted => 1,
            CommandStatus::Deleted => 2,
            CommandStatus::RateLimited => 3,
            CommandStatus::Ok => 4,
            CommandStatus::NotFound => 5,
            CommandStatus::Found => 6,
            CommandStatus::ConfigNotFound => 7,
            CommandStatus::KeyMoved => 8,
            CommandStatus::Booting => 9,
            CommandStatus::Queued => 10,
            CommandStatus::Error => 255,
        }
    }
}

pub enum TextPersistType {
    None,
    Cfg { data: Box<[u8]> },
    Add { data: Box<[u8]> },
    Del { data: Box<[u8]> },
}

pub enum BloomFilterPersistType {
    None,
    Cfg { data: Box<[u8]> },
    Data { data: Box<[u8]> },
}

pub enum CounterPersistType {
    None,
    Cfg { data: Box<[u8]> },
    Data { data: Box<[u8]> }, // { data: String },
}

pub enum RateLimiterPersistType {
    None,
    Cfg { data: Box<[u8]> },
}
