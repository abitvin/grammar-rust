namespace Abitvin.ByteScript.AstNode
{
    export class Inverse implements IAstNode
    {
        private _expr: IAstNode;

        constructor(expr: IAstNode)
        {
            this._expr = expr;
        }

        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().inverse();
        }

        public getChild(index: number): IAstNode
        {
            return index === 0 ? this._expr : null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 