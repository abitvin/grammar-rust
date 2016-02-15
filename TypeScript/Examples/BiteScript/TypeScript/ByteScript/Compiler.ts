///<reference path="../../References.ts"/>

namespace Abitvin.ByteScript
{
    interface IEmpty {}
    
    interface IParseContext 
    {
        astNode?: IAstNode;
        backupAstNode?: IAstNode;
        backupAstNode2?: IAstNode;
        backupId?: string;
        id?: string;
        kind?: Kind;
        lexeme?: string;
        numArguments?: number;
		parameters?: string[];
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
        Equals,
        Expression,
        Function,
        GetAtIndex,
        GetAtKey,
        GetAtScope,
        GreaterThen,
        GroupBegin,
        GroupEnd,
        Identifier,
        Inverse,
        InvokeFunction,
        Literal,
        LogicalAnd,
        LogicalOr,
        Module,
        Multiply,
        Power,
        Range,
        RangeFrom,
        RangeTo,
        SmallerThen,
        Substract,
    }
    
    class BiteRule extends Rule<IParseContext, IEmpty> {}

    export class Compiler
    {
        private static _initialized: boolean = false;
        private static _rootRule: BiteRule;
        private static _variables: { [id: string]: IVariable };

        public static compile(code: string): IAstNode
        {
            if (!Compiler._initialized)
                Compiler.initialize();

            try
            {
                const branch = new AstNode.Branch(Compiler._rootRule.scan(code).branches.map(n => n.astNode));
                const mainDef = new AstNode.Function({ parameters: [], branch: branch });
                const assign = new AstNode.AssignmentAtScope("main", mainDef);
                const getVar = new AstNode.GetVariableAtScope("main");
                const mainCall = new AstNode.FunctionCall("main", getVar, []);
                
                return new AstNode.Branch([assign, mainCall]);
            }
            catch (e)
            {
                throw e;
            }
        }

