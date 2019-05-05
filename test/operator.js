let assert = function(expr, ans) {
  if (expr !== ans) throw 100
}

assert(3 & 1, 1)
assert(10 | 5, 15)
assert(7 ^ ~2, -6)
assert(9 << 2, 36)
assert(9 >> 2, 2)
assert(19 >>> 2, 4)
assert(1 + 2, 3)
assert(1000000 + 2000000, 3000000)
assert(1 - 2, -1)
assert(5 - 2, 3)
assert(3 * 2, 6)
assert(5 / 2, 2.5)
assert(10 / 2, 5)
assert(1 + 2 * 3 ** 2 - 1, 18)
assert(2 == 2, true)
assert(2 === 2, true)
assert(2 != 2, false)
assert(2 !== 2, false)
assert(1 < 2, true)
assert(1 > 2, false)
assert(3 <= 3, true)
assert(3 <= 4, true)
assert(3 >= 3, true)
assert(3 >= 4, false)
