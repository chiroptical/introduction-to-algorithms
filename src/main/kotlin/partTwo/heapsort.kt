package partTwo

fun parentIndex(n: UInt): UInt = n / 2u

fun leftSubtreeIndex(n: UInt): UInt = 2u * n + 1u

fun rightSubtreeIndex(n: UInt): UInt = 2u * n + 2u

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

fun <T> MutableList<T>.swap(from: Int, to: Int) {
    val fromTemp = this.getOrNull(from)
    val toTemp = this.getOrNull(to)
    if (fromTemp != null && toTemp != null) {
        this[from] = toTemp
        this[to] = fromTemp
    }
}

// TODO Exercise 6.2-3: write minHeapify
// TODO Exercise 6.2-4: what happens when input[index] > than its' children?
// TODO Exercise 6.2-5: what happens when index > input.size / 2?
// TODO Exercise 6.2-6: is it possible to convert the recursive call to a loop?
fun <T : Comparable<T>> maxHeapify(input: MutableList<T>, index: UInt) {
    val heapSize = input.size.toUInt()
    val left = leftSubtreeIndex(index)
    val right = rightSubtreeIndex(index)
    var largest: UInt = index
    if (left < heapSize && input[left.toInt()] > input[index.toInt()]) {
        largest = left
    }
    if (right < heapSize && input[right.toInt()] > input[largest.toInt()]) {
        largest = right
    }
    if (largest != index) {
        input.swap(index.toInt(), largest.toInt())
        maxHeapify(input, largest)
    }
}