module Abitvin.ByteScript.AstNode
{
    export class LogicalAnd extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().logicalAnd( interperter.popVariable() );
        }
    }
} 