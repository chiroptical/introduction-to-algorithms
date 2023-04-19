package partTwo

import kotlin.test.Test
import kotlin.test.assertEquals

infix fun <T> T.isEqualTo(got: T) = assertEquals(this, got)

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