require 'fileutils'
require 'json'
require 'curb'
require 'logger'
require 'pry'

LOGGER = Logger.new($stderr)

class HelloOpenRouter
  def initialize(key)
    @key = key
  end

  def one_shot(message)
    url = "https://openrouter.ai/api/v1/chat/completions"
    request = {
      model: "moonshotai/kimi-k2:free",
      messages: [
        role: "user",
        content: message,
      ]
    }

    headers = {
      "Content-Type" => "application/json",
      "Authorization" => "Bearer #@key",
    }

    LOGGER.info("Prompting: #{request}")
    result = Curl.post(url, request.to_json) do |http|
      http.headers = headers
    end
    LOGGER.info("Response code #{result.status}")
    # binding.pry
    response = JSON.parse(result.body)
    LOGGER.debug(response)

    if result.status.start_with? "200"
      content = response["choices"][0]["message"]["content"]
      content
    else
      LOGGER.error(result)
      nil
    end
  end
end

key = File.read("./key.txt").strip
bot = HelloOpenRouter.new(key)
response = bot.one_shot("Can humans with tetrochromatic eyesight distinguish more colors than people with typical eyesight?")
puts response
