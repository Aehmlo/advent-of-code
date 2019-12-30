module Lib
    ( getInputLines,
      runPart,
      runParts,
    ) where

inputPath :: Int -> FilePath
inputPath day = "../inputs/" ++ (show day) ++ ".txt"

getInput :: Int -> IO String
getInput = readFile . inputPath

getInputLines :: Int -> IO [String]
getInputLines day = do input <- getInput day
                       return $ lines input

runPart :: (a -> String) -> a -> [String]
runPart fn input = ["Part 1: " ++ fn input]

runParts :: (a -> String) -> (a -> String) -> a -> [String]
runParts pt1 pt2 input = ["Part 1: " ++ pt1 input, "Part 2: " ++ pt2 input]