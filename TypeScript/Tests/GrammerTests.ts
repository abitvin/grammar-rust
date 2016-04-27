///<reference path="References.ts"/>

namespace Abitvin.Tests
{
    /*
        OK  literal:                Any char except the control characters
        OK  one rule:               < >             all except: >
        -   range:                  { , }
        -   any of:                 ( | )
        OK  alter:                  (~ | , | )      all except: , | )
        OK  between:                [ - ]           all except: - ]
        OK  all chars except:       [^ ]            all except: ]
        -   at least 1:             +
        -   maybe:                  ?
        -   none or many:           *
        -   any char:               .
        -   eof:                    $
        -   none or many spaces:    Space character    
        -   at least one space:     _
        
        asdasd<asdasd>
        (asdasd)
        asdasd[asd]
        

    */
            
    interface IEmpty {}
    
    const test = new Grammer<string, IEmpty>();
    
    test.add("test a", "aaa", (b, l) => l);
    test.add("test-b", "bbb", (b, l) => l);
    test.add("test_c", "ccc", (b, l) => l);
    test.add("<{[]}>", "ddd", (b, l) => l);
    test.add(" ", "eee", (b, l) => l);
    test.add("id-test", "(<test a>|<test-b>|<test_c>|<<{[]}\\>>|< >)+");
    console.log(test.scan("id-test", "aaabbbcccdddeeedddeeecccaaabbb"));
    
    test.add("alter-test", "(~\\n,\n|asdasd,asdasxd|aa,DD|yyy,ZZZ)+", (b, l) => l);
    console.log(test.scan("alter-test", "yyy\\n\\nasdasdaaasdasd\\naaasdasdyyy"));
    
    debugger;
    
    
    const calc = new Grammer<number, IEmpty>();
    calc.declare("add", "expr", "mul");
    calc.add("num", "[0-9]+", (b, l) => parseInt(l));
    calc.add("brackets", "\\(<expr>\\)");   // Identity function
    calc.add("mul", "(<num>|<brackets>)(\\*<mul>)?", b => b.length === 1 ? b : b[0] * b[1]);
    calc.add("add", "<mul>(\\+<add>)?", b => b.length === 1 ? b : b[0] + b[1]);
    calc.add("expr", "(<add>|<brackets>)");
    
    console.log(calc.scan("expr", "2*(3*4*5)")); // 120
    console.log(calc.scan("expr", "2*(3+4)*5")); // 70
    console.log(calc.scan("expr", "((2+3*4+5))")); // 19
    
    
    
    interface IBinaryOperator
    {
        operator: Operator;
        left: CalcAstNode;
        right: CalcAstNode;
    }  
    
    interface IAssignemnt
    {
        variable: string;
        expr: CalcAstNode;
    }

    interface IFunction
    {
        arguments: CalcAstNode[],
        name: string
    }
    
    enum Operator { multiply, add }
    
    type CalcAstNode = IBinaryOperator | number | IFunction | string | IAssignemnt;

    const calcAst = new Grammer<CalcAstNode, IEmpty>();
    calcAst.declare("add", "expr", "function", "mul", "base");
    
    //calcAst.ws("(\\ |\t)");
    
    calcAst.add("num", "[0-9]+", (b, l) => [parseInt(l)]);
    calcAst.add("symbol", "[a-z]+", (b, l) => [l]);
    
    calcAst.add("brackets", "\\( <expr> \\)");   // Identity function
    calcAst.add("mul", "<base>( \\* <mul>)?", b => b.length === 1 ? b : [{ operator: Operator.multiply, left: b[0], right: b[1] }]);
    calcAst.add("add", "<mul>( \\+ <add>)?", b => b.length === 1 ? b : [{ operator: Operator.add, left: b[0], right: b[1] }]);
    calcAst.add("expr", "<add>");
    calcAst.add("function", "<symbol>\\( (<expr>( \\, <expr>)*)? \\)", b => [{ name: <string>b[0], arguments: b.slice(1)}])
    calcAst.add("variable", "<symbol>")
    calcAst.add("base", "(<function>|<num>|<brackets>|<variable>)");

    calcAst.add("assignment", "<symbol> := <expr>", b => [{ variable: <string>b[0], expr: b[1]}]);
    calcAst.add("statement", " (<assignment>|<expr>) (;|$) ")
    calcAst.add("codeblock", "<statement>*");
    
    calcAst.add("code", "<codeblock>");
    
    const showAst = (code: string) => 
    {
        const result = calcAst.scan("code", code);
        console.log(JSON.stringify(result.branches));
    }

    showAst("14;");
    showAst("(14);");
    showAst("2* ( 3*4*5 );");
    showAst("2*(\t3+4)*5;");
    showAst("foo();");
    showAst("2*foo();");
    showAst("foo(7);");
    showAst("2*foo(7 +   bar( 4*5  ));");
    showAst("foo(  5,10   )");
    showAst("2*foo(5  ,  10  ,  34,345, 45)");
    showAst("x    :=  \t\t\t  2*y+1  ;  y := 12   ");
    
    /*
    //grammer.add("digit", "x+");
    //grammer.add("multiplication", "<digit> (* <multiplication>)?");
    //grammer.add("addition", "<multiplication> (+ <addition)?")
    */
    
    
    enum Tabbed {
        Indent,
        Bla,
        Bar
    }
    
    const tabbed = new Grammer<number, IEmpty>();
    tabbed.add("indent", "(\\ {4}|\t)", () => Tabbed.Indent);
    tabbed.add("bla", "bla", () => Tabbed.Bla);
    tabbed.add("bar", "bar", () => Tabbed.Bar);
    tabbed.add("stmt", "<indent>*(<bla>|<bar>)(\r?\n|$)");
    tabbed.add("code", "<stmt>*");
    
    const showTabbed = (code: string) =>
    {
        const result = tabbed.scan("code", code);
        
        if (result.isSuccess)
            console.log(result.branches.map(t => Tabbed[t]));    
        else
            console.error(result.errors);
    };
    
    showTabbed("\tbla");
    showTabbed("    \t    \tbla\n\t\tbar");
    showTabbed("    \t    \tbla\n\t\tbar\r\n        bar");
}