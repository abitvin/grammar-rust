/// <reference path="../../Rule.ts"/>

namespace Abitvin
{
    interface IEmpty {}
    
    interface IScanContext
    {
        prop?: string;
        value?: any;
    }
    
    class JsonRule extends Rule<IScanContext, IEmpty> 
    {
        private _ws = new Rule<IScanContext, IEmpty>().anyOf(" ", "\t");
        
        public ws(): this
        {
            return this.noneOrMany(this._ws);
        }
    }
    
    export class JsonReader
    {
        private static _root: JsonRule;
        private static _initialized: boolean = false;
        
        public static initialize(): void
        {
            const zero = new JsonRule().literal("0");
            const nonZeroDigit = new JsonRule().between("1", "9");
            const dec = new JsonRule().between("0", "9");
            const oct = new JsonRule().between("0", "7");
            const af = new JsonRule().between("a", "f");
            const AF = new JsonRule().between("A", "F");
            const letter = new JsonRule().between("a", "z");
            const LETTER = new JsonRule().between("A", "Z");
            const hex = new JsonRule().anyOf(dec, af, AF);
            const alphaNum = new JsonRule().anyOf(letter, LETTER, dec);
            
            const value = new JsonRule();
            
            // Boolean
            const parseBoolFn = (branches: IScanContext[], lexeme: string) => [{ value: lexeme === "true" }];
            const bool = new JsonRule(parseBoolFn).anyOf("true", "false");
            
            // Null
            const parseNullFn = (branches: IScanContext[], lexeme: string) => [{ value: null }];
            const nul = new JsonRule(parseNullFn).literal("null");
            
            // Undefined
            const parseUndefinedFn = (branches: IScanContext[], lexeme: string) => [{ value: undefined }];
            const und = new JsonRule(parseUndefinedFn).literal("undefined");
            
            // Number
            const parseHexFn = (branches: IScanContext[], lexeme: string) => [{ value: parseInt(lexeme.substr(2), 16) }];
            const parseOctFn = (branches: IScanContext[], lexeme: string) => [{ value: parseInt(lexeme.substr(1), 8) }];
            const parseFloatFn = (branches: IScanContext[], lexeme: string) => [{ value: parseFloat(lexeme) }];
            
            const nonZeroSignedInt = new JsonRule().maybe("-").one(nonZeroDigit).noneOrMany(dec);
            const signedInt = new JsonRule().anyOf(zero, nonZeroSignedInt);
            const fraction = new JsonRule().literal(".").atLeast(1, dec);
			const hexNum = new JsonRule(parseHexFn).literal("0x").atLeast(1, hex);
            const octNum = new JsonRule(parseOctFn).literal("0").atLeast(1, oct);
            const numDec = new JsonRule(parseFloatFn).one(signedInt).maybe(fraction);
            const numDecFraction = new JsonRule(parseFloatFn).literal(".").atLeast(1, dec);
            
            // String
            const combineCharsFn = (branches: IScanContext[], lexeme: string) => [{ value: branches.map(b => b.value).join("") }];
            const parseCharCodeFn = (branches: IScanContext[], lexeme: string) => [{ value: String.fromCharCode(parseInt(lexeme.substr(2), 16)) }];
            const passLexemeFn = (branches: IScanContext[], lexeme: string) => [{ value: lexeme }];
            
            const strEscapeControl = new JsonRule(passLexemeFn).alter("\\0", "\0", "\\b", "\b", "\\f", "\f", "\\n", "\n", "\\r", "\r", "\\t", "\t", "\\v", "\v", "\\\"", "\"");
            const strEscapeLatin1 = new JsonRule(parseCharCodeFn).literal("\\x").one(hex).one(hex);
            const strEscapeUTF16 = new JsonRule(parseCharCodeFn).literal("\\u").one(hex).one(hex).one(hex).one(hex);
            const strEscapeUnknown = new JsonRule(passLexemeFn).literal("\\");
            const strAllExceptBs = new JsonRule(passLexemeFn).allExcept("\"");
            const strChar = new JsonRule().anyOf([strEscapeControl, strEscapeLatin1, strEscapeUTF16, strEscapeUnknown, strAllExceptBs]);
            const strValue = new JsonRule(combineCharsFn).noneOrMany(strChar);
            const str = new JsonRule().literal("\"").one(strValue).literal("\"");
            
            // Array
            const parseArrayFn = (branches: IScanContext[], lexeme: string) => [{ value: branches.map(b => b.value) }];
            
            const arrItem = new JsonRule().literal(",").ws().one(value).ws();
            const arrItems = new JsonRule().one(value).ws().noneOrMany(arrItem).maybe(","); 
            const arr = new JsonRule(parseArrayFn).literal("[").ws().maybe(arrItems).ws().literal("]");
            
            // Object
            const parsePropNameFn = (branches: IScanContext[], lexeme: string) => [{ prop: lexeme }];
            const parsePropFn = (branches: IScanContext[], lexeme: string) => [{ prop: branches[0].prop, value: branches[1].value }];
            
            const parseObjFn = (branches: IScanContext[], lexeme: string) =>
            {
                const obj = {};
                branches.forEach(i => obj[i.prop] = i.value);
                return [{ value: obj }];
            };
            
            const varName = new JsonRule().anyOf(letter, LETTER).noneOrMany(alphaNum); // Note that this is not the full range of allowed characters in JavaScript variables.
            const objPropName = new JsonRule(parsePropNameFn).anyOf(str, varName);
            const objProp = new JsonRule(parsePropFn).one(objPropName).ws().literal(":").ws().one(value);
            const objItem = new JsonRule().literal(",").ws().one(objProp).ws();
            const objItems = new JsonRule().one(objProp).ws().noneOrMany(objItem).maybe(","); 
            const obj = new JsonRule(parseObjFn).literal("{").ws().maybe(objItems).ws().literal("}");
            
            value.anyOf(bool, nul, und, hexNum, octNum, numDec, numDecFraction, str, arr, obj);
            
            this._root = value;
        }
        
        public static read(input: string): RuleResult<IScanContext, IEmpty>
        {
            if (!this._initialized)
                this.initialize();
            
            return this._root.scan(input);
        }
    }
}