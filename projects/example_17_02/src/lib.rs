pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // このメソッドは記事のステートによらずに定義される（実装も `state` フィールドと関わりなし）
        // self.content.push_str(text);
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text);
    }

    pub fn content(&self) -> &str {
        // state の参照を `as_ref` で `Option<&Box<dyn State>>` に変換する
        //     as_ref については https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref を参照すること
        // これを `unwrap` で `&Box<dyn State>` にする（`state` が `None` ではありえないことが他のメソッドの定義からわかるので `unwrap` して問題ない）
        // `&Box<dyn State>` に `content` メソッドを呼び出すと、参照外し型強制が働くので `State` トレイトに実装された `content` メソッドが呼び出される
        // `as_ref` メソッドにより所有権関連のエラーが解消されているらしい：
        //     `unwrap` は所有権を奪うメソッドなので `self.state.unwrap()` しようとすると、参照であるはずの `self` の一部分の所有権を奪おうとしてしまう（`content` 関数が `%self` を引数に取っていることに注意）。
        //     しかし、当然これは許されていないのでコンパイルエラーを起こす
        //     そこで、`as_ref` を間に挟むと、`Option` を剥いても中身が参照なので参照の中身の所有権を奪おうとするという理不尽を解消できる
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // take については https://doc.rust-lang.org/std/option/enum.Option.html#method.take を参照せよ
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn add_text(&self, original_text: &str, _text_to_add: &str) -> String {
        original_text.to_string()
    }

    // State を参照して、`post.content` を返すか、空の文字列 "" を返すかどうか決める
    // デフォルト実装を追加しておくことで Draft と PendingReview 構造体での content の実装を省略する
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    // 状態を `PendingReview` に更新するメソッド
    // 現在の `State` を消費して新しい `State` を返す
    // `State` はトレイトなので、ここでは `Box<Self>` および `Box<dyn State>` を用いる
    // 受け取った引数を消費したいので、引数は `&self` ではなく `self: Box<Self>`
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn add_text(&self, original_text: &str, text_to_add: &str) -> String {
        // let mut s = original_text.to_string();
        // s.push_str(text_to_add);
        // s
        format!("{}{}", original_text, text_to_add)
    }

    // 状態が `Draft` なら何もせず `Box<PendingReview>` を返す
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingFirstReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingFirstReview {}

impl State for PendingFirstReview {
    // 状態が `PendingReview` ならそのまま `self` を返す（変更なし）
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingSecondReview {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct PendingSecondReview {}

impl State for PendingSecondReview {
    // 状態が `PendingReview` ならそのまま `self` を返す（変更なし）
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[test]
fn test() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    post.add_text("\nAnd a steak!");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("\nAnd dessert!");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("\nAnd coffee!");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("\nAnd coffee!");
    assert_eq!("I ate a salad for lunch today\nAnd a steak!", post.content());
}