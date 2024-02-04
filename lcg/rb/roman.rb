class RomanNumeral
    def initialize(string)
        @string = string
    end

    def to_i
        self.tokenize.map(&:to_i).reduce(&:+)
    end

    protected

    def tokenize
        self.next_token.flatten
    end

    def next_token(depth = 0)
        if depth > 30
            raise "next_token called recursively too many times; bottom of the stack is #{@string}"
        end
        if @string.empty?
            return []
        end
        token_classes.each do |token_klass|
            if @string.start_with?(token_klass.literal)
                remainder = RomanNumeral.new(@string.delete_prefix(token_klass.literal))
                return [token_klass.new, remainder.next_token(depth + 1)]
            end
        end
        raise "No token successfully prefix matched: #{@string}"
    end

    def token_classes
        # This also determines the parse order
        [CToken, XLToken, LToken, IXToken, XToken, IVToken, VToken, IToken]
    end
end

class CToken
    def to_i
        100
    end

    def self.literal
        "C"
    end
end

class XLToken
    def to_i
        40
    end

    def self.literal
        "XL"
    end
end

class LToken
    def to_i
        50
    end

    def self.literal
        "L"
    end
end

class IXToken
    def to_i
        9
    end

    def self.literal
        "IX"
    end
end

class XToken
    def to_i
        10 
    end

    def self.literal
        "X"
    end
end

class IToken
    def to_i
        1
    end

    def self.literal
        "I"
    end
end

class IVToken
    def to_i
        4
    end

    def self.literal
        "IV"
    end
end

class VToken
    def to_i
        5
    end

    def self.literal
        "V"
    end
end
