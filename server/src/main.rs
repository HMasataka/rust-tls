//! This is the simplest possible server using rustls that does something useful:
//! it accepts the default configuration, loads a server certificate and private key,
//! and then accepts a single client connection.
//!
//! Usage: cargo r --bin simpleserver <path/to/cert.pem> <path/to/privatekey.pem>
//!
//! Note that `unwrap()` is used to deal with networking errors; this is not something
//! that is sensible outside of example code.

use rustls_pemfile;
use std::env;
use std::error::Error as StdError;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{rustls, TlsAcceptor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mut args = env::args();
    args.next();
    let cert_file = args.next().expect("missing certificate file argument");
    let private_key_file = args.next().expect("missing private key file argument");

    let certs = rustls_pemfile::certs(&mut BufReader::new(&mut File::open(cert_file)?))
        .collect::<Result<Vec<_>, _>>()?;

    let private_key =
        rustls_pemfile::private_key(&mut BufReader::new(&mut File::open(private_key_file)?))?
            .unwrap();

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(format!("[::]:{}", 4443)).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            process(acceptor, stream).await;
        });
    }
}

async fn process(acceptor: TlsAcceptor, stream: TcpStream) {
    let mut stream = acceptor.accept(stream).await.unwrap();

    let mut buf = Vec::with_capacity(4096);
    stream.read_buf(&mut buf).await.unwrap();

    let msg = String::from_utf8(buf).expect("failed to convert String");
    let result = stream.write(msg.as_bytes()).await;

    println!(
        "wrote to stream; msg={:?}, success={:?}",
        msg,
        result.is_ok()
    );
}
