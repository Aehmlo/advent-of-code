module Main where

import Lib

import qualified Day1 as D1

import System.Environment

usage :: IO ()
usage = putStrLn "Usage: aoc (DAYS)"

putStrLns :: [String] -> IO ()
putStrLns = mapM_ putStrLn

runDay :: String -> IO ()
runDay "1" = getInputLines 1 >>= putStrLns . D1.main

main :: IO ()
main = do args <- getArgs
          case args of
              [] -> usage
              xs -> mapM_ runDay xs
