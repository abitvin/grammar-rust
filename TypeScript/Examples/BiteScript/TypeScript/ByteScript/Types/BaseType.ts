namespace Abitvin.ByteScript.Type
{
	export class BaseType implements IVariable
	{
		public add(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do addition from " + BaseType.getTypeName(this) + " onto " + BaseType.getTypeName(rhs) + ".");
		}

        public assign(v: IVariable): void
        {
            throw new Error("Cannot assing " + BaseType.getTypeName(v) + " to " + BaseType.getTypeName(this) + ".");
        }

		public assignAtIndex(index: IVariable, v: IVariable): void
        {
            throw new Error("Cannot assign at a index. " + BaseType.getTypeName(this) + " is not a list.");
        }

		public assignAtKey(key: string, v: IVariable): void
        {
            throw new Error("Cannot assign at a key. " + BaseType.getTypeName(this) + " is not a struct.");
        }

		public atIndex(rhs: IVariable): IVariable
        {
            throw new Error("The type " + BaseType.getTypeName(this) + " has no find-at-index operation.");
        }

		public atKey(key: string): IVariable
        {
            throw new Error("The type " + BaseType.getTypeName(this) + " has no find-at-key operation.");
        }

		public divide(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do division from " + BaseType.getTypeName(this) + " onto " + BaseType.getTypeName(rhs) + ".");
		}

		public inverse(): IVariable
		{
			throw new Error("Cannot do inversion of " + BaseType.getTypeName(this) + ".");
		}

		public modulate(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do modulation from " + BaseType.getTypeName(this) + " onto " + BaseType.getTypeName(rhs) + ".");
		}

		public multiply(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do multplication from " + BaseType.getTypeName(this) + " onto " + BaseType.getTypeName(rhs) + ".");
		}

		public power(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do the power from " + BaseType.getTypeName(this) + " with " + BaseType.getTypeName(rhs) + ".");
		}

		public substract(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do substraction from " + BaseType.getTypeName(this) + " onto " + BaseType.getTypeName(rhs) + ".");
		}

		// Relational operators.

		public equals(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do equality from " + BaseType.getTypeName(this) + " with " + BaseType.getTypeName(rhs) + ".");
		}

		public greaterThen(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do `>` from " + BaseType.getTypeName(this) + " with " + BaseType.getTypeName(rhs) + ".");
		}

		public logicalAnd(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do logical `and` from " + BaseType.getTypeName(this) + " with " + BaseType.getTypeName(rhs) + ".");
		}

		public logicalOr(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do logical `or` from " + BaseType.getTypeName(this) + " with " + BaseType.getTypeName(rhs) + ".");
		}

		public smallerThen(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do `<` from " + BaseType.getTypeName(this) + " with " + BaseType.getTypeName(rhs) + ".");
		}

        // List operations.

        public range(start: IVariable, end: IVariable): IVariable
        {
            throw new Error("Range-between operation not allowed on " + BaseType.getTypeName(this) + ".");
        }

        public rangeFromExpr(start: IVariable): IVariable
        {
            throw new Error("Range-from operation not allowed on " + BaseType.getTypeName(this) + ".");
        }

        public rangeToExpr(end: IVariable): IVariable
        {
            throw new Error("Range-to operation not allowed on " + BaseType.getTypeName(this) + ".");
        }

		// To primative converters.

		public toBoolean(): boolean
		{
			throw new Error("Cannot convert " + BaseType.getTypeName(this) + " to a boolean.");
		}

		public toFunction(): IFunction
		{
			throw new Error(BaseType.getTypeName(this) + " cannot be invoked.");
		}

		public toList(): IVariable[]
		{
			throw new Error(BaseType.getTypeName(this) + " is not a list.");
		}

		public toNumber(): number
		{
			throw new Error("Cannot convert " + BaseType.getTypeName(this) + " to a number.");
		}

		// Helpers.

		protected static getTypeName(v: IVariable): string
		{
			let name: string = v.constructor["name"];

			if (name)
				return name;

			name = this.constructor.toString();
			return name.substring(9, name.indexOf("("));
		}
	}
}