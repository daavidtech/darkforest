#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaId(u32);

impl ArenaId {
    pub fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArenaRecord<T> {
    pub id: ArenaId,
    pub object: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arena<T> {
    next_id: u32,
    records: Vec<ArenaRecord<T>>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            records: Vec::new(),
        }
    }

    pub fn spawn(&mut self, object: T) -> ArenaId {
        let id = ArenaId(self.next_id);
        self.next_id += 1;
        self.records.push(ArenaRecord { id, object });
        id
    }

    pub fn records(&self) -> impl Iterator<Item = &ArenaRecord<T>> {
        self.records.iter()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}
