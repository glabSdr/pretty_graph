use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn values(&self) -> Vec<String> {
        match self {
            Self::MapStrNodeBody(body) => body.payload.values().cloned().collect(),
            Self::VecStrNodeBody(body) => body.payload.clone()
        }
    }
}
