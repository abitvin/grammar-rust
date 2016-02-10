namespace Abitvin.ByteScript.Type
{
	export class Boolean extends BaseType implements IVariable
	{
		private _value: boolean;

		constructor( value: boolean )
		{
			super();
			this._value = value;
		}

        public assign( v: IVariable ): void
        {
            switch( v.constructor )
			{
				case Boolean: 
				case Number:
                    this._value = v.toBoolean();
                    return;
			}

			return super.assign( v );
        }
		
		public add( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean: return new Number( this.toNumber() + rhs.toNumber() );
				case Number: return new Number( this.toNumber() + rhs.toNumber() );
			}

			return super.add( rhs );
		}

        public logicalOr( rhs: IVariable ): IVariable
        {
            return new Boolean( this._value || rhs.toBoolean() );
        }

		public toBoolean(): boolean
		{
			return this._value;
		}

		public toNumber(): number
		{
			return this._value ? 1 : 0;
		}

        public toString(): string
        {
            return this._value ? "true" : "false";
        }
	}
}