module Abitvin.ByteScript.Type
{
	export class Number extends BaseType implements IVariable
	{
		private _value: number;

		constructor( value: number )
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
                    this._value = v.toNumber(); 
                    return;
			}

			return super.assign( v );
        }

		// Math operations.

		public add( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean:
				case Number:
					return new Number( this._value + rhs.toNumber() );
			}

			return super.add( rhs );
		}

		public divide( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean:
				case Number:
					return new Number( this._value / rhs.toNumber() );
			}

			return super.divide( rhs );
		}

        public equals( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean: 
				case Number:
					return new Boolean( this._value === rhs.toNumber() );
			}

			return super.equals( rhs );
		}

		public inverse(): IVariable
		{
			return new Number( -this._value );
		}

		public modulate( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean:
				case Number:
					return new Number( this._value % rhs.toNumber() );
			}

			return super.modulate( rhs );
		}

		public multiply( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean:
				case Number:
					return new Number( this._value * rhs.toNumber() );
			}

			return super.multiply( rhs );
		}

		public power( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean:
				case Number:
					return new Number( Math.pow( this._value, rhs.toNumber() ) );
			}

			return super.power( rhs );
		}

		public substract( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean:
				case Number:
					return new Number( this._value - rhs.toNumber() );
			}

			return super.substract( rhs );
		}
		
		// Relational operators.

		public logicalAnd( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean: 
				case Number:
					return new Boolean( this.toBoolean() && rhs.toBoolean() );
			}

			return super.logicalAnd( rhs );
		}

		public logicalOr( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean: 
				case Number:
					return new Boolean( this.toBoolean() || rhs.toBoolean() );
			}

			return super.logicalOr( rhs );
		}

        public smallerThen( rhs: IVariable ): IVariable
		{
			switch( rhs.constructor )
			{
				case Boolean: 
				case Number:
					return new Boolean( this._value < rhs.toNumber() );
			}

			return super.smallerThen( rhs );
		}

		// To primative converters.

		public toBoolean(): boolean
		{
			return this._value !== 0;
		}

		public toNumber(): number
		{
			return this._value;
		}

        public toString(): string
        {
            return this._value.toString();
        }
	}
}