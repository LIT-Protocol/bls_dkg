// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use super::encryptor::{Iv, Key};
use super::mode::Mode;
use super::sharexorname::ShareXorName;
use super::{Acknowledgment, Part};
use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use xor_name::XorName;

/// Messages used for running BLS DKG.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum Message {
    Initialization {
        key_gen_id: u64,
        context: ShareXorName,
        m: usize,
        n: usize,
        member_list: BTreeSet<XorName>,
        mode: Mode,
    },
    Proposal {
        key_gen_id: u64,
        context: ShareXorName,
        part: Part,
    },
    Complaint {
        key_gen_id: u64,
        target: u64,
        context: ShareXorName,
        msg: Vec<u8>,
    },
    Justification {
        key_gen_id: u64,
        context: ShareXorName,
        keys_map: BTreeMap<XorName, (Key, Iv)>,
    },
    Acknowledgment {
        key_gen_id: u64,
        context: ShareXorName,
        ack: Acknowledgment,
    },
}

impl fmt::Debug for Message {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Message::Initialization {
                key_gen_id,
                member_list,
                ..
            } => write!(
                formatter,
                "Initialization({:?} - {:?})",
                member_list, key_gen_id
            ),
            Message::Proposal { key_gen_id, .. } => write!(formatter, "Proposal({})", key_gen_id),
            Message::Complaint {
                key_gen_id, target, ..
            } => write!(formatter, "Complaint({} - {})", key_gen_id, target),
            Message::Justification { key_gen_id, .. } => {
                write!(formatter, "Justification({})", key_gen_id)
            }
            Message::Acknowledgment { key_gen_id, .. } => {
                write!(formatter, "Acknowledgment({})", key_gen_id)
            }
        }
    }
}

impl Message {
    pub fn get_context(&self) -> &ShareXorName {
        match &self {
            Message::Initialization {
                key_gen_id: _,
                context,
                m: _,
                n: _,
                member_list: _,
                mode: _,
            } => context,
            Message::Proposal {
                key_gen_id: _,
                context,
                part: _,
            } => context,
            Message::Complaint {
                key_gen_id: _,
                target: _,
                context,
                msg: _,
            } => context,
            Message::Justification {
                key_gen_id: _,
                context,
                keys_map: _,
            } => context,
            Message::Acknowledgment {
                key_gen_id: _,
                context,
                ack: _,
            } => context,
        }
    }
    pub fn get_epoch(&self) -> u64 {
        self.get_context().epochid
    }
}
