namespace Abitvin.ByteScript.AstNode
{
    export class Equals extends BaseOperation implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().equals(interperter.popVariable());
        }
    }
} 