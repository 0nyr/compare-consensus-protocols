use serde::{Serialize, Serializer};

pub struct Transaction {
    pub sender: String, // address or initial block reward hash
    pub recipient: String,
    pub amount: u64
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Transaction {
        Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount
        }
    }
}

// serde info: https://serde.rs/impl-serialize.html
// implement serialize and deserialize for Transaction
impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Transaction", 3)?;
        state.serialize_field("sender", &self.sender)?;
        state.serialize_field("recipient", &self.recipient)?;
        state.serialize_field("amount", &self.amount)?;
        state.end()
    }
}