# Basic Syntax Rules
* All parentheses must be balanced. Failure to properly open/close sets of parentheses will result in a syntax error.
* The words ` true ` and ` false ` are reserved to represent straight logical values. These are case-insensitive. 
* All non-reserved symbols are assumed to be terminal logical variables. 
    * These variables have string-class names, meaning any length of characters terminated with a space or reserved symbols is considered one terminal.
* The logical NOT symbol ` ! ` may ONLY directly precede a terminal logical variable or a parenthesized operation.

# Recognized Symbols
` & ` - Logical AND
` | ` - Logical OR
` = ` - Logical EQUIVALENCE
` ! ` - Logical NOT

# TBD Recognized Symbols (may be implemented later)
` ^ ` - Logical XOR/INEQUIVALENCE
` > ` - Logical IMPLIES
