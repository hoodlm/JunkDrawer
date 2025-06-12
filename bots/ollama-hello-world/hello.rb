require 'fileutils'
require 'base64'
require 'json'
require 'curb'
require 'logger'

LOGGER = Logger.new($stderr)

class HelloOllama
  def initialize(model_options)
    @model_options = model_options
  end

  def one_shot(message)
    url = "http://localhost:11434/api/generate"
    request = @model_options.merge({
      prompt: message,
      stream: false,
    })

    LOGGER.info("Prompting: #{request}")
    result = Curl.post(url, request.to_json) do |http|
      http.headers["Content-Type"] = "application/json"
    end
    LOGGER.info("Response code #{result.status}")

    if result.status.start_with? "200"
      response = JSON.parse(result.body)
      puts response["response"]
    else
      LOGGER.error(result)
    end
  end
end

bot = HelloOllama.new({
  model: "qwen3:14b",
  think: true,
})

bot.one_shot("Tell me a joke about computers")
