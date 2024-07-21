#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId, MessageId, msg};

pub struct ProxyMetadata ;

impl Metadata for ProxyMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

#[derive(TypeInfo, Encode, Decode, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    Hello,
    HowAreYou,
    MakeRandomNumber {
        range: u8,
    }
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    Hello,
    Fine,
    Number(u8),
    MessageSend,
}