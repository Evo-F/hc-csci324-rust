# Parsing Tips and Rules
* The expression as a whole is considered "Tier 0" - because of the tree-style parsing we are implementing, Tier 0 is the root node of the tree.
* Each set of parentheses creates a new lower tier, with an increased Tier number.
* If there are any logical conflicts in the same tier (different logical operators), a new tier is created internally based on the precedence rules.
* When tiers are referenced in this documentation, they are referred to as lists of LogicObjects. Each LogicObject is codified as `<sym> [subsidiaries]` where `<sym>` is the logical operator/mode for that LogicObject.

## Examples
Starting Expression: `a & b | c`

Tier 0: `& [a, (b | c)]`
Tier 1: `| [b, c]`, `term [a]`
Tier 2: `term [b, c]`

# Basic Parsing Procedure
The following procedure takes place after the user has entered a logical expression.
1. The expression is checked for syntax errors. If any are present, the user is re-prompted for an expression.
2. The parser then parses out all terminal symbols and places them in a HashMap as keys, with booleans as values.
3. The parser then searches the top level of the expression for any logical operators. If it finds any, it creates a LogicObject corresponding to the operator.
    * "Top level" here refers to "not enclosed with any parentheses." If there are parentheses present, the parsing becomes recursive.
    * The contents of the parentheses are treated as one LogicObject.
4. If there are multiple conflicting operators, refer to PRECEDENCE.md for full parser precedence rules. 
5. TODO
