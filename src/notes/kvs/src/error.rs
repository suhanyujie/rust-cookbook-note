pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serder_json::Error),

    #[fail(display = "key not found")]
    KeyNotFound,

}