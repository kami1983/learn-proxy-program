#![no_std]
use gstd::debug;
use gstd::{exec, msg, TypeInfo, Encode, Decode};

static mut SEED: u8 = 0;

#[derive(TypeInfo, Encode, Decode)]
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
}

#[no_mangle]
extern "C" fn init() {
    debug!("Target program init() running");
}

#[no_mangle]
extern "C" fn handle() {
    debug!("Target program handle() running");
    let action: Action = msg::load().expect("Error in decode message");
    let reply: Event = match action {
        Action::Hello => Event::Hello,
        Action::HowAreYou => Event::Fine,
        Action::MakeRandomNumber { range } => {
            let seed = unsafe { SEED };
            unsafe {
                SEED = seed.wrapping_add(1);
            }
            let mut random_input: [u8; 32] = exec::program_id().into();
            random_input[0] = random_input[0].wrapping_add(seed);
            let (random, _) = exec::random(random_input).unwrap();
            Event::Number(random[0] % range)
        }
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}