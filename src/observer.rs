extern crate alloc;

use core::cell::RefCell;
use alloc::rc::Rc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;

/*
 * Modified implementation of observer pattern for propagating events
 */

pub type Event = String;

pub struct Listener {
    ledger: RefCell<Vec<Event>>
}

impl Listener {
    pub fn new() -> Listener {
        Self {
            ledger: RefCell::new(Vec::new())
        }
    }

    fn receive(&self, e: &Event) {
        // appends to end of ledger
        self.ledger.borrow_mut().push(e.clone());
    }

    pub fn poll_evt(&self) -> Vec<Event> {
        let ret = self.ledger.borrow().clone();
        self.ledger.borrow_mut().clear();
        return ret;
    }
}

pub struct Observable {
    name: String,   // will be used to give source information
    subscribers: BTreeMap<Event, Vec<Rc<Listener>>>
}

// todo: unsubscribe
impl Observable {
    pub fn new(name: String) -> Observable {
        Self {
            name: name,
            subscribers: BTreeMap::new()
        }
    }

    // Subscribe an Observer to an event
    pub fn subscribe(&mut self, evt: Event, subscriber: Rc<Listener>) {
        match self.subscribers.get_mut(&evt) {
            Some(vec) => vec.push(subscriber),
            None => {
                let mut new_vec = Vec::new();
                new_vec.push(subscriber);
                self.subscribers.insert(evt, new_vec);
            }
        };
    }

    // Notify all subscribers to the given Event
    pub fn notify(&self, evt: Event) {
        match self.subscribers.get(&evt) {
            Some(to_notify) => {
                // immutable iteration
                for s in to_notify {
                    s.receive(&evt);
                }
            },
            None => {}
        }
    }
}

// todo: unit tests
