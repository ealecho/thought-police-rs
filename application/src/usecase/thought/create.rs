use crate::{
    gateway::respository::thought::{Record,Repo,SaveError}

};
use domain::thought::{Id, Thought, Title};


#[derive(Debug)]
pub struct Request {
    /// The title of new thought.
    pub title: String,
}

#[derive(Debug)]
pub struct Response {
    /// The ID of the newly created thought
    pub id: Id,
}


/// Create thought usecase interator
pub struct CreateThought<'r, 'g, R, G> {
    repo: &'r R,
    id_gen: &'g G,
}

impl<'r, 'g, R, G> CreateThought<'r, 'g, R, G> {
    pub fn new(repo: &'r R, id_gen: &'g G) -> Self {
        Self {repo, id_gen}
    }
}

