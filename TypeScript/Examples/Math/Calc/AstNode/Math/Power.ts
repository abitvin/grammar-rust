/// <reference path="../../../References.ts"/>

module Abitvin.Calc.AstNode
{
    export class Power extends BaseOperation implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().power(interperter.popVariable());
        }
    }
} 