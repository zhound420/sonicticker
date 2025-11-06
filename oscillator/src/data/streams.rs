use tokio::sync::mpsc;

use crate::models::PriceTick;

pub type TickSender = mpsc::Sender<PriceTick>;
pub type TickReceiver = mpsc::Receiver<PriceTick>;

pub fn channel(buffer: usize) -> (TickSender, TickReceiver) {
    mpsc::channel(buffer)
}
