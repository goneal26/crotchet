# Syntax

## Let

```
[let x 1]
[let y 1.0]
```

## Comments

```
[let pi 3.14] ; this is a single-line comment
; note "numbers" are all 64-bit floating point
```

## Basic Operations

```
; math
[+ 5 3] ; 5 + 3
[- 5 3] ; 5 - 3
[* 5 3] ; 5 * 3
[/ 5 3] ; 5 / 3

[round 3.14] ; round to nearest integer
; if rounding at a half, rounds up

; comparison (returns boolean)
[> 5 3] ; 5 > 3
[>= 5 3] ; 5 >= 3
[= 5 3] ; 5 == 3
[!= 5 3] ; 5 != 3
[< 5 3] ; 5 < 3
[<= 5 3] ; 5 <= 3
```

## Set

```
[let a 5]
[set a 10] ; mutation of 'a'
```

## If Expression

```
[if [> x 10] 1 2] ; returns 1 if true else 2
[if true -1 1] ; returns -1
; these REQUIRE a boolean-type condition
```

## Lambda Functions

```
[let sum 
   [fn [x y] [+ x y]]]

[sum 10 20.0]
```

## Input/Output

```
[print "Hello world!"] ; string literals

; printed with a single space between them,
; on a new line:
[print "five plus five is" 10]

; prompts user with message, returns input
; after trying to evaluate it as a 
; 64-bit floating point number
[let n [input "Enter a number: "]]
```

## Lists

```
[let mylist [list 1 2 3 4]] ; creates a list of 1,2,3,4
[first mylist] ; returns 1
[rest arr] ; returns the sublist [2, 3, 4]
```

## While loop

```
[let i 0]

[while [< i 5] [print i] [set i [+ i 1]]]
; first argument must be a conditional
; evaluates every subsequent argument before looping
```

## Random Number Generation

```
[let r [rand 1 6]]
; random float on interval [1, 6)
```

## Hello World Program

```
[
  [print "Hello world!"]
]
```
