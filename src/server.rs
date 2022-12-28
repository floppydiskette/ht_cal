use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use crate::datetime::HDateTime;
use crate::history::HistoryData;
use crate::packet::PacketData;

pub const HOST: &str = "0.0.0.0"; // should be firewalled
pub const PORT_Q: u16 = 3621;
pub const PORT_H: u16 = 3926;

pub async fn listen_query(hdt: Arc<Mutex<HDateTime>>) {
    let listener = tokio::net::TcpListener::bind((HOST, PORT_Q)).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let hdt = hdt.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut socket = socket;
            let n = socket.read(&mut buf).await.unwrap();
            if n == 0 {
                return;
            }
            let hdt = hdt.lock().await;
            let response = PacketData {
                year: hdt.year,
                month: hdt.month,
                day: hdt.day,
                second: hdt.second,
                time_since_second_ms: hdt.time_since_second.num_milliseconds() as u128,
                time_of_packet_sent_ms: chrono::Utc::now().timestamp_millis() as u128,
            };
            // serialise with rmp-serde
            let mut buf = Vec::new();
            rmp_serde::encode::write(&mut buf, &response).unwrap();
            socket.write_all(&buf).await.unwrap();
            // close the socket
            socket.shutdown().await.unwrap();
        });
    }
}

pub async fn listen_history(history: Arc<Mutex<HistoryData>>) {
    let listener = tokio::net::TcpListener::bind((HOST, PORT_H)).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let history = history.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut socket = socket;
            let n = socket.read(&mut buf).await.unwrap();
            if n == 0 {
                return;
            }
            let history = history.lock().await;
            // serialise with rmp-serde
            let mut buf = Vec::new();
            rmp_serde::encode::write(&mut buf, &history.clone()).unwrap();
            socket.write_all(&buf).await.unwrap();
            // close the socket
            socket.shutdown().await.unwrap();
        });
    }
}