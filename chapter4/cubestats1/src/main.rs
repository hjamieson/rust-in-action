#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
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

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
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

fn check_status(cube_sat: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
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
        CubeSat { id: sat_id }
    }
}

// returns the valid codesat identifiers.
fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));
    println!("{:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("{:?}", base_2);
    }

    println!("{:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq -= 42.21;
    println!("{:?}", base);
    println!("{:?}", base_3);
}
