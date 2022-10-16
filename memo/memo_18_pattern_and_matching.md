# １８章：パターンとマッチング

## 目次

- [１８章：パターンとマッチング](#１８章パターンとマッチング)
  - [目次](#目次)
  - [18.0 概要](#180-概要)
  - [18.1 パターンが使用されることのある箇所全部](#181-パターンが使用されることのある箇所全部)
  - [18.2 論駁可能性: パターンが合致しないかどうか](#182-論駁可能性-パターンが合致しないかどうか)
  - [18.3 パターン記法](#183-パターン記法)

## 18.0 概要

- パターンには二種類ある：
  - 論駁不可能：渡される可能性のあるあらゆる値に合致するパターン
  - 論駁可能：なんらかの可能性のある値に対して合致しないことがあるパターン

- パターンが使用されることのある箇所の一覧（〇では論駁可能なパターンを、×では論駁不可能なパターンを使う）：
   1. `match` アーム
      - 基本的に論駁可能なパターンを使うが、最後のアームでだけは論駁不可能なパターンを使ってよい
   2. 条件分岐 `if let` 式（〇）
   3. `while let` 条件ループ（〇）
   4. `for` ループ（×）
   5. `let` 文（×）
   6. 関数の引数（×）


## 18.1 パターンが使用されることのある箇所全部

1. `match` アーム

   - 2.に示す `if let` よりも網羅性がある

   ```rust
   match VALUE {
      // パターン => 式,
      PATTERN => EXPRESSION,
      PATTERN => EXPRESSION,
      PATTERN => EXPRESSION,
   }
   ```

2. 条件分岐 `if let` 式

   ```rust
   if let PATTERN = EXPRESSION {
       // ...
   }
   ```

   - `if let` には、パターンに合致しないときに走るコードを記す `else` を用意できる
     - `if let`, `else if`, `else if let` 式を混ぜて使える
   - 網羅性はない
   - 例：

     ```rust
     fn main() {
         let favorite_color: Option<&str> = None;
         let is_tuesday = false;
         let age: Result<u8, _> = "34".parse();
 
         if let Some(color) = favorite_color {
             println!("Using your favorite color, {color}, as the background");
         } else if is_tuesday {
             println!("Tuesday is green day!");
         } else if let Ok(age) = age {
             if age > 30 {
                 println!("Using purple as the background color");
             } else {
                 println!("Using orange as the background color");
             }
         } else {
             println!("Using blue as the background color");
         }
     }
     ```

3. `while let` 条件ループ

   - パターンが合致するならループを実行する

   ```rust
   while let PATTERN = EXPRESSION {
       // ...
   }
   ```

4. `for` ループ

   ```rust
   for PATTERN in EXPRESSION {
       // ...
   }
   ```

5. `let` 文

   ```rust
   let PATTERN = EXPRESSION;
   ```

6. 関数の引数
   - 以下の `x` の部分はパターン：

   ```rust
   fn hoge(x: i32) {
      // ...
   }
   ```

   - 例：

     ```rust
     fn print_coordinates(&(x, y): &(i32, i32)) {
         println!("Current location: ({}, {})", x, y);
     }
 
     fn main() {
         let point = (3, 5);
         print_coordinates(&point);
     }
     ```

## 18.2 論駁可能性: パターンが合致しないかどうか

- パターンには 2 つの形態がある: 論駁可能なものと論駁不可能なもの
  - 論駁不可能：渡される可能性のあるあらゆる値に合致するパターン
    - 例：`let x = 5;` の `x`
  - 論駁可能：なんらかの可能性のある値に対して合致しないことがあるパターン
    - 例：`if let Some(x) = a_value {...}` の `Some(x)`

- 論駁不可能なパターンのみを受け付ける：
  - 関数の引数、`let` 文、`for` ループ
  - 例：`let Some(x) = some_option_value;` は `Some(x)` が論駁可能なパターンなのでコンパイルエラーを起こす

- 論駁可能なパターンのみを受け付ける：
  - `if let`, `while let`
  - 例：以下のコードはコンパイルエラーを起こす（`x` は論駁不可能なので）：

    ```rust
    if let x = 5 {
        println!("{}", x);
    };
    ```

- `match` 式については、
  - 基本的に、論駁可能なパターンを使う
  - ただし、最後のアームでのみ論駁不可能なパターンを使用することが許される

## 18.3 パターン記法
