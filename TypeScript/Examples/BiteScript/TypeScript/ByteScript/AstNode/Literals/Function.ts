namespace Abitvin.ByteScript.AstNode
{
    export class Function extends BaseLiteral<IFunction> implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
            return new Type.Function(this._value);
        }
    }
} 