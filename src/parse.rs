#[derive(Copy, Clone, Debug)]
pub enum Error {
    InputTooShort,
    InputTooLong,
    MalformedInput,
}
