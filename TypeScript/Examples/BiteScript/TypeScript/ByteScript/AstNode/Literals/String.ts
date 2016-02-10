namespace Abitvin.ByteScript.AstNode
{
    export class String extends BaseLiteral<string> implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
            return new Type.String( this._value );
        }
    }
} 