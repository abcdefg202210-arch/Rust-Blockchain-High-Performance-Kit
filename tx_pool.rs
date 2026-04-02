use crate::transaction::Tx;

pub struct TxPool {
    pending: Vec<Tx>,
}

impl TxPool {
    pub fn new() -> Self {
        TxPool { pending: Vec::new() }
    }

    pub fn add(&mut self, tx: Tx) {
        self.pending.push(tx);
    }

    pub fn take_all(&mut self) -> Vec<Tx> {
        std::mem::take(&mut self.pending)
    }
}
