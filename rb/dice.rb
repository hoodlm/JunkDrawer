ROUNDS=10000
DICE_SIDES=6
N_DICE=6

def zilch_score(set)
  # first look for a run:
  if set == [6, 5, 4, 3, 2, 1]
    return 1500
  end
  score = 0
  # greedily collect 3-of-a-kinds, then fallback to single 1s/5s 
  prev = nil
  in_a_row = 0
  set.each do |die|
    if prev
      if die == prev
        in_a_row = in_a_row + 1
        if in_a_row == 3
          score = score + score_three_of_a_kind(die)
          in_a_row = 0
          next # can't be used for scoring twice
        end
      else
        in_a_row = 0
      end
    end
    if die == 1
      score = score + 100
    elsif die == 5
      score = score + 50
    end
    prev = die
  end
  score
end

def score_three_of_a_kind(die)
  if die == 1
    1000
  else
    die * 100
  end
end

results = []
ROUNDS.times do |n|
  set = (1..N_DICE).map { Random.rand(1..DICE_SIDES) }.sort.reverse
  results << [ set, zilch_score(set) ]
end

results.each { |r| puts "#{r}" }
puts "#{N_DICE} dice, #{DICE_SIDES}-sided dice, #{ROUNDS} rolls..."
non_scoring = results.count { |it| it[1] == 0 }
avg_score = results.map { |it| it[1] }.sum / results.size
puts "average: #{avg_score}"
puts "zilch %: #{100 * non_scoring / results.size}"
