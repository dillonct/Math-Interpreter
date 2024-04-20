## Warmup 1
A simple interpreter that recursively parses arithmetic integer expressions with operate precedence and nesting complexity. The interpreter can compute multiple computations, with each computation ending with a period. Ignores whitespace.

```2*2+20  +10  *((1000/4) - 50).       100 + (10*2)+3.``` prints **2024** and **123**

## Warmup 2
An interpreter that extends upon warmup 1 with variables and separates parsing and tokenizing the input. A computation starts with the keyword "computation" followed by variable declarations and expressions and ends with a period.

```computation var x <- 1; var y <- 2; var z <- 3; (((z*100)+y*10)+x); z-y-x.``` prints **321** and **0**
