#![no_std]

// 1️⃣ External packages (crates) import

use gstd::{debug, exec, msg, prelude::*, ActorId};

use circular_buffer::CircularBuffer;
use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

// 2️⃣ This defines the meta information about the contract
// for the GEAR IDEA portal to parse.
// It also defines the communication interface via input / output fields.
gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    handle:
        input: ChannelAction,
        output: ChannelOutput,
    state:
      output: Vec<Message>,
}

// 3️⃣ These are the data structures that will be used for communication

// Actions that can possibly be proccessed by the contract
#[derive(Debug, Decode, TypeInfo)]
enum ChannelAction {
    Meta,
    Subscribe,
    Unsubscribe,
    Post(String),
}

// Return types that this contract supports
#[derive(Encode, TypeInfo)]
enum ChannelOutput {
    Metadata(Meta),
    SingleMessage(Message),
}

// Post message within the channel
#[derive(Clone, Debug, Encode, TypeInfo)]
struct Message {
    text: String,
    timestamp: u32,
}

impl Message {
    fn new(text: String) -> Self {
        Self {
            text,
            timestamp: exec::block_height(),
        }
    }
}

// Meta information about a channel
#[derive(Debug, Encode, TypeInfo)]
struct Meta {
    name: String,
    description: String,
    owner_id: H256,
}

// The state interface of the contract (memory)
#[derive(Clone)]
struct State {
    owner_id: Option<ActorId>,
    name: Option<String>,
    description: Option<String>,
    subscribers: Vec<ActorId>,
    messages: Option<CircularBuffer<Message>>,
}

impl State {
    fn set_owner_id(&mut self, id: ActorId) {
        if self.owner_id.is_none() {
            self.owner_id = Some(id);
        } else {
            panic!("Owner ID for the channel was already set!")
        }
    }

    fn set_name(&mut self, name: &'static str) {
        if self.name.is_none() {
            self.name = Some(String::from(name));
        } else {
            panic!("Name for the channel was already set!")
        }
    }

    fn set_description(&mut self, desc: &'static str) {
        if self.description.is_none() {
            self.description = Some(String::from(desc));
        } else {
            panic!("Description for the channel was already set!")
        }
    }

    fn add_subscriber(&mut self, id: ActorId) {
        self.subscribers.push(id);
    }

    fn remove_subscriber(&mut self, id: ActorId) {
        self.subscribers.retain(|v| *v != id);
    }

    fn add_message(&mut self, message: Message) {
        self.messages
            .get_or_insert_with(|| CircularBuffer::new(5))
            .push(message);
    }

    fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| String::from("UNKNOWN"))
    }
}

// 4️⃣ The state itself (i.e. the variable state will be accessed through)
static mut STATE: State = State {
    name: None,
    description: None,
    owner_id: None,
    subscribers: Vec::new(),
    messages: None,
};

// 5️⃣ Init function that is executed once upon contract initialization
#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE.set_owner_id(msg::source());
    // TODO: Change the channel name
    STATE.set_name("Channel-Coolest-Name");
    // TODO: Change the channel description
    STATE.set_description("Channel-Coolest-Description");

    // TODO: Change the init message
    let init_message = Message::new(format!("Channel {:?} was created", STATE.name()));

    STATE.add_message(init_message);
    STATE.add_subscriber(STATE.owner_id.unwrap());

    debug!("Channel {:?} initialized successfully!", STATE.name());
}

// 6️⃣ Handle function that processes the incoming message
#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: ChannelAction = msg::load().unwrap_or_else(|_| {
        panic!(
            "CHANNEL {:?}: Unable to decode Channel Action",
            STATE.name()
        )
    });

    let source: ActorId = msg::source();

    debug!("CHANNEL {:?}: Received action: {:?}", STATE.name(), action);

    match action {
        ChannelAction::Meta => {
            let meta = Meta {
                name: STATE.name.clone().unwrap_or_default(),
                description: STATE.description.clone().unwrap_or_default(),
                owner_id: H256(STATE.owner_id.unwrap().into()),
            };

            msg::reply(
                ChannelOutput::Metadata(meta),
                exec::gas_available() - 100_000_000,
                0,
            );

            debug!("CHANNEL {:?}: Meta sent", STATE.name())
        }
        ChannelAction::Subscribe => {
            // TODO: Add a subscriber and reply

            debug!("CHANNEL {:?}: Subscriber added", STATE.name())
        }
        ChannelAction::Unsubscribe => {
            STATE.remove_subscriber(source);

            msg::reply((), 0, 0);

            debug!("CHANNEL {:?}: Subscriber removed", STATE.name())
        }
        ChannelAction::Post(text) => {
            if let Some(owner_id) = STATE.owner_id {
                if owner_id != source {
                    panic!("CHANNEL {:?}: Poster is not an owner", STATE.name())
                }

                let message = Message::new(text);

                STATE.add_message(message.clone());

                for id in &STATE.subscribers {
                    msg::send(*id, ChannelOutput::SingleMessage(message.clone()), 0, 0);
                }

                msg::reply((), 0, 0);

                debug!("Added a post: {:?}", message);
            } else {
                panic!("CHANNEL {:?}: Owner was not set", STATE.name())
            }
        }
    }
}

// 7️⃣ Meta (off-chain) function to get messages from the program state
#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let messages: Vec<Message> = STATE
        .messages
        .clone()
        .map(|v| v.into_iter().collect())
        .unwrap_or_default();
    let encoded = messages.encode();
    let result = gstd::macros::util::to_wasm_ptr(&encoded[..]);
    core::mem::forget(encoded);

    result
}
