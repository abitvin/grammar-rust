namespace Abitvin.ByteScript.AstNode
{
    export class Modules extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().modulate( interperter.popVariable() );
        }
    }
} 