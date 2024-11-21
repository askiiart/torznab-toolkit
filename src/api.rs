use rocket::get;

#[get("/api?t=caps")]
pub(crate) fn caps() -> Result<String, String> {}
