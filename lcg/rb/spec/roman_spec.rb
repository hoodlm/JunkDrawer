require_relative '../roman'

describe RomanNumeral do
    it 'converts IV to 4' do
        expect(RomanNumeral.new("IV").to_i).to eql 4
    end

    it 'converts V to 5' do
        expect(RomanNumeral.new("V").to_i).to eql 5
    end

    it 'converts III to 3' do
        expect(RomanNumeral.new("III").to_i).to eql 3
    end

    it 'converts I to 1' do
        expect(RomanNumeral.new("I").to_i).to eql 1
    end

    it 'converts IX to 9' do
        expect(RomanNumeral.new("IX").to_i).to eql 9
    end

    it 'converts VII to 7' do
        expect(RomanNumeral.new("VII").to_i).to eql 7
    end

    it 'converts XVII to 17' do
        expect(RomanNumeral.new("XVII").to_i).to eql 17
    end

    it 'converts XXXIX to 39' do
        expect(RomanNumeral.new("XXXIX").to_i).to eql 39 
    end

    it 'converts LIX to 59' do
        expect(RomanNumeral.new("LIX").to_i).to eql 59
    end

    it 'converts CCXLVI to 246' do
        expect(RomanNumeral.new("CCXLVI").to_i).to eql 246 
    end
end
