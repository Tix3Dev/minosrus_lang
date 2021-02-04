# minosrus_lang

**DISCLAIMER: This project is still in development.**

A simple interpreter in form of a repl for my own programming language called minosrus_lang. 

It's inspired by BASIC but also by new languages like Rust. In the future the interpreter will (eventually) be used for an OS. And it's my first project in Rust.

Short Paradigm:
- Imperative
- Procedural
- Structured

----

## Installing

- You can simply clone this repository with ```git clone https://github.com/Tix3Dev/minosrus_lang```.
- You need to be able to execute rust code. To install rust have a look at the [instructions](https://www.rust-lang.org/tools/install) of the official rust-lang page.
- Finally you can execute the project with ```cargo run```.

----

## Examples

Repl:
```
> PRINT "HELLO WORLD"
```
Output:
```
HELLO WORLD
```

Repl:
```
> LET COUNTER = 1
> WHILE COUNTER <= 5 START
	> PRINT | "COUNTER: " + STRING_FROM COUNTER |
	> LET COUNTER = | COUNTER + 1 |
	> END
``` 
Output:
```
COUNTER: 1
COUNTER: 2
COUNTER: 3
COUNTER: 4
COUNTER: 5
```

----

## Documentation

To get a better understanding of the language, you might consider looking at the documentation. You will also find some examples there. Here is the link to the [docs](https://github.com/Tix3Dev/minosrus_lang/tree/master/docs).

----

## Contributing

This is mainly a personal project, so if you want to contribute, please let me know first so we can talk about your desire.

Big thanks to [Rosca Alex](https://github.com/roscale) who is in account for verines and who is a really talented programmer.
