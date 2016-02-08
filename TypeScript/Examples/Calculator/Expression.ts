/// <reference path="References.ts"/>

namespace Abitvin
{
    interface IScanContext 
    {
        instruction: Instruction;
        precedence?: number;
        rightAssociativity?: boolean; 
        value?: number;
    }
    
    interface IToRpnContext
    {
        index: number;
        rpn: IScanContext[];
    }
    
    enum Instruction
    {
        Add = 1,
        Divide,
        GroupBegin,
        GroupEnd,
        Inverse,
        Module,
        Multiply,
        Power,
        Substract,
        Value
    }
    
    export class Expression
    {
        private static _initialized: boolean = false;
        private static _rootRule: Rule<IScanContext>;
        
        public static evaluate(expression: string): number
        {
            if (!Expression._initialized)
                Expression.initialize();

            try {
                return Expression._rootRule.scan(expression)[0].value;
            }
            catch (e) {
                throw e;
            }
        }
        
        private static initialize(): void
        {
            // Convert tokens to Reverse Polish notation using Dijkstra's Shunting-yard algorithm.
            // https://en.wikipedia.org/wiki/Reverse_Polish_notation
            // https://en.wikipedia.org/wiki/Shunting-yard_algorithm
            const toRpn = (tokens: IScanContext[], ctx: IToRpnContext): void => 
            {
                const opStack: IScanContext[] = [];
                let lastOp: IScanContext = null;
                let exitLoop: boolean = false;

                while (!exitLoop && ctx.index < tokens.length)
                {
                    const n: IScanContext = tokens[ctx.index];

                    switch (n.instruction)
                    {
                        case Instruction.Value:
                        {
                            ctx.rpn.push(n);
                            ctx.index++;
                            break;
                        }

                        case Instruction.GroupBegin:
                        {
                            ctx.index++;
                            toRpn(tokens, ctx);
                            break;
                        }

                        case Instruction.GroupEnd:
                        {
                            exitLoop = true;
                            ctx.index++;
                            break;
                        }

                        default:
                        {
                            if (opStack.length === 0)
                            {
                                lastOp = n;
                                opStack.push(n);
                            }
                            else
                            {
                                if (n.precedence < lastOp.precedence)
                                {
                                    while (opStack.length > 0)
                                        ctx.rpn.push(opStack.pop());
                                }
                                else if (n.precedence === lastOp.precedence && !n.rightAssociativity)
                                {
                                    ctx.rpn.push(opStack.pop());
                                }
                    
                                lastOp = n;
                                opStack.push(n);
                            }

                            ctx.index++;
                        }
                    }
                };

                while (opStack.length > 0)
                    ctx.rpn.push(opStack.pop());
            };
            
            const executeFn = (nodes: IScanContext[], lexeme: string): IScanContext[] => 
            {
                // For these operations we are swapping the values because popping order is wrong.
                const divFn = (right, left) => left / right;
                const modFn = (right, left) => left % right;
                const powFn = (right, left) => Math.pow(left, right);
                const subFn = (right, left) => left - right;
                
                const stack: number[] = [];
                const ctx: IToRpnContext = { index: 0, rpn: [] };
                
                toRpn(nodes, ctx);
                
                ctx.rpn.forEach(t =>
                {
                    switch (t.instruction)
                    {
                        case Instruction.Add: stack.push(stack.pop() + stack.pop()); break;
                        case Instruction.Divide: stack.push(divFn(stack.pop(), stack.pop())); break;
                        case Instruction.Inverse: stack.push(-stack.pop()); break;
                        case Instruction.Module: stack.push(modFn(stack.pop(), stack.pop())); break;
                        case Instruction.Multiply: stack.push(stack.pop() * stack.pop()); break;
                        case Instruction.Power: stack.push(powFn(stack.pop(), stack.pop())); break;
                        case Instruction.Substract: stack.push(subFn(stack.pop(), stack.pop())); break;
                        case Instruction.Value: stack.push(t.value); break;
                    }
                });
                
                return [{ instruction: Instruction.Value, value: stack.pop() }];
            }
            
            
            // Common
            const zero = new Rule<IScanContext>().literal("0");
			const nonZeroDigit = new Rule<IScanContext>().between("1", "9");
			const digit = new Rule<IScanContext>().between("0", "9");
            const ws = new Rule<IScanContext>().anyOf([" ", "\t"]);

            // Number
            const signedInteger = new Rule<IScanContext>().maybe("-").one(nonZeroDigit).noneOrMany(digit);
			const integer = new Rule<IScanContext>().anyOf([zero, signedInteger]);
            const decimalFraction = new Rule<IScanContext>().literal(".").atLeastOne(digit);
			const numbr = new Rule<IScanContext>((b, lexeme) => [{ instruction: Instruction.Value, value: parseFloat(lexeme) }]).one(integer).maybe(decimalFraction);
            
            // Expression group
            const grpBegin = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.GroupBegin }]).literal("(");
            const grpEnd = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.GroupEnd }]).literal(")");

            // Operations
            const opAdd = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Add, precedence: 5 }]).noneOrMany(ws).literal("+");
            const opDiv = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Divide, precedence: 6 }]).noneOrMany(ws).literal("/");
            const opMod = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Module, precedence: 6 }]).noneOrMany(ws).literal("%");
            const opMul = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Multiply, precedence: 6 }]).noneOrMany(ws).literal("*");
            const opPow = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Power, precedence: 7, rightAssociativity: true }]).noneOrMany(ws).literal("^");
            const opSub = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Substract, precedence: 5 }]).noneOrMany(ws).literal("-");

            // Unary operations
            const opInverse = new Rule<IScanContext>((b, l) => [{ instruction: Instruction.Inverse, precedence: 8 }]).literal("-");
            
            // Expression
            const operand = new Rule<IScanContext>();
            const operation = new Rule<IScanContext>().anyOf([opAdd, opDiv, opMod, opMul, opPow, opSub]).noneOrMany(ws).one(operand);
            const exprPart = new Rule<IScanContext>().one(operand).noneOrMany(operation);
            const exprGroup = new Rule<IScanContext>().one(grpBegin).noneOrMany(ws).one(exprPart).noneOrMany(ws).one(grpEnd);
            const unaryble = new Rule<IScanContext>().maybe(opInverse).anyOf([exprGroup]);
            operand.anyOf([numbr, unaryble]);
            
            // Root
            this._rootRule = new Rule<IScanContext>(executeFn).noneOrMany(ws).one(exprPart).noneOrMany(ws);
        }
    }
}