use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FSMState {
    pub state: String,
    pub doc: String,
    pub index: u8,
}
