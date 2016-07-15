///<reference path="References.ts"/>

namespace Abitvin.Tests
{
    interface IEmpty {}
    
    /*enum Token { Loop, Next, Invocation };
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
    */
    /*
    const assertEq = (a: any, b: any): void =>
    {
        if (a !== b)
            console.error("Nope!", a, b);
    };
    
    const dot = new Rule<boolean, IEmpty>(() => [true]).literal(".");
    const x = new Rule<boolean, IEmpty>(() => [false]).literal("x");
    
    const r = new Rule<boolean, IEmpty>((b, l) =>
    {
        assertEq(b.length, 3);
        assertEq(b[0], false);
        assertEq(b[1], false);
        assertEq(b[2], true);
        assertEq(l, "xx.");
        
        return b;
    });
    
    console.log(r.noneOrMany(dot).noneOrMany(x).scan("xx."));
    //let r = code2.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("xx.");
    */


    const num = new Rule<number, IEmpty>((b, l) => [3]).literal("3");
    //const xxx1 = new Rule<number, IEmpty>().one(num);
    const xxx2 = new Rule<number, IEmpty>().noneOrMany(num);

    const add = new Rule<number, IEmpty>((b, l) =>
    {
        switch (b.length)
        {
            case 0: throw new Error("Grammer error A");
            case 1: return b;
            case 2: return [b[0] + b[1]];
            default: throw new Error("Grammer error B");
        }
    //}).one(xxx1).one(xxx2);
    }).one(num).one(xxx2);

    console.log("=========================================================");
    console.log("Sumx:", add.scan("3"));
    //console.log("Sum:", add.scan("34"));


    
    /*
    let mut dummy = false;

        let num_fn = |_: Vec<i32>, l: &str, _: &mut bool| {
            vec![l.parse().unwrap()]
        };

        let mut digit = Rule::new(None);
        digit.char_in('0', '9');

        let mut num = Rule::new(Some(Box::new(num_fn)));
        num.one(digit);

        let mut xxx1 = Rule::new(None);
        unsafe { xxx1.one_raw(&num); }

        let mut xxx2 = Rule::new(None);
        unsafe { xxx2.none_or_many_raw(&num); }

        let add_fn = |b: Vec<i32>, _: &str, _: &mut bool| {
            match b.len() {
                0 => panic!("Grammer error AAA!"),
                1 => vec![b[0]],
                2 => vec![b[0] + b[1]],
                _ => panic!("Grammer error BBB!"),
            }
        };
        
        let mut add: Rule<i32, bool> = Rule::new(Some(Box::new(add_fn)));
        add.one(xxx1).one(xxx2);

        if let Ok(b) = add.scan("3", &mut dummy) {
            assert_eq!(b.len(), 1);
            assert_eq!(b[0], 3);
        }
        else {
            assert!(false);
        }

        if let Ok(b) = add.scan("34", &mut dummy) {
            assert_eq!(b.len(), 1);
            assert_eq!(b[0], 7);
        }
        else {
            assert!(false);
        }
    */
}