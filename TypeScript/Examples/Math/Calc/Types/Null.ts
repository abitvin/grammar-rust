/// <reference path="../../References.ts"/>

module Abitvin.Calc.Type
{
	export class Null extends BaseType implements IVariable
	{
        public toString(): string
        {
            return "null";
        }
	}
}