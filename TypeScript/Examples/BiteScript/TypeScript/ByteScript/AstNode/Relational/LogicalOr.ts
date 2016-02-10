namespace Abitvin.ByteScript.AstNode
{
    export class LogicalOr extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().logicalOr( interperter.popVariable() );
        }
    }
} 