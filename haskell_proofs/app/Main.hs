module Main where

import Lib

main :: IO ()
main = do
	putStrLn (show $ doSomething "1" 1)
	>> putStrLn (show $ doSomething' "1" 2)
	>> putStrLn (show $ doSomething'' () "1" 2)
