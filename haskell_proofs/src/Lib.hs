module Lib where

doSomething :: String -> Int -> Int
doSomething numString num = read numString + num

doSomething' :: String -> Int -> Int
doSomething' numString = (+) (read numString)

doSomething'' :: () -> String -> Int -> Int
doSomething'' _ numString = (+) (read numString)
