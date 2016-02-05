/// <reference path="../../References.ts"/>

module Abitvin.Calc.Type
{
	export class BaseType implements IVariable
	{
		public add(rhs: IVariable): IVariable
		{
			throw new Error("Cannot do addition from " + BaseType.getTypeName(this) + " onto " + BaseType.getTypeName(rhs) + ".");
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

        public toNumber(): number
		{
			throw new Error("Cannot convert " + BaseType.getTypeName(this) + " to a number.");
		}

		// Helpers.

		protected static getTypeName(v: IVariable): string
		{
			var name: string = v.constructor["name"];

			if (name)
				return name;

			name = this.constructor.toString();
			return name.substring(9, name.indexOf("("));
		}
	}
}