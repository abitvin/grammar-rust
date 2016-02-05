/// <reference path="../../../References.ts"/>

module Abitvin.Calc.AstNode
{
    export class Divide extends BaseOperation implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().divide(interperter.popVariable());
        }
    }
} 