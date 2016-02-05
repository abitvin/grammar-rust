/// <reference path="../References.ts"/>

module Abitvin.Calc
{
    export interface IAstNode
    {
        exit(interperter: Interpreter): IVariable;
        getChild(index: number, interperter?: Interpreter): IAstNode;
    }
} 