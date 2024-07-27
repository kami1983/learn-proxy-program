#![no_std]

use gstd::{ActorId, exec, MessageId, msg};
use gstd::debug;
use io::{Action, Event, ProxyMetadata};

type SentMessageId = MessageId;
type OriginMessageId = MessageId;

static mut SESSION: Option<Session> = None;

struct Session {
    target_program_id: ActorId,
    msg_ids: (SentMessageId, OriginMessageId, ActorId),
    // msg_id_to_actor_id: (MessageId, ActorId),
    session_status: SessionStatus,
}

#[derive(PartialEq, Debug)]
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
            msg_ids: (MessageId::zero(), MessageId::zero(), ActorId::zero()),
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

    match action {
        Action::SendMessage(message_action) => {
            if session.session_status == SessionStatus::Waiting {
                debug!("HANDLE:: Action::SendMessage and SessionStatus::Waiting");
                // let msg_id = msg::send(session.target_program_id, message_action, 0).expect("Unable to send message");
                debug!("HANDLE:: SessionStatus::MessageSend msg id");
                session.session_status = SessionStatus::MessageSent;
                session.msg_ids = (MessageId::zero(), msg::id(), msg::source());

                msg::send_delayed(exec::program_id(), Action::CheckPeply, 0 , 3 ).expect("Error in sending delayed message.");
                msg::reply(Event::MessageSent, 0 ).expect("Error in sending reply");
            }else{
                debug!("HANDLE:: send to Event::WrongStatus");
                msg::reply(Event::WrongStatus, 0 ).expect("Error in sending reply");
            }
        }
        Action::CheckPeply => {
            debug!("HANDLE:: Action::CheckPeply");
            debug!("msg::source() {:?} == exec::program_id() {:?}", msg::source(), exec::program_id() );
            if session.session_status == SessionStatus::MessageSent && msg::source() == exec::program_id() {
                debug!("HANDLE:: No response was received");
                msg::send(session.msg_ids.2, Event::NoReplyReceived, 0).expect("Error in sending message.");
                debug!("HANDLE:: SessionStatus::Waiting");
                session.session_status = SessionStatus::Waiting;
            }
        }
    }

    // match &session.session_status {
    //     SessionStatus::Waiting => {
    //         let msg_id = msg::send(session.target_program_id, action, 0).expect("Unable to send message");
    //         session.session_status = SessionStatus::MessageSent;
    //         session.msg_ids = (msg_id, msg::id());
    //         exec::wait();
    //     }
    //     SessionStatus::MessageSent => {
    //         msg::reply(Event::MessageAlreadySend, 0).expect("Error in sending reply");
    //     }
    //     SessionStatus::ReplyReceived(reply_message) => {
    //         msg::reply(reply_message, 0).expect("Error in sending reply");
    //         session.session_status = SessionStatus::Waiting
    //     }
    // }


}


#[no_mangle]
extern "C" fn handle_reply() {

    let reply_message_id = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("Session not initialized") };

    debug!("HANDLE_REPLY - session.session_status = {:?}", &session.session_status);
    // debug!("Msgid = {:?}", reply_message_id);

    if reply_message_id == session.msg_ids.0 && session.session_status == SessionStatus::MessageSent {
        // debug!("HANDLE_REPLY set SessionStatus::ReplyReceived");
        let reply: Event = msg::load().expect("Unable to decode Event");
        session.session_status = SessionStatus::ReplyReceived(reply);
    }
}

