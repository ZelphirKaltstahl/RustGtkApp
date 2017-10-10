pub struct VocStatusMessage {
    pub message: String,
    pub context: String
}

impl VocStatusMessage {
    pub fn new(message: String, context: String) -> VocStatusMessage {
        VocStatusMessage {
            message: message,
            context: context
        }
    }
}
