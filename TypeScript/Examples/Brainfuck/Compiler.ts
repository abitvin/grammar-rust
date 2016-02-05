/// <reference path="References.ts"/>

namespace Abitvin.Brainfuck
{
    export class Compiler
    {
        private _root: Rule<Token>;

        constructor()
        {
            const decByte = new Rule<Token>(() => [Token.DecrementByte]).literal("-");
            const decPointer = new Rule<Token>(() => [Token.DecrementPointer]).literal("<");
            const incByte = new Rule<Token>(() => [Token.IncrementByte]).literal("+");
            const incPointer = new Rule<Token>(() => [Token.IncrementPointer]).literal(">");
            const inputByte = new Rule<Token>( () => [Token.InputByte] ).literal(",");
            const printByte = new Rule<Token>(() => [Token.PrintByte]).literal(".");
            const startWhile = new Rule<Token>(() => [Token.StartWhile]).literal("[");
            const endWhile = new Rule<Token>(() => [Token.EndWhile]).literal("]");
            const ignore = new Rule<Token>().allExcept(["-", "+", "<", ">", ".", ",", "[", "]"]);

            const whileLoop = new Rule<Token>();
            const instruction = new Rule<Token>().anyOf([decByte, decPointer, incByte, incPointer, inputByte, printByte, whileLoop, ignore]);
            const branch = new Rule<Token>().noneOrMany(instruction);
            whileLoop.one(startWhile).one(branch).one(endWhile);

            this._root = branch;
        }

        public compile(code: string): Token[]
        {
            return this._root.scan(code);
        }
    }
}