#![allow(unused_variables, dead_code)]
use std::rc::Rc;
use std::cell::RefCell;

// #[derive(Debug)]
// enum StatusMessage {
//     Ok,
// }

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id, mailbox: Mailbox { messages: vec![] } }
    }
}

// fn check_status(sat_id: CubeSat) -> CubeSat {
//     println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
//     sat_id
// }

fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(
        RefCell::new(
            GroundStation {
                radio_freq: 87.65
            }
        )
    );

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;
    
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}