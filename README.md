# bdd

Binary Decision Diagram (BDD)

現時点では [miniBDD](http://www.cprover.org/miniBDD/) のRust移植程度の機能しかない。(というかその機能もない)

## BDDの具体例

-   `tests/circular.rs`
    -   循環グラフ(C6)の独立集合の数とカーネルの数をBDDを使って数え上げる
