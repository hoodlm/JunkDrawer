require 'fileutils'
require 'base64'
require 'json'
require 'curb'
require 'logger'

LOGGER = Logger.new($stderr)

class OllamaClient
  def initialize(model_options)
    @model_options = model_options
  end

  def zero_shot(message)
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
      return response["response"]
    else
      LOGGER.error(result)
      return "ERROR: #{result}"
    end
  end
end

def analyze(prompt, file)
  file_content = File::read(file)
  "#{prompt}\n\n#{file_content}"
end

llm_config = { 
  model: "qwen3:8b",
  think: false,
}

llm_client = OllamaClient.new(llm_config)

analyze_compound_interest = <<-PROMPT
Below, I've provided the notes of someone attempting to solve a compound interest math problem.
Your task is to extract the answer from their notes.

They may or may not arrive at the correct answer, but their work should arrive at a final answer of some kind.
The person's answer may be presented as a raw number or as a monetary figure.
The output may be shell-escaped (e.g. backslashes or extra decimals).
It may have two decimal places (cents) or may be rounded to the nearest dollar.
Any of these formats are acceptable.

If they 'round' the answer to the nearest dollar, but included a more precise figure earlier,
use the more precise figure as the answer.

Extract their answer and report it as a plain number in <ANSWER> XML tags, like this:

<ANSWER>12345</ANSWER>
or
<ANSWER>63341.93</ANSWER>

IMPORTANT: DO NOT ATTEMPT TO CALCULATE THE ANSWER YOURSELF!
If you are not sure which number is the intended answer,
just do NOT include <ANSWER> tags in your response.

Here are the notes from which to extract the answer:
PROMPT

input_directory = "./results/20250713T210330Z/math"
files = Dir::open(input_directory).children().filter { |f| f.include?("compound_interest") }

actual_answer = 26532.98
results = files.map do |f|
  # filename is like "gemma3n_e4b_compound_interest_an-uneducated-person_4.txt"
  # or more generally $MODEL_$TASK_$ROLE_$N.txt
  # extract this metadata
  file_matchdata = f.match(/(\S+)_compound_interest_(\S+)_(\d).txt/)
  model = file_matchdata[1]
  role = file_matchdata[2]
  n = file_matchdata[3]

  filepath = "#{input_directory}/#{f}"
  LOGGER.info("analyzing #{filepath}")
  prompt = analyze(analyze_compound_interest, filepath)
  raw_output = llm_client.zero_shot(prompt)
  LOGGER.info(raw_output)
  matchdata = raw_output.match(/<ANSWER>(\S+)<\/ANSWER>/)
  extracted_answer = (matchdata && matchdata[1] && matchdata[1].to_f) || nil
  error = if extracted_answer then (extracted_answer - actual_answer) else nil end
  "#{f},#{model},#{role},#{n},#{extracted_answer},#{error}"
end

puts "filename,model,role,test_run,answer,error"
results.each { |r| puts r }
