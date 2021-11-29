/*
 *  Copyright © 2021 James Robinson.
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::io::{Read, Write};

use nanomsg::{Protocol, Socket};

use crate::protocol::Message;

// This transceiver represents an underlying socket that allows data to be transmitted and received.
// It serves to abstract away the underlying protocol/library (for example ZMQ or nanomsg).
pub struct Transceiver {
    socket: Socket,
    pub bound: bool,
}

impl Transceiver {
    pub fn new(address: String, protocol: Protocol) -> Transceiver {
        // Create socket with designated protocol and set reception timeout (in ms). This is the
        // time a socket will block for if it does not receive a message.
        let mut socket = Socket::new(protocol).unwrap();
        socket.set_receive_timeout(100).unwrap(); // maybe make this a parameter in future

        // An address can be bound to only once, but connected to many times. If attempting to bind
        // to an address that is already bound to, an "address in use" error will be raised. So,
        // detect this and try to connect to the address instead. If this too fails, allow the error
        // to propagate.
        let bound = match socket.bind(&address) {
            Ok(_) => {
                // bound
                true
            }
            Err(_) => {
                socket.connect(&address).unwrap();
                // connected
                false
            }
        };

        Transceiver { socket, bound }
    }

    fn send(&mut self, string: String) {
        // Send a string on the socket.
        self.socket.write(string.as_ref()).unwrap();
    }

    pub fn send_message(&mut self, message: Message) {
        // Send a message on the socket.
        self.send(message.serialise());
    }

    fn receive(&mut self) -> String {
        // Attempt to receive a message in serialised string form. If one is waiting, return the string.
        // Otherwise, return an empty string.
        let mut buffer = String::new();
        let message = self.socket.read_to_string(&mut buffer);

        match message {
            Ok(_) => buffer,
            Err(_) => "".to_string(),
        }
    }

    pub fn receive_message(&mut self) -> Option<Message> {
        // Attempt to receive a message. If one is waiting, return the message. Otherwise return None.
        let received = self.receive();
        if received.is_empty() {
            None
        } else {
            Some(Message::deserialise(received))
        }
    }

    pub fn send_receive(&mut self, send: Message) -> Message {
        // Send a request, and await a response, returning this message.
        self.send_message(send);
        self.receive_message().unwrap()
    }
}
