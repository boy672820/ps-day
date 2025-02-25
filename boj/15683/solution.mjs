export function solution([n, m], map) {
  let count = 0
  let ranges = []

  traverse(0, 0)

  let ans = count

  const combinations = ranges.reduce((acc, chunk) =>
    acc.flatMap(prev => chunk.map(item => [...prev, item])),
    [[]]
  )

  combinations.forEach((combination) => {
    const set = combination.reduce((acc, coords) => {
      coords.forEach((coord) => acc.add(coord.hash()))
      return acc
    }, new Set())
    ans = Math.min(ans, count - set.size)
  })

  return ans

  function traverse(y, x) {
    if (y >= n) return
    if (x >= m) return traverse(y + 1, 0)

    if (map[y][x] === '0') {
      count += 1
    }

    if (['1', '2', '3', '4', '5'].includes(map[y][x])) {
      const cctvType = map[y][x]
      const viewRange = getCctvViewRange(cctvType, x, y)
      ranges.push(viewRange)
    }

    traverse(y, x + 1)
  }

  function getCctvViewRange(type, x, y) {
    let top = []
    for (let i = y - 1; i >= 0; i -= 1) {
      if (map[i][x] !== '0') break
      top.push(Coordinate.of(x, i))
    }

    let bottom = []
    for (let i = y + 1; i < n; i += 1) {
      if (map[i][x] !== '0') break
      bottom.push(Coordinate.of(x, i))
    }

    let left = []
    for (let i = x - 1; i >= 0; i -= 1) {
      if (map[y][i] !== '0') break
      left.push(Coordinate.of(i, y))
    }

    let right = []
    for (let i = x + 1; i < m; i += 1) {
      if (map[y][i] !== '0') break
      right.push(Coordinate.of(i, y))
    }

    switch (type) {
      case '1':
        return [top, bottom, left, right]
      case '2':
        return [join(top, bottom), join(left, right)]
      case '3':
        return [join(left, top), join(top, right), join(right, bottom), join(bottom, left)]
      case '4':
        return [join(left, top, right), join(left, bottom, right)]
      case '5':
        return [join(left, top, right, bottom)]
      default:
        throw new Error('Unknown cctv type')
    }
  }
}

const join = (a, b, c, d) => [...a, ...b, ...(c || []), ...(d || [])]


class Coordinate {
  constructor(x, y) {
    this.x = x
    this.y = y
  }

  static of = (x, y) => new Coordinate(x, y)
  // static of = (x, y) => `${x}-${y}`
  hash = () => `${this.x}-${this.y}`
}
