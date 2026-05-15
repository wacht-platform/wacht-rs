/// Server response from a binary download endpoint (e.g. board-item
/// filesystem `download` route). Body bytes are accompanied by the MIME
/// type and suggested filename pulled from `Content-Type` and
/// `Content-Disposition`.
#[derive(Debug, Clone)]
pub struct BinaryFileResponse {
    pub data: Vec<u8>,
    pub mime_type: Option<String>,
    pub file_name: Option<String>,
}
