module Parser.DataTypeSpec (spec) where

import Test.Hspec

import Text.Parsec (parse)

import Core.DataType
import Parser.DataType

spec :: Spec
spec = do
    describe "data type parser" $ do
            it "i8" $ do
                let ans = parse dataType "test" "i8"
                shouldBe ans (Right $ Signed { size=1 })
            it "i16" $ do
                let ans = parse dataType "test" "i16"
                shouldBe ans (Right $ Signed { size=2 })
            it "i32" $ do
                let ans = parse dataType "test" "i32"
                shouldBe ans (Right $ Signed { size=4 })
            it "u8" $ do
                let ans = parse dataType "test" "u8"
                shouldBe ans (Right $ Unsigned { size=1 })
            it "u16" $ do
                let ans = parse dataType "test" "u16"
                shouldBe ans (Right $ Unsigned { size=2 })
            it "u32" $ do
                let ans = parse dataType "test" "u32"
                shouldBe ans (Right $ Unsigned { size=4 })
            it "bool" $ do
                let ans = parse dataType "test" "bool"
                shouldBe ans (Right $ Boolean)