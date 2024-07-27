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
    SendMessage(MessageAction),
    CheckPeply,
}

#[derive(TypeInfo, Encode, Decode, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MessageAction {
    Hello,
    HowAreYou,
    MakeRandomNumber {
        range: u8,
    }
}

#[derive(TypeInfo, Encode, Decode, PartialEq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    Hello,
    Fine,
    Number(u8),
    MessageAlreadySend,
    MessageSent,
    WrongStatus,
    NoReplyReceived,
}