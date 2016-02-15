namespace Abitvin.ByteScript.AstNode
{
    export class Range implements IAstNode
    {
        private _lhs: IAstNode;
        private _start: IAstNode;
        private _end: IAstNode;

        constructor(lhs: IAstNode, start: IAstNode, end: IAstNode)
        {
            this._lhs = lhs;
            this._start = start;
            this._end = end;
        }

        public exit(interperter: Interpreter): IVariable
        {
            return interperter.popVariable().range(interperter.popVariable(), interperter.popVariable());
        }

        public getChild(index: number): IAstNode
        {
            if (index === 0) return this._end;
            if (index === 1) return this._start;
            if (index === 2) return this._lhs;
            return null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
}  