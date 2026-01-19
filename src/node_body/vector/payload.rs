use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn push_str(&mut self, v: &str) {
        match self {
            Self::VecStrNodeBody(body) => {
                body.payload.push(v.to_string());
            },
            _ => panic!("push_str allowed for vector nodes only")
        }
    }

    pub fn get_string_by_index(&self, i: usize) -> Option<String> {
        match self {
            Self::VecStrNodeBody(body) => {
                body.payload.get(i).cloned()
            },
            _ => panic!("get_string allowed for vector nodes only")
        }
    }

    pub fn remove_string_by_index(&mut self, i: usize) -> String {
        match self {
            Self::VecStrNodeBody(body) => {
                body.payload.remove(i)
            },
            _ => panic!("remove_string allowed for vector nodes only")
        }
    }

    pub fn contains_string(&self, v: &String) -> bool {
        match self {
            Self::VecStrNodeBody(body) => {
                body.payload.contains(v)
            },
            _ => panic!("get_string allowed for vector nodes only")
        }
    }
}