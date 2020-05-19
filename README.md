# RLISP
[![RRCWT](https://github.com/sKyrBBit/RLISP/workflows/RLISP/badge.svg)](https://github.com/sKyrBBit/RLISP/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Runtime: RCWT](https://img.shields.io/badge/Runtime-RCWT-blue.svg)](https://github.com/groupylang/RRCWT)
## Grammar
```
input : list input
      | /* empty */
      ;
list  : "(" pair
      | """ list
      | ATOM
      ;
pair  : ")"
      | list cdr
      ;
cdr   : ")"
      | "." list ")"
      | list cdr
      ;
```
