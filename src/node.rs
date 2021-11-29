/*
 *  Copyright © 2021 James Robinson.
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use nanomsg::Protocol;

use crate::protocol::{Message, Transceiver};

pub struct Node {
    name: String,
    address: String,
    formation: Transceiver,
    registry: HashMap<String, String>, // { name => address }
}

impl Node {
    pub fn new(name: String, formation_address: String, own_address: String) -> Node {
        let formation = Transceiver::new(formation_address, Protocol::Bus);

        let mut registry = HashMap::new();
        registry.insert(name.clone(), own_address.clone());

        Node {
            name,
            address: own_address,
            formation,
            registry,
        }
    }

    pub fn is_broker(&self) -> bool {
        self.formation.bound
    }

    pub fn send_hail(&mut self) {
        println!("Sending Hail");
        self.formation.send_message(Message::Hail {
            name: self.name.clone(),
            address: self.address.clone(),
        });
    }

    pub fn send_pair(&mut self, destination: String) {
        println!("Requesting Pair with {}", destination);
        self.formation.send_message(Message::Pair {
            originator: self.name.clone(),
            destination,
        });
    }

    fn handle_pair(&self, originator: String, destination: String) -> Message {
        let origin_address = self.registry.get(&originator);

        match origin_address {
            None => Message::Fail {
                offender: originator,
                explanation: "Origin name not found in store".to_string(),
            },
            Some(address) => Message::Link {
                origin_name: originator,
                destination_name: destination,
                address: address.clone(),
            },
        }
    }

    pub(crate) fn check_for_message(&mut self) {
        println!("Checking for messages");
        let response = self.formation.receive_message();

        if response.is_none() {
            // no message was waiting
            return;
        }

        match response.unwrap() {
            Message::Hail { name, address } => {
                assert!(self.is_broker(), "Broker sent Hail?!");

                println!("Hail from node with name {}!", name);
                self.registry.insert(name, address);

                println!("Registry state: {:?}", self.registry);
            }

            Message::Pair {
                originator,
                destination,
            } => {
                assert!(self.is_broker(), "Broker sent Pair?!");

                println!(
                    "Pair requested with origin {} and destination {}!",
                    originator, destination
                );

                self.formation
                    .send_message(self.handle_pair(originator, destination));
            }

            Message::Fail {
                offender,
                explanation,
            } => {
                if offender == self.name {
                    eprintln!("Received Fail from broker! Explanation: {}", explanation);
                }
            }

            Message::Link {
                origin_name,
                destination_name,
                address,
            } => {
                let payload = if origin_name == self.name {
                    "Hello world from origin"
                } else if destination_name == self.name {
                    "Hello world from destination"
                } else {
                    return;
                }
                .to_string();

                let mut pair = Transceiver::new(address, Protocol::Pair);
                let reply = pair.send_receive(Message::Data { payload });

                match reply {
                    Message::Data { payload } => {
                        println!("Got data with payload '{}'", payload);
                    }
                    _ => {
                        eprintln!("Illegal message in pair")
                    }
                }
            }

            _ => {
                eprintln!("Illegal message in formation")
            }
        }
    }
}
