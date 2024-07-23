takeaway:

## Parser
Now, when writing a parser ourselves, we have to make some trade-offs, yes. Our parser won’t
be the fastest of all time, we won’t have formal proof of its correctness and its error-recovery
process and detection of erroneous syntax won’t be bullet proof

rules:
1. "let statements" - `let <identifier> = <expression>;`
```rust
let x = 5;
let y = 10;
let foobar = add(5, 5);
let barfoo = 5 * 5 / 10 + 18 - add(5, 5) + multiply(124);
let anotherName = barfoo;

let add = fn(x, y) {
    return x + y;
};
```
