/// <reference path="../References.ts"/>

module Abitvin.Calc
{
    export interface IX
    {

    }

    interface IParseContext 
    {
        astNode?: IAstNode;
        kind?: Kind;
        lexeme?: string;
        precedence?: number;
        rightAssociativity?: boolean;
    }

    interface IToRpnContext
    {
        index: number;
        rpn: IParseContext[];
    }

    enum Kind
    {
        Add,
        Divide,
        Expression,
        GroupBegin,
        GroupEnd,
        Inverse,
        Literal,
        Module,
        Multiply,
        Power,
        Substract,
    }

    export class Compiler
    {
        private static _initialized: boolean = false;
        private static _rootRule: Rule2<IParseContext, IX>;

        public static compile(code: string): IAstNode
        {
            if (!Compiler._initialized)
                Compiler.initialize();

            try {
                return Compiler._rootRule.evaluate(code)[0].astNode;
            }
            catch (e) {
                throw e;
            }
        }

        private static initialize(): void
        {
            var toRpn = (nodes: IParseContext[], ctx: IToRpnContext): void => 
            {
                var opStack: IParseContext[] = [];
                var lastOp: IParseContext = null;
                var exitLoop: boolean = false;

                while (!exitLoop && ctx.index < nodes.length)
                {
                    var n: IParseContext = nodes[ctx.index];

                    switch (n.kind)
                    {
                        case Kind.Expression:
                        case Kind.Literal:
                        {
                            ctx.rpn.push(n);
                            ctx.index++;
                            break;
                        }

                        case Kind.GroupBegin:
                        {
                            ctx.index++;
                            toRpn(nodes, ctx);
                            break;
                        }

                        case Kind.GroupEnd:
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

            var buildAst = (nodes: IParseContext[], lexeme: string): IParseContext[] => 
            {
                var ctx: IToRpnContext = { index: 0, rpn: [] };
                toRpn(nodes, ctx);

                var stack: IParseContext[] = [];

                ctx.rpn.forEach((n: IParseContext) =>
                {
                    if (n.kind === Kind.Expression || n.kind === Kind.Literal)
                    {
                        stack.push(n);
                    }
                    else
                    {
                        if (n.kind === Kind.Inverse)
                        {
                            stack.push({ astNode: new AstNode.Inverse(stack.pop().astNode) });
                        }
                        else
                        {
                            var right: IAstNode = stack.pop().astNode;
                            var left: IAstNode = stack.pop().astNode;
                        
                            switch( n.kind )
                            {
                                case Kind.Add: stack.push({ astNode: new AstNode.Addition(left, right) }); break;
                                case Kind.Divide: stack.push({ astNode: new AstNode.Divide(left, right) }); break;
                                case Kind.Module: stack.push({ astNode: new AstNode.Modules(left, right) }); break;
                                case Kind.Multiply: stack.push({ astNode: new AstNode.Multiply(left, right) }); break;
                                case Kind.Power: stack.push({ astNode: new AstNode.Power(left, right) }); break;
                                case Kind.Substract: stack.push({ astNode: new AstNode.Substract(left, right) }); break;
                            }
                        }
                    }
                });

                var last: IParseContext = stack.pop();
                last.kind = Kind.Expression;
                last.lexeme = lexeme;
                return [last];
            };

            var grpBeginNode = (n: IParseContext[], l: string): IParseContext[] =>
				[{ kind: Kind.GroupBegin }];

            var grpEndNode = ( n: IParseContext[], l: string ): IParseContext[] =>
				[{ kind: Kind.GroupEnd }];

            var numberNode = (n: IParseContext[], lexeme: string): IParseContext[] =>
				[{ astNode: new AstNode.Number( parseFloat( lexeme ) ), kind: Kind.Literal }];
            
            // Operators order by precedence
            // I use the URL below as a reference but the precedence is reversed.
            // http://en.cppreference.com/w/cpp/language/operator_precedence

            var opAddNode = (): IParseContext[] =>
				[{ kind: Kind.Add, precedence: 5 }];

            var opSubNode = (): IParseContext[] =>
				[{ kind: Kind.Substract, precedence: 5 }];

            var opDivNode = (): IParseContext[] =>
				[{ kind: Kind.Divide, precedence: 6 }];

            var opModNode = (): IParseContext[] =>
				[{ kind: Kind.Module, precedence: 6 }];

            var opMulNode = (): IParseContext[] =>
				[{ kind: Kind.Multiply, precedence: 6 }];
            
            var opPowNode = (): IParseContext[] =>
				[{ kind: Kind.Power, precedence: 7, rightAssociativity: true }];

            var opInverseNode = (): IParseContext[] =>
				[{ kind: Kind.Inverse, precedence: 8 }];
            
            // Common
            var zero = new Rule2<IParseContext, IX>().literal("0");
			var nonZeroDigit = new Rule2<IParseContext, IX>().between("1", "9");
			var digit = new Rule2<IParseContext, IX>().between("0", "9");
            var ws = new Rule2<IParseContext, IX>().anyOf([" ", "\t"]);

            // Number
            var signedInteger = new Rule2<IParseContext, IX>().maybe("-").one(nonZeroDigit).noneOrMany(digit);
			var integer = new Rule2<IParseContext, IX>().anyOf([zero, signedInteger]);
            var decimalFraction = new Rule2<IParseContext, IX>().literal(".").atLeastOne(digit);
			var numbr = new Rule2<IParseContext, IX>(numberNode).one(integer).maybe(decimalFraction);
            
            // Expression group
            var grpBegin = new Rule2<IParseContext, IX>(grpBeginNode).literal("(");
            var grpEnd = new Rule2<IParseContext, IX>(grpEndNode).literal(")");

            // Mathematical operators
            var opAdd = new Rule2<IParseContext, IX>(opAddNode).noneOrMany(ws).literal("+");
            var opDiv = new Rule2<IParseContext, IX>(opDivNode).noneOrMany(ws).literal("/");
            var opMod = new Rule2<IParseContext, IX>(opModNode).noneOrMany(ws).literal("%");
            var opMul = new Rule2<IParseContext, IX>(opMulNode).noneOrMany(ws).literal("*");
            var opPow = new Rule2<IParseContext, IX>(opPowNode).noneOrMany(ws).literal("^");
            var opSub = new Rule2<IParseContext, IX>(opSubNode).noneOrMany(ws).literal("-");

            // Unary operations
            var opInverse = new Rule2<IParseContext, IX>(opInverseNode).literal("-");
            
            // Expression
            var operand = new Rule2<IParseContext, IX>();
            var operation = new Rule2<IParseContext, IX>().anyOf([opAdd, opDiv, opMod, opMul, opPow, opSub]).noneOrMany(ws).one(operand);
            var exprLoop = new Rule2<IParseContext, IX>().one(operand).noneOrMany(operation);
            var exprGroup = new Rule2<IParseContext, IX>().one(grpBegin).noneOrMany(ws).one(exprLoop).noneOrMany(ws).one(grpEnd);
            var unaryble = new Rule2<IParseContext, IX>().maybe(opInverse).anyOf([exprGroup]);
            operand.anyOf([numbr, unaryble]);
            var expr = new Rule2<IParseContext, IX>(buildAst).one(exprLoop);
            
            // Root
            this._rootRule = expr;
        }
    }
}