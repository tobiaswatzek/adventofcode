package days

type RunDayData struct {
	Number        int
	InputFilePath string
}

func RunDay(data RunDayData) (string, string, error) {
	return dayRegistry[data.Number-1](data.InputFilePath)
}

type dayFunc func(string) (string, string, error)

var dayRegistry []dayFunc = []dayFunc{
	day1,
	day2,
	day3,
	day4,
}
