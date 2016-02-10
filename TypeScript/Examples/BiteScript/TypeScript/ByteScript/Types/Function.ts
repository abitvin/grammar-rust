namespace Abitvin.ByteScript.Type
{
	export class Function extends BaseType implements IVariable
	{
        private _function: IFunction;

		constructor( fn: IFunction )
		{
			super();
			
            this._function = {
                branch: fn.branch,
                parameters: fn.parameters
            };
		}

		public toFunction(): IFunction
		{
			return this._function;
		}

        public toString(): string
        {
            return "function(" + this._function.parameters.join(",") + ")";
        }
	}
}