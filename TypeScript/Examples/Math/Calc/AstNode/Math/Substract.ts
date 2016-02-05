/// <reference path="../../../References.ts"/>

module Abitvin.Calc.AstNode
{
    export class Substract extends BaseOperation implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().substract(interperter.popVariable());
        }
    }
} 