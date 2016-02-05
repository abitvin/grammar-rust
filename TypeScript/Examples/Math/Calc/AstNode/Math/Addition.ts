/// <reference path="../../../References.ts"/>

module Abitvin.Calc.AstNode
{
    export class Addition extends BaseOperation implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().add(interperter.popVariable());
        }
    }
}