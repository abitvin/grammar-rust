namespace Abitvin.ByteScript.AstNode
{
    export class Boolean extends BaseLiteral<boolean> implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
            return new Type.Boolean( this._value );
        }
    }
} 