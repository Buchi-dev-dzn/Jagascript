## 🥔 じゃがすくりぷと（Jagascript）

> 「じゃ」と「が」だけで書ける、最低限構文の低レベル遊び言語！

---

## 🧠 これは何？

**じゃがすくりぷと**は、日本語の「じゃ」と「が」だけで構成された、ミニマルでTuring Completeなオリジナル言語です。  
命令はすべて `"じゃ"` と `"が"` の組み合わせで表現されます！
Rust製のインタプリタを含んでおり、CLIから `.jgs` ファイルを実行できます。

⸻

▶︎ 🚀 使い方

.jgs 拡張子のファイルを以下のように実行：

jaga examples/hello.jgs


⸻

📖 命令一覧

じゃが語	命令	説明
じゃ : ポインタを右へ移動
が : ポインタを左へ移動
じゃが : 現在の値をインクリメント
がじゃ : 現在の値をデクリメント
じゃじゃ : 現在の値を文字で出力
じゃじゃが : 現在の値を数値で出力
がが : 入力（1バイト）
じゃがが : ループ開始（0ならスキップ）
ががじゃ : ループ終了（非0なら戻る）

⸻

💡 サンプル

じゃがじゃがじゃが   # + + +（3）
じゃじゃが           # 出力（3）


---

## 🚀 インストール

```bash
git clone https://github.com/your-username/jagascript.git
cd jagascript
cargo build --release
mv target/release/jagascript ~/.cargo/bin/jaga

※ ~/.cargo/bin に PATH を通していない場合は .zshrc などで追加してください。
