namespace Abitvin.ByteScript.AstNode
{
    export class Branch implements IAstNode
    {
        private _nodes: IAstNode[];

        constructor(nodes: IAstNode[])
        {
            this._nodes = nodes;
        }

        public exit(interperter: Interpreter): IVariable
        {
            return null;
        }

        public getChild(index: number): IAstNode
        {
            return this._nodes[index] || null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 