///<reference path="../References.ts"/>

namespace Abitvin
{
    enum Kind
    {
        Alpha,
        Comment,
        Digit,
        NoCode
    }
    
    interface IEmpty {}

    function interperter(): void
    {
        const code: string = document.getElementById( "code" ).textContent;
        const program: ByteScript.IAstNode = ByteScript.Compiler.compile( code );
        const interpreter = new ByteScript.Interpreter( program );

        const step = () =>
        {
            if( interpreter.step() )
                setTimeout( step, 0 );
        };

        step();
    }

    function errors(): void
    {
        console.log( "Running: errors" );

        const code = "var asd = 14";

        const alpha = new Rule<boolean, IEmpty>().between( "a", "z" );
        const digit = new Rule<boolean, IEmpty>().between( "0", "9" );
        const id = new Rule<boolean, IEmpty>().atLeast(1, alpha);
        const integer = new Rule<boolean, IEmpty>().atLeast(1, digit);
        const ws = new Rule<boolean, IEmpty>().anyOf([ " ", "\t" ]);
        
        const varStmt = new Rule<boolean, IEmpty>().literal( "var" ).atLeast(2, ws).one( id ).noneOrMany( ws ).literal( "=" ).noneOrMany( ws ).one( integer );

        varStmt.scan( code );
    }

    function noCode(): void
    {
        const code: string = "";

        const alpha = new Rule<Kind, IEmpty>(() => [Kind.Alpha] ).between("a", "z");
        const digit = new Rule<Kind, IEmpty>(() => [Kind.Digit] ).between("0", "9");
        const noCode = new Rule<Kind, IEmpty>(() => [Kind.NoCode] ).literal("");

        const root = new Rule<Kind, IEmpty>().anyOf(alpha, digit, noCode);
        
        const result = root.scan(code);
        
        if (result.isSuccess) 
            result.branches.forEach( kind => console.log( Kind[kind] ) );
        else
            console.log("Error", result.errors);
    }
    
    // errors();
    // interperter();
    // noCode();
} 