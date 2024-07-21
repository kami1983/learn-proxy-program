#![no_std]

use gstd::{ActorId, MessageId, msg};
use gstd::debug;
use io::{Action, Event, ProxyMetadata};

static mut SESSION: Option<Session> = None;

struct Session {
    target_program_id: ActorId,
    msg_id_to_actor_id: (MessageId, ActorId),
}

#[no_mangle]
extern "C" fn init() {
    // TODO:: Maybe change ActorId to ProgramId, as it is more descriptive
    let target_program_id: ActorId = msg::load().expect("Unable to decode init");
    debug!("!!! INIT !!! target_program_id = {:?}", &target_program_id);
    unsafe {
        SESSION = Some(Session {
            target_program_id,
            msg_id_to_actor_id: (MessageId::zero(), ActorId::zero()),
        });
    }

}

#[no_mangle]
extern "C" fn handle() {
    debug!("!!! HANDLE !!!");
    debug!("Message ID: {:?}", msg::id());

    let action: Action = msg::load().expect("Unable to decode Action");
    debug!("Message payload: {:?}", action);

    let session = unsafe { SESSION.as_mut().expect("Session not initialized") };
    let msg_id = msg::send(session.target_program_id, action, 0).expect("Unable to send message");
    session.msg_id_to_actor_id = (msg_id, msg::source());
    msg::reply( Event::MessageSend, 0).expect("Unable to reply");

}


#[no_mangle]
extern "C" fn handle_reply() {
    let reply_message_id = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("Session not initialized") };
    let (msg_id, actor) = session.msg_id_to_actor_id;
    if reply_message_id == msg_id {
        let reply: Event = msg::load().expect("Unable to decode Event");
        msg::send(actor, reply, 0).expect("Unable to send message");

    }
}

