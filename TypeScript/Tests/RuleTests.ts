///<reference path="References.ts"/>

namespace Abitvin.Tests
{
    interface IEmpty {}
    
    enum Token { Loop, Next, Invocation };
    class R extends Rule<Token, IEmpty> {}
    
    const idChar = new R().between("A", "Z");
    const id = new R().atLeast(1, idChar);
    
    //const reserved = new R().anyOf("LOOP", "NEXT");
    //const invoke = new R(() => Token.Invocation).not(reserved).one(id);
    const invoke = new R(() => Token.Invocation).not("NEXT").one(id);
    
    const loop = new R(() => Token.Loop).literal("LOOP");
    const next = new R(() => Token.Next).literal("NEXT");
    
    const codeBlockMore = new R().literal(" ").one(invoke);
    const codeBlock = new R().one(invoke).noneOrMany(codeBlockMore);
    
    const loopStmt = new R().one(loop).literal(" ").maybe(codeBlock).literal(" ").one(next);
    
    const log = (code: string) =>
    {
        const result = loopStmt.scan(code);
        
        if (result.isSuccess)
            console.log(result.branches.map(b => Token[b]));
        else
            console.error(result.errors); 
    }
    
    log("LOOP  NEXT");
    log("LOOP AWESOME NEXT");
    log("LOOP VERY AWESOME NEXT");
    log("LOOP A B C D NEXT");
}