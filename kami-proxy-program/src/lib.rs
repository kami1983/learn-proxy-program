#![no_std]

use gstd::{ActorId, exec, MessageId, msg};
use gstd::debug;
use io::{Action, Event, ProxyMetadata};

type SentMessageId = MessageId;
type OriginMessageId = MessageId;

static mut SESSION: Option<Session> = None;

struct Session {
    target_program_id: ActorId,
    msg_ids: (SentMessageId, OriginMessageId),
    // msg_id_to_actor_id: (MessageId, ActorId),
    session_status: SessionStatus,
}

#[derive(PartialEq)]
enum SessionStatus {
    Waiting,
    MessageSent,
    ReplyReceived(Event),
}

#[no_mangle]
extern "C" fn init() {
    // TODO:: Maybe change ActorId to ProgramId, as it is more descriptive
    let target_program_id: ActorId = msg::load().expect("Unable to decode init");
    debug!("!!! INIT !!! target_program_id = {:?}", &target_program_id);
    unsafe {
        SESSION = Some(Session {
            target_program_id,
            // msg_id_to_actor_id: (MessageId::zero(), ActorId::zero()),
            msg_ids: (MessageId::zero(), MessageId::zero()),
            session_status: SessionStatus::Waiting,
        });
    }

}

#[no_mangle]
extern "C" fn handle() {
    debug!("!!! HANDLE !!!");
    debug!("Message ID: {:?}", msg::id());

    let action: Action = msg::load().expect("Unable to decode Action");
    debug!("Message payload: {:?}", action);

    let session: &mut Session = unsafe { SESSION.as_mut().expect("Session not initialized") };

    match &session.session_status {
        SessionStatus::Waiting => {
            let msg_id = msg::send(session.target_program_id, action, 0).expect("Unable to send message");
            session.session_status = SessionStatus::MessageSent;
            session.msg_ids = (msg_id, msg::id());
            exec::wait();
        }
        SessionStatus::MessageSent => {
            msg::reply(Event::MessageAlreadySend, 0).expect("Error in sending reply");
        }
        SessionStatus::ReplyReceived(reply_message) => {
            msg::reply(reply_message, 0).expect("Error in sending reply");
            // msg::reply(Event::Hello, 0).expect("Error in sending reply");
            session.session_status = SessionStatus::Waiting
        }
    }

    // let msg_id = msg::send(session.target_program_id, action, 0).expect("Unable to send message");
    // session.msg_id_to_actor_id = (msg_id, msg::source());
    // msg::reply( Event::, 0).expect("Unable to reply");

}


#[no_mangle]
extern "C" fn handle_reply() {
    debug!("HANDLE_REPLY");
    let reply_message_id = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("Session not initialized") };
    // let (msg_id, actor) = session.msg_id_to_actor_id;
    if reply_message_id == session.msg_ids.0 && session.session_status == SessionStatus::MessageSent {
        let reply: Event = msg::load().expect("Unable to decode Event");
        session.session_status = SessionStatus::ReplyReceived(reply);
        let origin_message_id = session.msg_ids.1;
        exec::wake(origin_message_id).expect("Failed to wake message");
        // msg::send(actor, reply, 0).expect("Unable to send message");


    }
}

