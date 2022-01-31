use crate::{
    gateway::respository::thought::{Record,Repo,SaveError},
    identifier::{NewId,NewIdError},
    usecase::thought::validate::{validate_thought, ThoughtInvalidity},

};
use domain::thought::{Id, Thought, Title};
use thiserror::Error;


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

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    Invalidity(#[from] ThoughtInvalidity),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl<'r, 'g, R, G> CreateThought<'r, 'g, R, G>
where
    R: Repo,
    G: NewId<Id>
    
    {
        /// Create a new thought with the given title
        pub fn exec(&self, req: Request) -> Result<Response, Error> {
            log::debug!("Create new thought: {:?}", req);
            let title = Title::new(req.title);
            let id = self.id_gen.new_id().map_err(|err|{
                log::warn!("{}", err);
                Error::NewId
            })?;

            let thought = Thought{id, title};
            validate_thought(&thought)?;
            let record = Record {thought};
            self.repo.save(record)?;
            Ok(Response{id})
        }
    }