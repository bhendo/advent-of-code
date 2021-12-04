package day4

type BingoSpace struct {
	Number int
	Marked bool
}

type BingoBoard [5][5]BingoSpace

func stampBoard(number int, board *BingoBoard) *BingoBoard {
	for i, row := range board {
		for j, space := range row {
			if space.Number == number {
				board[i][j].Marked = true
				return board
			}
		}
	}
	return board
}

func checkBoard(board *BingoBoard) bool {
	if board == nil {
		return false
	}
	for i := 0; i < 5; i++ {
		if board[i][0].Marked && board[i][1].Marked && board[i][2].Marked && board[i][3].Marked && board[i][4].Marked {
			return true
		} else if board[0][i].Marked && board[1][i].Marked && board[2][i].Marked && board[3][i].Marked && board[4][i].Marked {
			return true
		}
	}
	return false
}

func scoreBoard(number int, board *BingoBoard) (out int) {
	for _, row := range board {
		for _, space := range row {
			if !space.Marked {
				out += space.Number
			}
		}
	}
	return number * out
}

func Part1(numbers []int, boards []*BingoBoard) (out int) {
	for i, number := range numbers {
		for _, board := range boards {
			stampBoard(number, board)
			if i >= 5 {
				if checkBoard(board) {
					return scoreBoard(number, board)
				}
			}
		}
	}
	return
}

func Part2(numbers []int, boards []*BingoBoard) (out int) {
	completed := map[int]bool{}
	for _, number := range numbers {
		for i, board := range boards {
			if !completed[i] {
				stampBoard(number, board)
				if checkBoard(board) {
					score := scoreBoard(number, board)
					completed[i] = true
					if len(completed) == len(boards) {
						return score
					}
				}
			}
		}
	}
	return
}
