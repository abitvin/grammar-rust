namespace Abitvin.ByteScript.AstNode
{
    export class AssignmentAtIndex implements IAstNode
    {
        private _expr: IAstNode;
        private _index: IAstNode;
        private _var: IAstNode;

        constructor(v: IAstNode, index: IAstNode, expr: IAstNode)
        {
            this._expr = expr;
            this._index = index;
            this._var = v;
        }

        public exit(interperter: Interpreter): IVariable
        {
            interperter.popVariable().assignAtIndex(interperter.popVariable(), interperter.popVariable());
            return null;
        }

        public getChild(index: number): IAstNode
        {
            if (index === 0) return this._expr;
            if (index === 1) return this._index;
            if (index === 2) return this._var;
            return null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 