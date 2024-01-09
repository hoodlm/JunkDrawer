#! /usr/bin/env ruby

require 'csv'
require 'date'

def main()
    files = validate_files(ARGV)
    records = files
        .map { |file| parse_file(file) }
        .flatten()
        .sort_by { |x| x[:date] }

    out = CSV.generate do |csv|
        csv << ["date", "title", "artist", "submitter"]
        records.each do |record|
            csv << [record[:date], record[:title], record[:artist], record[:submitter]]
        end
    end
    puts out
end

def validate_files(files)
    if files.empty?()
        puts "Supply at least one CSV file"
        puts "Usage: #{File.basename($0)} 1.csv [2.csv ...]"
        exit 1
    end

    files.each do |file|
        unless File.file?(file)
            fail "File not found: #{file}"
        end
    end
    return files
end



SONG_ROW=/^SONG( TITLE)?/i
ARTIST_ROW=/^ARTIST/i
SUBMITTER_ROW=/^SUBMITTER/i
SONG_ARTIST_OR_SUBMITTER=/^(SONG|ARTIST|SUBMITTER)/i
WEEK_HEADER=/^(Ep|W\d{1,2})/i

EXTRACT_YEAR_FROM_FILENAME=/(\d{4}).csv$/

EXTRACT_DATE_FROM_CELL=/.+ (\w{3} \d{1,2})/
DATE_FORMAT = "%b %d" # "Jul 13"
    

def parse_file(file)
    records = []

    # Filename should follow format like "Mixtape Monday - 2020.csv"
    year_parse = file.match(EXTRACT_YEAR_FROM_FILENAME)
    year = (year_parse && year_parse [1]) || fail("Could not parse year from filename #{file}")

    week = {}
    week_records = []

    expected_next_token = :week
    CSV.foreach(file) do |row|
        case expected_next_token
        when :week
            next if row.empty?
            if row.any? { |cell| cell && cell.match(SONG_ARTIST_OR_SUBMITTER) }
                error_unexpected_row!(file, row, expected_next_token)
            elsif row.any? { |cell| cell && cell.match(WEEK_HEADER) }
                # try to parse some kind of date out of this row...
                this_date = nil
                row.find do |cell|
                    if cell.nil?
                        # skip
                    else
                        month_day_substring = cell.match(EXTRACT_DATE_FROM_CELL)
                        month_day = month_day_substring && month_day_substring[1] || next
                        parsed_month_day = Date._strptime(month_day, "%b %d")
                        this_date = Date.new(year.to_i, parsed_month_day[:mon], parsed_month_day[:mday])
                    end
                end
                if this_date.nil?
                    fail "#{file}: Expected to find a valid date in row: #{row}"
                end
                week[:date] = this_date.strftime("%Y-%m-%d")
                expected_next_token = :song_title
            end
        when :song_title
            next if row.empty?
            if row.any? { |cell| cell && cell.match(SONG_ROW) }
                row.each_with_index do |cell, n|
                    if n == 0
                        # this is the "SONG TITLE" header; skip this one
                    elsif cell.nil?
                        # also skip
                    else
                        record = week.clone()
                        record[:title] = cell.strip()
                        week_records << record
                    end
                end
                    
                expected_next_token = :artist_name
            else
                error_unexpected_row!(file, row, expected_next_token)
            end
        when :artist_name
            next if row.empty?
            if row.any? { |cell| cell && cell.match(ARTIST_ROW) }
                row.each_with_index do |cell, n|
                    if n == 0
                        # this is the "ARTIST" header; skip this one
                    elsif cell.nil?
                        # also skip
                    else
                        unless week_records[n - 1]
                            fail "#{file}: too many ARTISTS in row: #{row} ; current records #{week_records}"
                        end
                        week_records[n - 1][:artist] = cell.strip()
                    end
                end
                expected_next_token = :submitter
            else
                error_unexpected_row!(file, row, expected_next_token)
            end
        when :submitter
            if row.any? { |cell| cell && cell.match(SUBMITTER_ROW) }
                row.each_with_index do |cell, n|
                    if n == 0
                        # this is the "SUBMITTER" header; skip this one
                    elsif cell.nil?
                        # also skip
                    else
                        unless week_records[n - 1]
                            fail "#{file}: too many SUBMITTERS in row: #{row} ; current records #{week_records}"
                        end
                        week_records[n - 1][:submitter] = cell.strip()
                    end
                end
                # done parsing this week; reset...
                week_records.each do |record|
                    records << record.clone()
                end
                week_records.clear()
                week.clear()
                expected_next_token = :week
            else
                error_unexpected_row!(file, row, expected_next_token)
            end
        else
            fail "Unhandled state #{expected_next_token}"
        end
    end
    return records.flatten
end

def error_unexpected_row!(file, row, expected_next_token)
    fail "#{file}: Unexpected row #{row.join(',')}; expected_next_token #{expected_next_token}"
end

main()
