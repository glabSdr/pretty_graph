use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn set(&mut self, k: &str, v: &str) {
        match self {
            Self::MapStrNodeBody(body) => {
                body.payload.insert(k.to_string(), v.to_string());
            },
            _ => panic!("set allowed for map nodes only")
        }
    }

    pub fn get(&self, k: &str) -> Option<String> {
        match self {
            Self::MapStrNodeBody(body) => {
                body.payload.get(k).cloned()
            },
            _ => panic!("get allowed for map nodes only")
        }
    }

    pub fn remove(&mut self, k: &str) -> Option<String> {
        match self {
            Self::MapStrNodeBody(body) => {
                body.payload.remove(k)
            },
            _ => panic!("remove allowed for map nodes only")
        }
    }

    pub fn keys(&self) -> Vec<String> {
        match self {
            Self::MapStrNodeBody(body) => {
                body.payload.keys().map(|k| k.clone()).collect()
            },
            _ => panic!("keys allowed for map nodes only")
        }
    }

    pub fn values(&self) -> Vec<String> {
        match self {
            Self::MapStrNodeBody(body) => {
                body.payload.values().cloned().collect()
            },
            _ => panic!("values allowed for map nodes only")
        }
    }
}
