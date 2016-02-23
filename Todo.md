Todo
====

Ideas
-----
* Parse BNF grammer?
* Make colorfier; maybe for another repository. (Working on it...)
* Make examples for parsing different language formatting styles, for example: Python using spaces/tabs, JavaScript using {}, etc. 

TypeScript general
------------------
* Replace Build.ts with Visual Studio Code task. 

TypeScript/Rule.ts
------------------
* Tell in the documentation why do you need to do anyOf([float, signedInt]) and not anyOf([signedInt, float]).
* Error with line count. 
* Add rule counts in trail to allow (easy) parsing, for example Python syntax that uses whitespace to delimit program blocks.
* Create unit tests
* Use algebraic types for checking errors. This will go better in TypeScript 1.8; And 1.8 is out now!
* Allow reading from different streams, for example an array of numbers or booleans.
* Implement Rule parsing language which I worked on before.
* Maybe still implement `fallback(branchFn)` method? Which is not a scanning method but acts like a safety net for bundling all characters not passing the scanning rules. 

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