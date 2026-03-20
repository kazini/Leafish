/// Integration test: verify Leafish can complete the 1.20.4 login handshake
/// against a real MCHPRS server at localhost:25565.
///
/// Run with:  cargo test --test protocol_1_20_4 -- --nocapture
///
/// Stages tested:
///   1. Status ping  → verifies server is 1.20.4 (protocol 765)
///   2. Login handshake → Handshake → LoginStart → LoginSuccess (or SetInitialCompression first)
///   3. First play-state packet → confirms we can read at least one packet post-login
use leafish_protocol::protocol::{packet, State, VarInt, Conn};

const SERVER: &str = "localhost:25565";
const PROTOCOL: i32 = 765;

/// Stage 1: status ping must report protocol 765 and version "1.20.4"
#[test]
fn test_status_reports_1_20_4() {
    let conn = Conn::new(SERVER, PROTOCOL).expect("failed to connect for status");
    let (status, ping) = conn.do_status().expect("status ping failed");
    println!("Server version: {} (protocol {})", status.version.name, status.version.protocol);
    println!("Players: {}/{}", status.players.online, status.players.max);
    println!("Ping: {}ms", ping.as_millis());
    assert_eq!(
        status.version.protocol, PROTOCOL,
        "expected protocol {PROTOCOL}, got {}",
        status.version.protocol
    );
}

/// Stage 2+3: complete offline login and read the first play-state packet
#[test]
fn test_offline_login_completes() {
    use packet::handshake::serverbound::Handshake;
    use packet::login::serverbound::LoginStart;
    use packet::Packet;

    let mut conn = Conn::new(SERVER, PROTOCOL).expect("failed to connect for login");

    // Handshake → login state
    conn.write_packet(Handshake {
        protocol_version: VarInt(PROTOCOL),
        host: "localhost".to_string(),
        port: 25565,
        next: VarInt(2), // 2 = login
    })
    .expect("failed to write Handshake");
    conn.state = State::Login;

    // LoginStart
    conn.write_packet(LoginStart {
        username: "TestPlayer".to_string(),
    })
    .expect("failed to write LoginStart");

    // Read until LoginSuccess (absorb SetInitialCompression / LoginPluginRequest if present)
    let uuid = loop {
        let pkt = conn.read_packet().expect("failed to read login packet");
        match pkt {
            Packet::SetInitialCompression(c) => {
                println!("SetInitialCompression threshold={}", c.threshold.0);
                conn.set_compression(c.threshold.0);
            }
            Packet::LoginPluginRequest(req) => {
                println!("LoginPluginRequest channel={}", req.channel);
                // Respond with failure (unknown plugin channel)
                conn.write_packet(packet::login::serverbound::LoginPluginResponse {
                    message_id: req.message_id,
                    successful: false,
                    data: vec![],
                })
                .expect("failed to write LoginPluginResponse");
            }
            Packet::LoginSuccess_UUID(val) => {
                println!("LoginSuccess_UUID: {} {:?}", val.username, val.uuid);
                conn.state = State::Play;
                break val.uuid;
            }
            Packet::LoginSuccess_UUID_Properties(val) => {
                println!(
                    "LoginSuccess_UUID_Properties: {} {:?} ({} properties)",
                    val.username,
                    val.uuid,
                    val.properties.data.len()
                );
                conn.state = State::Play;
                break val.uuid;
            }
            Packet::LoginDisconnect(d) => {
                panic!("Server disconnected during login: {:?}", d.reason);
            }
            other => {
                panic!("Unexpected login packet: {:?}", other);
            }
        }
    };

    println!("Logged in as UUID {:?}", uuid);

    // Send LoginAcknowledged to transition server to Configuration state
    conn.write_packet(packet::login::serverbound::LoginAcknowledged { empty: () })
        .expect("failed to write LoginAcknowledged");
    conn.state = State::Configuration;

    // Stage 3: process Configuration state until FinishConfiguration
    loop {
        let pkt = conn.read_packet().expect("failed to read configuration packet");
        match pkt {
            Packet::FinishConfiguration(_) => {
                println!("FinishConfiguration received — entering Play state");
                conn.write_packet(packet::configuration::serverbound::AcknowledgeConfiguration {
                    empty: (),
                })
                .expect("failed to write AcknowledgeConfiguration");
                conn.state = State::Play;
                break;
            }
            Packet::ClientboundKnownPacks(packs) => {
                println!("ClientboundKnownPacks: {} packs", packs.known_packs.data.len());
                // Reply with empty known-packs list
                conn.write_packet(
                    packet::configuration::serverbound::ServerboundKnownPacks {
                        known_packs: leafish_protocol::protocol::LenPrefixed::new(vec![]),
                    },
                )
                .expect("failed to write ServerboundKnownPacks");
            }
            Packet::ConfigPluginMessage(msg) => {
                println!("ConfigPluginMessage channel={}", msg.channel);
            }
            Packet::ConfigPing(ping) => {
                println!("ConfigPing id={} — sending ConfigPong", ping.id);
                conn.write_packet(packet::configuration::serverbound::ConfigPong { id: ping.id })
                    .expect("failed to write ConfigPong");
            }
            Packet::ConfigRegistryData(_) => { /* large payload, skip */ }
            Packet::ConfigFeatureFlags(_) | Packet::ConfigUpdateTags(_) => {}
            other => {
                println!("Configuration packet (ignored): {:?}", other);
            }
        }
    }

    // Stage 4: read the first play-state packet without crashing
    match conn.read_packet() {
        Ok(first) => println!("First play packet: {:?}", first),
        Err(e) => panic!("Failed to read first play packet: {:?}", e),
    }
}
