## Readme

This is a command line program for interactively deriving valid judgements in system F1, the type system of first-order typed lambda calculus. Due to typesetting reasons, please refer to the [pdf document](doc.pdf) for details.

Here is a demo.

```
env_empty
0:       ∅ |- ◇
type_const 0 t
0:       ∅ |- ◇
1:       ∅ |- t
env_x 1 x
0:       ∅ |- ◇
1:       ∅ |- t
2:       x : t |- ◇
type_const 2 t
0:       ∅ |- ◇
1:       ∅ |- t
2:       x : t |- ◇
3:       x : t |- t
env_x 3 y
0:       ∅ |- ◇
1:       ∅ |- t
2:       x : t |- ◇
3:       x : t |- t
4:       x : t, y : t |- ◇
val_x 4 y 
0:       ∅ |- ◇
1:       ∅ |- t
2:       x : t |- ◇
3:       x : t |- t
4:       x : t, y : t |- ◇
5:       x : t, y : t |- y : t
val_func 5
0:       ∅ |- ◇
1:       ∅ |- t
2:       x : t |- ◇
3:       x : t |- t
4:       x : t, y : t |- ◇
5:       x : t, y : t |- y : t
6:       x : t |- λy:t.y : t -> t
exit
```