# MiniImp

This project is not meant to be usefull but more like learning experience

MiniImp is a simple intepreted programming langauge program code
is first parsed using pest which is PEG grammar parser
library. Grammar for the language can be found from
"grammar.pest" file. When the code is parsed to AST
it is compiled to vm bytecode and can be executed.

**Aarchitecture of the compiler**

![alt text](architecture.png "Title")

## Architecture

### Bytecode

**Load**

**Store**

**LoadConst**

**BinMul**

**BinAdd**

**BinMinus**

**BinDivide**

**Jump**

**JumpIfFalse**

**Call**

**CmpEq**


## Langauge reference

Supported statements

* `while`
* `if`
* `set`
* `var`
* `call`

### while

```c
while_stmt = { "while" ~ expr ~ scope }

// Infinite loop
while true begin 
    print("Hello world")
end.

```

### if

```basic
if_stmt = { "if" ~ expr ~ "then" ~ stmts? ~ "end." }

// 
if 5 < 10 then

end.
```

### set

```js
set_stmt = { "set" ~ identifier ~ "=" ~ expr ~ ";" }

var x = 5;
```

### var

```js
var_stmt = { "var" ~ identifier ~ ("=" ~ expr)? ~ ";" }

set x = 10;
```

### calls

```js
call_stmt = { expr ~ "(" ~ arg* ~ ")" ~ ";" }

// Prints hello world to console
print("Hello world")

// Sleeps for 2500 milliseconds
sleep(2500)

// Draws line to window
line(x, y, x2, y2, color);

// Draws rectangle to window
rectangle(x, y, width, height, color)

// Draws circle to window
circle(x, y, r, color)

// Clears all drawings
clear()
```