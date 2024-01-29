> Blazingly fast interpreter for Folders ⚡

# Folders 📂
Folders is a language where the program is encoded in to a directory structure. This *esoteric programming language* was [created by Daniel Temkin](https://danieltemkin.com/Esolangs/Folders/)

## Usage
You can either execute a Folder code directly or transpile it to **Rust** code.
```bash
cargo run ./samples/SimpleEcho <<< Hello
> Hello
```
You can translate Folders to actual source code with the ```-t``` or ```--transpile``` option.
```bash
cargo run -- -t ./samples/HelloWorld
```
will output the following code
```Rust
use std::io::Write;

fn main() {
        print!("{}", "Hello, World!".to_owned());
        std::io::stdout().flush().unwrap();      
}
```

> The output code is not the best Rust written out there but still is Rust

## Instructions
Commands are read in alphabetical order.

The number of folders within the first sub-folder are read as the type, command, or expression. The second and sometimes third folder are read as the content (in the case of If or While statements).

The number of folders in the first sub-folder determines the expression, type, or command:

### Commands:

Commands take the following form
| Command | # of folders | Details |
| ------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| if | 0 folders | Second sub-folder holds expression, third holds list of commands |
| while | 1 folder | Second sub-folder holds expression, third holds list of commands |
| declare | 2 folders | Second sub-folder holds type, third holds var name (in number of folders, ex. zero folders becomes "var_0") |
| let | 3 folders | Second sub-folder hold variable name (in number of folders), third holds an expression |
| print | 4 folders | Second sub-folder holds expression |
| input | 5 folders | Second sub-folder holds variable name | 

### Expressions:

Expression folders take the following form:
| Type | # of folders | Details |
|---------------|-----------|-----------------------------------------------------------------------------------------------------------|
| Variable | 0 folders | Second sub-folder holds variable name |
| Add | 1 folder | Second sub-folder holds the first expression to add, third holds second expression |
| Subtract | 2 folders | Second sub-folder holds the first expression to subtract, third holds second expression |
| Multiply | 3 folders | Second sub-folder holds the first expression to multiply, third holds second expression |
| Divide | 4 folders | Second sub-folder holds the first expression to divide, third holds second expression |
| Literal Value | 5 folders | Second sub-folder holds the type of the value (as described by types below, ex. two folders for a string) |
| Equal To | 6 folders | Second and third folders hold expressions to compare |
| Greater Than | 7 folders | Second and third folders hold expressions to compare (takes the form : second folder > third folder) |

### Types

And finally type folders take the following form:

| Type   | # of folders |
| ------ | ------------ |
| int    | 0 folders    |
| float  | 1 folder     |
| string | 2 folders    |
| char   | 3 folders    |

#### Literals
All literals are represented in hex. A group of four folders is a single hex digit. Each of the four folders either contains a sub-folder, marking it as a 1, or is empty, marking it as 0.

#### Ints and Chars
A folder holding a series of folders, each of which is a hex digit, as described under literal above.

#### Strings
Strings are made up of Unicode characters. Each string has a series of folders, one per character. Each character has a series of sub-folders for each hex digit, as described above.


> That's enough torture for me 🤣