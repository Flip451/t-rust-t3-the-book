# １８章：パターンとマッチング

## 目次

- [１８章：パターンとマッチング](#１８章パターンとマッチング)
  - [目次](#目次)
  - [18.0 概要](#180-概要)
  - [18.1 パターンが使用されることのある箇所全部](#181-パターンが使用されることのある箇所全部)

## 18.0 概要

- aaa

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

4. `for` loop

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
