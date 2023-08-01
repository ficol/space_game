use std::net::{IpAddr, Ipv4Addr};

pub const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub const GAME_UPDATE_TICK_SECONDS: f64 = 0.000001;
pub const GAME_STATE_TICK_SECONDS: f64 = 0.001;
