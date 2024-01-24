#! /usr/bin/env ruby

require 'csv'

def main()
    file = validate_file(ARGV)
    puts render_one_table(file)
end

def validate_file(args)
    if args.empty?() || args.size > 1
        puts "Supply exactly one CSV file"
        puts "Usage: #{File.basename($0)} in.csv"
      exit 1
    end
    
    file = args[0]
    unless File.file?(file)
        fail "File not found: #{file}"
    end
    return file
end

def render_one_table(file)
    csv_table = CSV.read(file)
    width_vector = get_column_width_vector(csv_table)
    md_table = []
    md_table << format_row(csv_table[0], width_vector)
    md_table << horizontal_rule(width_vector)
    csv_table[1..].each do |row|
        md_table << format_row(row, width_vector)
    end
    md_table.join("\n")
end

def horizontal_rule(width_vector)
    total_width = (width_vector.reduce(&:+) + 2 * (width_vector.size) - 1)
    "|" + "-" * total_width + "|"
end

def format_row(row, width_vector)
    if row.size != width_vector.size
        fail "row.size #{row.size} != width_vector.size #{width_vector.size}" 
    end
    out = [""]
    row.each_with_index do |field, index|
        width = width_vector[index]
        field ||= ""
        out << field.ljust(width + 1)
    end
    out << ""
    out.join("|")
end

def get_column_width_vector(csv_table)
    max_column_widths = csv_table[0].map { |header| header.length }
    csv_table[1..].each do |row|
        row.each_with_index do |field, index|
            if field && max_column_widths[index] < field.length
                max_column_widths[index] = field.length
            end
        end
    end
    return max_column_widths
end

main()
