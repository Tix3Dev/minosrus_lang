# minosrus_lang

**DISCLAIMER: This project is still in development.**

A simple interpreter in form of a repl for my own programming language called minosrus_lang. 

It's inspired by BASIC but also by new languages like Rust. In the future the interpreter will (eventually) be used for an OS. And it's my first project in Rust.

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
	> PRINT | "COUNTER: " + COUNTER |
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

## Installing

You can simply clone the repository and execute it via ```cargo run```

----

## Contributing

This is mainly a personal project, so if you want to contribute, please let me know first so we can talk about your desire.

Big thanks to [Rosca Alex](https://github.com/roscale) who is in account for verines and who is a really talented programmer.
