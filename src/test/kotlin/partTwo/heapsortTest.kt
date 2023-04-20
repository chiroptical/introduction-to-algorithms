package partTwo

import kotlin.test.Test
import kotlin.test.assertEquals

infix fun <T> T.isEqualTo(expected: T) = assertEquals(expected, this)

internal class Indices {
    @Test
    fun `can get parent index for zero`() {
        parentIndex(0u) isEqualTo 0u
    }

    @Test
    fun `can get parent index for one`() {
        parentIndex(1u) isEqualTo 0u
    }

    @Test
    fun `can get parent index for three`() {
        parentIndex(3u) isEqualTo 1u
    }

    @Test
    fun `can get parent index for five`() {
        parentIndex(5u) isEqualTo 2u
    }

    @Test
    fun `can get subtree indices for zero`() {
        val idx = 0u
        leftSubtreeIndex(idx) isEqualTo 1u
        rightSubtreeIndex(idx) isEqualTo 2u
    }

    @Test
    fun `can get subtree indices for one`() {
        val idx = 1u
        leftSubtreeIndex(idx) isEqualTo 3u
        rightSubtreeIndex(idx) isEqualTo 4u
    }

    @Test
    fun `can get subtree indices for two`() {
        val idx = 2u
        leftSubtreeIndex(idx) isEqualTo 5u
        rightSubtreeIndex(idx) isEqualTo 6u
    }

    @Test
    fun `can get subtree indices for three`() {
        val idx = 3u
        leftSubtreeIndex(idx) isEqualTo 7u
        rightSubtreeIndex(idx) isEqualTo 8u
    }
}

internal class MinElementsBy {

    @Test
    fun `it handles 0`() {
        minElementsBy(0u) isEqualTo 0u
    }

    @Test
    fun `it handles 1`() {
        minElementsBy(1u) isEqualTo 1u
    }

    @Test
    fun `it handles 2`() {
        minElementsBy(2u) isEqualTo 2u
    }

    @Test
    fun `it handles 3`() {
        minElementsBy(3u) isEqualTo 4u
    }

    @Test
    fun `it handles 4`() {
        minElementsBy(4u) isEqualTo 8u
    }
}

internal class MaxElementsBy {
    @Test
    fun `it handles 0`() {
        maxElementsBy(0u) isEqualTo 0u
    }

    @Test
    fun `it handles 1`() {
        maxElementsBy(1u) isEqualTo 1u
    }

    @Test
    fun `it handles 2`() {
        maxElementsBy(2u) isEqualTo 3u
    }

    @Test
    fun `it handles 3`() {
        maxElementsBy(3u) isEqualTo 8u
    }

    @Test
    fun `it handles 4`() {
        maxElementsBy(4u) isEqualTo 15u
    }
}

internal class MaxHeapify {
    @Test
    fun `figure 6_2 test`() {
        val heap = mutableListOf<Int>(16, 4, 10, 14, 7, 9, 3, 2, 8, 1)
        maxHeapify(heap, 1u)
        heap isEqualTo mutableListOf<Int>(16, 14, 10, 8, 7, 9, 3, 2, 4, 1)
    }

    @Test
    fun `exercise 6_2-1 test`() {
        val heap = mutableListOf<Int>(27, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0)
        maxHeapify(heap, 2u)
        heap isEqualTo mutableListOf<Int>(27, 17, 10, 16, 13, 9, 1, 5, 7, 12, 4, 8, 3, 0)
    }
}