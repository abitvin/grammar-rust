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
        var code: string = document.getElementById( "code" ).textContent;
        var program: ByteScript.IAstNode = ByteScript.Compiler.compile( code );
        var interpreter = new ByteScript.Interpreter( program );

        var step = () =>
        {
            if( interpreter.step() )
                setTimeout( step, 0 );
        };

        step();
    }

    function errors(): void
    {
        console.log( "Running: errors" );

        var code = "var asd = 14";

        var alpha = new Rule<boolean, IEmpty>().between( "a", "z" );
        var digit = new Rule<boolean, IEmpty>().between( "0", "9" );
        var id = new Rule<boolean, IEmpty>().atLeast(1, alpha);
        var integer = new Rule<boolean, IEmpty>().atLeast(1, digit);
        var ws = new Rule<boolean, IEmpty>().anyOf([ " ", "\t" ]);
        
        var varStmt = new Rule<boolean, IEmpty>().literal( "var" ).atLeast(2, ws).one( id ).noneOrMany( ws ).literal( "=" ).noneOrMany( ws ).one( integer );

        varStmt.scan( code );
    }

    function noCode(): void
    {
        var code: string = "";

        var alpha = new Rule<Kind, IEmpty>( () => [Kind.Alpha] ).between( "a", "z" );
        var digit = new Rule<Kind, IEmpty>( () => [Kind.Digit] ).between( "0", "9" );
        var noCode = new Rule<Kind, IEmpty>( () => [Kind.NoCode] ).literal( "" );

        var root = new Rule<Kind, IEmpty>().anyOf([ alpha, digit, noCode ]);

        root.scan( code ).forEach( kind => console.log( Kind[kind] ) );
    }
    
    // errors();
    // interperter();
    // noCode();
} 