namespace Abitvin.ByteScript.AstNode
{
    // TODO: BaseLiteral<T> struct type.
    export class Struct extends BaseLiteral<boolean> implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
            return new Type.Struct();
        }
    }
} 