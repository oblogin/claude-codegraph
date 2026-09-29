//! `codegraph-core` — graph model types, file facts, module traits (`LanguageModule`,
//! `Resolver`, `SymbolIndex`), key builders and hash normalisation. No IO, no SQLite
//! (ADR-006 §1–§3, §6).
//!
//! Skeleton (F04-01): only the basic addressing types; the model and traits come
//! with the first language module.

use std::fmt;

/// Stable external address of a graph node, e.g. `cs:Assembly-CSharp:Game.Player`
/// (ADR-006 §2). Opaque for the agent; the internal row id never leaves the store.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(pub Box<str>);

impl Key {
    pub fn new(key: impl Into<Box<str>>) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// xxh3-128 digest (content, body, signature and interface hashes; ADR-006 §2).
pub type Hash128 = [u8; 16];

/// Position in a file: 1-based line; `col` counts UTF-8 bytes of the line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub line: u32,
    pub col: u32,
}

/// Source range of a declaration or reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_displays_as_its_string() {
        let key = Key::new("rs:ggez:graphics::canvas::Canvas::draw");
        assert_eq!(key.to_string(), "rs:ggez:graphics::canvas::Canvas::draw");
        assert_eq!(key.as_str(), key.0.as_ref());
    }
}
