Todo
====

Ideas
-----
* Parse BNF grammer?
* Make colorfier; maybe for another repository.
* Make examples for parsing different language formatting styles, for example: Python using spaces/tabs, JavaScript using {}, etc. 

TypeScript general
------------------
* Use `const` and `let` instead of `var` when we can.

TypeScript/Rule.ts
------------------
* Tell in the documentation why do you need to do anyOf([float, signedInt]) and not anyOf([signedInt, float]).
* Return a failed parsing object instead of an exception when scanning.
    - Return the most matched rule trail, for example to give detailed syntax errors or auto-completion.
    - Line counting.
* Add rule counts in trail to allow (easy) parsing, for example Python syntax that uses whitespace to delimit program blocks.
* Create unit tests
* Use algebraic types for checking errors. This will go better in TypeScript 1.8
* Do we really need the setBranchFn()? 

TypeScript/Examples
-------------------
* Create Tabbed hierachie key value parser.
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
* Fancy the visuals.

TypeScript/Examples/INI Reader
--------------------------------
* Fancy the visuals.

TypeScript/Examples/JSON Reader
--------------------------------
* Fancy the visuals.