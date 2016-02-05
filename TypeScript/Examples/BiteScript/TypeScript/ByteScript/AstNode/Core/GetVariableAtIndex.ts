module Abitvin.ByteScript.AstNode
{
    export class GetVariableAtIndex extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
            return interperter.popVariable().atIndex( interperter.popVariable() );
        }
    }
} 