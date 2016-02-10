namespace Abitvin.ByteScript.AstNode
{
    export class BaseOperation
    {
        protected _left: IAstNode;
        protected _right: IAstNode;

        constructor( left: IAstNode, right: IAstNode )
        {
            this._left = left;
            this._right = right;
        }

        public getChild( index: number ): IAstNode
        {
            if( index === 0 ) return this._right;
            if( index === 1 ) return this._left;
            return null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 