module Abitvin.ByteScript.Type
{
	export class String extends BaseType implements IVariable
	{
        private _value: string;

        constructor( value: string )
        {
            this._value = value;
            super();
        }

        public add( rhs: IVariable ): IVariable
        {
            switch( rhs.constructor )
            {
                case Boolean: 
                case Number: 
                case String: 
                    return new String( this._value + rhs.toString() );
            }

            return super.add( rhs );
        }

        public assign( v: IVariable ): void
        {
            this._value = v.toString();
        }

        public atIndex( rhs: IVariable ): IVariable
        {
            switch( rhs.constructor )
            {
                case Boolean: 
                case Number: 
                    var index: number = rhs.toNumber();

                    if( index < 0 || index >= this._value.length )
                        throw new Error( "Runtime out-of-bounds error." );

                    return new String( this._value[index] );
            }

            return super.atIndex( rhs );
        }

        public multiply( rhs: IVariable ): IVariable
        {
            switch( rhs.constructor )
            {
                case Boolean: 
                case Number: 
                {
                    var count: number = rhs.toNumber();
                    var value: string = "";

                    while( count-- > 0 )
                        value += this._value;

                    return new String( value );
                }
            }

            return super.multiply( rhs );
        }

        public range( start: IVariable, end: IVariable ): IVariable
        {
            var startIndex: number = start.toNumber();
            var endIndex: number = end.toNumber();

            this.checkBounds( startIndex, endIndex );

            return new String( this._value.substring( startIndex, endIndex + 1 ) );
        }

        public rangeFromExpr( start: IVariable ): IVariable
        {
            var startIndex: number = start.toNumber();
            var endIndex: number = this._value.length - 1;

            this.checkBounds( startIndex, endIndex );

            return new String( this._value.substring( startIndex, endIndex + 1 ) );
        }

        public rangeToExpr( end: IVariable ): IVariable
        {
            var startIndex: number = 0;
            var endIndex: number = end.toNumber();

            this.checkBounds( startIndex, endIndex );

            return new String( this._value.substring( startIndex, endIndex + 1 ) );
        }

        public toString(): string
        {
            return this._value;
        }

        // Helpers.

        private checkBounds( start: number, end: number ): void
        {
            if( start < 0 || start >= this._value.length )
                throw new Error( "Runtime out-of-bounds start index at " + start + " error." );

            if( end < 0 || end >= this._value.length )
                    throw new Error( "Runtime out-of-bounds end index at " + end + " error." );

            if( end < start )
                    throw new Error( "Runtime error. End index of " + end + " smaller then start index of " + start + "." );
        }
	}
}