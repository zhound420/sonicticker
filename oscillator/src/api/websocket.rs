use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use serde::Serialize;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::{models::AudioPacket, state::AppState};

#[derive(Debug, serde::Deserialize)]
pub struct AudioStreamQuery {
    asset: Option<String>,
}

pub async fn upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<AudioStreamQuery>,
) -> impl IntoResponse {
    let asset = query.asset.unwrap_or_else(|| "btcusdt".to_string());
    ws.on_upgrade(move |socket| handle_socket(socket, state, asset))
}

async fn handle_socket(mut socket: WebSocket, state: AppState, asset: String) {
    info!(%asset, "WebSocket subscriber connected");
    let mut stream = state.subscribe(&asset);

    loop {
        tokio::select! {
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Ping(payload))) => {
                        if socket
                            .send(Message::Pong(payload))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(err)) => {
                        warn!(%asset, %err, "WebSocket incoming error");
                        break;
                    }
                    _ => {}
                }
            }
            packet = stream.recv() => {
                match packet {
                    Ok(packet) => {
                        if let Err(err) = send_packet(&mut socket, &packet).await {
                            error!(%asset, %err, "WebSocket send error");
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        warn!(%asset, skipped, "WebSocket lagged");
                        continue;
                    }
                    Err(_) => break,
                }
            }
        }
    }

    info!(%asset, "WebSocket disconnected");
}

async fn send_packet(socket: &mut WebSocket, packet: &AudioPacket) -> anyhow::Result<()> {
    let metadata = AudioMetadata {
        asset: &packet.asset,
        sample_rate: packet.chunk.sample_rate,
        frames: packet.chunk.frames,
        channels: packet.chunk.channels,
        timestamp: packet.chunk.timestamp.to_rfc3339(),
        metrics: &packet.metrics,
        params: &packet.params,
        payload_bytes: packet.chunk.samples.len(),
    };

    let meta_json = serde_json::to_string(&metadata)?;
    socket.send(Message::Text(meta_json)).await?;
    socket
        .send(Message::Binary(packet.chunk.samples.clone()))
        .await?;
    Ok(())
}

#[derive(Serialize)]
struct AudioMetadata<'a> {
    asset: &'a str,
    sample_rate: u32,
    frames: usize,
    channels: u8,
    timestamp: String,
    metrics: &'a crate::models::MarketMetrics,
    params: &'a crate::models::MusicalParams,
    payload_bytes: usize,
}
