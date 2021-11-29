/*
 *  Copyright © 2021 James Robinson.
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::env::args;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

mod node;
mod protocol;
use crate::node::Node;

fn main() {
    let mut args: Vec<String> = args().collect();
    if args.len() <= 4 {
        args.push("".to_string());
    }

    if let [_, name, formation_address, own_address, pair_to] = &args[..] {
        let mut node = Node::new(name.clone(), formation_address.clone(), own_address.clone());

        loop {
            node.check_for_message();
            if node.is_broker() {
                sleep(Duration::from_millis(500));
            } else {
                node.send_hail();
                sleep(Duration::from_millis(2000));
            }

            if !pair_to.is_empty() {
                node.send_pair(pair_to.clone());
            }
        }
    } else {
        eprintln!("Usage: vee NAME FORMATION_ADDRESS OWN_ADDRESS [PAIR_TO]");
        exit(2);
    }
}
