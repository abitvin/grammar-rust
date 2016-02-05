module Abitvin.ByteScript.AstNode
{
    export class Power extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().power( interperter.popVariable() );
        }
    }
} 