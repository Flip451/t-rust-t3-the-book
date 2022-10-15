pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    } 
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewFirstPost {
        PendingReviewFirstPost { content: self.content }
    }
}

pub struct PendingReviewFirstPost {
    content: String,
}

impl PendingReviewFirstPost {
    pub fn approve(self) -> PendingSecondReviewPost {
        PendingSecondReviewPost { content: self.content }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

pub struct PendingSecondReviewPost {
    content: String,
}

impl PendingSecondReviewPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

