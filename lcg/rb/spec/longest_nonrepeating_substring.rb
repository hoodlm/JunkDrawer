require_relative '../longest_nonrepeating_substring'

describe LongestNonrepeatingSubstring do
    it 'solves provided example abcabcbb' do
        expect(subject.solve("abcabcbb")).to eql "abc"
    end

    it 'solves provided example bbbbb' do
        expect(subject.solve("bbbbb")).to eql "b"
    end

    it 'solves provided example pwwkew' do
        expect(subject.solve("pwwkew")).to eql "wke"
    end

    it 'handles empty string' do
        expect(subject.solve("")).to eql ""
    end

    it 'handles string with one char' do
        expect(subject.solve("a")).to eql "a"
    end

    it 'handles string with all unique chars' do
        expect(subject.solve("abcdefgh")).to eql "abcdefgh"
    end

    it 'handles whitespace' do
        expect(subject.solve("aaba ca")).to eql "ba c"
    end

    it 'handles numbers' do
        expect(subject.solve("aaba1ca")).to eql "ba1c"
    end

    it 'handles a very long string' do
        long_string = "abcd" * 50_000
        expect(subject.solve(long_string)). to eql "abcd"
    end
end
