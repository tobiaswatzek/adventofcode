package main

import "fmt"
import "math"

type point struct {
	x int
	y int
}

func manhattanDistance(a, b point) float64 {
	// |x1−x2|+|y1−y2|
	return math.Abs(float64(a.x-b.x)) + math.Abs(float64(a.y-b.y))
}

func spiralManhattanDistanceFromOrigin(target int) float64 {
	/*
		17  16  15  14  13
		18   5   4   3  12
		19   6   1   2  11
		20   7   8   9  10
		21  22  23---> ...
	*/
	numOfElements := 1
	thickness := 1
	for ; target > numOfElements; thickness++ {
		numOfElements += 8 * thickness
	}
	thickness--
	elementsPerSide := 1 + 2*thickness
	var targetPoint point
	/*
		LU | RU
		-------
		LD | RD
	*/
	rd := numOfElements
	ld := rd - (elementsPerSide - 1)
	lu := ld - (elementsPerSide - 1)
	ru := lu - (elementsPerSide - 1)

	if (target <= rd) && (target >= ld) {
		middle := rd - (elementsPerSide / 2)
		targetPoint = point{x: target - middle, y: -thickness}
	} else if (target < ld) && (target >= lu) {
		middle := ld - (elementsPerSide / 2)
		targetPoint = point{x: -thickness, y: middle - target}
	} else if (target < lu) && (target >= ru) {
		middle := lu - (elementsPerSide / 2)
		targetPoint = point{x: middle - target, y: thickness}
	} else if target < ru {
		middle := ru - (elementsPerSide / 2)
		targetPoint = point{x: thickness, y: target - middle}
	}
	center := point{x: 0, y: 0}
	return manhattanDistance(center, targetPoint)
}

type direction int

const (
	northwest direction = iota
	north
	northeast
	east
	southeast
	south
	southwest
	west
)

var directionList = [...]direction{
	northwest,
	north,
	northeast,
	east,
	southeast,
	south,
	southwest,
	west,
}

func valueForPoint(p point, m map[point]int) int {
	if m[p] != 0 {
		return m[p]
	}
	var sum int
	for _, dir := range directionList {
		sum += getNeighbourValueByDirection(p, m, dir)
	}
	return sum
}

func getNeighbourPointByDirection(p point, dir direction) point {
	pDir := p
	switch dir {
	case northwest:
		pDir.x--
		pDir.y++
		break
	case north:
		pDir.y++
		break
	case northeast:
		pDir.x++
		pDir.y++
		break
	case east:
		pDir.x++
		break
	case southeast:
		pDir.x++
		pDir.y--
		break
	case south:
		pDir.y--
		break
	case southwest:
		pDir.x--
		pDir.y--
		break
	case west:
		pDir.x--
		break
	default:
		panic("Unknown direction.")
	}
	return pDir
}

func getNeighbourValueByDirection(p point, m map[point]int, dir direction) int {
	n := getNeighbourPointByDirection(p, dir)
	return m[n]
}

func turnAroundDirection(dir direction) direction {
	switch dir {
	case northwest:
		return southeast
	case north:
		return south
	case northeast:
		return southwest
	case east:
		return west
	case southeast:
		return northwest
	case south:
		return north
	case southwest:
		return northeast
	case west:
		return east
	}
	panic("Unknown direction.")
}

func spiralStressTest(target int) int {
	currentDirection := east
	currentPoint := point{x: 0, y: 0}
	start := 1
	var value int
	m := make(map[point]int)
	m[currentPoint] = start
	for i := 1; value < target; i++ {
		value = valueForPoint(currentPoint, m)
		m[currentPoint] = value
		if currentDirection == east {
			currentPoint.x++
		} else if currentDirection == north {
			currentPoint.y++
		} else if currentDirection == west {
			currentPoint.x--
		} else if currentDirection == south {
			currentPoint.y--
		}

		if currentDirection == east && getNeighbourValueByDirection(currentPoint, m, north) == 0 {
			currentDirection = north
		} else if currentDirection == north && getNeighbourValueByDirection(currentPoint, m, west) == 0 {
			currentDirection = west
		} else if currentDirection == west && getNeighbourValueByDirection(currentPoint, m, south) == 0 {
			currentDirection = south
		} else if currentDirection == south && getNeighbourValueByDirection(currentPoint, m, east) == 0 {
			currentDirection = east
		}
	}
	return value
}

func main() {
	target := 289326
	// Manhattan distance from origin
	fmt.Println("Distance: ")
	fmt.Println(spiralManhattanDistanceFromOrigin(1))    // 0
	fmt.Println(spiralManhattanDistanceFromOrigin(12))   // 3
	fmt.Println(spiralManhattanDistanceFromOrigin(23))   // 2
	fmt.Println(spiralManhattanDistanceFromOrigin(1024)) // 31
	fmt.Print("Result: ")
	fmt.Println(spiralManhattanDistanceFromOrigin(target)) // 419

	fmt.Println("Stress test: ")
	fmt.Print("Result: ")
	fmt.Println(spiralStressTest(target))

}
