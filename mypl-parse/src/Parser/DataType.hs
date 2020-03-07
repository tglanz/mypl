module Parser.DataType where

import Data.List (intercalate)

import qualified Lexer
import Core.DataType

import Text.Parsec (choice, try, (<|>), (<?>))
import Text.Parsec.String (Parser)

dataTypeU8 = Lexer.reserved "u8" >>= \p -> return $ Unsigned { size = 1 }
dataTypeU16 = Lexer.reserved "u16" >>= \p -> return $ Unsigned { size = 2 }
dataTypeU32 = Lexer.reserved "u32" >>= \p -> return $ Unsigned { size = 4 }
dataTypeI8 = Lexer.reserved "i8" >>= \p -> return $ Signed { size = 1 }
dataTypeI16 = Lexer.reserved "i16" >>= \p -> return $ Signed { size = 2 }
dataTypeI32 = Lexer.reserved "i32" >>= \p -> return $ Signed { size = 4 }
dataTypeBool = Lexer.reserved "bool" >>= \p -> return $ Boolean

allDataTypeParsers :: [Parser DataType]
allDataTypeParsers =
    [ dataTypeU8
    , dataTypeU16
    , dataTypeU32
    , dataTypeI8
    , dataTypeI16
    , dataTypeI32
    , dataTypeBool
    ]

dataType :: Parser DataType
dataType = choice (try <$> allDataTypeParsers) <?> intercalate ", " Lexer.types    where 