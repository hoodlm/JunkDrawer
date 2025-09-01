require 'aws-sdk-ec2'
require 'terminal-table'

# Prints the top 25 cheapest 'current gen' x86_64 instance types sorted by current $/vCPU pricing.

# use us-east-1 for 'global' calls to get list of regions and list of instance types
ec2 = Aws::EC2::Client.new(profile: 'ec2-read-only', region: 'us-east-1')
puts "Fetching regions..."
regions = ec2.describe_regions.regions.map { |it| it.region_name }
puts "Fetching instance types..."
raw_instance_types = []
ec2.describe_instance_types({
  filters: [
    { name: "current-generation", values: ["true"] }
  ]
}).each do |response|
  raw_instance_types << response.instance_types
end
raw_instance_types.flatten!

instance_types = raw_instance_types
  .filter { |it| it.v_cpu_info.default_v_cpus > 4 }
  .filter { |it| it.supported_usage_classes.include? "spot" }
  .filter { |it| it.processor_info.supported_architectures.include? "x86_64" }
  .reject { |it| it.instance_type.include? "flex" }

puts "#{raw_instance_types.size} instance types found, filtered down to #{instance_types.size}"

threads = regions.map do |region|
  puts "fetching spot instance prices in #{region}"
  Thread.new do
    region_spot_prices = []
    ec2 = Aws::EC2::Client.new(profile: 'ec2-read-only', region: region)

    end_time = Time.now
    start_time = end_time
    spot_price_history_req = {
      start_time: start_time,
      end_time: end_time,
      instance_types: instance_types.map { |it| it.instance_type },
      product_descriptions: ["Linux/UNIX"]
    }
    ec2.describe_spot_price_history(spot_price_history_req).each do |response|
      region_spot_prices << response.spot_price_history
    end
    region_spot_prices.flatten
  end
end

spot_prices = []
threads.each do |t|
  spot_prices << t.value
end

spot_prices.flatten!
puts "For #{instance_types.size} instance types, found #{spot_prices.size} spot prices across #{regions.size} regions"

header = ["$/vCPU/hr", "$/hr", "instance-type", "az", "n vCPUs", "mem (G)"]
data = []
spot_prices.flatten.each do |spot_price|
  price = spot_price.spot_price
  instance_type = spot_price.instance_type
  az = spot_price.availability_zone_id
  instance_data = instance_types.find { |it| it.instance_type == instance_type }
  v_cpus = instance_data.v_cpu_info.default_v_cpus
  mem_gb = instance_data.memory_info.size_in_mi_b / 1024
  cost_per_vcpu = price.to_f / v_cpus.to_f
  data << [
    cost_per_vcpu,
    price,
    instance_type,
    az,
    v_cpus,
    mem_gb,
  ]
end

sorted = data.sort_by { |it| it.first }
table = Terminal::Table.new do |t|
  t << header
  t.add_separator
  sorted[1..25].each do |data|
    t.add_row(data)
  end
end
puts table
