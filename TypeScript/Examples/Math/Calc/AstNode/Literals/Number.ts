/// <reference path="../../../References.ts"/>

module Abitvin.Calc.AstNode
{
    export class Number extends BaseLiteral<number> implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
            return new Type.Number(this._value);
        }
    }
} 