# ２章：数当てゲームをプログラムする
## 目次
- [２章：数当てゲームをプログラムする](#２章数当てゲームをプログラムする)
  - [目次](#目次)
  - [2.1 create pj](#21-create-pj)
  - [2.2 main.rs の編集（予想を処理する）](#22-mainrs-の編集予想を処理する)
  - [2.3 秘密の数字を生成する](#23-秘密の数字を生成する)
    - [外部クレートを使用する](#外部クレートを使用する)
    - [`Cargo.lock` ファイルについて](#cargolock-ファイルについて)
    - [クレートのアップデート](#クレートのアップデート)
    - [rs ファイル内でクレートを利用してみる](#rs-ファイル内でクレートを利用してみる)
  - [2.4 予想と秘密の数字を比較する](#24-予想と秘密の数字を比較する)
  - [2.5 ループで複数回の予想を可能にする](#25-ループで複数回の予想を可能にする)

## 2.1 create pj
```sh
# projects directory
$ cargo new guessing_game --bin
```

## 2.2 main.rs の編集（予想を処理する）
1. edit `src/main.rs`
   ```rs
   use std::io;

   fn main() {
       println!("Guess the number!");
       println!("Please input your guess.");
       let mut guess = String::new();

       io::stdin().read_line(&mut guess)
           .expect("Failed to read line");
       println!("You guessed: {}", guess);
   }
   ```
   - `use`: スコープにライブラリを導入
   - `std`: 標準ライブラリ
   - `std::io`: 入出力用のライブラリ
   - `stdin` 関数は、`std::io::Stdin` オブジェクトを返し、この型は、ターミナルの標準入力へのハンドルを表す
   - `std::io::Stdin.read_line`: ユーザから入力を受け付け
   - `&mut guess`: 変数 `guess` の可変参照
   - Error handling について
     - `read_line` メソッドは `io::Result` 型を返す
     - `io::Result` オブジェクトには、呼び出し可能な `expect` メソッドがある
       - `expect` メソッドは、`io::Result` オブジェクトが `Err` 値の場合、プログラムをクラッシュさせ、引数として渡されたメッセージを表示します
       - `io::Result` オブジェクトが `Ok` 値の場合、`Ok` 列挙子が保持する返り値を取り出して、ただその値を返す
2. 最初の部分をテストする
   ```sh
   $ cargo run
   ```

## 2.3 秘密の数字を生成する
### 外部クレートを使用する
1. install `rand`
   ```sh
   # @ projects/guessing_game
   $ cargo add rand # rand は乱数生成用のライブラリクレート
   ```
   - 上記のコマンドで `Cargo.toml` が編集される 
     - `[dependencies]` に `rand = "0.8.5"` が追記される
     - `0.8.5` という数字は、実際には `^0.8.5` の省略記法で、これは、「バージョン 0.8.5 と互換性のある公開 API を持つ任意のバージョン」を意味します
2. ビルドしてみる
   ```sh
   $ cargo build
      Compiling libc v0.2.124
      Compiling cfg-if v1.0.0
      Compiling ppv-lite86 v0.2.16
      Compiling getrandom v0.2.6
      Compiling rand_core v0.6.3
      Compiling rand_chacha v0.3.1
      Compiling rand v0.8.5
      Compiling guessing_game v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/guessing_game)
       Finished dev [unoptimized + debuginfo] target(s) in 1.81s
   ```
   - Cargo はレジストリ (registry、登録所) から最新バージョンを拾ってきます
     - レジストリとは、Crates.io のデータのコピー
       - Crates.io とは、Rust のエコシステムにいる人間が、他の人が使えるように自分のオープンソースの Rust プロジェクトを投稿する場所
   - レジストリの更新後、Cargo は `[dependencies]` セクションをチェックし、まだ取得していないクレートを全部ダウンロードします
   - クレートのダウンロード完了後、コンパイラは依存ファイルをコンパイルし、依存が利用可能な状態でプロジェクトをコンパイルします。

### `Cargo.lock` ファイルについて
   - 再現可能なビルドを保証する
   - build 時に参照するライブラリクレートのバージョンを固定する

### クレートのアップデート
   ```sh
   $ cargo update
   ```
   - `Cargo.lock` の内容を無視して `Cargo.toml` 内のすべての指定に合致する最新バージョンを計算する
   - それがうまくいったら Cargo はそれらのバージョンを `Cargo.lock` に記述する
   - マイナーバージョン変更は `cargo update` で対応できるが、メジャーなアップデート対応するには、`Cargo.toml` を編集（`cargo-edit` を使うか直接編集する）する必要がある

### rs ファイル内でクレートを利用してみる
1. 乱数を生成する (`main.rs` を編集する) ([ref](https://docs.rs/rand/0.8.0/rand/trait.Rng.html#method.gen_range))
   ```rs
   use std::io;
   use rand::Rng;

   fn main() {
       println!("Guess the number!");

       let mut rng = rand::thread_rng();
       let secret_number:u32 = rng.gen_range(1..101);
       println!("Secret number is: {}", secret_number);

       println!("Please input your guess.");
       let mut guess = String::new();

       io::stdin()    //  stdin 関数は、std::io::Stdin オブジェクトを返し、この型は、ターミナルの標準入力へのハンドルを表す
           .read_line(&mut guess)    // ユーザから入力を受け付け
           .expect("Failed to read line");    // read_line メソッドは io::Result 型を返す. io::Result` オブジェクトが `Err` 値の場合、`expect` メソッドはプログラムをクラッシュさせ、引数として渡されたメッセージを表示します.
       println!("You guessed: {}", guess);
   }
   ```
   - `use rand::Rng;`: random number generator 用のトレイトの導入
   - `rand::thread_rng`: 乱数生成器を返す関数
     - 乱数生成器は、実行スレッドに固有で、OS により、シード値を与えられています
   - `gen_range`: Rng トレイトで定義されている
   - 各クレートの使用方法は各クレートのドキュメントを確認する必要がある (以下のコマンドでドキュメントをブラウザから確認できる)
      ```sh
      $ cargo doc --open
      ```

## 2.4 予想と秘密の数字を比較する
1. main.rs を編集
   ```rs
   use std::io;
   use std::cmp::Ordering;
   use rand::Rng;

   fn main() {
       println!("Guess the number!");

       let mut rng = rand::thread_rng();
       let secret_number:u32 = rng.gen_range(1..101);
       println!("Secret number is: {}", secret_number);

       println!("Please input your guess.");
       let mut guess = String::new();

       io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");
       
       println!("You guessed: {}", guess);

       let guess: u32 = guess.trim().parse()
           .expect("Please type a number!");

       match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too large!"),
           Ordering::Equal => println!("You win!")
       }
   }
   ```
   - `use std::cmp::Ordering;`: cmp::Ordering クレートを導入
     - `cmp` メソッドの返り値を取り扱う際に使用する型
   - `cmp` メソッドは、2 値を比較し、比較できるものに対してなら何に対しても呼び出せます
     - このメソッドは、比較したいものへの参照を取ります: ここでは、`guess` 変数と `secret_number` 変数を比較しています
     - このメソッドは `Ordering` 列挙型の値を返します
     - match 式を使用して、`guess` 変数と `secret_number` を `cmp` に渡して返ってきた `Ordering` の列挙子に基づき、次の動作を決定
   - `trim`: 両端の空白をすべて除去
   - `parse`: 文字列を解析して何らかの数値にします。
     - 返り値が `Result` 型なので `expect` メソッドを呼び出して、エラーを吐いたらパニックを起こすようにしている
       - 例えば、この例では数字以外を入力するとパニックが起こってプログラムが停止する
   - 変数 `guess` は `let` で二回宣言されている
     - この二度目の宣言の後では新しい宣言の方が優先され、前宣言は無視されるようになる
     - このように Rust では新しい値で変数を **覆い隠す** (shadow) ことが許されている
     - この機能は、値を別の型に変換したいシチュエーションでよく使われます。
       - シャドーイング (shadowing) のおかげで別々の変数を 2 つ作らされることなく、変数名を再利用することができる
2. 動作チェック
   ```sh
   $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/guessing_game`
   Guess the number!
   Secret number is: 35
   Please input your guess.
   21
   You guessed: 21

   Too small!
   ```

## 2.5 ループで複数回の予想を可能にする
1. `loop` キーワードは、無限ループを作り出します。これを追加して、ユーザが何回も予想できるようにしましょう:
   ```diff
   use std::io;
   use std::cmp::Ordering;
   use rand::Rng;

   fn main() {
       println!("Guess the number!");

       let mut rng = rand::thread_rng();
       let secret_number:u32 = rng.gen_range(1..101);
       println!("Secret number is: {}", secret_number);

   +   loop {
           println!("Please input your guess.");
           let mut guess = String::new();
       
           io::stdin()
               .read_line(&mut guess)
               .expect("Failed to read line");
           
           println!("You guessed: {}", guess);
       
           let guess: u32 = guess.trim().parse()
               .expect("Please type a number!");
       
           match guess.cmp(&secret_number) {
               Ordering::Less => println!("Too small!"),
               Ordering::Greater => println!("Too large!"),
   -           Ordering::Equal => println!("You win!")
   +           Ordering::Equal => {
   +               println!("You win!");
   +               break;
   +           }
           }
   +   }
   }
   ```

2. 不正な入力を処理する機能を追加する
   - `Result` 型を `expect` メソッドで処理する代わりに、`match` 式で条件分岐して処理する：
   ```diff
   use rand::Rng;
   use std::cmp::Ordering;
   use std::io;

   fn main() {
       println!("Guess the number!");

       let mut rng = rand::thread_rng(); // gen_range メソッドの第一引数は &mut self なので mutable として定義する必要がある
       let secret_number: u32 = rng.gen_range(1..101);
       println!("Secret number is: {}", secret_number);

       loop {
           println!("Please input your guess.");
           let mut guess = String::new();

           io::stdin() //  stdin 関数は、std::io::Stdin オブジェクトを返し、この型は、ターミナルの標準入力へのハンドルを表す
               .read_line(&mut guess) // ユーザから入力を受け付け
               .expect("Failed to read line"); // read_line メソッドは io::Result 型を返す. io::Result` オブジェクトが `Err` 値の場合、`expect` メソッドはプログラムをクラッシュさせ、引数として渡されたメッセージを表示します.
   -       let guess: u32 = guess.trim().parse()
   -           .expect("Please type a number!");
   +       let guess: u32 = match guess.trim().parse() {
   +           Ok(num) => num,
   +           Err(_) => {
   +               println!("Please enter a number!");
   +               continue;
   +           }
   +       };
           println!("You guessed: {}", guess);

           match guess.cmp(&secret_number) {
               Ordering::Less => println!("Too small!"),
               Ordering::Greater => println!("Too large!"),
               Ordering::Equal => {
                   println!("You win!");
                   break;
               }
           }
       }
   }
   ```

3. 最後に `secret_number` の表示の部分をコメントアウトして完成
   ```sh
   $ cargo run
      Compiling guessing_game v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/guessing_game)
       Finished dev [unoptimized + debuginfo] target(s) in 0.27s
        Running `target/debug/guessing_game`
   Guess the number!
   Please input your guess.
   50
   You guessed: 50
   Too large!
   Please input your guess.
   25
   You guessed: 25
   Too large!
   Please input your guess.
   12
   You guessed: 12
   Too large!
   Please input your guess.
   6
   You guessed: 6
   You win!
   ```