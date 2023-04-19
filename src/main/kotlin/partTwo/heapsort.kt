package partTwo

fun parentIndex(n: UInt): UInt = n / 2u

fun leftSubtreeIndex(n: UInt): UInt = 2u * n

fun rightSubtreeIndex(n: UInt): UInt = 2u * n + 1u

// max-heap sort ensures, for every non-root node, a[parent(i)] > a[i]

// Exercise, 6.1-1: What are the minimum and maximum number of elements in a heap of size h?
// h = 1: [1, 1]
// h = 2: [2, 3]
// h = 3: [4, 8]
// h = 4: [8, 15]
//
// 2 << 0 -> 2
// 2 << 1 -> 4
// 2 << 2 -> 8
fun minElementsBy(height: UInt): UInt =
    when (height) {
        0u, 1u -> height
        else -> 2u shl (height.toInt() - 2)
    }

fun maxElementsBy(height: UInt): UInt =
    when (height) {
        0u, 1u -> height
        else -> height * height - 1u
    }