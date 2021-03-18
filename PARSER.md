# Basic Parsing Procedure
The following procedure takes place after the user has entered a logical expression.
1. The expression is checked for syntax errors. If any are present, the user is re-prompted for an expression.
2. The parser then parses out all terminal symbols and places them in a HashMap as keys, with booleans as values.
3. The parser then searches the top level of the expression for any logical operators. If it finds any, it creates a LogicObject corresponding to the operator.
    * "Top level" here refers to "not enclosed with any parentheses." If there are parentheses present, the parsing becomes recursive.
    * The contents of the parentheses are treated as one LogicObject.
4. If there are multiple conflicting operators, refer to PRECEDENCE.md for full parser precedence rules. 
5. TODO
