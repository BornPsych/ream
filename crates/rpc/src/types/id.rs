use core::fmt;
use std::{fmt::Display, str::FromStr};

use alloy_primitives::{B256, hex};
use ream_bls::PubKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ID {
    Finalized,
    Genesis,
    Head,
    Justified,
    Slot(u64),
    /// expected to be a 0x-prefixed hex string.
    Root(B256),
}

impl FromStr for ID {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "finalized" => Ok(ID::Finalized),
            "genesis" => Ok(ID::Genesis),
            "head" => Ok(ID::Head),
            "justified" => Ok(ID::Justified),
            _ => {
                if s.starts_with("0x") {
                    B256::from_str(s)
                        .map(ID::Root)
                        .map_err(|err| format!("Invalid hex root: {err}"))
                } else {
                    s.parse::<u64>()
                        .map(ID::Slot)
                        .map_err(|err| format!("Invalid slot: {err}"))
                }
            }
        }
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ID::Finalized => write!(f, "finalized"),
            ID::Genesis => write!(f, "genesis"),
            ID::Head => write!(f, "head"),
            ID::Justified => write!(f, "justified"),
            ID::Slot(slot) => write!(f, "{slot}"),
            ID::Root(root) => write!(f, "0x{}", hex::encode(root)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidatorID {
    Index(u64),
    /// expected to be a 0x-prefixed hex string.
    Address(PubKey),
}

impl FromStr for ValidatorID {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            PubKey::from_str(s)
                .map(ValidatorID::Address)
                .map_err(|err| format!("Invalid hex root: {err}"))
        } else {
            s.parse::<u64>()
                .map(ValidatorID::Index)
                .map_err(|err| format!("Invalid slot: {err}"))
        }
    }
}

impl Display for ValidatorID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidatorID::Index(i) => write!(f, "{i}"),
            ValidatorID::Address(pub_key) => write!(f, "0x{:?}", pub_key.to_bytes()),
        }
    }
}
