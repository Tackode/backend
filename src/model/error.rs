use custom_error::custom_error;

custom_error! { pub Error
    LocalConnectionFailed{source: r2d2::Error} = "Unable to connect to local database ({source}).",
    DatabaseError{diesel_error: diesel::result::Error} = "Unable to run some operations on updatable model ({diesel_error}).",
    NotFound = "Not found.",
    NotFoundWithName{name: String} = "{name} not found.",
}

pub fn is_one(count: usize, name: &str) -> Result<(), Error> {
    if count == 1 {
        Ok(())
    } else {
        Err(Error::NotFoundWithName {
            name: String::from(name),
        })
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => Error::NotFound,
            _ => Error::DatabaseError {
                diesel_error: error,
            },
        }
    }
}
