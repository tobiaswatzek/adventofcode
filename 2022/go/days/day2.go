package days

import (
	"adventofcode2022/util"
	"errors"
	"fmt"
	"strings"
)

/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/

func day2(inputFilePath string) (string, string, error) {
	input := util.ReadFile("day2.txt")
	lines := strings.Split(input, "\n")

	scorePartOne := 0
	scorePartTwo := 0

	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}

		columns := strings.Split(line, " ")

		// Part 1
		opponentMove, err := parseColumnOne(columns[0])
		if err != nil {
			return "", "", err
		}
		myMove, err := parseColumnTwoToMove(columns[1])
		if err != nil {
			return "", "", err
		}
		scorePartOne += shapeScore(myMove)
		scorePartOne += roundScore(opponentMove, myMove)

		// Part Two
		expectedOutcome, err := parseColumnTwoToOutcome(columns[1])
		if err != nil {
			return "", "", err
		}
		myShape := shapeForOutcome(expectedOutcome, opponentMove)
		scorePartTwo += shapeScore(myShape)
		scorePartTwo += roundScore(opponentMove, myShape)
	}

	return fmt.Sprint(scorePartOne), fmt.Sprint(scorePartTwo), nil
}

type outcome int

const (
	lose outcome = iota
	draw
	win
)

type shape int

const (
	rock shape = iota
	paper
	scissors
)

func roundScore(opponentMove, myMove shape) int {
	if opponentMove == myMove {
		return 3
	}

	if opponentMove == rock && myMove == scissors {
		return 0
	}

	if opponentMove == paper && myMove == rock {
		return 0
	}

	if opponentMove == scissors && myMove == paper {
		return 0
	}

	return 6
}

func shapeScore(s shape) int {
	switch s {
	case rock:
		return 1
	case paper:
		return 2
	case scissors:
		return 3
	default:
		panic("Shape is not known")
	}
}

func shapeForOutcome(expectedOutcome outcome, opponentMove shape) shape {
	if expectedOutcome == draw {
		return opponentMove
	}

	if expectedOutcome == win {
		switch opponentMove {
		case rock:
			return paper
		case paper:
			return scissors
		case scissors:
			return rock
		default:
			panic("Unknown move")
		}
	}

	switch opponentMove {
	case rock:
		return scissors
	case paper:
		return rock
	case scissors:
		return paper
	default:
		panic("Unknown move")
	}
}

func parseColumnOne(c string) (shape, error) {
	switch c {
	case "A":
		return rock, nil
	case "B":
		return paper, nil
	case "C":
		return scissors, nil
	}

	return rock, errors.New("cannot parse move")
}

func parseColumnTwoToMove(c string) (shape, error) {
	switch c {
	case "X":
		return rock, nil
	case "Y":
		return paper, nil
	case "Z":
		return scissors, nil
	}

	return rock, errors.New("cannot parse move")
}

func parseColumnTwoToOutcome(c string) (outcome, error) {
	switch c {
	case "X":
		return lose, nil
	case "Y":
		return draw, nil
	case "Z":
		return win, nil
	}

	return lose, errors.New("cannot parse outcome")
}
