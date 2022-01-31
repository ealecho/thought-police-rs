//! All value objects and information that
//! belong to [Thought]s.

mod id;
mod title;

pub use self::{id::*, title::*};

/// Anything you wan to rememeber
#[derive(Debug, Clone)]
pub struct Thought {
    pub id: Id,
    pub title: Title,
}
