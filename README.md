## Warmup 1
A simple interpreter that recursively parses arithmetic integer expressions with operate precedence and nesting complexity. The interpreter can compute multiple computations, with each computation ending with a period.

```1  +2  *(3+4).7 * 6.``` prints **15** and **42**

## Warmup 2
An interpreter that extends upon warmup 1 with variables and separates the parser and tokenizer operations. A computation starts with the keyword "computation" followed by variable declarations and expressions and ends with a period.

```computation var x <- 2 * 3; var y <- 7; (((y * x))); i-5-1.```\ \
prints ```42, 0```
