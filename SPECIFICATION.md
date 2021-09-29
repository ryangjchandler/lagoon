# The Lagoon Specification

Lagoon is a dynamic, weakly-typed and minimal scripting language. It draws inspiration from a handful of modern languages including JavaScript, Rust and PHP.

## "Hello, world!"

Here is an example "Hello, world!" program written in Lagoon:

```rust
fn hello(name) {
    println("Hello, " + name + "!")
}

hello("GitHub")
```

Function declarations begin with the `fn` token, followed by an identifier. This identifier can only contain alphanumeric and `_` characters. Parameter lists are comma-separated and have no type declarations.

Functions are called by postfixing an identifier with `()`. You maybe pass a comma-separated list of arguments too.

String literals are wrapped in `"` characters and can be concatenated using the `+` operator.

## Types

Lagoon has support for all of the common scalar types found in most programming languages.

### Strings

Strings are literal expressions wrapped in `"` characters. They support regular escape sequences such as `\n`, `\t`, etc. You can also escape any nested `"` characters using a `\` character.

### Numbers

Lagoon uses a single `number` type to represent both integers and floats. Internally, they are stored as 64-bit floating points. This is very similar to JavaScript and dramatically simplifies the internal code structure.

### Booleans

You can use the `true` and `false` constant to create boolean values. When converted to strings, `true` becomes the literal `"1"` and `false` becomes the literal `"0"`. This behaviour is also found when converting booleans to numeric values.

### `null`

The use of `null` isn't recommended. Internally it is used as the default value for uninitialised variable declarations.

### Structures

Structures, or more commonly "structs", are an efficient way of abstracting away common data models in scripts. Here's an example `struct`:

```rust
struct Person {
    name,
    email
}
```

All `struct` definitions start with the `struct` keyword, followed by an alphanumeric identifier. You then define a list of comma-separated identifiers known as "fields".

> **HINT**: we recommend using `PascalCase` for all `struct` identifiers and `camelCase` for property names. 

Fields are values that you wish to store inside of the structure. They can contain any value you wish as there is no runtime or parse-time type checking taking place.

If you want to create a new instance of a `struct`, you can do the following:

```rust
struct Person {
    name,
    email
}

let ryan = Person {
    name: "Ryan",
    email: "lagoon@ryangjchandler.co.uk",
}
```

## Functions

As we saw further up, functions can be declared with the `fn` keyword:

```rust
fn hello(name) {
    
}
```

If you wish to return a value from a function, you can use a `return` statement.

```rust
fn what_is_my_name() {
    return "Ryan"
}

let name = what_is_my_name() // returns "Ryan"
```

### Structure methods

Lagoon provides first-class support for structure methods. Using our `Person` example from earlier, let's create a setter method that updates the `name` field:

```rust
struct Person {
    name,
    email
}

Person.set_name = fn (self, name) {
    self.name = name
}
```

Structure methods are defined outside of the definition itself. This creates an extremely flexible syntax that allows third-party scripts to modify existing structures.

All instance methods must accept `self` as their first argument. This is similar to `this` or `$this` in JavaScript and PHP (respectfully), but more inline with the Rust syntax used inside of `impl` blocks.

Lagoon also has support for static methods. These are methods that do not have access to `self` and instead operate in a separate context. A good exampe of a static method would be a "constructor" method:

```rust
struct Person {
    name,
    email
}

Person.new = fn (name, email) {
    return Person {
        name: name,
        email: email,
    }
}

let ryan = Person.new("Ryan", "lagoon@ryangjchandler.co.uk")
```

This method does not define a `self` parameter and is therefore static. It does not operate on an instance of `Person` and, in this scenario, instead returns an instance of `Person`.

> **NOTE**: we recommend using `camelCase` for all method names and `new` as the name of the constructor.