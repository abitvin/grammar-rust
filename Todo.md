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
* Add `maxOf(n, rule)` or `utmost(n, rule)` or `atMost(n, rule)`
* Idea to create a Grammer<T> class around the Rule<T>s.
    - This reduces a lot of tidious Rule<T>s
    - New methods: nl, ws, wss, wssnl?
    - Embedded whitespace rules.
    - Line counting.  
* Return a failed parsing object instead of an exception when scanning.
    - Return the most matched rule trail, for example to give detailed syntax errors or auto-completion.
* Add rule counts in trail to allow (easy) parsing, for example Python syntax that uses whitespace to delimit program blocks.

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