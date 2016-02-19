/// <reference path="References.ts"/>

namespace Abitvin.Brainfuck
{
    interface IEmpty {}
    
    class BfRule extends Rule<Token, IEmpty> {};
    
    export class Compiler
    {
        private _root: BfRule;

        constructor()
        {
            const decByte = new BfRule(() => [Token.DecrementByte]).literal("-");
            const decPointer = new BfRule(() => [Token.DecrementPointer]).literal("<");
            const incByte = new BfRule(() => [Token.IncrementByte]).literal("+");
            const incPointer = new BfRule(() => [Token.IncrementPointer]).literal(">");
            const inputByte = new BfRule( () => [Token.InputByte] ).literal(",");
            const printByte = new BfRule(() => [Token.PrintByte]).literal(".");
            const startWhile = new BfRule(() => [Token.StartWhile]).literal("[");
            const endWhile = new BfRule(() => [Token.EndWhile]).literal("]");
            const ignore = new BfRule().all();

            const whileLoop = new BfRule();
            const instruction = new BfRule().anyOf(decByte, decPointer, incByte, incPointer, inputByte, printByte, whileLoop, ignore);
            const branch = new BfRule().noneOrMany(instruction);
            whileLoop.one(startWhile).one(branch).one(endWhile);

            this._root = branch;
        }

        public compile(code: string): Token[]
        {
            const result: RuleResult<Token, IEmpty> = this._root.scan(code);
            
            if (!result.isSuccess)
                throw result.errors;
                
            return result.branches;
        }
    }
}