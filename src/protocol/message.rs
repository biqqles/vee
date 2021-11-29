/*
 *  Copyright © 2021 James Robinson.
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

// Defines all valid protocol messages and their arguments.
#[derive(Serialize, Deserialize)] // allows serde to serialise each struct
pub enum Message {
    Hail {
        name: String,
        address: String,
    },
    Pair {
        originator: String,
        destination: String,
    },
    Link {
        origin_name: String,
        destination_name: String,
        address: String,
    },
    Fail {
        offender: String,
        explanation: String,
    },
    Data {
        payload: String,
    },
}

impl Message {
    pub fn serialise(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialise(encoded: String) -> Message {
        // Deserialise bytes to an instance of this struct.
        serde_json::from_str(&encoded).unwrap()
    }
}
