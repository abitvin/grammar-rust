namespace Abitvin.ByteScript.AstNode
{
    export class GetVariableAtKey implements IAstNode
    {
        private _key: string;
        private _var: IAstNode;

        constructor( v: IAstNode, key: string )
        {
            this._key = key;
            this._var = v;
        }

        public exit( interperter: Interpreter ): IVariable
        {
            return interperter.popVariable().atKey( this._key );
        }

        public getChild( index: number ): IAstNode
        {
            return index === 0 ? this._var : null;
        }

        public isDefinitionScope(): boolean 
        {
            return false;
        }
    }
} 