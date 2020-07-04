# Note

## The Stack and the Heap

| Differences | stack | heap |
| --- | :---: | :---: |
| Structure | FILO | malloc & free (OS) |
| Data Size | Known(fixed) | Unknown |
| Allocation | Fast | Slow |
| Accessing | Fast | Slow |
| Usage | function calls

## Ownership Rules

- Each value in Rust has a variable that's called its *owner*.
- There can only be one owner at a time. (Mutex?)
- When the owner goes out of scope, the value will be dropped.

## Understanding the `String` type

- This type is allocated on the heap
- Able to store an unfixed amount of data

| string literal | String |
| :---: | :---:|
| fixed at compile time | growable |
| immutable | mutable |
| stack | heap |

Allocating and freeing memory is a difficult problem.
In Rust, the memory is automatically return once the variable that owns it goes out of scope, with a help of `drop` function.

## Move

When two variables in same scope references same location on heap, there is *double free* problem specifically if those variables goes out of the scope.
Rust solves this problem invalidating the first variable, called *moving*.

- No deep copy by default
- Multiple references to same location is prevented.

## Clone

Same as deep copy in other programmin languages

## Copy

- Stack-only data such as integers.
- `Copy` trait
- No `Drop` trait

| `Copy` | Not `Copy` |
| --- | --- |
| `i32`, `(u32` `f64)` | `(i32` `String)` |

## Functions

Passing variables to a function will move or copy.
Returng a variable from a function will transfer ownership of the variable.
