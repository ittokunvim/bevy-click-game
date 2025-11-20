# bevy-click-game

`bevy-click-game`は、ゲームエンジン`Bevy`で作られたクリックゲームです。

## ゲーム概要

画面内にあるボールを全て消すことができるとクリアです。

また画面内のボールをより早く消すことができると高得点を得ることができます。

## ゲーム情報

ゲームタイトル `いっとくクリックゲーム`

画面サイズ `640x480`

## 遊び方

リポジトリをクローンしてから、`cargo run`を実行することで遊ぶことができます。

## 操作方法

- ゲームを始める、ボールを消す: 左クリック
- 画面を遷移する: キーボード

## Wasmに変換する

ゲームをWasmに変換する場合は、以下のコマンドを実行します。

```sh
# ビルド
cargo build --release --target wasm32-unknown-unknown
# 変換
wasm-bindgen --target web --out-dir ./examples --no-typescript \
target/wasm32-unknown-unknown/release/ittoku_click_game.wasm
# 実行
npx http-server examples
```

## クレジット

開発者 [ittokunvim](https://github.com/ittokunvim)

ゲームエンジン [Bevy](https://bevyengine.org)

タイトル画像 [Wallpapers](https://wallpapers.com/)

フォント [美咲フォント](https://littlelimit.net/misaki.htm)

ポーズボタン画像 [ICOOON MONO](https://icooon-mono.com/)

BGM、効果音 [効果音ラボ](https://soundeffect-lab.info)

画像編集 [Pixlr](https://pixlr.com)

Wasm変換 [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
