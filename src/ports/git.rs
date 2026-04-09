/// Version-control capability boundary.
///
/// Intent-focused contract for workflows that need repository metadata
/// without binding to a concrete git command implementation.
#[allow(dead_code)]
pub trait GitPort {
    type Error;

    fn current_branch(&self) -> Result<String, Self::Error>;
    fn changed_files(&self) -> Result<Vec<String>, Self::Error>;
    fn commit(&self, message: &str) -> Result<(), Self::Error>;
}
