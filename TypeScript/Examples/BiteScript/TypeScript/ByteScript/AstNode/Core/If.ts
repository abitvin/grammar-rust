module Abitvin.ByteScript.AstNode
{
    export class If implements IAstNode
    {
        private _nodes: IAstNode[];

        constructor( nodes: IAstNode[] )
        {
            this._nodes = nodes;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            return null;
        }

        public getChild( index: number, interperter: Interpreter ): IAstNode
        {
			if( ( index === this._nodes.length ) || ( index > 0 && interperter.popVariable().toBoolean() ) )
				return null;

			return this._nodes[index] || null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 