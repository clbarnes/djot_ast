# djot_ast

An implementation of [djot](https://djot.net/)'s Abstract Syntax Tree for light markup.

Intended for drop-in compatibility with the [reference AST implementation](https://github.com/jgm/djot.js/blob/main/src/ast.ts).

## Test files

[pandoc-manual.dj](tests/fixtures/pandoc-manual.dj) is a djotified version of the pandoc manual, kindly provided by [Bruce D'Arcus](https://github.com/bdarcus) [here](https://github.com/jgm/djot/issues/227#issuecomment-1612235687).

[readme.dj](tests/fixtures/readme.dj) is from the [djot.js](https://github.com/jgm/djot.js) benchmarks [here](https://github.com/jgm/djot.js/tree/main/bench).

The reference JSON representations are generated using the [typescript implementation](https://github.com/jgm/djot.js).
