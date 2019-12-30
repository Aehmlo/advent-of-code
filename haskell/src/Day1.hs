module Day1 where

import Lib

fuel :: Int -> Int
fuel = subtract 2 . (`div` 3)

part1 :: [String] -> Int
part1 ms = sum $ map (fuel . read) ms

main :: [String] -> [String]
main = runPart (show . part1)