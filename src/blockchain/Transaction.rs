pub struct Transaction {
    pub sender: String, // address or initial block reward hash
    pub recipient: String,
    pub amount: u64
}

imp Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Transaction {
        Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount
        }
    }
}

// implement serialize and deserialize for Transaction
impl Serialize for Transaction {
    fn serialize(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut s = s.serialize_struct("Transaction", 3)?;
        s.serialize_field("sender", &self.sender)?;
        s.serialize_field("recipient", &self.recipient)?;
        s.serialize_field("amount", &self.amount)?;
        s.end()
    }
}