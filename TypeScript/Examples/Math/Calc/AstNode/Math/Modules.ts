/// <reference path="../../../References.ts"/>

module Abitvin.Calc.AstNode
{
    export class Modules extends BaseOperation implements IAstNode
    {
        public exit(interperter: Interpreter): IVariable
        {
			return interperter.popVariable().modulate(interperter.popVariable());
        }
    }
} 