/// <reference path="../../Rule.ts"/>

namespace Abitvin
{
    interface IScanContext
    {
        prop?: string;
        value?: any;
    }
    
    export class JsonReader
    {
        private static _root: Rule<IScanContext>;
        private static _initialized: boolean = false;
        
        public static initialize(): void
        {
            const zero = new Rule<IScanContext>().literal("0");
            const nonZeroDigit = new Rule<IScanContext>().between("1", "9");
            const ws = new Rule<IScanContext>().anyOf([" ", "\n", "\r", "\t"]);
            const dec = new Rule<IScanContext>().between("0", "9");
            const oct = new Rule<IScanContext>().between("0", "7");
            const af = new Rule<IScanContext>().between("a", "f");
            const AF = new Rule<IScanContext>().between("A", "F");
            const hex = new Rule<IScanContext>().anyOf([dec, af, AF]);
            
            // TODO const varName = new 
            const value = new Rule<IScanContext>();
            
            // Boolean
            const parseBoolFn = (branches: IScanContext[], lexeme: string) => [{ value: lexeme === "true" }];
            const bool = new Rule<IScanContext>(parseBoolFn).anyOf(["true", "false"]);
            
            // Null
            const parseNullFn = (branches: IScanContext[], lexeme: string) => [{ value: null }];
            const nul = new Rule<IScanContext>(parseNullFn).literal("null");
            
            // Undefined
            const parseUndefinedFn = (branches: IScanContext[], lexeme: string) => [{ value: undefined }];
            const und = new Rule<IScanContext>(parseUndefinedFn).literal("undefined");
            
            // Number
            const parseHexFn = (branches: IScanContext[], lexeme: string) => [{ value: parseInt(lexeme.substr(2), 16) }];
            const parseOctFn = (branches: IScanContext[], lexeme: string) => [{ value: parseInt(lexeme.substr(1), 8) }];
            const parseFloatFn = (branches: IScanContext[], lexeme: string) => [{ value: parseFloat(lexeme) }];
            
            const nonZeroSignedInt = new Rule<IScanContext>().maybe("-").one(nonZeroDigit).noneOrMany(dec);
            const signedInt = new Rule<IScanContext>().anyOf([zero, nonZeroSignedInt]);
            const fraction = new Rule<IScanContext>().literal(".").atLeastOne(dec);
			const hexNum = new Rule<IScanContext>(parseHexFn).literal("0x").atLeastOne(hex);
            const octNum = new Rule<IScanContext>(parseOctFn).literal("0").atLeastOne(oct);
            const numDec = new Rule<IScanContext>(parseFloatFn).one(signedInt).maybe(fraction);
            const numDecFraction = new Rule<IScanContext>(parseFloatFn).literal(".").atLeastOne(dec);
            
            // String
            const combineCharsFn = (branches: IScanContext[], lexeme: string) => [{ value: branches.map(b => b.value).join("") }];
            const parseCharCodeFn = (branches: IScanContext[], lexeme: string) => [{ value: String.fromCharCode(parseInt(lexeme.substr(2), 16)) }];
            const passLexemeFn = (branches: IScanContext[], lexeme: string) => [{ value: lexeme }];
            
            const strEscapeControl = new Rule<IScanContext>(passLexemeFn).alter(["\\0", "\0", "\\b", "\b", "\\f", "\f", "\\n", "\n", "\\r", "\r", "\\t", "\t", "\\v", "\v", "\\\"", "\""]);
            const strEscapeLatin1 = new Rule<IScanContext>(parseCharCodeFn).literal("\\x").one(hex).one(hex);
            const strEscapeUTF16 = new Rule<IScanContext>(parseCharCodeFn).literal("\\u").one(hex).one(hex).one(hex).one(hex);
            const strEscapeUnknown = new Rule<IScanContext>(passLexemeFn).literal("\\");
            const strAllExceptBs = new Rule<IScanContext>(passLexemeFn).allExcept(["\""]);
            const strChar = new Rule<IScanContext>().anyOf([strEscapeControl, strEscapeLatin1, strEscapeUTF16, strEscapeUnknown, strAllExceptBs]);
            const strValue = new Rule<IScanContext>(combineCharsFn).noneOrMany(strChar);
            const str = new Rule<IScanContext>().literal("\"").one(strValue).literal("\"");
            
            // Array
            const parseArrayFn = (branches: IScanContext[], lexeme: string) => [{ value: branches.map(b => b.value) }];
            
            const arrItem = new Rule<IScanContext>().literal(",").noneOrMany(ws).one(value).noneOrMany(ws);
            const arrItems = new Rule<IScanContext>().one(value).noneOrMany(ws).noneOrMany(arrItem).maybe(","); 
            const arr = new Rule<IScanContext>(parseArrayFn).literal("[").noneOrMany(ws).maybe(arrItems).noneOrMany(ws).literal("]");
            
            // Object
            const parsePropNameFn = (branches: IScanContext[], lexeme: string) => [{ prop: lexeme }];
            const parsePropFn = (branches: IScanContext[], lexeme: string) => [{ prop: branches[0].prop, value: branches[1].value }];
            
            const parseObjFn = (branches: IScanContext[], lexeme: string) =>
            {
                const obj = {};
                branches.forEach(i => obj[i.prop] = i.value);
                return [{ value: obj }];
            };
            
            
            const objPropName = new Rule<IScanContext>(parsePropNameFn).anyOf([str/* TODO , varName */]);
            const objProp = new Rule<IScanContext>(parsePropFn).one(objPropName).noneOrMany(ws).literal(":").noneOrMany(ws).one(value);
            const objItem = new Rule<IScanContext>().literal(",").noneOrMany(ws).one(objProp).noneOrMany(ws);
            const objItems = new Rule<IScanContext>().one(objProp).noneOrMany(ws).noneOrMany(objItem).maybe(","); 
            const obj = new Rule<IScanContext>(parseObjFn).literal("{").noneOrMany(ws).maybe(objItems).noneOrMany(ws).literal("}");
            
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