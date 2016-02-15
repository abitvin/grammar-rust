namespace Abitvin.ByteScript.AstNode
{
    export class Print implements IAstNode
    {
        private _expr: IAstNode;

        constructor(expr: IAstNode)
        {
            this._expr = expr;
        }

        public exit(interperter: Interpreter): IVariable
        {
            console.log(`Print: ${interperter.popVariable()}`);
            return null;
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