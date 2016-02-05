module Abitvin.ByteScript
{
    export interface IVariable
    {
        assign( v: IVariable ): void;
        assignAtIndex( index: IVariable, v: IVariable ): void;
        assignAtKey( key: string, v: IVariable ): void;

        // Retrieve operations.
        atIndex( v: IVariable ): IVariable;
        atKey( key: string ): IVariable;
        
        // Function invocation.
        //invoke( id: string, args: IVariable[], interperter: Interpreter ): IAstNode;

		// Math operations
		add( rhs: IVariable ): IVariable;
		divide( rhs: IVariable ): IVariable;
		inverse(): IVariable;
		modulate( rhs: IVariable ): IVariable;
		multiply( rhs: IVariable ): IVariable;
		power( rhs: IVariable ): IVariable;
		substract( rhs: IVariable ): IVariable;
		
		// Logical 
		equals( rhs: IVariable ): IVariable;
		greaterThen( rhs: IVariable ): IVariable;
		logicalAnd( rhs: IVariable ): IVariable;
		logicalOr( rhs: IVariable ): IVariable;
		smallerThen( rhs: IVariable ): IVariable;

        // List operations
		range( start: IVariable, end: IVariable ): IVariable;
        rangeFromExpr( start: IVariable ): IVariable;
        rangeToExpr( end: IVariable ): IVariable;
        
		// To primative type converters.
        toBoolean(): boolean;
		toFunction(): IFunction;
		toNumber(): number;
		toList(): IVariable[];
		toString(): string;
    }
}