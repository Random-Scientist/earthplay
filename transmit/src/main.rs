use std::{
    io::ErrorKind,
    net::{SocketAddr, SocketAddrV4},
    sync::atomic::AtomicU64,
};

use chacha20_poly1305::ChaCha20Poly1305;
use ed25519_dalek::SigningKey;
use mdns_sd::{ScopedIp, ServiceDaemon};
use plist::Dictionary;
use proto::stuff::AirplayFeatures;
use rand::Rng;

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Serialize;

use tokio::io::AsyncWriteExt;
use url::Url;

#[tokio::main]
async fn main() {
    let daemon = ServiceDaemon::new().unwrap();
    let browse = daemon.browse("_airplay._tcp.local.").unwrap();

    let service = loop {
        let resp = browse.recv_async().await.unwrap();
        if let mdns_sd::ServiceEvent::ServiceResolved(resolved_service) = resp {
            let features = AirplayFeatures::parse(
                resolved_service.get_property("features").unwrap().val_str(),
            )
            .unwrap();

            if features.contains(
                AirplayFeatures::SUPPORTS_AUDIO | AirplayFeatures::SUPPORTS_BUFFERED_AUDIO,
            ) {
                break resolved_service;
            }
        }
    };
    let ScopedIp::V4(v4) = service.addresses.iter().find(|v| v.is_ipv4()).unwrap() else {
        unreachable!();
    };
    let client = Client::new();

    let url = Url::parse(&format!("http://{}:{}", v4.addr(), service.port)).unwrap();
    let mut default_headers = HeaderMap::new();
    default_headers.append("X-Apple-ProtocolVersion", HeaderValue::from_static("1"));
    default_headers.append(
        "Content-Type",
        HeaderValue::from_static("application/x-apple-binary-plist"),
    );

    let mut rng = rand::rng();
    let body = SigningKey::generate(&mut rng);
    let mut bytes = vec![0x10u8];
    bytes.extend(body.verifying_key().as_bytes());

    let r = client
        .post(url.join("auth-setup").unwrap())
        .body(bytes)
        .send()
        .await
        .unwrap();
    dbg!(r);

    let sock = tokio::net::TcpSocket::new_v4().unwrap();
    let addr = std::net::SocketAddr::V4(SocketAddrV4::new(*v4.addr(), service.port));
    let mut stream = sock.connect(addr).await.unwrap();
    let (crypto, setup) = setup(addr);
    // probably needs pairing to work. How tf does pairing work?
    stream.write_all(&setup).await.unwrap();
    let mut buf = Vec::with_capacity(4096);
    let mut buf_size: usize = 0;
    'read: loop {
        stream.readable().await.unwrap();
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                buf_size = n;
                break 'read;
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                panic!("{e}");
            }
        }
    }
    let resp_bytes = String::from_utf8_lossy(&buf[0..buf_size]);
    println!("{resp_bytes}");
    dbg!(resp_bytes);
}
static SEQ: AtomicU64 = AtomicU64::new(0);
// some kind of evil mode bs
fn setup(for_addr: SocketAddr) -> (chacha20_poly1305::ChaCha20Poly1305, Vec<u8>) {
    use std::io::Write;

    let mut rng = rand::rng();
    let (keyb, nonceb) = (rng.random(), rng.random());
    let key = chacha20_poly1305::Key::new(keyb);
    let nonce = chacha20_poly1305::Nonce::new(nonceb);
    let mut val = Dictionary::new();
    val.insert("eiv".into(), plist::Value::Data(nonceb.into()));
    val.insert("ekey".into(), plist::Value::Data(keyb.into()));
    val.insert("et".into(), plist::Value::Integer(0.into()));
    val.insert("timingProtocol".into(), plist::Value::String("NTP".into()));
    let bytes = bplist_to_buf(&val);
    let seqn = SEQ.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    let mut out = Vec::new();

    write!(
        &mut out,
        "SETUP rtsp://{for_addr} RTSP/1.0\r\nCSeq: {seqn}\r\nContent-Length: {}\r\n\r\n",
        bytes.len()
    )
    .unwrap();
    out.extend(bytes);
    (ChaCha20Poly1305::new(key, nonce), out)
}

fn bplist_to_buf(v: impl Serialize) -> Vec<u8> {
    let mut out = Vec::new();
    plist::to_writer_binary(&mut out, &v).unwrap();
    out
}
