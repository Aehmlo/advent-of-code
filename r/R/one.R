library(dplyr)
library(readr)

read_file("../../inputs/1.txt") %>%
	strsplit("\n") %>%
	unlist %>%
	as.numeric ->
	input


# Part one

fuel <- function(mass) {
	floor(mass / 3) - 2
}

total.fuel <- function(mass) {
	sum(fuel(mass))
}

total.fuel(input)

# Part two

fuels <- function(mass) {
	f <- fuel(mass)
	while (all(f >= 0)) {
		f <- c(f, fuel(tail(f, 1)))
	}
	f[f >= 0]
}

total.fuels <- function(mass) {
	mass %>%
		lapply(fuels) %>%
		unlist %>%
		sum
}

total.fuels(input)