use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn values(&self) -> Vec<String> {
        match self {
            Self::MapStrNodeBody(body) => body.payload.values().cloned().collect(),
            Self::VecStrNodeBody(body) => body.payload.clone()
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::MapStrNodeBody(body) => body.payload.clear(),
            Self::VecStrNodeBody(body) => body.payload.clear()
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::MapStrNodeBody(body) => body.payload.len(),
            Self::VecStrNodeBody(body) => body.payload.len()
        }
    }

    pub fn contains_string(&self, v: &String) -> bool {
        match self {
            Self::MapStrNodeBody(body) => {
                for exist_str in body.payload.values() {
                    if exist_str == v {
                        return true;
                    }
                }

                false
            },
            Self::VecStrNodeBody(body) => {
                body.payload.contains(v)
            }
        }
    }
}