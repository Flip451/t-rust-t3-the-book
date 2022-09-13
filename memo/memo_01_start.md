# １章
## 目次
- [１章](#１章)
  - [目次](#目次)
  - [1.1 Install](#11-install)
    - [install (cargo もインストールされる)](#install-cargo-もインストールされる)
    - [update](#update)
    - [Rust と rustup をアンインストールする](#rust-と-rustup-をアンインストールする)
    - [version](#version)
    - [ローカルのドキュメンテーション](#ローカルのドキュメンテーション)
  - [1.2 Hello, world!](#12-hello-world)
  - [1.3 Hello, Cargo!](#13-hello-cargo)
    - [version](#version-1)
    - [Cargo でプロジェクトを作成する](#cargo-でプロジェクトを作成する)
    - [Cargo プロジェクトをビルドし、実行する （`cargo build`）](#cargo-プロジェクトをビルドし実行する-cargo-build)
    - [build と実行を一緒くたに行う（`cargo run`）](#build-と実行を一緒くたに行うcargo-run)
    - [コンパイル可能かチェックする（`cargo check`）](#コンパイル可能かチェックするcargo-check)
    - [リリースビルドを行う（`cargo build --release`）](#リリースビルドを行うcargo-build---release)

## 1.1 Install
### install (cargo もインストールされる)
```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Rust is installed now. Great!
```

### update
```sh
$ rustup update
```

### Rust と rustup をアンインストールする
```sh
$ rustup self uninstall
```

### version
```sh
$ rust --version
```

### ローカルのドキュメンテーション
```sh
$ rustup doc
```
- デフォルトでは動作しなかったので調査・対応
- [WSL2でRustドキュメントを開く](https://osanshouo.github.io/blog/2021/04/09-rustdoc-wls/) を参考に対応
  - `/home/flip451/settings/browser.sh` を作成
     ```sh
     #! /bin/bash
     WINPATH="file:///$(wslpath -m ${1})"
     /mnt/c/Program\ Files/Google/Chrome/Application/chrome.exe $WINPATH
     ```
  - `~/.bashrc` に以下の行を追加
     ```bashrc
     export BROWSER=/home/flip451/settings/browser.sh
     ```
  - VSCode のターミナルからの呼び出しにうまく反応しなかったので、設定から `Open-in-browser: Default` に `/home/flip451/settings/browser.sh` を設定した

## 1.2 Hello, world!
1. mkdir
   ```sh
   $ mkdir ~/projects
   $ cd $_
   $ mkdir hello_world
   $ cd $_
   ```

2. touch `main.rs`
   ```rs
   fn main() {
       println!("Hello, world!");
   }
   ```

3. execute
   ```sh
   $ rustc main.rs   # コンパイル
   $ ./main          # 実行
   Hello, world!
   ```

4. コード規約
   - Rust のスタイルは、タブではなく、4 スペースでインデントする

## 1.3 Hello, Cargo!
1. cargo は、ビルドシステム兼パッケージマネージャ
   1. コードのビルドやコードが依存しているライブラリをダウンロードし、それらのライブラリをビルドする

### version
```sh
$ cargo --version
cargo 1.60.0 (d1fd9fe 2022-03-01)
```

### Cargo でプロジェクトを作成する
```sh
# projects ディレクトリ内
$ cargo new hello_cargo --bin   # hello_cargo という新しいバイナリの実行可能ファイルを作成
```
- Cargo を使用すると、プロジェクトを体系化する手助けをしてくれ
ます。
  - Cargo は、ソースファイルが `src` ディレクトリにあることを期待します。
  - プロジェクトの最上位のディレクトリは、README ファイル、ライセンス情報、設定ファイル、あるいは、他のコードに関連しないもののためのもの

### Cargo プロジェクトをビルドし、実行する （`cargo build`）
```sh
# projects/hello_cargo ディレクトリ
$ cargo build
# --> projects/hello_cargo 内に target ディレクトリが作成される

$ ./target/debug/hello_corgo     # 実行
Hello, world!
```
- ビルドの結果をコードと同じディレクトリに保存するのではなく、Cargo は `target/debug` ディレクトリに格納する
- 初めて `cargo build` を実行すると、Cargo が最上位に `Cargo.lock` も作成します
- このファイルは、自分のプロジェクトの依存の正確なバージョンを追いかけます
- 絶対にこのファイルを手動で変更する必要はない; Cargo が中身を管理してくれる

### build と実行を一緒くたに行う（`cargo run`）
```sh
# projects/hello_cargo ディレクトリ
$ cargo run
   Finished dev [unoptimized + debuginfo] target(s) in 0.00s
    Running `target/debug/hello_cargo`
Hello, world!
```

### コンパイル可能かチェックする（`cargo check`）
```sh
$ cargo check
Checking hello_cargo v0.1.0 (/home/flip451/Oniwa/tutorial/t-rust/t3-the-book/projects/hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 0.08s
```
- コンパイルできることを確かめますが、実行可能ファイルは生成しません

### リリースビルドを行う（`cargo build --release`）
```sh
$ cargo build --release
```
- このコマンドは、`target/debug` ではなく、`target/release` に実行可能ファイルを作成します
