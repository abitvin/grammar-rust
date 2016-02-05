/// <reference path="../../References.ts"/>

module Abitvin.Calc.Type
{
	export class Number implements IVariable
	{
		private _value: number;
        
		constructor(value: number)
		{
			this._value = value;
		}

        public add(rhs: IVariable): IVariable
		{
            return new Number(this._value + rhs.toNumber());
		}

		public divide(rhs: IVariable): IVariable
		{
            return new Number(this._value / rhs.toNumber());
		}

        public inverse(): IVariable
		{
			return new Number(-this._value);
		}

		public modulate(rhs: IVariable): IVariable
		{
            return new Number(this._value % rhs.toNumber());
		}

		public multiply(rhs: IVariable): IVariable
		{
            return new Number(this._value * rhs.toNumber());
		}

		public power(rhs: IVariable): IVariable
		{
            return new Number(Math.pow(this._value, rhs.toNumber()));
		}

		public substract(rhs: IVariable): IVariable
		{
            return new Number(this._value - rhs.toNumber());
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