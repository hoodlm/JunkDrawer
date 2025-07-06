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
  model: "gemma3n:latest",
  think: false,
})

def build_prompt(expense)
<<-PROMPT
Pick the best category for the given expense. Respond with a brief justification, the name of the category from the list below, and confidence 1-5.

EXAMPLE INPUT 1:
Expense: "Zoo"

EXAMPLE OUTPUT 2:
justification: "The Zoo is a recreational activity one would do with a family"
category: "Entertainment"
confidence: 5


EXAMPLE INPUT 1:
Expense: "7-11"

EXAMPLE OUTPUT 2:
justification: "7-11 probably refers to the convenience store. One could purchase gas or convenience store items there, but more likely gas"
category: "Transportation"
confidence: 2

Choose from only these categories:

<categories>
Food
Entertainment
Household Items
Utilities
Home Improvement
Transportation
</categories>

---

Expense: "#{expense}"
PROMPT
end

[
  "Human Service Department",
  "Indian Mountain Machine Shop",
  "UBank",
  "Jellico Electric & Water System",
  "Subway",
  "Days Inn",
  "McDonald's",
  "Shell",
  "Douglas Oil Shop",
  "Crouches Creek Baptist Church",
  "Patriotic Palace",
  "Jimmys Market 18",
  "Rolling Coal",
  "bp",
  "Douglas Cemetery #4",
].each do |expense|
  prompt = build_prompt(expense)
  bot.one_shot(prompt)
end
