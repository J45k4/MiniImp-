# MiniImp

This project is not meant to be usefull but more like learning experience

MiniImp is a simple intepreted programming langauge program code
is first parsed using pest which is PEG grammar parser
library. Grammar for the language can be found from
"grammar.pest" file. The core is first parsed using the parser
and then compiled to bytecode which is executed by the virtual machine.

**Aarchitecture of the intepreter**

![alt text](architecture.png "Title")

## Architecture

### Bytecode

**Load**

Loads value pointed by arg to top of the stack.

**Store**

Pops top of the stack and stores it to value pointed by arg.

**LoadConst**


**BinMul**

Pops TOS and TOS1 multiplies them and pushes to the stack. 

**BinAdd**

Pops TOS and TOS1 adds them and pushes the result to stack.

**BinMinus**

Pops TOS and TOS1 subtracts them and pushes the result to stack.

**BinDivide**

Pops TOS and TOS1 divides them and pushes the result to stack.

**Jump**

Sets instruction pointer value to arg value.

**JumpIfFalse**

Pops top of the stack and if the value is truthy then continues otherwise
sets intruction pointer value to arg.

**Call**

Pops as many values from stack as arg value and then attemps to call with 
next stack value.

**CmpEq**

Pops TOS and TOS1 and compares them acording to the arg operator
and then pushes result to stack.


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