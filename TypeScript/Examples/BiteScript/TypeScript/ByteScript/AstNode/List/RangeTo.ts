module Abitvin.ByteScript.AstNode
{
    export class RangeTo extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
            return interperter.popVariable().rangeToExpr( interperter.popVariable() );
        }
    }
}  