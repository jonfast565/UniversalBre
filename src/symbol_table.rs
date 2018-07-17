use std::collections::HashMap;

pub struct SymbolRecord {
    id: String,
    offset: u8,
    scope: u8
}

pub struct SymbolTable {
    records: HashMap<String, SymbolRecord>, //TODO: Use a map type if possible
    offset: u8
}

impl SymbolTable {
    pub fn init() -> SymbolTable {
        SymbolTable {
            records: HashMap::<String, SymbolRecord>::new(),
            offset: 0
        }
    }

    pub fn add_record(&mut self, id: String, name: String, scope: u8) {
        self.records.insert(name, SymbolRecord{
            id: id,
            offset: self.offset,
            scope: scope
        });
        self.offset += 1;
    }

    pub fn search_record(&mut self, name: String) -> Option<&SymbolRecord> {
        self.records.get(&name)
    }
}

