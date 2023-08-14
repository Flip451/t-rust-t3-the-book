# ５章

## 目次

- [５章](#５章)
  - [目次](#目次)
  - [5.0 概要](#50-概要)
  - [5.1 定義・インスタンス化・フィールドへのアクセス](#51-定義インスタンス化フィールドへのアクセス)
    - [構造体の基本](#構造体の基本)
    - [タプル構造体](#タプル構造体)
    - [ユニット様 (よう) 構造体](#ユニット様-よう-構造体)
    - [構造体と所有権](#構造体と所有権)
  - [5.2 構造体を使ったプログラム例](#52-構造体を使ったプログラム例)
    - [長方形の面積を求める簡単なプログラム例](#長方形の面積を求める簡単なプログラム例)
    - [トレイトの導出で有用な機能を追加](#トレイトの導出で有用な機能を追加)
  - [5.3 メソッド記法](#53-メソッド記法)
    - [メソッドの基本](#メソッドの基本)
    - [自動参照および自動参照外し](#自動参照および自動参照外し)
    - [より引数の多いメソッドの例](#より引数の多いメソッドの例)
    - [関連関数](#関連関数)

## 5.0 概要

- struct (構造体)
- メソッド
- 関連関数

## 5.1 定義・インスタンス化・フィールドへのアクセス

### 構造体の基本

- 定義

  ```rust
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }
  ```

- インスタンス化

  ```rust
  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };
  ```

- 構造体インスタンスの特定のフィールドにアクセスする

  ```rust
  println!("user1.username: {}", user1.username); // user1.username: someusername123
  ```

- 可変性と値の代入
  - 構造体の一部のフィールドのみを可変にすることはできない (インスタンス全体が可変でなければならない)

  ```rust
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.com");
  ```

- 初期化省略記法

  ```rust
  fn build_user(email: String, username: String) -> User {
    User {
      email,    // email: email と書かずに済む
      username, // username: username と書かずに済む
      active: true,
      sign_in_count: 1,
    }
  }
  ```

- 構造体更新記法で他のインスタンスからインスタンスを生成

  ```rust
  // 新しい User インスタンス用の値に新しい email と
  // username をセットしつつ、残りの値は、user1 変数のフィールド値を使う
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };
  ```

### タプル構造体

- **タプル全体に命名**し、そのタプルを**他のタプルとは異なる型に**したい場合に有用
- 定義・インスタンス化

  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  ```

### ユニット様 (よう) 構造体

- 一切フィールドのない構造体
- ユニット型 `()` と似た振る舞いをする
- ある型に**トレイトを実装するが型自体に保持させるデータはない**場面に有用

### 構造体と所有権

- 構造体のインスタンスには全データを所有してもらう必要があり
- 構造体全体が有効な間は、各フィールドに対応するデータがずっと有効である必要がある
- 構造体に、他の何かに所有されたデータへの参照を保持させることもできるが、そうするには**ライフタイム**という機能を使用しなければならない（１０章でみる）
- ライフタイムを指定せずに構造体に参照を保持させようとするとうまく動かない
  - たとえば、次のコードはコンパイルエラーを起こす (コンパイラは、ライフタイム指定子が必要だと怒る)：

    ```rust
    struct User {
      username: &str,
      email: &str,
      sign_in_count: u64,
      active: bool,
    }

    fn main() {
      let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
      };
    }
    ```

## 5.2 構造体を使ったプログラム例

### 長方形の面積を求める簡単なプログラム例

  ```rust
  // 型を定義
  struct Rectangle {
      width: u32,
      height: u32,
  }

  fn main() {
      // インスタンス化
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };

      println!("The area of rect1 is {} square pixels.", area(&rect1));
  }

  // 長方形の面積を求める関数
  fn area(rectangle: &Rectangle) -> u32 {
      rectangle.width * rectangle.height
  }
  ```

### トレイトの導出で有用な機能を追加

- 構造体の定義に `#[derive(Debug)]` という注釈を追記することで、その構造体を `{:?}` や `{:#?}` でフォーマットできるようになる
- 例：

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };

      println!("rect1 is {:?}", rect1);
      println!("rect1 is {:#?}", rect1);
  }
  ```

  ```sh
  rect1 is Rectangle { width: 30, height: 50 }
  rect1 is Rectangle {
      width: 30,
      height: 50,
  }
  ```

## 5.3 メソッド記法

### メソッドの基本

- 構造体の文脈 (あるいは enum かトレイトオブジェクト (cf. 17 章) の文脈) で定義される
- 最初の引数は必ず `self`

- 定義 (`impl` キーワードを使用)

  ```rust
  struct Rectangle {
    width: u32,
    height: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }
  ```

- 呼び出し

  ```rust
  let rect1 = Rectangle {
      width: 30,
      height: 50,
  };

  println!("The area of rect1 is {} square pixels.", rect1.area());
  ```

### 自動参照および自動参照外し

- Rust には、**自動参照および自動参照外し**という機能がある
- メソッド呼び出しは、この動作が行われる数少ない箇所
- `object.something()` とメソッドを呼び出すと `object` の型がメソッドの第一引数の型と合致するように、自動で `&` か `&mut` 、`*` が付与される
- 例：以下はすべて同じ動作をする

  ```rust
  println!("The area of rect1 is {} square pixels.", rect1.area());
  println!("The area of rect1 is {} square pixels.", &rect1.area());
  println!("The area of rect1 is {} square pixels.", &&rect1.area());
  println!("The area of rect1 is {} square pixels.", &&&rect1.area());
  println!("The area of rect1 is {} square pixels.", &&&&rect1.area());
  ```

### より引数の多いメソッドの例

- ある長方形がほかの長方形の中に含まれるか否かを検証するメソッドを作成する

  ```rust
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }

      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }

  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
      let rect2 = Rectangle {
          width: 10,
          height: 40,
      };
      let rect3 = Rectangle {
          width: 60,
          height: 45,
      };

      println!("rect1 is {:#?}", rect1);
      println!("rect2 is {:#?}", rect2);
      println!("rect3 is {:#?}", rect3);

      println!("Rect1 can hold rect2.: {}", rect1.can_hold(&rect2));
      println!("Rect1 can hold rect3.: {}", rect1.can_hold(&rect3));
  }
  ```

  ```sh
  rect1 is Rectangle {
      width: 30,
      height: 50,
  }
  rect2 is Rectangle {
      width: 10,
      height: 40,
  }
  rect3 is Rectangle {
      width: 60,
      height: 45,
  }
  Rect1 can hold rect2.: true
  Rect1 can hold rect3.: false
  ```

### 関連関数

- `impl` ブロック内に `self` を引数に取らない関数を定義できる
- この関数は、関連関数と呼ばれる
- 例：正方形のインスタンスを作成する関連関数の定義

  ```rust
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn square(width: u32) -> Rectangle {
          Rectangle { width: width, height: width}
      }
  }
  ```

- 関連関数の呼び出し

  ```rust
  let square1 = Rectangle::square(100);
  ```
