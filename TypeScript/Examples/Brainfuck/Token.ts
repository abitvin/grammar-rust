/// <reference path="References.ts"/>

namespace Abitvin.Brainfuck
{
    export enum Token
    {
        DecrementByte = 1,
        DecrementPointer,
        EndWhile,
        IncrementByte,
        IncrementPointer,
        InputByte,
        PrintByte,
        StartWhile
    }
}