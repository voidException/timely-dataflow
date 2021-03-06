pub use communication::channels::Data;
pub use communication::allocator::ProcessCommunicator;
pub use communication::exchange::{ExchangeReceiver, exchange_with};
pub use communication::observer::Observer;
pub use communication::allocator::{Communicator};
pub use communication::pushpull::{Pushable, Pullable, PushableObserver};

pub mod channels;
pub mod allocator;
pub mod exchange;
pub mod observer;
pub mod pushpull;
