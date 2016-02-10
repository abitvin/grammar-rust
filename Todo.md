Todo
====

Ideas
-----
* Parse BNF grammer?
* Make colorfier; maybe for another repository.

TypeScript general
------------------
* Replace `module` with `namespace` keyword.
* Use `const` and `let` instead of `var` when we can.

TypeScript/Rule.ts
------------------
* Format code to new guidelines.
* Support for variadic arguments.
* Tell in the documentation why do you need to do anyOf([float, signedInt]) and not anyOf([signedInt, float]).
* Replace `atLeastOne(rule)` with `atLeast(n, rule)` or `minOf(n, rule)`
* Add `maxOf(n, rule)` or `utmost(n, rule)`
* Add `preciseOf(n, rule)` or `exact(n, rule)`
* Idea to create a Grammer<T> class around the Rule<T>s.
    - This reduces a lot of tidious Rule<T>s
    - New methods: nl, ws, wss, wssnl?
    - Embedded whitespace rules.   

TypeScript/Examples
-------------------
* Create JSON parser.
* Create Tabbed hierachie key value parser.
* Create XML parser.
* Create INI parser.

TypeScript/Examples/BiteScript
------------------------------
* Rename namespaces.
* Rename IInterpreterScope to IInterperterScope.
* Remove Rule.ts
* Refactor it everything else.
* Rename it to ToyScript the emphasize that it's a toy language?

TypeScript/Examples/Brainfuck
-----------------------------
* Test these examples: http://www.hevanet.com/cristofd/brainfuck/

TypeScript/Examples/Common rules
* Add common rules for example: parsing numbers, strings, array

TypeScript/Examples/Math
------------------------
* Remove Rule2.ts prototype
* Refactor
* Rename "IInterpreterScope" to "IInterperterScope"