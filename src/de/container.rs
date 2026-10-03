use super::Error;

/// Borrowed structural observations of one encoded value.
///
/// Inspection does not construct the decoded value. Container observations follow
/// their contents and describe immediate children, not all descendants. Text
/// chunks may split UTF-8 code points and may not outlive the callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inspection<'a> {
    TextChunk(&'a [u8]),
    /// An upper bound on the retained capacity of the ordinary decoded String.
    TextEnd {
        capacity_bound: usize,
    },
    Sequence {
        len: usize,
        size_hint: Option<usize>,
    },
    Map {
        len: usize,
    },
    TypedArray {
        len: usize,
        element_size: usize,
    },
}

/// The shallow shape of the next value. Typed arrays and scalar values are leaves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Leaf,
    Seq,
    Map,
}

/// The next child expected by an open container.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Slot {
    Element,
    Key,
    Value,
    End,
}

/// Nonborrowing container cursor for iterative decoding.
///
/// Complete children in order using [`super::Decoder::finish_child`]. A cursor contains
/// no source or decoded children. Discard the decoder after interrupted or failed
/// consumption; dropping a cursor does not consume the rest of its input.
#[derive(Debug)]
pub struct Container {
    kind: Kind,
    slot: Slot,
    len: usize,
    size_hint: Option<usize>,
}

impl Container {
    /// Construct framing state after a decoder has consumed the opening delimiter.
    pub fn new<E: Error>(kind: Kind, size_hint: Option<usize>, empty: bool) -> Result<Self, E> {
        let slot = match kind {
            Kind::Seq => Slot::Element,
            Kind::Map => Slot::Key,
            Kind::Leaf => return Err(E::custom("a leaf is not a container")),
        };
        Ok(Self {
            kind,
            slot: if empty { Slot::End } else { slot },
            len: 0,
            size_hint,
        })
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }
    pub fn slot(&self) -> Slot {
        self.slot
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn size_hint(&self) -> Option<usize> {
        self.size_hint
    }

    /// Advance after a decoder validates the delimiter following one child.
    pub fn advance<E: Error>(&mut self, ended: bool) -> Result<(), E> {
        match self.slot {
            Slot::Key if !ended => self.slot = Slot::Value,
            Slot::Element | Slot::Value => {
                self.len = self
                    .len
                    .checked_add(1)
                    .ok_or_else(|| E::custom("container size overflow"))?;
                self.slot = if ended {
                    Slot::End
                } else if self.kind == Kind::Map {
                    Slot::Key
                } else {
                    Slot::Element
                };
            }
            _ => return Err(E::custom("unexpected end of container")),
        }
        Ok(())
    }
}
