// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-bounce-keys: bounce keys component model.
//!
//! Real implementation: keys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
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
pub enum BounceKeysError {
    EmptyKeyName,
    DuplicateKey(String),
    EmptyEventSubject,
}

impl fmt::Display for BounceKeysError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKeyName => write!(f, "key name must not be empty"),
            Self::DuplicateKey(id) => write!(f, "duplicate key {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BounceKeysError {}

pub struct BounceKeys {
    pub name: String,
    keys: BTreeMap<String, Key>,
    events: BTreeMap<String, Event>,
}

impl BounceKeys {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            keys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_key(&mut self, item: Key) -> Result<(), BounceKeysError> {
        if item.name.trim().is_empty() {
            return Err(BounceKeysError::EmptyKeyName);
        }
        let id = item.id.clone();
        if self.keys.contains_key(&id) {
            return Err(BounceKeysError::DuplicateKey(id.clone()));
        }
        self.keys.insert(id, item);
        Ok(())
    }

    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

    pub fn get_key(&self, id: &str) -> Option<&Key> {
        self.keys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BounceKeysError> {
        if event.subject.trim().is_empty() {
            return Err(BounceKeysError::EmptyEventSubject);
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

    fn sample_key(id: &str, name: &str) -> Key {
        Key {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = BounceKeys::new("BounceKeys");
        assert_eq!(m.name, "BounceKeys");
        assert_eq!(m.key_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_key() {
        let mut m = BounceKeys::new("BounceKeys");
        m.add_key(sample_key("a1", "Alpha")).unwrap();
        assert_eq!(m.key_count(), 1);
        assert!(m.get_key("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = BounceKeys::new("BounceKeys");
        assert!(m.add_key(sample_key("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = BounceKeys::new("BounceKeys");
        m.add_key(sample_key("x", "One")).unwrap();
        assert!(m.add_key(sample_key("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = BounceKeys::new("BounceKeys");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
