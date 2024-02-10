class LinkedList
    attr_accessor :val, :next
    def initialize(val = 0, _next = nil)
        @val = val
        @next = _next
    end

    def vals_to_s
        out = []
        out << val
        out << @next.vals_to_s unless @next.nil?
        out.flatten.join(',')
    end
end

module LinkedListDigits
    def self.solve(list1, list2)
        number1 = list_to_int(list1)
        number2 = list_to_int(list2)
        result = number1 + number2
        return int_to_list(result)
    end

    protected

    def self.list_to_int(list)
        deque = []
        current = list
        deque << current.val
        while (current.next != nil)
            current = current.next
            deque << current.val
        end

        result = 0
        radix = 1

        deque.each do |digit|
            result = result + digit * radix
            radix = radix * 10
        end
        return result
    end

    def self.int_to_list(int, accum_list = nil)
        previous = nil
        int.to_s.chars.each do |digit|
            next_node = LinkedList.new(digit.to_i, previous)
            previous = next_node
        end
        return previous
    end
end
