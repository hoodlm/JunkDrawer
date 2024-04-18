import org.example.Validator
import org.junit.jupiter.api.Assertions.*
import kotlin.test.Test

class ValidatorTest {
    companion object {
        val validator = Validator()
    }

    @Test
    fun givenHappyCase() {
        val board: Array<CharArray> = arrayOf(
            charArrayOf('5','3','.','.','7','.','.','.','.'),
            charArrayOf('6','.','.','1','9','5','.','.','.'),
            charArrayOf('.','9','8','.','.','.','.','6','.'),
            charArrayOf('8','.','.','.','6','.','.','.','3'),
            charArrayOf('4','.','.','8','.','3','.','.','1'),
            charArrayOf('7','.','.','.','2','.','.','.','6'),
            charArrayOf('.','6','.','.','.','.','2','8','.'),
            charArrayOf('.','.','.','4','1','9','.','.','5'),
            charArrayOf('.','.','.','.','8','.','.','7','9'),
        )
        assertTrue(validator.isValid(board))
    }

    @Test
    fun givenFailureCase() {
        val board: Array<CharArray> = arrayOf(
            charArrayOf('8','3','.','.','7','.','.','.','.'),
            charArrayOf('6','.','.','1','9','5','.','.','.'),
            charArrayOf('.','9','8','.','.','.','.','6','.'),
            charArrayOf('8','.','.','.','6','.','.','.','3'),
            charArrayOf('4','.','.','8','.','3','.','.','1'),
            charArrayOf('7','.','.','.','2','.','.','.','6'),
            charArrayOf('.','6','.','.','.','.','2','8','.'),
            charArrayOf('.','.','.','4','1','9','.','.','5'),
            charArrayOf('.','.','.','.','8','.','.','7','9'),
        )
        assertFalse(validator.isValid(board))
    }

    @Test
    fun collisionInRow() {
        val board: Array<CharArray> = arrayOf(
            charArrayOf('5','3','.','.','7','.','.','.','.'),
            charArrayOf('6','.','.','1','9','5','.','.','.'),
            charArrayOf('.','9','8','.','.','.','.','6','.'),
            charArrayOf('8','.','3','.','6','.','.','.','3'), // <-- 3 in row 4
            charArrayOf('4','.','.','8','.','3','.','.','1'),
            charArrayOf('7','.','.','.','2','.','.','.','6'),
            charArrayOf('.','6','.','.','.','.','2','8','.'),
            charArrayOf('.','.','.','4','1','9','.','.','5'),
            charArrayOf('.','.','.','.','8','.','.','7','9'),
        )
        assertFalse(validator.isValid(board))
    }

    @Test
    fun collisionInColumn() {
        val board: Array<CharArray> = arrayOf(
            charArrayOf('5','3','.','.','7','.','.','.','.'),
            charArrayOf('6','.','.','1','9','5','.','.','.'),
            charArrayOf('.','9','8','.','.','.','.','6','.'),
            charArrayOf('8','.','.','.','6','.','.','.','3'),
            charArrayOf('4','.','.','8','.','3','.','.','1'),
            charArrayOf('7','.','.','.','2','.','.','.','6'),
            charArrayOf('.','6','.','.','.','.','2','8','.'),
            charArrayOf('.','.','.','4','1','9','.','.','5'),
            charArrayOf('5','.','.','.','8','.','.','7','9'), // <-- 5 in col 1
        )
        assertFalse(validator.isValid(board))
    }

    @Test
    fun testUniqueNine() {
        val allDigits = charArrayOf('1', '2', '3', '4', '5', '6', '7', '8', '9')
        assertTrue(validator.isUniqueNine(allDigits))
        assertTrue(validator.isUniqueNine(allDigits.reversedArray()))

        val someWildcards = charArrayOf('1', '.', '3', '4', '.', '.', '.', '8', '9')
        assertTrue(validator.isUniqueNine(someWildcards))

        val duplicateEight = charArrayOf('1', '2', '3', '4', '5', '6', '7', '8', '8')
        assertFalse(validator.isUniqueNine(duplicateEight))

        val duplicateEightWithWildcards = charArrayOf('1', '.', '.', '4', '.', '6', '7', '8', '8')
        assertFalse(validator.isUniqueNine(duplicateEightWithWildcards))
    }

    @Test
    fun testGetColumns() {
        val board: Array<CharArray> = arrayOf(
            charArrayOf('5','3','.','.','7','.','.','.','.'),
            charArrayOf('6','.','.','1','9','5','.','.','.'),
            charArrayOf('.','9','8','.','.','.','.','6','.'),
            charArrayOf('8','.','.','.','6','.','.','.','3'),
            charArrayOf('4','.','.','8','.','3','.','.','1'),
            charArrayOf('7','.','.','.','2','.','.','.','6'),
            charArrayOf('.','6','.','.','.','.','2','8','.'),
            charArrayOf('.','.','.','4','1','9','.','.','5'),
            charArrayOf('.','.','.','.','8','.','.','7','9'),
        )
        val columns = validator.getColumns(board)
        assertEquals(columns[0].toList(), charArrayOf('5', '6', '.', '8', '4', '7', '.', '.', '.').toList())
        assertEquals(columns[8].toList(), charArrayOf('.', '.', '.', '3', '1', '6', '.', '5', '9').toList())
    }

    @Test
    fun testGetSquares() {
        val board: Array<CharArray> = arrayOf(
            charArrayOf('5','3','.','.','7','.','.','.','.'),
            charArrayOf('6','.','.','1','9','5','.','.','.'),
            charArrayOf('.','9','8','.','.','.','.','6','.'),
            charArrayOf('8','.','.','.','6','.','.','.','3'),
            charArrayOf('4','.','.','8','.','3','.','.','1'),
            charArrayOf('7','.','.','.','2','.','.','.','6'),
            charArrayOf('.','6','.','.','.','.','2','8','.'),
            charArrayOf('.','.','.','4','1','9','.','.','5'),
            charArrayOf('.','.','.','.','8','.','.','7','9'),
        )
        val squares = validator.getSquares(board)
        assertEquals(squares[0].toList(), charArrayOf('5', '3', '.', '6', '.', '.', '.', '9', '8').toList())
        assertEquals(squares[2].toList(), charArrayOf('.', '.', '.', '.', '.', '.', '.', '6', '.').toList())
        assertEquals(squares[7].toList(), charArrayOf('.', '.', '.', '4', '1', '9', '.', '8', '.').toList())
        assertEquals(squares[8].toList(), charArrayOf('2', '8', '.', '.', '.', '5', '.', '7', '9').toList())
    }
}