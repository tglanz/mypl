module Lexer where
    
import qualified Text.Parsec.Token as Token
import qualified Text.Parsec.Language as Language

import Text.Parsec.String (Parser)

commentLine :: String
commentLine = "//"

reservedOpNames :: [String]
reservedOpNames = 
    [ "+"
    , "-"
    , "*"
    , "/" 
    ]

types :: [String]
types =
    [ "i8"
    , "i16"
    , "i32"
    , "u8"
    , "u16"
    , "u32"
    , "bool"
    ]

keywords :: [String]
keywords = [ "record"
    , "union"
    , "function"
    , "match"
    , "return"
    ]


reservedNames :: [String]
reservedNames =
    keywords ++ types

caseSensitive :: Bool
caseSensitive = True

definition :: Language.LanguageDef ()
definition = Language.emptyDef
    { Token.commentLine = commentLine
    , Token.reservedOpNames = reservedOpNames
    , Token.reservedNames = reservedNames
    , Token.caseSensitive = caseSensitive
    }

lexer :: Token.TokenParser ()
lexer = Token.makeTokenParser definition

integer :: Parser Integer
integer = Token.integer lexer

float :: Parser Double
float = Token.float lexer

parens :: Parser a -> Parser a
parens = Token.parens lexer

braces :: Parser a -> Parser a
braces = Token.braces lexer

commaSep :: Parser a -> Parser [a]
commaSep = Token.commaSep lexer

semiSep :: Parser a -> Parser [a]
semiSep = Token.semiSep lexer

identifier :: Parser String
identifier = Token.identifier lexer

reserved :: String -> Parser ()
reserved = Token.reserved lexer

reservedOp :: String -> Parser ()
reservedOp = Token.reservedOp lexer

symbol :: String -> Parser String
symbol = Token.symbol lexer

rightArrow :: Parser String
rightArrow = symbol "->"

colon :: Parser String
colon = Token.colon lexer