        private static initialize(): void
        {
            const toRpn = (nodes: IParseContext[], ctx: IToRpnContext): void => 
            {
                const opStack: IParseContext[] = [];
                let lastOp: IParseContext = null;
                let exitLoop: boolean = false;

                while(!exitLoop && ctx.index < nodes.length)
                {
                    const n: IParseContext = nodes[ctx.index];

                    switch (n.kind)
                    {
                        case Kind.Expression:
                        case Kind.Function:
                        case Kind.Identifier:
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

            const buildAst = (nodes: IParseContext[], lexeme: string): IParseContext[] => 
            {
                const ctx: IToRpnContext = { index: 0, rpn: [] };
                toRpn(nodes, ctx);

                let id: string;
                const stack: IParseContext[] = [];

                ctx.rpn.forEach(n =>
                {
                    if (n.kind === Kind.Expression || n.kind === Kind.Function || n.kind === Kind.Identifier || n.kind === Kind.Literal)
                    {
                        stack.push(n);
                    }
                    else
                    {
                        // TODO: Should we change this to a switch statement?
                        if (n.kind === Kind.GetAtIndex)
                        {
                            const index: IAstNode = stack.pop().astNode;
                            const v: IAstNode = stack.pop().astNode;

                            stack.push({ 
                                astNode: new AstNode.GetVariableAtIndex(v, index), 
                                backupAstNode: v,
                                backupAstNode2: index,
                                kind: Kind.GetAtIndex
                            });
                        }
                        else if (n.kind === Kind.GetAtKey)
                        {
                            id = stack.pop().id;
                            const node = stack.pop().astNode;

                            stack.push({ 
                                astNode: new AstNode.GetVariableAtKey(node, id), 
                                backupAstNode: node,
                                backupId: id,
                                kind: Kind.GetAtKey
                            });
                        }
                        else if (n.kind === Kind.GetAtScope)
                        {
                            id = stack.pop().id;

                            stack.push({ 
                                astNode: new AstNode.GetVariableAtScope(id), 
                                backupId: id, 
                                kind: Kind.GetAtScope
                            });
                        }
                        else if (n.kind === Kind.Inverse)
                        {
                            stack.push({ astNode:  new AstNode.Inverse(stack.pop().astNode) });
                        }
                        else if (n.kind === Kind.InvokeFunction)
                        {
                            const args: IAstNode[] = [];

                            while (n.numArguments-- > 0)
                                args.push(stack.pop().astNode);

                            // TODO: Sometimes it's not anonymous, when for example it's not directly returned.
                            stack.push({ astNode: new AstNode.FunctionCall("<anonymous>", stack.pop().astNode, args) });
                        }
                        else if (n.kind === Kind.Range)
                        {
                            const end: IAstNode = stack.pop().astNode;
                            const start: IAstNode = stack.pop().astNode;
                            const lhs: IAstNode = stack.pop().astNode;

                            stack.push({ astNode: new AstNode.Range(lhs, start, end) });
                        }
                        else
                        {
                            const right: IAstNode = stack.pop().astNode;
                            const left: IAstNode = stack.pop().astNode;
                        
                            switch(n.kind)
                            {
                                case Kind.Add: stack.push({ astNode: new AstNode.Addition(left, right) }); break;
                                case Kind.Divide: stack.push({ astNode: new AstNode.Divide(left, right) }); break;
                                case Kind.Equals: stack.push({ astNode: new AstNode.Equals(left, right) }); break;
                                case Kind.GreaterThen: stack.push({ astNode: new AstNode.GreaterThen(left, right) }); break;
                                case Kind.LogicalAnd: stack.push({ astNode: new AstNode.LogicalAnd(left, right) }); break;
                                case Kind.LogicalOr: stack.push({ astNode: new AstNode.LogicalOr(left, right) }); break;
                                case Kind.Module: stack.push({ astNode: new AstNode.Modules(left, right) }); break;
                                case Kind.Multiply: stack.push({ astNode: new AstNode.Multiply(left, right) }); break;
                                case Kind.Power: stack.push({ astNode: new AstNode.Power(left, right) }); break;
                                case Kind.RangeFrom: stack.push({ astNode: new AstNode.RangeFrom(left, right) }); break;
                                case Kind.RangeTo: stack.push({ astNode: new AstNode.RangeTo(left, right) }); break;
                                case Kind.SmallerThen: stack.push({ astNode: new AstNode.SmallerThen(left, right) }); break;
                                case Kind.Substract: stack.push({ astNode: new AstNode.Substract(left, right) }); break;
                            }
                        }
                    }
                });

                const last: IParseContext = stack.pop();
                last.kind = Kind.Expression;
                last.lexeme = lexeme;
                return [last];
            };

            const assignmentStmtNode = (nodes: IParseContext[], l: string): IParseContext[] =>
            {
                const ctx: IToRpnContext = { index: 0, rpn: [] };
                toRpn(nodes, ctx);

                const lhs: IParseContext = nodes[0];

                switch(lhs.astNode.constructor)
                {
                    case AstNode.GetVariableAtKey: return [{ astNode: new AstNode.AssignmentAtKey(lhs.backupAstNode, lhs.backupId, nodes[1].astNode) }];
                    case AstNode.GetVariableAtIndex: return [{ astNode: new AstNode.AssignmentAtIndex(lhs.backupAstNode, lhs.backupAstNode2, nodes[1].astNode) }];
                    case AstNode.GetVariableAtScope: return [{ astNode: new AstNode.AssignmentAtScope(lhs.backupId, nodes[1].astNode) }];
                }

                throw new Error("Compiler error at assignment.");
            };
            
            const booleanNode = (n: IParseContext[], lexeme: string): IParseContext[] =>
				[{ astNode: new AstNode.Boolean(lexeme === "true"), kind: Kind.Literal }];

            const branchNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.Branch(nodes.map(n => n.astNode)) }];

            const commentNode = (n: IParseContext[], l: string): IParseContext[] =>
				[];

            const conditionalNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.Conditional(nodes[0].astNode, nodes[1].astNode) }];

