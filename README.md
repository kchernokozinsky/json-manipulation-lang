# JML (JSON Manipulation Language) Overview

JML (JSON Manipulation Language) is a purely functional domain-specific language designed for seamless manipulation and transformation of JSON data. With a focus on immutability and function composition, JML allows users to build complex JSON structures and perform operations using functions, lambdas, and other functional constructs.

## Basic Syntax

### Variables
In JML, variables are immutable once assigned. You can bind values to names:

```jml
a = 1
d = "some_string"
```

### JSON Object Construction

Variables can be bound to any expression, and their evaluation is done lazily — meaning that the variable is only computed when it is actually used. The code is divided into two sections: the header, which contains all variable bindings (statements), and the body, separated by the `---` keyword, which includes a single expression. Variables are evaluated as they are accessed in the body, optimizing performance by avoiding unnecessary calculations.

**Header:**
```jml
a = 1
d = "some_string"
c = \x y. x + y
```

**Body:**
```jml
{
    "key1": a, 
    "key2": d,
    "key3": c(5, 3)
}
```

### Lambdas
JML supports lambda expressions, which are anonymous functions. The syntax for lambdas is `\x y. expression`, where `x` and `y` are parameters:

```jml
add = \x y. x + y
---
add(3, 4)
```

### Function Composition
Functions can be composed to create more complex operations:

```jml
double = \x. x * 2
increment = \x. x + 1
double_then_increment = \x. increment(double(x))
---
double_then_increment(5)  // returns 11
```

### Get by Index
Access elements within arrays using a functional approach:

```jml
arr = [1, 2, 3, 4]
get = \index arr. arr[index]
---
get(2, arr)  // returns 3
```

### Pure Functions
All functions in JML are pure, meaning they have no side effects and always produce the same output for the same input:

```jml
square = \x. x * x
sum_of_squares = \x y. square(x) + square(y)
---
sum_of_squares(3, 4)  // returns 25
```

### Higher-Order Functions
JML allows functions to take other functions as arguments or return them as results:

```jml
apply_twice = \f x. f(f(x))
increment = \x. x + 1
---
apply_twice(increment, 5)  // returns 7
```

### Recursion
Recursion is used for looping or repeated computation:

```jml
factorial = \n. if n == 0 then 1 else n * factorial(n - 1)
---
factorial(5)  // returns 120
```

## Examples

### Complex JSON Construction with Functions

```jml
a = 2
b = 3.5
concat = \x y. x + " " + y
---
{
    "sum": a + b,
    "description": concat("total is", (a + b)),
    "nested": {
        "array": [a, b, a + b],
        "fn_example": (\x y. x + y)(a, b)
    }
}
```

### Functional JSON Manipulation

```jml
sum = \x y. x + y
double = \x. x * 2
---
{
    "total": sum(10, 20),
    "doubled_total": double(sum(10, 20))
}
```
# CLI Usage Guide

## Overview

The json-manipulation-lang CLI allows you to execute JML (JSON Manipulation Language) scripts on JSON data from the command line. This guide provides instructions and examples on how to use the CLI effectively.

## Basic Usage

The CLI provides a `run` subcommand to execute JML scripts.

```bash
json-manipulation-lang run [OPTIONS]
```

### Options for `run` Subcommand

- `-f, --file <FILE>`: (Required) Path to the JML source file to be parsed and evaluated.
- `-o, --output <FILE>`: Optional path to write the output as JSON. Defaults to standard output.
- `-v, --variables <name=path>`: Provide variables and their corresponding JSON paths (file path or URL). This option can be used multiple times for multiple variables.

## Example: Running a JML Script on a JSON Variable

Suppose you have a JML script `script.jml` that processes a JSON variable `data`.

**Contents of script.jml:**

```jml
filter(data, \ elem. elem.value > 10)
```

**JSON Data File `data.json`:**

```json
[
  {"id": 1, "value": 5},
  {"id": 2, "value": 15},
  {"id": 3, "value": 25}
]
```

**Command:**

```bash
jml-cli run -f script.jml -v data=data.json -o output.json
```

**Explanation:**

- `run`: Subcommand to execute the script.
- `-f script.jml`: Specifies the JML script file to run.
- `-v data=data.json`: Sets the variable `data` to the contents of `data.json`.
- `-o output.json`: Writes the output to `output.json`.

**Output (`output.json`):**

```json
[
  {"id": 2, "value": 15},
  {"id": 3, "value": 25}
]
```

