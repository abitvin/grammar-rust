module Abitvin.ByteScript.AstNode
{
    export class Multiply extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().multiply( interperter.popVariable() );
        }
    }
} 