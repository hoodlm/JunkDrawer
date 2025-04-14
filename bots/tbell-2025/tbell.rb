require 'fileutils'
require 'telegram/bot'
require 'base64'

LOGGER = Logger.new($stderr)
CONFIG_DIRECTORY = ENV['HOME'] + "/.tbellbot"
FileUtils.mkdir_p(CONFIG_DIRECTORY)
TOKEN = File.open(CONFIG_DIRECTORY + "/token.txt").read.chomp

class CommandDispatcher
  def handle_message(message)
    method = message.text.downcase
    return nil unless method.start_with?('/')

    case method.split.first
    when '/help'
      help()
    when '/taco'
      taco()
    when '/troll'
      troll()
    when '/dg'
      dg(method)
    else
      fallback(method)
    end
  end

  def fallback(request)
    "Hey #{Base64.decode64('ZnVja2JhZw==')}, i don't know what '#{request}' means. do you need some /help ?"
  end

  def help
    "Supported commands: /taco /troll /dg [keyword]"
  end

  def taco
    TacoPicker.generate
  end

  def dg(message)
    @@dg ||= QuoteDatabase.new(CONFIG_DIRECTORY + "/dg.txt")
    search_term = message.split()[1..].join(" ").chomp
    lyric = if search_term.empty?
      @@dg.random
    elsif (search_term.to_i > 0)
      @@dg.sample(search_term.to_i.clamp(1..16))
    else
      @@dg.search(search_term) || "(no lyrics found)"
    end
    lyric.upcase
  end

  def troll
    rating = Random.rand(0..10)
    "#{rating}/10"
  end
end

class TacoPicker
  ADDITIONAL_ADJECTIVE_PROBABILITY = 0.40
  ADJECTIVES = [
    "7-layer", "Loaded", "Bell", "Nacho", "Grilled", "Stuft", "Double Decker",
    "Volcano", "Doritos Locos", "Nacho Cheese", "Cheesy",
    "Soft", "Crunchy",
    "Bean", "Cheese", "Steak", "Chicken", "Beef", "Bean", "Rice",
    "Triple", "Double",
    "Mexican",
  ]

  DISHES = [
    "Taco", "Burrito", "Quesadilla", "Crunchwrap", "Nachos", "Chalupa", "Gordita", "Bowl",
  ]

  SUFFIX_PROBABILITY = 0.45
  SUFFIXES = [
    "Supreme", "Crunch", "Combo",
  ]

  def self.generate
    result = []
    # Always guarantee one adjective
    result += ADJECTIVES.sample(1)
    # Roll the dice to add more
    while Random.rand < ADDITIONAL_ADJECTIVE_PROBABILITY
      result += ADJECTIVES.sample(1)
    end 

    result += DISHES.sample(1)
    if Random.rand < SUFFIX_PROBABILITY
      result += SUFFIXES.sample(1)
    end

    result.join(" ")
  end
end

class QuoteDatabase
  def initialize(db_path)
    @quotes = File.open(db_path)
      .read
      .split("\n")
      .reject { |it| it.empty? }
      .map { |it| it.chomp }
      .uniq
  end

  def sample(n)
    @quotes.sample(n).join("\n")
  end

  def random
    @quotes.sample(1).first
  end

  def search(search_term)
    LOGGER.info("searching for quotes that include: #{search_term}")
    matches = @quotes.filter do |quote|
      quote.downcase.include?(search_term.downcase)
    end
    matches.sample(1).first
  end
end

c = CommandDispatcher.new

Telegram::Bot::Client.run(TOKEN, logger: LOGGER) do |bot|
  bot.listen do |message|
    response = begin
      c.handle_message(message)
    rescue => ex
      LOGGER.error($@)
      "ERROR: #{ex}"
    end  
    if response
      if response.length > 2000
        LOGGER.warn("Truncating response to 2000 characters")
        response = response[0..2000]
      end
      LOGGER.info("Responding: #{response}")
      bot.api.send_message(chat_id: message.chat.id, text: response)
    end
  end
end

