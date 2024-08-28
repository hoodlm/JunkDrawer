def dollar_a_day(dollars: 1.0, annualized_interest: 0.05, years: 10)
  accum = 0
  balance = 0
  days = 365 * years
  daily_interest_rate = annualized_interest / 365.0

  puts "Saving $#{dollars} a day for #{years} years (#{days} days) at interest rate #{annualized_interest} (daily: #{daily_interest_rate})"

  days.times do |day|
    balance = balance * (1.0 + daily_interest_rate)
    balance = balance + dollars
  end

  saved = dollars * days
  puts "Total saved: #{saved}, after #{years} years is worth #{balance}: earned #{balance - saved} in interest"
end

def dollar_a_month(dollars: 1.0, annualized_interest: 0.05, years: 10)
  accum = 0
  balance = 0
  months = 12 * years
  monthly_interest_rate = annualized_interest / 12.0

  puts "Saving $#{dollars} a month for #{years} years (#{months} months) at interest rate #{annualized_interest} (monthly: #{monthly_interest_rate})"

  months.times do |month|
    balance = balance * (1.0 + monthly_interest_rate)
    balance = balance + dollars
  end

  saved = dollars * months
  puts "Total saved: #{saved}, after #{years} years is worth #{balance}: earned #{balance - saved} in interest"
end
