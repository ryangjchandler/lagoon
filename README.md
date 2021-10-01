# 🏞 The Lagoon Language

<p align="center">
    <img src="./art/splash.jpg">
</p>

Lagoon is a dynamic, weakly-typed and minimal scripting language. It draws inspiration from a handful of modern languages including JavaScript, Rust and PHP.

If you want to learn more about the language itself, you can read the [SPECIFICATION](./SPECIFICATION.md) or take a look at the collection of [examples](./examples) in this repository.

## Example

```
-- This is a recursive fibonacci function. It accepts a number `n`.
fn fib(n) {
    if n < 2 {
        return n
    }

    return fib(n - 1) + fib(n - 2)
}

println(fib(10)) -- `println` is a function in the standard library.
```

## Theory

Lagoon parses a string of source code into a token stream and [abstract syntax tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

The generated AST is then passed to an interpreter. It uses a tree-walk approach to recursively iterate through each node in the tree.

> A tree-walk interpreter is one of the slowest methods of execution, but it's also one of the simplest. In the future Lagoon will likely move to a bytecode interpreter but whilst the language itself is still in early development, a tree-walk interpreter will continue to be used.

At the highest level, all operations in Lagoon are parsed as statements. A statement can contain one or more expressions. Those expressions are generally used to manipulate the execution environment and provide information to your script.

## Development Checklist

Lagoon is nowhere near being feature complete or syntax complete. Below is a small checklist of things that we still need to add and design before marking it as "stable".

* [x] `&&`, `||` - Boolean infix operators
* [x] `>`, `>=`, `<`, `<=` - Mathematical comparison operators
* [x] `**` - Exponent / power operator
* [ ] `&`, `|`, `^`, `~`, `<<`, `>>` - Bitwise operators
* [x] Lists
* [x] `for..in` statements
* [x] `in`, `not in` operators
* [x] Scalar objects (methods on scalar types)
* [ ] Module system
* [x] Standard Library
* [x] Nicer error reporting
* [x] Transpile to JavaScript
* [x] Nicer command-line interface
* [ ] Constant declarations (`const`)
* [ ] Migrate parser to on-demand token stream
* [ ] Line/column numbers in errors
* [ ] Build a virtual machine to replace the interpreter

## Contributing

If you would like to contribute to Lagoon, please feel free to fork this repository and open a pull request. All contributions are highly appreciated, no matter your Rust knowledge.

## Credit

* [Ryan Chandler](https://github.com/ryangjchandler)
* [All contributors](https://github.com/ryangjchandler/lagoon/contributors)
* [Barbara Šipek on Unsplash](https://unsplash.com/photos/QQEMsVHNLq0)