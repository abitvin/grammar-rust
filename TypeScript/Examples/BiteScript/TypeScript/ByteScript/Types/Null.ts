namespace Abitvin.ByteScript.Type
{
	export class Null extends BaseType implements IVariable
	{
        public toString(): string
        {
            return "null";
        }
	}
}