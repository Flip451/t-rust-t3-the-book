# １９章ー２節：高度な機能ー発展的なトレイト

## 目次

- [１９章ー２節：高度な機能ー発展的なトレイト](#１９章ー２節高度な機能ー発展的なトレイト)
  - [目次](#目次)
  - [19.2.0 発展的なトレイトについての概要](#1920-発展的なトレイトについての概要)
  - [19.2.1 関連型](#1921-関連型)
    - [関連型とジェネリクスの比較](#関連型とジェネリクスの比較)
  - [19.2.2 デフォルトのジェネリック型引数と演算子オーバーロード](#1922-デフォルトのジェネリック型引数と演算子オーバーロード)
    - [デフォルトのジェネリック型](#デフォルトのジェネリック型)
    - [演算子オーバーロード](#演算子オーバーロード)
  - [19.2.3 メソッドのフルパス記法：同じ名称のメソッドが複数ある場合の処置](#1923-メソッドのフルパス記法同じ名称のメソッドが複数ある場合の処置)
  - [19.2.4 スーパートレイト](#1924-スーパートレイト)
  - [19.2.5 ニュータイプパターン](#1925-ニュータイプパターン)

## 19.2.0 発展的なトレイトについての概要

- トレイトの関連型
  - `type` キーワードを用いることで、トレイトに関連型を定義できる
  - 関連型は、そのトレイトのメソッドのシグネチャの一部に利用できる

- デフォルト型引数
  - トレイトを定義する際に、ジェネリック型のデフォルト型を `impl Trait名<ジェネリック型名=デフォルト型>` のように定義できる
  - もし、このトレイトを実装するときに、ジェネリック型のデフォルト値をそのまま採用しても問題なく機能する場合、ジェネリック型の指定を省略できる

- 演算子オーバーロード
  - トレイトを実装することで `+`, `*` などの演算子をオーバーロードできる
  - このとき、トレイトのジェネリック型を指定することで、演算子の右辺の型などを指定することもできる
    - たとえば、二次元行列をあらわす `Matrix` と二次元ベクトルをあらわす `Point` 構造体があるとき、 `Matrix` 同士の積を定義できるのみならず、`Matrix` と `Point` の積も定義できる

- ある構造体に実装されているトレイトが同名のメソッドを持つ場合、それらのメソッドをどう区別して呼び出せばよいかの方法 &rarr; [19.2.3 メソッドのフルパス記法：同じ名称のメソッドが複数ある場合の処置](#1923-メソッドのフルパス記法同じ名称のメソッドが複数ある場合の処置)

- スーパートレイト
  - あるトレイトについて、そのトレイトが実装される際に、別のあるトレイトが実装されていることを前提とすることができる
  - このとき、実装を前提とされるトレイトを**スーパートレイト**という
  - スーパートレイトを指定してトレイトを定義するには `trait 実装するトレイト名: スーパートレイト名 {...}` のようにする

- ニュータイプパターン
  - 孤児のルール（オーファンルール）を回避する方法
  - タプル構造体で型を皮を被せてごまかす
  - `struct タプル構造体名(トレイトを実装したい型)` とすれば、トレイトを実装したい型をほとんどそのままにクレートのローカルのものに変換できる

## 19.2.1 関連型

- トレイトの定義内で `type 関連型名` というように関連型を定義することで、トレイトのメソッド定義において、シグネチャの一部にその関連型を利用できるようになる
- トレイトを具体的な構造体や Enum に実装する際は、メソッドを実装するのみならず、関連型として具体的な型を指定する必要がある

- 例：`Iterator` トレイト

```rust
pub trait Iterator {
    // 関連型を定義
    type Item;

    // next メソッドのシグネチャの一部で関連型 `Self::Item`を使用
    fn next(&mut self) -> Option<Self::Item>;
}
```

### 関連型とジェネリクスの比較

- 上記の定義は以下のようには書けないことに注意：

  ```rust
  pub trait Iterator<T> {
      fn next(&mut self) -> Option<T>;
  }
  ```

- この制約のために、ある構造体や Enum に対して、複数回同種の（関連型が異なるだけの）トレイトを実装することはできないことに注意

## 19.2.2 デフォルトのジェネリック型引数と演算子オーバーロード

### デフォルトのジェネリック型

- 適当なジェネリックな型引数を持つトレイトを実装するとき、そのジェネリック型にデフォルトの型を実装することができる

  - 例：たとえば、`Add` トレイトの実装は以下のようになっている

    ```rust
    // RHS はジェネリック型
    // <RHS=Self> の Self は RHS のデフォルト値
    // rhs は ”right hand side”（右辺） の省略形
    impl Add<RHS=Self> {
      type Output;

      fn add(self, rhs: RHS) -> Self::Output;
    }
    ```

- もし、このトレイトを実装するときに、トレイトのジェネリック型のデフォルト値をそのまま採用しても問題なく機能する場合、ジェネリック型の指定を省略できる

  - 例：以下の `Point` 型に対しては、`RHS=Self` としても問題なく機能するので `impl Add<RHS=Self> for Point {...` のように書かずに `<RHS=Self>` の部分を省略して書いてもよい

    - なお、省略する場合しない場合の比較については、[演算子オーバーロード](#演算子オーバーロード)の例を参照せよ

    ```rust
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
    ```

### 演算子オーバーロード

- 演算子オーバーロードを用いることで `+` や `*` などの演算子の振る舞いをカスタマイズできる

- このとき、たとえば `Add` トレイトのように、ジェネリックな型引数をもつトレイトを適当な構造体や Enum に実装することがある

- 以下、自分で構成した例：
  - `Matrix` 構造体同士の積を定義する際には、デフォルト型引数を使用して `impl<T> Mul for Matrix<T> {...}` としている一方で
  - `Matrix` 構造体と `Point` 構造体の積を定義する際には、`impl<T> Mul<Point<T>> for Matrix<T> {...}` のように、`Mul` のジェネリックな型引数の具象型を明示的に指定している

  ```rust
  use std::ops::{Add, Mul};

  fn main() {
      let p = Point {
          x: 2, y: 3
      };
      let m1 = Matrix {
          attr_11: 5,
          attr_12: 6,
          attr_21: 7,
          attr_22: 8,
      };
      let m2 = Matrix {
          attr_11: 1,
          attr_12: 0,
          attr_21: 0,
          attr_22: 1,
      };
      println!("m1 * m2 = {:?}", m1 * m2);
      println!("m1 * p = {:?}", m1 * p);
      println!("m2 * p = {:?}", m2 * p);
  }

  #[derive(Debug, Clone, Copy)]
  struct Point<T> {
      x: T,
      y: T,
  }

  #[derive(Debug, Clone, Copy)]
  struct Matrix<T> {
      attr_11: T,
      attr_12: T,
      attr_21: T,
      attr_22: T,
  }

  impl<T> Mul for Matrix<T>
  where
      T: Mul<Output = T> + Add<Output = T> + Clone + Copy,
  {
      type Output = Self;

      fn mul(self, rhs: Self) -> Self::Output {
          let attr_11 = self.attr_11 * rhs.attr_11 + self.attr_12 * rhs.attr_21;
          let attr_12 = self.attr_11 * rhs.attr_12 + self.attr_12 * rhs.attr_22;
          let attr_21 = self.attr_21 * rhs.attr_11 + self.attr_22 * rhs.attr_21;
          let attr_22 = self.attr_21 * rhs.attr_12 + self.attr_22 * rhs.attr_22;
          Self::Output {
              attr_11,
              attr_12,
              attr_21,
              attr_22,
          }
      }
  }

  impl<T> Mul<Point<T>> for Matrix<T>
  where
      T: Mul<Output = T> + Add<Output = T> + Clone + Copy,
  {
      type Output = Point<T>;

      fn mul(self, rhs: Point<T>) -> Self::Output {
          let x = self.attr_11 * rhs.x + self.attr_12 * rhs.y;
          let y = self.attr_21 * rhs.x + self.attr_22 * rhs.y;
          Self::Output { x, y }
      }
  }
  ```

## 19.2.3 メソッドのフルパス記法：同じ名称のメソッドが複数ある場合の処置

- ある構造体に実装されているトレイトが同名のメソッドを持つ場合、それらのメソッドをどう区別して呼び出せばよいのか？

1. メソッドの場合
   - &rarr; 以下の例のように `トレイト名::メソッド名(&構造体のインスタンス変数)` のように定義できる
   - 例：
     - 構造体 `Human` に `Pilot` トレイトと `Wizard` トレイトが実装されている
     - これらのトレイトはともに `fly` というメソッドを持つとする
     - さらに、`Human` 構造体自身も `fly` というメソッドを持つとする

      ```rust
      trait Pilot {
          fn fly(&self);
      }

      trait Wizard {
          fn fly(&self);
      }

      struct Human;

      impl Pilot for Human {
          fn fly(&self) {
              println!("This is your captain speaking.");
          }
      }

      impl Wizard for Human {
          fn fly(&self) {
              println!("Up!");
          }
      }

      impl Human {
          fn fly(&self) {
              println!("*waving arms furiously*");
          }
      }
      ```

     - これらの `fly` メソッドを区別して呼び出すには、各々以下のようにする：

      ```rust
      fn main() {
          let person = Human;
          
          // Human 構造体に定義された fly メソッドを呼び出す(通常通りの記法は構造体本体のメソッドを優先的に呼び出す)
          person.fly();
          
          // Human 構造体に定義された fly メソッドを呼び出す(より明確にどのメソッドを呼び出すかを指定する)
          Human::fly(&person);
          
          // Pilot トレイトに定義された fly メソッドを呼び出す（トレイト名を指定する必要がある）
          Pilot::fly(&person);
          
          // Wizard トレイトに定義された fly メソッドを呼び出す（トレイト名を指定する必要がある）
          Wizard::fly(&person);
      }
      ```

2. 関連関数の場合

   &rarr; フルパス記法を使わないと同名の関数を区別できない場合がある
      - 例えば、一つのトレイトを実装する型が二つあるとき（関連関数は `self` 引数を持たないので）メソッドの時と同じようには対応しきれない
        > 一方、メソッドを呼び出す場合は、`トレイト名::メソッド名(&インスタンス名)` というように、インスタンスを指定することを介して、構造体の型を指定するので紛れがない
      - フルパス記法では関連関数の呼び出しを `<構造体名 as トレイト名>::関連関数名(...)` などのように書く
        - このように書くと、ある構造体が実装するあるトレイトの関連関数を呼び出せる
      - 一方、`構造体名::関連関数名(...)` と書くと、その構造体本体に定義された関連関数が呼び出される

      - 例：

        ```rust
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        fn main() {
            // Dog 構造体の baby_name 関連関数を呼び出す
            println!("A baby dog is called a {}", Dog::baby_name());  // A baby dog is called a Spot

            // フルパス記法で Dog トレイトが実装する Animal トレイトの baby_name 関連関数を呼び出す
            println!("A baby dog is called a {}", <Dog as Animal>::baby_name());  // A baby dog is called a puppy
        }
        ```

- 注意：1. 2. どちらの場合もフルパス記法は使える
  - フルパス記法は `<Type as Trait>::function(receiver_if_method, next_arg, ...);` という形をとる
  - メソッドの場合は、`receiver_if_method` の部分にインスタンスへの参照などを渡す
  - 関連関数の場合は、`receiver_if_method` の部分を指定せず、`next_arg, ...` の部分のみを書く
- ただ、「コンパイラが、どの実装を呼び出すかを特定するのに十分な情報があれば省略ができる」というだけのこと

## 19.2.4 スーパートレイト

- あるトレイトについて、そのトレイトが実装される際に、別のあるトレイトが実装されていることを要請することができる
- このとき、実装を要請されるトレイトを**スーパートレイト**という
- スーパートレイトを指定してトレイトを定義するには `trait 実装するトレイト名: スーパートレイト名 {...}` のようにする

- 例：アスタリスクでできたフレームに囲われた値を出力する `outline_print` メソッドの実装を要請する `OutlinePrint` トレイトを作りたいとする
  - このとき、明らかに `OutlinePrint` メソッドの実装は、`Display` メソッドの実装を前提としている
  - このトレイトは以下のように定義できる：

    ```rust
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            // to_string メソッドは Display トレイトを実装するどんな型にも自動的に実装されているので利用可能
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    ```

  - このとき `OutlinePrint` トレイトをある構造体に実装するには、その構造体に `Display` メソッドも実装する必要が生じる

> cf. ブランケット実装
>
> - あるトレイト `Hoge` を実装するすべての型にトレイト `Fuga` を実装する：
>
>  ```rust
>  impl<T: Hoge> Fuga for T {
>    // Fuga を保持するために必要なメソッド群の定義
>  }
>  ```

## 19.2.5 ニュータイプパターン

- 10 章で 型にトレイトを実装するには、トレイトか型がクレートにローカルである必要があることを述べた
- この制約は**ニュータイプパターン**を使用して回避できる
- つまり、外部の型にタプル構造体の皮を被せれば、クレートにローカルの型になるのでトレイトを実装できるようになる
- 例：`Vec<T>` に `Display` トレイトを実装する
  - `Vec<T>` に直接 `Display` トレイトを実装することはできない（オーファンルール・孤児のルール）
  - しかし、`Vec<T>` のインスタンスを保持する `Wrapper` 構造体を定義することで、`Wrapper` には `Display` トレイトを実装できる

  ```rust
  use std::fmt;

  struct Wrapper(Vec<String>);

  impl fmt::Display for Wrapper {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "[{}]", self.0.join(", "))
      }
  }

  fn main() {
      let w = Wrapper(vec![String::from("hello"), String::from("world")]);
      println!("w = {}", w);
  }
  ```

- しかし、この方法には、「タプル構造体の皮を被せてできた新しい型には、保持している値のメソッドが定義されていない」という問題点がある
  - メソッドをそのまま使えるようにするには
    - 直接新しい型にメソッドを実装するか
    - 内部の型が持つメソッドをすべて新しい方に持たせるために `Deref` トレイトを実装する