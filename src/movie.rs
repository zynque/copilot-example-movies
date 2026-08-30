#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movie {
    id: usize,
    title: String,
    seen: bool,
}

impl Movie {
    pub(crate) fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            seen: false,
        }
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn seen(&self) -> bool {
        self.seen
    }

    pub(crate) fn toggle_seen(&mut self) {
        self.seen = !self.seen;
    }
}
