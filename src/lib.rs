#![no_std]

// 1️⃣ External packages (crates) import

extern crate alloc;
use circular_buffer::CircularBuffer;
use codec::{Decode, Encode};
use gstd::{debug, exec, msg, prelude::*, ProgramId};
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
}

// 3️⃣ These are the data structures that will be used for communication

// Post message within the channel
#[derive(Debug, Encode, TypeInfo, Clone)]
struct Message {
    text: String,
    timestamp: u32,
}

// Meta information about a channel
#[derive(Debug, Encode, TypeInfo)]
struct Meta {
    name: String,
    description: String,
    owner_id: H256,
}

// Actions that can possibly be proccessed by the contract
#[derive(Debug, Decode, TypeInfo)]
enum ChannelAction {
    Meta,
    ChannelFeed,
    Subscribe,
    Unsubscribe,
    Post(String),
}

// Return types that this contract supports
#[derive(Debug, Encode, TypeInfo)]
enum ChannelOutput {
    Metadata(Meta),
    SingleMessage(Message),
    MessageList(Vec<Message>),
}

// The state interface of the contract (memory)
struct State {
    channel_name: String,
    channel_description: String,
    owner_id: Option<ProgramId>,
    subscribers: Vec<ProgramId>,
    messages: Option<CircularBuffer<Message>>,
}

// Helper function to convert GSTD's ProgramId type to Hex
fn program_id_to_hex(program_id: ProgramId) -> H256 {
    let ProgramId(bytes) = program_id;
    return H256::from(bytes);
}

// Methods to be executed on state
impl State {
    fn set_owner_id(&mut self, user_id: ProgramId) {
        self.owner_id = Some(user_id);
    }

    fn add_subscriber(&mut self, subscriber_id: ProgramId) {
        self.subscribers.push(subscriber_id);
    }

    fn remove_subscriber(&mut self, subscriber_id: ProgramId) {
        let index = self
            .subscribers
            .iter()
            .position(|x| *x == subscriber_id)
            .expect("Subscriber doesn't exist.");
        self.subscribers.remove(index);
    }

    fn add_message(&mut self, message: Message) {
        self.messages.as_mut().unwrap().push(message);
    }
}

// 4️⃣ The state itself (i.e. the variable state will be accessed through)
static mut STATE: State = State {
    channel_name: String::new(),
    channel_description: String::new(),
    owner_id: None,
    subscribers: Vec::new(),
    messages: None,
};

// Reserve constant that is required to successfully terminate execution
const GAS_RESERVE: u64 = 100_000_000;

// 5️⃣ Init function that is executed once upon contract initialization
#[no_mangle]
pub unsafe extern "C" fn init() {
    // ⚠️ TODO: Fill in information about your channel
    STATE.channel_name = "???".to_string();
    STATE.channel_description = "???".to_string();
    // We are allocating a buffer of 5 messages in order to save memory in the contract
    STATE.messages = Some(CircularBuffer::new(5));
    STATE.set_owner_id(msg::source());

    // Alternative to a timestamp
    let bh: u32 = exec::block_height();

    // First message in the channel that declares its creation
    let init_message = Message {
        text: format!("Channel {} was created", STATE.channel_name).to_string(),
        timestamp: bh,
    };

    STATE.add_message(init_message);

    // First subscriber will be the author
    STATE.add_subscriber(STATE.owner_id.unwrap());
}

// 6️⃣ Handle function that processes the incoming message
#[no_mangle]
pub unsafe extern "C" fn handle() {
    // Decode an incoming action from the message
    let action: ChannelAction = msg::load().expect("Unable to decode Channel Action");
    let bh: u32 = exec::block_height();

    // Retreive sender's ID
    let source: ProgramId = msg::source();

    debug!("Received action: {:?}", action);

    // We will reuse a Message struct to respond to the sender with a success message
    let success_msg = Message {
        text: "success".to_string(),
        timestamp: 0,
    };

    // Switch structure based on the incoming action
    match action {
        ChannelAction::Meta => {
            let meta = Meta {
                name: STATE.channel_name.clone(),
                description: STATE.channel_description.clone(),
                owner_id: program_id_to_hex(STATE.owner_id.unwrap()),
            };

            debug!("Sending meta information: {:?}", meta);

            // Output has to be wrapped in type ChannelOutput::Metadata in order to be properly decoded
            msg::reply(
                ChannelOutput::Metadata(meta),
                exec::gas_available() - GAS_RESERVE,
                0,
            );
        }
        ChannelAction::ChannelFeed => {
            // convert CircularBuffer to a Vector
            let message_vector: Vec<Message> = STATE.messages.clone().unwrap().collect();

            debug!("Sending channel feed: {:?}", message_vector);

            // Output has to be wrapped in type ChannelOutput::MessageList in order to be properly decoded
            msg::reply(ChannelOutput::MessageList(message_vector), 0, 0);
        }
        ChannelAction::Subscribe => {
            // ⚠️ TODO: Add the subscriber
            // HINT: Subscriber is the one who sent the message to the contract, it is of type ProgramID

            debug!("Added a new subscriber: {:?}", source);

            // Output has to be wrapped in type ChannelOutput::SingleMessage in order to be properly decoded
            msg::reply(ChannelOutput::SingleMessage(success_msg), 0, 0);
        }
        ChannelAction::Unsubscribe => {
            STATE.remove_subscriber(source);

            debug!("Removed a subscriber: {:?}", source);

            // Output has to be wrapped in type ChannelOutput::SingleMessage in order to be properly decoded
            msg::reply(ChannelOutput::SingleMessage(success_msg), 0, 0);
        }
        ChannelAction::Post(text) => {
            // Make sure only the owner of the channel can add posts to it
            if source != STATE.owner_id.unwrap() {
                debug!("User not authorized to add a post: {:?}", source);
                return;
            }

            // Build a message struct from the input provided
            let message = Message {
                text: text,
                timestamp: bh,
            };

            // Send out notifications to the channel's subscribers
            for subscriber_id in STATE.subscribers.iter() {
                debug!("Sending a notification to: {:?}", &subscriber_id);

                msg::send(
                    *subscriber_id,
                    ChannelOutput::SingleMessage(message.clone()),
                    0,
                    0,
                );
            }

            STATE.add_message(message.clone());

            debug!("Added a new post to the channel.");

            msg::reply(ChannelOutput::SingleMessage(success_msg), 0, 0);
        }
    }
}
