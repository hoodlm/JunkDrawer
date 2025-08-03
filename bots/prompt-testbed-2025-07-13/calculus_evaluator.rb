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
  model: "qwen3:4b",
  think: false,
}

llm_client = OllamaClient.new(llm_config)

analyze_calculus = <<-PROMPT
Below, I've provided the notes of someone attempting to solve a calculus problem.
They will show their notes and eventually arrive at a final answer.
Your task is to check if the answer they arrived at is correct.

The ACTUAL answer may either of these forms of the equation:

(A)  y'(x) = (3 x^2 + 4 x - 3)/(2 x^(3/2))
(B)  y'(x) = -3/(2 x^(3/2)) + (3 sqrt(x))/2 + 2/sqrt(x)

The terms may be algebraically rearranged.

(C)  y'(x) = (3 sqrt(x))/2 + 2/sqrt(x) - 3/(2 x^(3/2))
(D)  y'(x) = (3 sqrt(x))/2 + 2/sqrt(x) - 3/(2x * sqrt(x))

Both C and D are equivalent to B and are considered CORRECT, ACTUAL answers.

The supplied answer may be formatted in other markup syntax, for example:

\\frac{3}{2}\\sqrt{x} + \\frac{2}{\\sqrt{x}} - \\frac{3}{2x\\sqrt{x}}}

This is a different syntax but semantically identical to D.

Please think out loud and take notes. Follow these steps:

(1) Identify the submitted answer from the supplied notes
(2) Analyze whether the submitted answer is equivalent to one of the ACTUAL, CORRECT answers above.
(3) After your notes provide your analysis to me in a structured form:

Do the notes demonstrate a serious attempt at solving the problem?

<ATTEMPTED>Yes/No</ATTEMPTED>

Is the submitted answer equivalent to one of the CORRECT answers?

<CORRECT>Yes/No/NotSure</CORRECT>

How many algebraic steps if any did you have to do to transform the supplied answer into one of the correct answers given above?
(If you think CORRECT is 'No' then put 0 here)

<ALGEBRA>3</ALGEBRA>

Finally, take one last look at your work. How confident are you that you made the correct judgment on CORRECTNESS (1-5, with 5 being most confident)

<CONFIDENCE>4</CONFIDENCE>

Example output:

<ATTEMPTED>Yes</ATTEMPTED>
<CORRECT>Yes</CORRECT>
<ALGEBRA>1</ALGEBRA>
<CONFIDENCE>3</CONFIDENCE>

Here are the notes:
PROMPT

input_directory = "./results/20250713T210330Z/math"
files = Dir::open(input_directory).children().filter { |f| f.include?("calculus") }

results = files.map do |f|
  # filename is like "gemma3n_e4b_calculus_an-uneducated-person_4.txt"
  # or more generally $MODEL_$TASK_$ROLE_$N.txt
  # extract this metadata
  file_matchdata = f.match(/(\S+)_calculus_(\S+)_(\d).txt/)
  model = file_matchdata[1]
  role = file_matchdata[2]
  n = file_matchdata[3]

  filepath = "#{input_directory}/#{f}"
  LOGGER.info("analyzing #{filepath}")
  prompt = analyze(analyze_calculus, filepath)
  raw_output = llm_client.zero_shot(prompt)
  LOGGER.info(raw_output)
  attempted = raw_output.match(/<ATTEMPTED>(\S+)<\/ATTEMPTED>/)
  attempted = attempted && attempted[1] || "ERROR"
  correct = raw_output.match(/<CORRECT>(\S+)<\/CORRECT>/)
  correct = correct && correct[1] || "ERROR"
  alg_steps = raw_output.match(/<ALGEBRA>(\S+)<\/ALGEBRA>/)
  alg_steps = alg_steps && alg_steps[1] || "ERROR"
  confidence = raw_output.match(/<CONFIDENCE>(\S+)<\/CONFIDENCE>/)
  confidence = confidence && confidence[1] || "ERROR"
  csv_row = "#{f},#{model},#{role},#{n},#{correct},#{attempted},#{alg_steps},#{confidence}"
  puts csv_row
  csv_row
end

puts "filename,model,role,test_run,correct_answer,attempted,algebra_steps,graders_confidence"
results.each { |r| puts r }
