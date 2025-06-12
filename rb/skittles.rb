ROUNDS=1_000_000
PACK_SIZE=13
N_COLORS=5

def predicate(pack)
  pack.to_set.size <= 2
end

pass = 0
ROUNDS.times do |n|
  pack = (1..PACK_SIZE).map { Random.rand(N_COLORS) }
  if predicate(pack)
    pass = pass + 1
    puts "#{n}/#{ROUNDS}: #{pack}"
  end
end

rate = pass.to_f / ROUNDS.to_f
puts "#{pass}/#{ROUNDS} = #{rate}"
