struct Server {
    config: ServerConfig,
    players: Vec<Player>,
}

struct Player {
    id: u8,
    name: String,
    connection: u8,
}

struct ServerConfig {
    port: u32,
    max_players: u8,
}