/// <reference path="../../References.ts"/>

module Abitvin.Calc
{
    export interface IVariable
    {
        add(rhs: IVariable): IVariable;
		divide(rhs: IVariable): IVariable;
		inverse(): IVariable;
		modulate(rhs: IVariable): IVariable;
		multiply(rhs: IVariable): IVariable;
		power(rhs: IVariable): IVariable;
		substract(rhs: IVariable): IVariable;

        toNumber(): number;
		toString(): string;
    }
}