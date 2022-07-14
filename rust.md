# rust memo

## terminal command

- パッケージ作成
    `cargo new <パッケージ名>`
- 実行
    `cargo run`

## 用語

- バインド
    代入と同義
- ドロップ
    スコープを抜けたときにメモリが解放されること

-
-
-
-
-
-
-
-

## 文法

- mod
    moduleのimport
- ディレクトリ名::moduleのファイル名::関数()
    ディレクトリにあるmoduleで定義されている関数を実行
- mut
    rustでは変数はデフォでimutableなのでmutableにしたい場合mutをつける

    ```rust
        let mut x  = 1;
    ```

- 出力
    {}で第二引数を指定

    ```rust
        let x = 1;
        println!("answer is {}", x);
    ```

- 使用していない変数の定義
    意図的に使用しない変数を定義する場合は変数名の頭に＿をつける

- pointerの表示

    ```rust
        let x = 1;
        println!("pointer is {:p}", &x);
    ```

-シャドーイング
    rustでは以下のように変数を再定義すると元の変数の値が隠れて新しい値に更新される。

```rust
        let x = 1;
        let x = x + 1; // x is 2
    ```
- 配列
    配列は定義時にサイズを決めなければいけない。

-
-
