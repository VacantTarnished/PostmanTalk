#[derive(Responder)]
#[response(status = 200, content_type = "application/json")]
pub struct JsonResponder {
    pub data: String,
}