#![no_std]

// 1️⃣ External packages (crates) and internal modules import
use gstd::{debug, msg, prelude::*};

mod common;
mod state;

use common::{ChannelAction, ChannelOutput, Message, Meta};
use state::State;

pub use state::meta_state;

// 2️⃣ This defines the meta information about the contract
// for the Gear IDEA portal to parse.
// It also defines the communication interface via input / output fields.
gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    handle:
        input: ChannelAction,
        output: ChannelOutput,
    state:
      output: Vec<Message>,
}

// 3️⃣ The state itself (i.e. the variable state will be accessed through)
static mut STATE: State = State::new();

// 4️⃣ Init function that is executed once upon contract initialization
#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE.set_owner_id(msg::source());
    // ⚠️ TODO: Change the channel name
    STATE.set_name("Channel-Coolest-Name");
    // ⚠️ TODO: Change the channel description
    STATE.set_description("Channel-Coolest-Description");

    // ⚠️ TODO: Change the init message
    let init_message = Message::new(format!("Channel {:?} was created", STATE.name()));

    STATE.add_message(init_message);
    STATE.add_subscriber(msg::source());

    debug!("Channel {:?} initialized successfully!", STATE.name());
}

// 5️⃣ Handle function that processes the incoming message
#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: ChannelAction = msg::load().expect(&format!(
        "CHANNEL {:?}: Unable to decode Channel Action",
        STATE.name()
    ));

    debug!("CHANNEL {:?}: Received action: {:?}", STATE.name(), action);

    match action {
        ChannelAction::Meta => {
            let meta = ChannelOutput::Metadata(Meta::new(
                STATE.name(),
                STATE.description(),
                STATE.owner(),
            ));

            msg::reply(meta, 0, 0);

            debug!("CHANNEL {:?}: Meta sent", STATE.name())
        }
        ChannelAction::Subscribe => {
            // ⚠️ TODO: Add a subscriber and reply

            debug!("CHANNEL {:?}: Subscriber added", STATE.name())
        }
        ChannelAction::Unsubscribe => {
            STATE.remove_subscriber(msg::source());

            msg::reply((), 0, 0);

            debug!("CHANNEL {:?}: Subscriber removed", STATE.name())
        }
        ChannelAction::Post(text) => {
            if !STATE.is_owner(msg::source()) {
                panic!("CHANNEL {:?}: Poster is not an owner", STATE.name())
            }

            let message = Message::new(text);

            STATE.add_message(message.clone());

            for sub in STATE.subs() {
                msg::send(sub, ChannelOutput::SingleMessage(message.clone()), 0, 0);
            }

            msg::reply((), 0, 0);

            debug!("Added a post: {:?}", message);
        }
    }
}
