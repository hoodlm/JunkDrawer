require 'ostruct'

# Given a positive integer n, generate an n x n matrix filled with elements from 1 to n^2 in spiral order.

class MatrixGenerator

  STATES = [:right, :down, :left, :up]

  def generate(n)
    matrix = Array.new(n)
    n.times do |i|
      matrix[i] = Array.new(n, "*")
    end

    count_to = n * n
    current_value = 1
    coord = OpenStruct.new(column: 0, row: 0)
    state = STATES[0]

    until (current_value > count_to)
      puts state
      puts matrix.to_s
      puts coord.to_s
      if (coord.column < 0 || coord.column >= n || coord.row < 0 || coord.row >= n)
        fail "Coordinates are outside bounds: #{coord}"
      end
      fail "Cursor has walked back onto a populated square" if matrix[coord.row][coord.column] != "*"
      matrix[coord.row][coord.column] = current_value
      puts "wrote #{current_value} at #{coord}"
      puts matrix.to_s
      current_value += 1

      case state
      when :right
        state = :down if (matrix[coord.row][coord.column + 1].nil? || matrix[coord.row][coord.column + 1] != "*")
      when :down
        state = :left if (matrix[coord.row + 1].nil? || matrix[coord.row + 1][coord.column] != "*")
      when :left
        state = :up if (matrix[coord.row][coord.column - 1].nil? || matrix[coord.row][coord.column - 1] != "*")
      when :up
        state = :right if (matrix[coord.row - 1].nil? || matrix[coord.row - 1][coord.column] != "*")
      end

      puts "applying move - new state is #{state}"
      case state
      when :right
        coord.column += 1
      when :down
        coord.row += 1
      when :left
        coord.column -= 1
      when :up
        coord.row -= 1
      end
      puts "next coordinate: #{coord}"
      puts "-----"
    end

    return matrix
  end
end
