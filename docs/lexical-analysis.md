# Lexical Analysis

The process of reading the source code and convert it into a stream of Tokens is called Lexical Analysis, and is also known as Tokenization or Scanning.

Tokens are the smalles sequences of characters that has meaning for us.

Let's take a look at a code example from `mypl`:

    cst number: i8 = 3;

The Tokens are objects that represents the following: `cst`, `number`, `:`, `i8`, `=`, `3` and `;`.

The Tokens are define [here](../mypl_lex//src/token.rs).

The object that scans the source code is the [Tokenizer](../mypl_lex_/src/tokenizer.rs). Its job is to scan the source, character by character and understand how to build Tokens.

To keep track of the origins of elements that originated from the source, such as Tokens, we encode the notion of a [Span](../mypl_lex/src/span.rs). In essence, A Span is just a tuple of `start` and `end`  which represents location within a specific source.

Some useful references:

- [Rustc's AST Tokens](https://github.com/rust-lang/rust/blob/master/compiler/rustc_ast/src/token.rs)

