use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Mode {
    Initial,
    Refresh,
    Recovery(u64),
}
