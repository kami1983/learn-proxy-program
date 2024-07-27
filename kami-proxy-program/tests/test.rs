use gstd::ActorId;
use gstd::debug;
use gtest::{Log, Program, System};
use io::{Action, Event, MessageAction};

const USER: u64 = 3;
const TARGET_PROGRAM_ADDRESS: u64 = 2;


// #[test]
// fn success_test() {
//     let system = System::new();
//     system.init_logger();
//     let proxy_program = Program::current(&system);
//     let target_program = Program::from_file(&system, "target/wasm32-unknown-unknown/debug/kami_target_program.opt.wasm");
//     let result = target_program.send_bytes(USER, []);
//     assert!(!result.main_failed());
//
//     let target_program_address: ActorId = TARGET_PROGRAM_ADDRESS.into();
//
//     let result = proxy_program.send(USER, target_program_address);
//     assert!(!result.main_failed());
//
//     let result = proxy_program.send(USER, Action::MakeRandomNumber { range: 1 });
//     assert!(!result.main_failed());
//
//     let log = Log::builder().source(1).dest(USER).payload(Event::Number(0));
//     assert!(result.contains(&log));
//
//     // let mailbox = system.get_mailbox(USER);
//     // let log = Log::builder().source(1).dest(USER).payload(Event::Number(0));
//     // assert!(mailbox.contains(&log));
//
// }

// #[test]
// fn delay_test() {
//     let system = System::new();
//     system.init_logger();
//
//     let proxy_program = Program::current(&system);
//     let target_program = Program::from_file(&system, "target/wasm32-unknown-unknown/debug/kami_target_program.opt.wasm");
//
//     // 初始化 target 合约
//     let result = target_program.send_bytes(USER, []);
//     assert!(!result.main_failed());
//
//     // 初始化 proxy 合约
//     let target_program_address: ActorId = TARGET_PROGRAM_ADDRESS.into();
//     let result = proxy_program.send(USER, target_program_address);
//     assert!(!result.main_failed());
//
//     // 发送第一个消息
//     let result = proxy_program.send(USER, Action::SendMessage(MessageAction::MakeRandomNumber { range: 1 }));
//     assert!(!result.main_failed());
//
//     let log = Log::builder().source(1).dest(USER).payload(Event::MessageSent);
//     assert!(result.contains(&log));
//
//     let result = proxy_program.send(USER, Action::SendMessage(MessageAction::MakeRandomNumber { range: 1 }));
//     assert!(!result.main_failed());
//
//     let log = Log::builder().source(1).dest(USER).payload(Event::WrongStatus);
//     assert!(result.contains(&log));
//
//     let result = system.spend_blocks(3);
//
//     let mailbox = system.get_mailbox(USER);
//     let log = Log::builder().source(1).dest(USER).payload(Event::NoReplyReceived);
//     println!("log = {:?}", log);
//     assert!(mailbox.contains(&log));
//
// }

#[test]
fn test() {

    let system = System::new();
    system.init_logger();

    let proxy_program: Program<'_> = Program::current(&system);
    let target_program = Program::from_file(&system, "target/wasm32-unknown-unknown/debug/kami_target_program.opt.wasm");

    let result = target_program.send_bytes(USER, []);
    assert!(!result.main_failed());

    let target_program_address: ActorId = TARGET_PROGRAM_ADDRESS.into();
    let result = proxy_program.send(USER, target_program_address);
    assert!(!result.main_failed());

    let result = proxy_program.send(USER, Action::SendMessage(MessageAction::MakeRandomNumber { range: 1 } ));
    assert!(!result.main_failed());

    let log: Log = Log::builder().source(1).dest(3).payload(Event::MessageSent);
    assert!(result.contains(&log));

    let result = proxy_program.send(USER, Action::SendMessage(MessageAction::MakeRandomNumber { range: 1 } ));
    assert!(!result.main_failed());

    let log: Log = Log::builder().source(1).dest(3).payload(Event::WrongStatus);
    assert!(result.contains(&log));


    let result = system.spend_blocks( 3);

    let mailbox = system.get_mailbox(USER);
    let log: Log = Log::builder().source(1).dest(3).payload(Event::NoReplyReceived);
    assert!(mailbox.contains(&log));
}
