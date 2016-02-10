namespace Abitvin.ByteScript.AstNode
{
    export class Addition extends BaseOperation implements IAstNode
    {
        public exit( interperter: Interpreter ): IVariable
        {
			return interperter.popVariable().add( interperter.popVariable() );
        }
    }
}