module Syntax where

type Name = String
type Type = String

data Argument = Argument Name Type
  deriving (Eq, Show)

data Statement
  = Function Name [Argument] Type Expression
  | Assignment
  deriving (Eq, Show)
  
data Expression
  = BinaryOperation Operator Expression Expression
  | Variable Name Type
  | Void
  deriving (Eq, Show)

-- data Expr
--   = Float Double
--   | BinOp Op Expr Expr
--   | Var String
--   | Call Name [Expr]
--   | Function Name [Expr] Expr
--   | Extern Name [Expr]
--   deriving (Eq, Ord, Show)

data Operator
  = Plus
  | Minus
  | Times
  | Divide
  deriving (Eq, Ord, Show)