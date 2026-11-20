// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mdns: mdns component model.
//!
//! Real implementation: mdns_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;
pub mod sigmadnsmanager2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MdnsItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MdnsError {
    EmptyMdnsItemName,
    DuplicateMdnsItem(String),
    EmptyEventSubject,
}

impl fmt::Display for MdnsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMdnsItemName => write!(f, "mdns_item name must not be empty"),
            Self::DuplicateMdnsItem(id) => write!(f, "duplicate mdns_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MdnsError {}

pub struct Mdns {
    pub name: String,
    mdns_items: BTreeMap<String, MdnsItem>,
    events: BTreeMap<String, Event>,
}

impl Mdns {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mdns_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mdns_item(&mut self, item: MdnsItem) -> Result<(), MdnsError> {
        if item.name.trim().is_empty() {
            return Err(MdnsError::EmptyMdnsItemName);
        }
        let id = item.id.clone();
        if self.mdns_items.contains_key(&id) {
            return Err(MdnsError::DuplicateMdnsItem(id.clone()));
        }
        self.mdns_items.insert(id, item);
        Ok(())
    }

    pub fn mdns_item_count(&self) -> usize {
        self.mdns_items.len()
    }

    pub fn get_mdns_item(&self, id: &str) -> Option<&MdnsItem> {
        self.mdns_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MdnsError> {
        if event.subject.trim().is_empty() {
            return Err(MdnsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mdns_item(id: &str, name: &str) -> MdnsItem {
        MdnsItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Mdns::new("Mdns");
        assert_eq!(m.name, "Mdns");
        assert_eq!(m.mdns_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mdns_item() {
        let mut m = Mdns::new("Mdns");
        m.add_mdns_item(sample_mdns_item("a1", "Alpha")).unwrap();
        assert_eq!(m.mdns_item_count(), 1);
        assert!(m.get_mdns_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Mdns::new("Mdns");
        assert!(m.add_mdns_item(sample_mdns_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Mdns::new("Mdns");
        m.add_mdns_item(sample_mdns_item("x", "One")).unwrap();
        assert!(m.add_mdns_item(sample_mdns_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Mdns::new("Mdns");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
