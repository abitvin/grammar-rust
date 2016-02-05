/// <reference path="../References.ts"/>

module Abitvin.Calc
{
    export interface IInterpreterScope
    {
		index: number;
        node: IAstNode;
        parent: IInterpreterScope;
        return: { value: IVariable };
        stackLength: number;
    }
} 