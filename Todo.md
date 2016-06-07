Todo
====

Ideas
-----
* Parse BNF grammer?
* Make colorfier; maybe for another repository. (Working on it...)
* Make examples for parsing different language formatting styles, for example: Python using spaces/tabs, JavaScript using {}, etc. 

Rust general
------------
* Use tips from here: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
* Interesting read, maybe not for the Rule library, but anyway: https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html

TypeScript general
------------------
* Replace Build.ts with Visual Studio Code task. 

TypeScript/Grammer.ts
---------------------
* Refactor and cleanup code.
* Show nicer errors.

TypeScript/Rule.ts
------------------
* Tell in the documentation why do you need to do anyOf([float, signedInt]) and not anyOf([signedInt, float]).
* Error with line count. 
* Create unit tests
* Use algebraic types for checking errors. This will go better in TypeScript 1.8; And 1.8 is out now!
* Allow reading from different streams, for example an array of numbers or booleans.
* Optimize `anyOf`. Now the items must all be a string or a Rule. But that is not really needed.
* Now if ranges with minimum of 0 (or undefined rules) do not match we don't invoke the branche function. I think we should... For example parsing an empty string.
* I think if you use `noneOrMany` or expecting 0 rules you should always return the branches. Which makes it more invarient to implement to branch functions. Now it doesn't and you need to use `if` statements to do some checking.     
  
TypeScript/Examples
-------------------
* Create Tabbed hierarchy key value parser.
* Create XML parser.

TypeScript/Examples/BiteScript
------------------------------
* Format code to new guidelines.
* Rename namespaces.
* Refactor everything else.
* Rename it to ToyScript the emphasize that it's a toy language?

TypeScript/Examples/Brainfuck
-----------------------------
* Test these examples: http://www.hevanet.com/cristofd/brainfuck/

TypeScript/Examples/Common rules
--------------------------------
* Add Grammer API rule expressions
* Fancy the visuals.

TypeScript/Examples/INI Reader
--------------------------------
* Move to Grammer API.
* Fancy the visuals.

TypeScript/Examples/JSON Reader
--------------------------------
* Move to Grammer API.
* Fancy the visuals.