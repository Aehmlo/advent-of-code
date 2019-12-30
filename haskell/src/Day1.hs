module Day1 where

import Lib

fuel :: Int -> Int
fuel = subtract 2 . (`div` 3)

part1 :: [String] -> Int
part1 ms = sum $ map (fuel . read) ms

recursiveFuel :: Int -> Int
recursiveFuel m = sum . tail . takeWhile (>= 0) $ iterate fuel m

part2 :: [String] -> Int
part2 ms = sum $ map (recursiveFuel . read) ms

main :: [String] -> [String]
main = runParts (show . part1) (show . part2)