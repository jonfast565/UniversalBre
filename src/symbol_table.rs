use std::collections::HashMap;

pub struct SymbolRecord {
    id: String,
    offset: u8
}

pub struct SymbolTable {
    records: HashMap<String, SymbolRecord> //TODO: Use a map type if possible
}

impl SymbolTable {
    pub fn init() -> SymbolTable {
        SymbolTable {
            records: HashMap::<String, SymbolRecord>::new()
        }
    }

    pub fn add_record(&mut self, id: String, name: String, offset: u8) {
        self.records.insert(name, SymbolRecord{
            id: id,
            offset: offset
        });
    }
}

