module Core.DataType where

data DataType
    = Signed { size :: Word }
    | Unsigned { size :: Word }
    | Boolean
    deriving (Show, Eq);