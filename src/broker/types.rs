use crate::wallet::types::TxProof;
use grin_wallet_libwallet::Slate;

use common::config::Wallet713Config;
use common::Error;
use contacts::Address;

pub enum CloseReason {
    Normal,
    Abnormal(Error),
}

pub trait Publisher {
    fn post_slate(&self, slate: &Slate, to: &dyn Address) -> Result<(), Error>;
}

pub trait Subscriber {
    fn start(&mut self, handler: Box<dyn SubscriptionHandler + Send>) -> Result<(), Error>;
    fn stop(&mut self) -> bool;
    fn is_running(&self) -> bool;
}

pub trait SubscriptionHandler: Send {
    fn on_open(&self);
    fn on_slate(&self, from: &dyn Address, slate: &mut Slate, proof: Option<&mut TxProof>, Option<Wallet713Config>);
    fn on_close(&self, result: CloseReason);
    fn on_dropped(&self);
    fn on_reestablished(&self);
}
