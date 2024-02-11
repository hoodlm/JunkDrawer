class LongestNonrepeatingSubstring
    def solve(string)
        if (string.empty?)
            return ""
        end


        start_index = 0
        end_index = 0
        longest_sequence = string[start_index..end_index]

        until end_index > string.length
            end_index += 1
            candidate_sequence = string[start_index..end_index]
            unique_characters = candidate_sequence.chars().uniq

            all_unique = unique_characters.size == candidate_sequence.size
            until all_unique
                start_index += 1
                if start_index > end_index
                    fail "BUG: start_index overran end_index #{start_index} > #{end_index}"
                end
                candidate_sequence = string[start_index..end_index]
                all_unique = unique_characters.size == candidate_sequence.size
            end
            if longest_sequence.size < candidate_sequence.size
                longest_sequence = candidate_sequence
            end
        end

        return longest_sequence
    end
end
