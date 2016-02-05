/// <reference path="../../Rule.ts"/>

namespace Abitvin
{
    interface IParseContext
    {
        prop?: string;
        value?: any;
    }
    
    export class JsonReader
    {
        private static _root: Rule<IParseContext>;
        private static _initialized: boolean = false;
        
        public static initialize(): void
        {
            const zero = new Rule<IParseContext>().literal("0");
            const nonZeroDigit = new Rule<IParseContext>().between("1", "9");
            const ws = new Rule<IParseContext>().anyOf([" ", "\n", "\r", "\t"]);
            const dec = new Rule<IParseContext>().between("0", "9");
            const oct = new Rule<IParseContext>().between("0", "7");
            const af = new Rule<IParseContext>().between("a", "f");
            const AF = new Rule<IParseContext>().between("A", "F");
            const hex = new Rule<IParseContext>().anyOf([dec, af, AF]);
            
            const value = new Rule<IParseContext>();
            
            // Boolean
            const parseBoolFn = (branches: IParseContext[], lexeme: string) => [{ value: lexeme === "true" }];
            const bool = new Rule<IParseContext>(parseBoolFn).anyOf(["true", "false"]);
            
            // Null
            const parseNullFn = (branches: IParseContext[], lexeme: string) => [{ value: null }];
            const nul = new Rule<IParseContext>(parseNullFn).literal("null");
            
            // Undefined
            const parseUndefinedFn = (branches: IParseContext[], lexeme: string) => [{ value: undefined }];
            const und = new Rule<IParseContext>(parseUndefinedFn).literal("undefined");
            
            // Number
            const parseHexFn = (branches: IParseContext[], lexeme: string) => [{ value: parseInt(lexeme.substr(2), 16) }];
            const parseOctFn = (branches: IParseContext[], lexeme: string) => [{ value: parseInt(lexeme.substr(1), 8) }];
            const parseFloatFn = (branches: IParseContext[], lexeme: string) => [{ value: parseFloat(lexeme) }];
            
            const nonZeroSignedInt = new Rule<IParseContext>().maybe("-").one(nonZeroDigit).noneOrMany(dec);
            const signedInt = new Rule<IParseContext>().anyOf([zero, nonZeroSignedInt]);
            const fraction = new Rule<IParseContext>().literal(".").atLeastOne(dec);
			const hexNum = new Rule<IParseContext>(parseHexFn).literal("0x").atLeastOne(hex);
            const octNum = new Rule<IParseContext>(parseOctFn).literal("0").atLeastOne(oct);
            const numDec = new Rule<IParseContext>(parseFloatFn).one(signedInt).maybe(fraction);
            const numDecFraction = new Rule<IParseContext>(parseFloatFn).literal(".").atLeastOne(dec);
            
            // String
            const combineCharsFn = (branches: IParseContext[], lexeme: string) => [{ value: branches.map(b => b.value).join("") }];
            const parseCharCodeFn = (branches: IParseContext[], lexeme: string) => [{ value: String.fromCharCode(parseInt(lexeme.substr(2), 16)) }];
            const passLexemeFn = (branches: IParseContext[], lexeme: string) => [{ value: lexeme }];
            
            const strEscapeControl = new Rule<IParseContext>(passLexemeFn).alter(["\\0", "\0", "\\b", "\b", "\\f", "\f", "\\n", "\n", "\\r", "\r", "\\t", "\t", "\\v", "\v", "\\\"", "\""]);
            const strEscapeLatin1 = new Rule<IParseContext>(parseCharCodeFn).literal("\\x").one(hex).one(hex);
            const strEscapeUTF16 = new Rule<IParseContext>(parseCharCodeFn).literal("\\u").one(hex).one(hex).one(hex).one(hex);
            const strEscapeUnknown = new Rule<IParseContext>(passLexemeFn).literal("\\");
            const strAllExceptBs2 = new Rule<IParseContext>(passLexemeFn).allExcept(["\""]);
            const strChar = new Rule<IParseContext>().anyOf([strEscapeControl, strEscapeLatin1, strEscapeUTF16, strEscapeUnknown, strAllExceptBs2]);
            const strValue = new Rule<IParseContext>(combineCharsFn).noneOrMany(strChar);
            const str = new Rule<IParseContext>().literal("\"").one(strValue).literal("\"");
            
            // Array
            const parseArrayFn = (branches: IParseContext[], lexeme: string) => [{ value: branches.map(b => b.value) }];
            
            const arrItem = new Rule<IParseContext>().literal(",").noneOrMany(ws).one(value).noneOrMany(ws);
            const arrItems = new Rule<IParseContext>().one(value).noneOrMany(ws).noneOrMany(arrItem).maybe(","); 
            const arr = new Rule<IParseContext>(parseArrayFn).literal("[").noneOrMany(ws).maybe(arrItems).noneOrMany(ws).literal("]");
            
            // Object
            const parsePropNameFn = (branches: IParseContext[], lexeme: string) => [{ prop: lexeme }];
            const parsePropFn = (branches: IParseContext[], lexeme: string) => [{ prop: branches[0].prop, value: branches[1].value }];
            
            const parseObjFn = (branches: IParseContext[], lexeme: string) =>
            {
                const obj = {};
                branches.forEach(i => obj[i.prop] = i.value);
                return [{ value: obj }];
            };
            
            const objPropName = new Rule<IParseContext>(parsePropNameFn).anyOf([str]);
            const objProp = new Rule<IParseContext>(parsePropFn).one(objPropName).noneOrMany(ws).literal(":").noneOrMany(ws).one(value);
            const objItem = new Rule<IParseContext>().literal(",").noneOrMany(ws).one(objProp).noneOrMany(ws);
            const objItems = new Rule<IParseContext>().one(objProp).noneOrMany(ws).noneOrMany(objItem).maybe(","); 
            const obj = new Rule<IParseContext>(parseObjFn).literal("{").noneOrMany(ws).maybe(objItems).noneOrMany(ws).literal("}");
            
            value.anyOf([bool, nul, und, hexNum, octNum, numDec, numDecFraction, str, arr, obj]);
            
            this._root = value;
        }
        
        public static read(input: string): any
        {
            if (!this._initialized)
                this.initialize();
            
            return this._root.scan(input)[0].value;
        }
    }
}