            const funcCallNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.FunctionCall(nodes[0].lexeme, nodes[0].astNode, nodes.splice(1).map(n => n.astNode)), kind: Kind.Function }];

            const funcNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				 [{ astNode: new AstNode.Function({ parameters: nodes[0].parameters, branch: nodes[1].astNode }), kind: Kind.Literal }];

            const grpBeginNode = (n: IParseContext[], l: string): IParseContext[] =>
				[{ kind: Kind.GroupBegin }];

            const grpEndNode = (n: IParseContext[], l: string): IParseContext[] =>
				[{ kind: Kind.GroupEnd }];

            const idNode = (n: IParseContext[], lexeme: string): IParseContext[] =>
				[{ id: lexeme }];

            const ifStmtNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.If(nodes.map(n => n.astNode)) }];

            const listNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.List(nodes.map(n => n.astNode)), kind: Kind.Literal }];

            const numberNode = (n: IParseContext[], lexeme: string): IParseContext[] =>
				[{ astNode: new AstNode.Number(parseFloat(lexeme)), kind: Kind.Literal }];

            const parametersNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ parameters: nodes.map(n => n.id) }];

            const printStmtNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.Print(nodes[0].astNode) }];

            const returnStmtNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.Return(nodes[0].astNode) }];

            const stringNode = (n: IParseContext[], lexeme: string): IParseContext[] =>
				[{ astNode: new AstNode.String(lexeme), kind: Kind.Literal }];

            // TODO: Initialize variable to the structure.
            const structNode = (n: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.Struct(false), kind: Kind.Literal }];

            const variableNode = (n: IParseContext[], lexeme: string): IParseContext[] =>
				[{ astNode: new AstNode.Variable(lexeme), kind: Kind.Identifier }];

            const whileStmtNode = (nodes: IParseContext[], l: string): IParseContext[] =>
				[{ astNode: new AstNode.While(nodes[0].astNode, nodes[1].astNode) }];

            // Operators order by precedence
            // I use the URL below as a reference but the precedence is reversed.
            // http://en.cppreference.com/w/cpp/language/operator_precedence

            const opLogicalOrNode = (): IParseContext[] =>
				[{ kind: Kind.LogicalOr, precedence: 1 }];

            const opLogicalAndNode = (): IParseContext[] =>
				[{ kind: Kind.LogicalAnd, precedence: 2 }];

            const opEqualsNode = (): IParseContext[] =>
				[{ kind: Kind.Equals, precedence: 3 }];

            const opGreaterThenNode = (): IParseContext[] =>
				[{ kind: Kind.GreaterThen, precedence: 4 }];

            const opSmallerThenNode = (): IParseContext[] =>
				[{ kind: Kind.SmallerThen, precedence: 4 }];

            const opAddNode = (): IParseContext[] =>
				[{ kind: Kind.Add, precedence: 5 }];

            const opSubNode = (): IParseContext[] =>
				[{ kind: Kind.Substract, precedence: 5 }];

            const opDivNode = (): IParseContext[] =>
				[{ kind: Kind.Divide, precedence: 6 }];

            const opModNode = (): IParseContext[] =>
				[{ kind: Kind.Module, precedence: 6 }];

            const opMulNode = (): IParseContext[] =>
				[{ kind: Kind.Multiply, precedence: 6 }];
            
            const opPowNode = (): IParseContext[] =>
				[{ kind: Kind.Power, precedence: 7, rightAssociativity: true }];

            const opInverseNode = (): IParseContext[] =>
				[{ kind: Kind.Inverse, precedence: 8 }];

            const opGetAtIndexNode = (nodes: IParseContext[]): IParseContext[] =>
				[{ kind: Kind.GetAtIndex, precedence: 9 }, nodes[0]];
            
            const opGetAtKeyNode = (nodes: IParseContext[]): IParseContext[] =>
				[{ kind: Kind.GetAtKey, precedence: 9 }, { kind: Kind.Identifier, id: nodes[0].id }];
            
            const opGetAtScopeNode = (nodes: IParseContext[]): IParseContext[] =>
				[{ kind: Kind.GetAtScope, precedence: 9 }, { kind: Kind.Identifier, id: nodes[0].id }];
            
            const opInvokeFuncNode = (nodes: IParseContext[]): IParseContext[] =>
				(nodes.unshift({ kind: Kind.InvokeFunction, numArguments: nodes.length, precedence: 9 }), nodes);
            
            const opRangeNode = (nodes: IParseContext[]): IParseContext[] =>
				[{ kind: Kind.Range, precedence: 9 }, nodes[0], nodes[1]];
            
            const opRangeFromNode = (nodes: IParseContext[]): IParseContext[] =>
				[{ kind: Kind.RangeFrom, precedence: 9 }, nodes[0]];

            const opRangeToNode = (nodes: IParseContext[]): IParseContext[] =>
				[{ kind: Kind.RangeTo, precedence: 9 }, nodes[0]];
            
            // Predefines
            const expr = new BiteRule(buildAst);
            const stmt = new BiteRule();
            const funcCallStmt = new BiteRule(funcCallNode);

            // Common
            const zero = new BiteRule().literal("0");
			const nonZeroDigit = new BiteRule().between("1", "9");
			const digit = new BiteRule().between("0", "9");
            const az = new BiteRule().between("a", "z");
            const ws = new BiteRule().anyOf(" ", "\t");
            const eol = new BiteRule().anyOf("\r\n", "\n", "\r");
            const emptyLine = new BiteRule().noneOrMany(ws).one(eol);
            const branch = new BiteRule(branchNode).noneOrMany(stmt);
            const end = new BiteRule().noneOrMany(ws).literal("end");

            // Comment
            const commentChar = new BiteRule().allExcept("\n", "\r");
            const comment = new BiteRule(commentNode).literal("//").noneOrMany(commentChar);

			// Identifier, variable and types
			const bool = new BiteRule(booleanNode).anyOf("false", "true");
            const id = new BiteRule(idNode).atLeast(1, az);
			const signedInteger = new BiteRule().maybe("-").one(nonZeroDigit).noneOrMany(digit);
			const variable = new BiteRule(variableNode).atLeast(1, az);
            const integer = new BiteRule().anyOf(zero, signedInteger);
            const decimalFraction = new BiteRule().literal(".").atLeast(1, digit);
			const numbr = new BiteRule(numberNode).one(integer).maybe(decimalFraction);
            
            const strEscape = new BiteRule().alter("\\\"", "\"");
            const strAllExcept = new BiteRule().allExcept("\"");
            const strChar = new BiteRule().anyOf(strEscape, strAllExcept);
            const strValue = new BiteRule(stringNode).noneOrMany(strChar);
            const str = new BiteRule().literal("\"").one(strValue).literal("\"");

            const listLoop = new BiteRule().noneOrMany(ws).literal(",").noneOrMany(ws).one(expr);
            const listStart = new BiteRule().noneOrMany(ws).one(expr).noneOrMany(listLoop).noneOrMany(ws);
            const list = new BiteRule(listNode).literal("[").maybe(listStart).literal("]");

            const struct = new BiteRule(structNode).literal("{}");

            const funcArgumentsLoop = new BiteRule().noneOrMany(ws).literal(",").noneOrMany(ws).one(expr);
            const funcArguments = new BiteRule().noneOrMany(ws).one(expr).noneOrMany(funcArgumentsLoop).noneOrMany(ws);
            const funcOp = new BiteRule(opInvokeFuncNode).literal("(").maybe(funcArguments).literal(")");

            const funcParametersLoop = new BiteRule().atLeast(1, ws).one(id);
            const funcParametersStart = new BiteRule().noneOrMany(ws).one(id).noneOrMany(funcParametersLoop).noneOrMany(ws);
            const funcParameters = new BiteRule(parametersNode).maybe(funcParametersStart);
            const func = new BiteRule(funcNode).literal("fn").noneOrMany(ws).literal("(").one(funcParameters).literal(")").one(eol).one(branch).one(end);

            // Get variable.
			const atIndex = new BiteRule().literal("[").noneOrMany(ws).one(expr).noneOrMany(ws).literal("]");
            const atKey = new BiteRule().literal(".").one(id);
            const atScope = new BiteRule().one(id);

            const opGetAtIndex = new BiteRule(opGetAtIndexNode).one(atIndex);
            const opGetAtKey = new BiteRule(opGetAtKeyNode).one(atKey);
            const opGetAtScope = new BiteRule(opGetAtScopeNode).one(atScope);
            const getAtIndexOrKey = new BiteRule().anyOf(opGetAtIndex, opGetAtKey);
            const getVar = new BiteRule(buildAst).one(opGetAtScope).noneOrMany(getAtIndexOrKey);
            
            // Expression group
            const grpBegin = new BiteRule(grpBeginNode).literal("(");
            const grpEnd = new BiteRule(grpEndNode).literal(")");

            // Mathematical operators
            const opAdd = new BiteRule(opAddNode).noneOrMany(ws).literal("+");
            const opDiv = new BiteRule(opDivNode).noneOrMany(ws).literal("/");
            const opMod = new BiteRule(opModNode).noneOrMany(ws).literal("%");
            const opMul = new BiteRule(opMulNode).noneOrMany(ws).literal("*");
            const opPow = new BiteRule(opPowNode).noneOrMany(ws).literal("^");
            const opSub = new BiteRule(opSubNode).noneOrMany(ws).literal("-");

            // Unary operations
            const opInverse = new BiteRule(opInverseNode).literal("-");
            
            // Range operations
            const opRange = new BiteRule(opRangeNode).literal("[").noneOrMany(ws).one(expr).noneOrMany(ws).literal("..").noneOrMany(ws).one(expr).noneOrMany(ws).literal("]");
            const opRangeFrom = new BiteRule(opRangeFromNode).literal("[").one(expr).noneOrMany(ws).literal("..").noneOrMany(ws).literal("]");
            const opRangeTo = new BiteRule(opRangeToNode).literal("[").noneOrMany(ws).literal("..").one(expr).literal("]");

            // Relational operators
            const opEq = new BiteRule(opEqualsNode).noneOrMany(ws).literal("==");
            const opGt = new BiteRule(opGreaterThenNode).noneOrMany(ws).literal(">");
            const opSt = new BiteRule(opSmallerThenNode).noneOrMany(ws).literal("<");

            // Logical operators
            const opLAnd = new BiteRule(opLogicalAndNode).atLeast(1, ws).literal("and ");
            const opLOr = new BiteRule(opLogicalOrNode).atLeast(1, ws).literal("or ");
            
            // Expressions
            const getOpsOrFuncInvocation = new BiteRule().anyOf(opGetAtIndex, opGetAtKey, opRange, opRangeFrom, opRangeTo, funcOp);
            const operand = new BiteRule();
            const operation = new BiteRule().anyOf(opAdd, opDiv, opMod, opMul, opPow, opSub, opEq, opLAnd, opLOr, opGt, opSt).noneOrMany(ws).one(operand);
            const exprLoop = new BiteRule().one(operand).noneOrMany(operation);
            const exprGroup = new BiteRule().one(grpBegin).noneOrMany(ws).one(exprLoop).noneOrMany(ws).one(grpEnd);
            const unaryble = new BiteRule().maybe(opInverse).anyOf(variable, exprGroup);
            operand.anyOf(bool, numbr, list, str, func, struct, unaryble).noneOrMany(getOpsOrFuncInvocation);
            expr.one(exprLoop);
            
            // Print statement
            const printStmt = new BiteRule(printStmtNode).literal("print").atLeast(1, ws).one(expr);

            // Assigment statement
            const assignmentStmt = new BiteRule(assignmentStmtNode).one(getVar).noneOrMany(ws).literal("=").noneOrMany(ws).one(expr);

            // If statement
            const elseStmt = new BiteRule().noneOrMany(ws).literal("else").noneOrMany(ws).one(eol).one(branch);
            const elseIfStmt = new BiteRule(conditionalNode).noneOrMany(ws).literal("else if").atLeast(1, ws).one(expr).noneOrMany(ws).one(eol).one(branch);
            const ifStmt = new BiteRule(conditionalNode).literal("if").atLeast(1, ws).one(expr).noneOrMany(ws).one(eol).one(branch);
            const ifStmtRoot = new BiteRule(ifStmtNode).one(ifStmt).noneOrMany(elseIfStmt).maybe(elseStmt).one(end);

            // While statement
			const whileStmt = new BiteRule(whileStmtNode).literal("while").atLeast(1, ws).one(expr).noneOrMany(ws).one(eol).one(branch).one(end);

            // Function invocement
            funcCallStmt.one(getVar).literal("(").maybe(funcArguments).literal(")");
            
            // Return statement
            const returnStmt = new BiteRule(returnStmtNode).literal("return").atLeast(1, ws).one(expr);

            // Any statement (implementation)
            stmt.noneOrMany(ws).anyOf(emptyLine, comment, assignmentStmt, funcCallStmt, ifStmtRoot, printStmt, returnStmt, whileStmt).noneOrMany(ws).maybe(eol);

            // Root
            this._rootRule = new BiteRule().atLeast(1, stmt);
        }
    }
} 