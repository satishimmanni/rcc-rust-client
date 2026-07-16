use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum ClusterState {
    Booting = 1,                //initial server boot
    Normal = 2,                 //cluster is at normal state
    ScalingUp = 3,              //adding servers, bucket movement
    SclaingDown = 4,            //removing servers, bucket movement
    ScaledUp = 5,               //new server added to cluster, next step is moving files
    ScaledDown = 6,             //server removed from cluster, next step is moving files
    ScalingUpMoving = 7,        //server added, moving bucket files
    SclaingDownMoving = 8,      //server removed, moving bucket files
    ScaledUpMoved = 9,          //server added, files moved
    ScaledDownMoved = 10,       //server removed, files moved
    ServerProcessingFiles = 11, //loading local files
    ServerProcessingQueue = 12, //processing the requests in queue while scaling, or loading files
    ServerScalingStarted = 13,
}

impl From<ClusterState> for u8 {
    fn from(status: ClusterState) -> u8 {
        u8::from(&status)
    }
}

impl From<&ClusterState> for u8 {
    fn from(status: &ClusterState) -> u8 {
        match status {
            ClusterState::Booting => 1,
            ClusterState::Normal => 2,
            ClusterState::ScalingUp => 3,
            ClusterState::SclaingDown => 4,
            ClusterState::ScaledUp => 5,
            ClusterState::ScaledDown => 6,
            ClusterState::ScalingUpMoving => 7,
            ClusterState::SclaingDownMoving => 8,
            ClusterState::ScaledUpMoved => 9,
            ClusterState::ScaledDownMoved => 10,
            ClusterState::ServerProcessingFiles => 11,
            ClusterState::ServerProcessingQueue => 12,
            ClusterState::ServerScalingStarted => 13,
        }
    }
}

impl TryFrom<u8> for ClusterState {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(ClusterState::Booting),
            2 => Ok(ClusterState::Normal),
            3 => Ok(ClusterState::ScalingUp),
            4 => Ok(ClusterState::SclaingDown),
            5 => Ok(ClusterState::ScaledUp),
            6 => Ok(ClusterState::ScaledDown),
            7 => Ok(ClusterState::ScalingUpMoving),
            8 => Ok(ClusterState::SclaingDownMoving),
            9 => Ok(ClusterState::ScaledUpMoved),
            10 => Ok(ClusterState::ScaledDownMoved),
            11 => Ok(ClusterState::ServerProcessingFiles),
            12 => Ok(ClusterState::ServerProcessingQueue),
            13 => Ok(ClusterState::ServerScalingStarted),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid cluster state value",
            )),
        }
    }
}
