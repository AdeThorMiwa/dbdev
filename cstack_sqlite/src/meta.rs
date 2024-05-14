pub mod handlers {
    pub enum MetaHandleError {
        UnrecognisedCommand,
    }

    pub fn handle(_input: &str) -> Result<(), MetaHandleError> {
        Err(MetaHandleError::UnrecognisedCommand)
    }
}
