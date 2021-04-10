The following logical operators are listed in order from most precedence to least precedence.
If the parser encounters two operators on the same level ("tier"), it will attempt to "condense" lower-precedence operators before higher-precedence onces. 
This technically means that lower-precedence operators will be "resolved" **first**, but higher-precedence operators are "stronger" and more resistant to getting condensed.
When operators are "condensed", they are functionally moved into a lower tier. 

For example, a & b | c would be functionally parsed as a & (b | c). 
a & b | c ^ d would be functionally parsed as (a & (b | c)) ^ d

## Precedence Listing
1. Equivalence (=)
5. AND (&)
6. OR (|)
7. NOT(!)
8. TERMINALS

## Notes 
