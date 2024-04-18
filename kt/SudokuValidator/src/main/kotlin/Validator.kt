package org.example

class Validator {
    fun isValid(sudokuBoard: Array<CharArray>): Boolean {
        val rows = sudokuBoard
        val cols = getColumns(sudokuBoard)
        val squares = getSquares(sudokuBoard)

        val allSets = rows + cols + squares
        assert(allSets.size == 27) { "Expected exactly 27 digit sets: 9 rows, 9 cols, 9 squares" }

        return allSets.all {
            isUniqueNine(it)
        }
    }

    fun getColumns(sudokuBoard: Array<CharArray>): Array<CharArray> {
        return (0..8).map { n ->
            sudokuBoard.map { row ->
                row.get(n)
            }.toCharArray()
        }.toTypedArray()
    }

    fun getSquares(sudokuBoard: Array<CharArray>): Array<CharArray> {
        return listOf(0, 3, 6).map { rowOffset ->
            listOf(0, 3, 6).map { colOffset ->
                val square: MutableList<Char> = mutableListOf()
                listOf(0, 1, 2).map { rowInnerOffset ->
                    listOf(0, 1, 2).map { colInnerOffset ->
                        val row = rowOffset + rowInnerOffset
                        val col = colOffset + colInnerOffset
                        square.add(sudokuBoard[row][col])
                    }
                }
                square.toCharArray()
            }
        }.flatten().toTypedArray()
    }

    fun isUniqueNine(sudokuRow: CharArray): Boolean {
        assert(sudokuRow.size == 9) { "Expected row to be charArray of length 9: $sudokuRow" }

        val excludeWildcards = sudokuRow.toMutableList().filterNot { it == '.' }
        val excludeWildcardsUniqueSet = excludeWildcards.toSet()
        return excludeWildcardsUniqueSet.size == excludeWildcards.size
    }
}