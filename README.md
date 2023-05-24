Implements the following features:
1. comparison operators 
2. if expressions 

You only have to do one of these. No matter which one you choose, you will have to write the grammar, implement the associated parser combinators, modify the interpreter to execute the new feature, and finally write tests to validate its functionality.



- Evaluation order: Condition is evaluated first, followed by the true/false branch.
- Type consistency: Ensure both expressions have compatible types.
- Short-circuit evaluation: If condition is true, false branch is not evaluated.
- Return value: The if-expression should return a single value that can be assigned to a variable or used in an expression.
