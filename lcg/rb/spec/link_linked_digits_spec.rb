require_relative "../linked_list_digits"

describe LinkedListDigits do
    it "solves given problem" do
        list1  = LinkedList.new(2, LinkedList.new(4, LinkedList.new(3)))
        list2  = LinkedList.new(5, LinkedList.new(6, LinkedList.new(4)))
        expected = LinkedList.new(7, LinkedList.new(0, LinkedList.new(8)))

        result = LinkedListDigits.solve(list1, list2)
        expect(result.vals_to_s).to eql expected.vals_to_s
    end

    it "solves different number of digits" do
        list1  = LinkedList.new(4, LinkedList.new(3))
        list2  = LinkedList.new(5, LinkedList.new(6, LinkedList.new(4)))
        expected = LinkedList.new(9, LinkedList.new(9, LinkedList.new(4)))

        result = LinkedListDigits.solve(list1, list2)
        expect(result.vals_to_s).to eql expected.vals_to_s
    end

    it "handles single digits" do
        list1 = LinkedList.new(3)
        list2 = LinkedList.new(4)
        expected = LinkedList.new(7)

        result = LinkedListDigits.solve(list1, list2)
        expect(result.vals_to_s).to eql expected.vals_to_s
    end

    it "handles empty list" do
        list1 = LinkedList.new()
        list2  = LinkedList.new(4, LinkedList.new(3))
        expected = list2

        result = LinkedListDigits.solve(list1, list2)
        expect(result.vals_to_s).to eql expected.vals_to_s
    end
end
