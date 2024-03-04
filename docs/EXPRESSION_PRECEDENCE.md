# Expression Precedence

Mostly follows the same rules as 
[rust](https://doc.rust-lang.org/reference/expressions.html#expression-precedence).

| Operator/Expression                         | Associativity       |
|---------------------------------------------|---------------------|
| Method Calls                                |                     |
| Field expressions                           | left to right       |
| Function Calls/Indexing                     |                     |
| Unary - ! ~                                 |                     |
| as                                          | left to right       |
| * / %                                       | left to right       |
| + -                                         | left to right       |
| << >>                                       | left to right       |
| &                                           | left to right       |
| ^                                           | left to right       |
| \|                                          | left to right       |
| == != < > <= >=                             | require parentheses |
| &&                                          | left to right       |
| ^^                                          | left to right       |
| \|\|                                        | left to right       |
| ..< ..=                                     | require parentheses |
| = += -= *= /= %= <br/> ~= &= \|= ^= <<= >>= | right to left       |
| `ret` `break` closures                      |                     |
