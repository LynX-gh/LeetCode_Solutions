// INV holds the modular inverses used for division.
var INV = map[int]int{
	1: 1,
	3: 7,
	7: 3,
	9: 9,
}

// mult multiplies two triple-tuples element-wise, with the third element computed mod 10.
func mult(a, b [3]int) [3]int {
	return [3]int{
		a[0] + b[0],
		a[1] + b[1],
		(a[2] * b[2]) % 10,
	}
}

// div divides tuple a by tuple b using the modular inverses.
func div(a, b [3]int) [3]int {
	return [3]int{
		a[0] - b[0],
		a[1] - b[1],
		(a[2] * INV[b[2]]) % 10,
	}
}

// red reduces a slice of pairs by computing the sum of (first * second) mod 10.
func red(pairs [][2]int) int {
	sum := 0
	for _, pair := range pairs {
		sum = (sum + (pair[0]*pair[1])%10) % 10
	}
	return sum
}

// TWOS and FIVES provide the cycles for factors of 2 and 5.
var TWOS = []int{2, 4, 8, 6}
var FIVES = []int{5}

// simplify reduces a triple (twos, fives, res) to a single value mod 10.
func simplify(tup [3]int) int {
	twos, fives, res := tup[0], tup[1], tup[2]
	if twos != 0 {
		res = (res * TWOS[(twos-1)%len(TWOS)]) % 10
	}
	if fives != 0 {
		res = (res * FIVES[(fives-1)%len(FIVES)]) % 10
	}
	return res
}

// hasSameDigits returns true if the final two digits computed from s are the same.
func hasSameDigits(s string) bool {
	// n is defined as len(s)-2.
	n := len(s) - 2

	// facts will store the factorial decomposition tuples.
	facts := make([][3]int, 0, n+1)
	facts = append(facts, [3]int{0, 0, 1})

	// Build facts until its length is n+1.
	for n >= len(facts) {
		num := len(facts)
		twos := 0
		for num%2 == 0 {
			num /= 2
			twos++
		}
		fives := 0
		for num%5 == 0 {
			num /= 5
			fives++
		}
		facts = append(facts, mult(facts[len(facts)-1], [3]int{twos, fives, num}))
	}

	// Compute Pascal's coefficients modulo 10.
	pascal := make([]int, n+1)
	for k := 0; k <= n; k++ {
		pascal[k] = simplify(div(div(facts[n], facts[k]), facts[n-k]))
	}

	// Build the weighted pairs for red.
	m := n + 1 // since len(pascal) == n+1, and s[1:] also has n+1 elements.
	aPairs := make([][2]int, m)
	bPairs := make([][2]int, m)
	for i := 0; i < m; i++ {
		aPairs[i] = [2]int{int(s[i] - '0'), pascal[i]}
	}
	for i := 0; i < m; i++ {
		bPairs[i] = [2]int{int(s[i+1] - '0'), pascal[i]}
	}

	a := red(aPairs)
	b := red(bPairs)
	return a == b
}