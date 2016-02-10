namespace Abitvin.ByteScript.AstNode
{
    export class Substract extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().substract( interperter.popVariable() );
        }
    }
} 