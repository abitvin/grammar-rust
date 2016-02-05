module Abitvin.ByteScript.AstNode
{
    export class AssignmentAtKey implements IAstNode
    {
        private _expr: IAstNode;
        private _key: string;
        private _var: IAstNode;

        constructor( v: IAstNode, key: string, expr: IAstNode )
        {
            this._expr = expr;
            this._key = key;
            this._var = v;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            interperter.popVariable().assignAtKey( this._key, interperter.popVariable() );
            return null;
        }

        public getChild( index: number ): IAstNode
        {
            if( index === 0 ) return this._expr;
            if( index === 1 ) return this._var;
            return null;
        }

        public isDefinitionScope(): boolean
        {
            return false;
        }
    }
} 