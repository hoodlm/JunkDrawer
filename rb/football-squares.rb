# bengals 2024 scores. these are cumulative scores, NOT box scores.
SCORES = [
  [[0, 0], [0, 10], [7, 13], [10, 16]],
  [[3, 3], [16, 10], [22, 17], [25, 26]],
  [[7, 7], [13, 21], [20, 28], [33, 38]],
  [[7, 0], [21, 14], [31, 21], [34, 24]],
  [[0, 7], [17, 14], [24, 21], [38, 41]],
  [[7, 0], [7, 0], [10, 7], [17, 7]],
  [[7, 0], [7, 6], [21, 6], [21, 14]],
  [[7, 0], [10, 10], [17, 24], [17, 37]],
  [[7, 7], [17, 10], [31, 10], [41, 24]],
  [[7, 0], [14, 7], [21, 14], [34, 35]],
  [[3, 7], [6, 24], [20, 27], [27, 34]],
  [[14, 7], [21, 27], [24, 34], [38, 44]],
  [[7, 7], [17, 10], [17, 17], [27, 20]],
  [[7, 14], [24, 14], [31, 14], [37, 27]],
  [[7, 0], [17, 0], [17, 6], [24, 6]],
  [[0, 3], [7, 3], [10, 10], [30, 24]],
  [[10, 0], [13, 7], [16, 7], [19, 17]],
]

def assert(fact, message)
  fail message unless fact
end

# Sanity check scores...
SCORES.each_with_index do |game, n|
  q1 = game[0]
  q2 = game[1]
  q3 = game[2]
  final = game[3]

  assert(q1[0] <= q2[0] && q2[0] <= q3[0] && q3[0] <= final[0], "game #{n}, team0 scores decreased in some quarter")
  assert(q1[1] <= q2[1] && q2[1] <= q3[1] && q3[1] <= final[1], "game #{n}, team1 scores decreased in some quarter")
end

# initiate payout array, 10x10 initialized to zeroes
payouts = Array.new(10) { Array.new(10, 0) }

SCORES.each do |game|
  4.times do |q|
    payout = if ((q + 1) == 4) then 350 else 200 end
    q_score = game[q]
    score_home = q_score[0]
    score_away = q_score[1]
    # add payout to the lucky square...
    current = payouts[score_home % 10][score_away % 10]
    new = current + payout
    payouts[score_home % 10][score_away % 10] = new
  end
end

expected_total = (SCORES.size * 3 * 200) + (SCORES.size * 350)
actual_total = payouts.flatten.reduce(&:+)

if actual_total != expected_total
  puts "WARN: actual total #{actual_total} does not equal expected total #{expected_total}"
end

table = [[' ', (0..9).to_a].flatten]
payouts.each_with_index do |payout, n|
  row = [n]
  row << payout
  table << row
end


# pretty-printer
table.each do |row|
  puts row.join("\t")
end

# compute some stats
avg = actual_total / 100.0
zeros = payouts.flatten.count { |it| it == 0 }
better_than_breakeven = payouts.flatten.count { |it| it > 200 }

puts "Average: #{avg}"
puts "Number of zeroes: #{zeros}"
puts "Squares more than $200: #{better_than_breakeven}"

buckets = payouts.flatten.sort.uniq

buckets.each do |bucket|
  count = payouts.flatten.count { |it| it == bucket }
  puts "$#{bucket} -> #{count}"
end
