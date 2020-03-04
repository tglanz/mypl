module Parser where

import qualified Text.Parsec
import qualified Text.Parsec.Char
import qualified Text.Parsec.String

import Text.Parsec (try, (<|>), (<?>))

import qualified Text.Parsec.Expr as Expr
import qualified Text.Parsec.Token as Token

import qualified Lexer
import qualified Syntax

binary s f assoc = Expr.Infix (Lexer.reservedOp s >> return (Syntax.BinaryOperation f)) assoc

table = [[binary "*" Syntax.Times Expr.AssocLeft,
          binary "/" Syntax.Divide Expr.AssocLeft]
        ,[binary "+" Syntax.Plus Expr.AssocLeft,
          binary "-" Syntax.Minus Expr.AssocLeft]]

whitespace = Token.whiteSpace Lexer.lexer
colon = Token.colon Lexer.lexer
eof = Text.Parsec.eof

oneOfTypes = 
  Text.Parsec.choice $ 
  Text.Parsec.try <$>
  Lexer.symbol <$>
  Lexer.types

argument = do
  name <- Lexer.identifier
  colon
  tipe <- oneOfTypes
  return $ Syntax.Argument name tipe

function = do
  Lexer.reserved "function"
  name <- Lexer.identifier
  arguments <- Lexer.parens $ Lexer.commaSep argument
  Lexer.rightArrow
  tipe <- oneOfTypes
  expression <- Lexer.braces $ expr
  return $ Syntax.Function name arguments tipe expression

term :: Text.Parsec.String.Parser Syntax.Expression
term = try expr
      <|> Lexer.parens expr

expr :: Text.Parsec.String.Parser Syntax.Expression
expr = Expr.buildExpressionParser table term
  
mainParser = do
    whitespace
    expression <- expr
    eof
    return expression

functionParser = do
    whitespace
    func <- function
    eof
    return func

testMain = Text.Parsec.parse mainParser "source" -- "not working as of now"
testFunction = Text.Parsec.parse functionParser "source" -- "function add(x: i32, y: u8) -> u8"


-- int :: Parser Expr
-- int = do
--   n <- integer
--   return $ Float (fromInteger n)

-- floating :: Parser Expr
-- floating = do
--   n <- float
--   return $ Float n

-- variable :: Parser Expr
-- variable = do
--   var <- identifier
--   return $ Var var

-- function :: Parser Expr
-- function = do
--   reserved "function"
--   name <- identifier
--   args <- parens $ many variable
--   body <- expr
--   return $ Function name args body

-- extern :: Parser Expr
-- extern = do
--   reserved "extern"
--   name <- identifier
--   args <- parens $ many variable
--   return $ Extern name args

-- call :: Parser Expr
-- call = do
--   name <- identifier
--   args <- parens $ commaSep expr
--   return $ Call name args

-- factor :: Parser Expr
-- factor = try floating
--       <|> try int
--       <|> try extern
--       <|> try function
--       <|> try call
--       <|> variable
--       <|> parens expr

-- contents :: Parser a -> Parser a
-- contents p = do
--   Token.whiteSpace lexer
--   r <- p
--   eof
--   return r

-- toplevel :: Parser [Expr]
-- toplevel = many $ do
--     def <- defn
--     reservedOp ";"
--     return def

-- uptoEOF :: Parser a -> Parser a
-- uptoEOF p = do
--   Token.whiteSpace lexer
--   r <- p
--   eof
--   return r

-- parseExpr :: String -> Either ParseError Expr
-- parseExpr s = parse (contents expr) "<stdin>" s

-- parseToplevel :: String -> Either ParseError [Expr]
-- parseToplevel s = parse (contents toplevel) "<stdin>" s

