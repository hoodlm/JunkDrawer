def dollar_a_day(dollars: 1.0, annualized_interest: 0.05, years: 10)
  balance = 0
  days = 365 * years
  daily_interest_rate = annualized_interest / 365.0

  puts "Saving $#{dollars} a day for #{years} years (#{days} days) at interest rate #{annualized_interest} (daily: #{daily_interest_rate})"
  balance = save_and_compound(dollars: dollars, interest_periods: days, interest_per_period: daily_interest_rate)

  saved = dollars * days
  puts "Total saved: #{saved}, after #{years} years is worth #{balance}: earned #{balance - saved} in interest"
end

def dollar_a_month(dollars: 1.0, annualized_interest: 0.05, years: 10)
  balance = 0
  months = 12 * years
  monthly_interest_rate = annualized_interest / 12.0

  puts "Saving $#{dollars} a month for #{years} years (#{months} months) at interest rate #{annualized_interest} (monthly: #{monthly_interest_rate})"
  balance = save_and_compound(dollars: dollars, interest_periods: months, interest_per_period: monthly_interest_rate)

  saved = dollars * months
  puts "Total saved: #{saved}, after #{years} years is worth #{balance}: earned #{balance - saved} in interest"
end

def save_and_compound(dollars: 1.0, interest_periods: 12, interest_per_period: 0.004166666666666667)
  balance = 0
  interest_periods.times do |n|
    balance = balance * (1.0 + interest_per_period)
    balance = balance + dollars
  end
  balance
end
