# dutty
simple teletype test dummy written in Rust.

## Usage
```sh
$ dutty 
Standard output is a TERMINAL
Standard input is a TERMINAL
Standard error is a TERMINAL
```
Here, `dutty` recognised that its stdin, stdout and stderr streams were all from the terminal; so it told us that these streams were all terminals.

Let's try to redirect its standard IO to teletypes:
1. Stdin
  ```sh
  $ dutty < somefile
  Standard output is a TERMINAL
  Standard input is a TELETYPE
  Standard error is a TERMINAL
  ```
  Here, we redirected `dutty`'s standart input to a file (which is a teletype), so it told us that stdin was a teletype.

  This also works with pipes:
  ```sh
  $ head /dev/random | dutty
  Standard output is a TERMINAL
  Standard input is a TELETYPE
  Standard error is a TERMINAL
  ```
  You may also notice that `dutty` colorizes the words `TERMINAL` and `TELETYPE` green and red, respectively. It does this even when its stdout or stderr is redirected, cause it uses the *stubborn I/O* (trying to write exactly to the console, regardless of the state of its stdio).

2. Stdout
  `dutty` has a similar behaviour for stdout.

  ```sh
  $ dutty | cat
  Standard output is a TELETYPE
  Standard input is a TERMINAL
  Standard error is a TERMINAL
  ```
3. Stderr
  `dutty` has the same behavior when its standard error is redirected.
  ```sh
  $ dutty 2>| cat
  Standard output is a TERMINAL
  Standard input is a TERMINAL
  Standard error is a TELETYPE
  ```

  ```sh
  $ dutty |& cat
  Standard output is a TELETYPE
  Standard input is a TERMINAL
  Standard error is a TELETYPE
  ```




