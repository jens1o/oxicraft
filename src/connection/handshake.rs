use crate::coding::short::UnsignedShort;
use crate::coding::string::MinecraftString;

#[derive(Debug)]
pub enum HandshakeNextState {
    Status = 1,
    Login = 2,
}

#[derive(Serialize)]
pub struct ServerListPingVersion {
    name: MinecraftString,
    protocol: UnsignedShort,
}

#[derive(Serialize)]
pub struct ServerListPingPlayers {
    max: UnsignedShort,
    online: UnsignedShort,
    sample: Vec<()>, // TODO: Not implemented yet
}

#[derive(Serialize)]
pub struct ServerListPingDescription {
    text: MinecraftString,
}

#[derive(Serialize)]
pub struct ServerListPingResponse {
    version: ServerListPingVersion,
    players: ServerListPingPlayers,
    description: ServerListPingDescription,
}

pub fn mock_slp() -> ServerListPingResponse {
    ServerListPingResponse {
        version: ServerListPingVersion {
            name: "1.13.1".to_owned(),
            protocol: 404,
        },
        players: ServerListPingPlayers {
            max: 100,
            online: 5,
            sample: vec![],
        },
        description: ServerListPingDescription {
            text: "Java edition doesn't necessarily mean everything is run by Java.".to_owned(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ServerListPingDescription, ServerListPingPlayers, ServerListPingResponse,
        ServerListPingVersion,
    };

    #[test]
    fn test_build_slp_response() {
        let slp = ServerListPingResponse {
            version: ServerListPingVersion {
                name: "1.13.1".to_owned(),
                protocol: 404,
            },
            players: ServerListPingPlayers {
                max: 100,
                online: 5,
                sample: vec![],
            },
            description: ServerListPingDescription {
                text: "Random test message".to_owned(),
            },
        };

        let json = serde_json::to_string(&slp).unwrap();

        let expected = r##"{"version":{"name":"1.13.1","protocol":404},"players":{"max":100,"online":5,"sample":[]},"description":{"text":"Random test message"}}"##;

        assert_eq!(expected.to_owned(), json);
    }
}
