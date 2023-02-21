pub enum FileCreateResult<> {
    OK(),
    FileExists(),
    Error(String),
}
