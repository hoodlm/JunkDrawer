require 'fileutils'
require 'base64'
require 'json'
require 'curb'
require 'logger'

LOGGER = Logger.new($stderr)

def build_prompt(role, task)
<<-PROMPT
INSTRUCTION: Tailor your thought process, recommendations, and word choice to align with the provided role.
<ROLE>
You are #{role}
</ROLE>
#{task.instruction}
PROMPT
end

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

class Task
  attr_reader :name, :instruction
  def initialize(name, instruction)
    @name = name
    @instruction = instruction
  end
end

computer_joke = Task.new("computer_joke", "Tell me a joke about computers")
explain_borrow_checker = Task.new("explain_borrow_checker", "Explain the Rust borrow checker")
explain_async = Task.new("explain_async", "Explain the 'async' keyword in Rust")
rust_code_review = Task.new("rust_code_review", <<-TEXT
Your task is to analyze the following code sample in the INPUT block.

<INPUT>
    fn collect_tokens(&self, input: &String) -> Vec<Token> {
        let mut tokens = Vec::new();
        if input.is_empty() {
            return tokens;
        }
        Token::all().iter().find(|token_kind| {
            let regex = self.token_matcher.regex(token_kind);
            let token_match = regex.find(input);
            if token_match.is_some() {
                tokens.push(Token {
                    name: **token_kind,
                    value: self.token_matcher.pack_value(token_kind, token_match.unwrap().as_str()),
                });
                let skip_index = token_match.unwrap().end();
                let remaining_input = &input[skip_index..].to_string();
                let mut more_tokens = self.collect_tokens(remaining_input);
                tokens.append(&mut more_tokens);
                true
            } else {
                false
            }
        });
        return tokens;
    }
</INPUT>

There are four subtasks below. Complete each sub-task in-order.
(PURPOSE) Identify what the purpose of the code is.
(QUALITY) Evaluate the quality of this code for correctness, readability, and overall maintainability.
(PERFORMANCE) Evaluate the performance characteristics of this code.
(FIXES) List three fixes or improvements to the code in order of importance.
TEXT
)

calculus = Task.new("calculus", "Find the derivative of this function: y = (x^2 + 4x + 3)/sqrt(x) ")
compound_interest = Task.new("compound_interest", "If I invest $10000 at an average rate of return of 5% per year, how much money should I expect to have in 20 years?")

literature_math_in_medieval = Task.new("math_in_medieval_review", <<-PROMPT
Write a short critical response to this excerpt of text from 'Mathematics for the Nonmathematician' by Morris Kline.

<excerpt>
Mathematics in the Medieval Period

We see that a new civilization did arise in Europe, but form the standpoint of
the perpetuation of mathematical learning or the creation of mathematics, it
was totally ineffective. Although this civilization did spread ethical teachings,
fostered Gothic architecture and great religious paintings, no scientific,
technological, or mathematical concept gained any foothold. In none of the
civilizations which have contributed to the modern age was mathematical learning
reduced to so low a level.
</excerpt>
PROMPT
                                      )

literature_myst = Task.new("myst_tiana_review", <<-PROMPT
Write a short analytical essay on this excerpt of text from 'Myst: The Book of Tiana' by Rand Miller.

<excerpt>
Watching Veovis from across the pillared hallway, seeing how easily the young Lord
moved among his peers, how relaxed he was dealing with the high and mighty of D'ni
society, Aitrus found it strong how close they had grown since their reunion thirty
years ago. If you had asked him then who might have been his closest friend and
confidant in later years, he might have chosen anyone but Lord Rakeri's son, but so it was.
In the public's eyes they were inseparable.

Inseparable, perhaps, yet very different in their natures. And maybe that was why it
worked so well, for both had a perfect understanding of who the otehr was.

Had they been enemies, then there would have been no late-night debates, no agreements
to differ, no grudging concessions between them, no final meeting of minds, and that
would, in time, have been a tragedy for the Council, for many now recognized that
in the persons of Veovis and Aitrus were the seeds of D'ni's future.

Their friendship had thus proved a good omen, not merely for them but for the great D'ni State.
</excerpt>
PROMPT
                          )


software_roles = [
  "an LLM assistant",
  "the smartest person in the world",
  "a junior software engineer",
  "an expert Rust programmer",
  "a C++ developer",
  "an expert in Compiler software development",
  "a Harvard English professor specializing in 20th century American Literature",
  "an uneducated person",
  "a person with below-average intelligence",
  "a microwave oven",
]

math_roles = [
  "an LLM assistant",
  "the smartest person in the world",
  "an undergraduate Mathematics student",
  "an MIT Math professor",
  "a Harvard English professor specializing in 20th century American Literature",
  "an uneducated person",
  "a person with below-average intelligence",
  "a microwave oven",
]

lit_roles = [
  "an LLM assistant",
  "the smartest person in the world",
  "an undergraduate English Literature student",
  "a 12 year-old",
  "an MIT Math professor",
  "a Harvard English professor specializing in 20th century American Literature",
  "a registered Democrat voter",
  "a registered Republican voter",
  "an uneducated person",
  "a person with below-average intelligence",
  "a microwave oven",
]

lit_tasks = [literature_math_in_medieval, literature_myst]
software_tasks = [computer_joke, explain_borrow_checker, explain_async, rust_code_review]
math_tasks = [calculus, compound_interest]

timestamp = Time.now.utc.iso8601.gsub(/[:-]/, "")
result_dir = "./results/#{timestamp}"
FileUtils::mkdir_p(result_dir)

llm_configs = [
  {
    model: "gemma3n:e4b",
    think: false,
  },
  {
    model: "llama3.1:8b",
    think: false,
  },
  {
    model: "qwen3:4b",
    think: true,
  },
  {
    model: "qwen3:8b",
    think: true,
  },
]

runs = 5

llm_configs.each do |llm_config|
  llm_client = OllamaClient.new(llm_config)
  model_name = llm_config[:model]
  lit_tasks.each do |task|
    lit_roles.each do |role|
      runs.times do |n|
        prompt = build_prompt(role, task)
        response = llm_client.zero_shot(prompt)
        LOGGER.info("#{model_name}: Completed #{task.name} as role '#{role}'")
        output_taskname = task.name
        output_role = role.gsub(/\s/, "-")
        outfile_name = "#{model_name}_#{output_taskname}_#{output_role}_#{n}".gsub(/\s/, "-").gsub(":","_").gsub(".", "_")
        outdir = "#{result_dir}/literature"
        FileUtils::mkdir_p(outdir)
        outfile = "#{outdir}/#{outfile_name}.txt"
        File.write(outfile, response)
        LOGGER.info("Wrote #{outfile}")
      end
    end
  end
  software_tasks.each do |task|
    software_roles.each do |role|
      runs.times do |n|
        prompt = build_prompt(role, task)
        response = llm_client.zero_shot(prompt)
        LOGGER.info("#{model_name}: Completed #{task.name} as role '#{role}'")
        output_taskname = task.name
        output_role = role.gsub(/\s/, "-")
        outfile_name = "#{model_name}_#{output_taskname}_#{output_role}_#{n}".gsub(/\s/, "-").gsub(":","_").gsub(".", "_")
        outdir = "#{result_dir}/software"
        FileUtils::mkdir_p(outdir)
        outfile = "#{outdir}/#{outfile_name}.txt"
        File.write(outfile, response)
        LOGGER.info("Wrote #{outfile}")
      end
    end
  end
  math_tasks.each do |task|
    math_roles.each do |role|
      runs.times do |n|
        prompt = build_prompt(role, task)
        response = llm_client.zero_shot(prompt)
        LOGGER.info("#{model_name}: Completed #{task.name} as role '#{role}'")
        output_taskname = task.name
        output_role = role.gsub(/\s/, "-")
        outfile_name = "#{model_name}_#{output_taskname}_#{output_role}_#{n}".gsub(/\s/, "-").gsub(":","_").gsub(".", "_")
        outdir = "#{result_dir}/math"
        FileUtils::mkdir_p(outdir)
        outfile = "#{outdir}/#{outfile_name}.txt"
        File.write(outfile, response)
        LOGGER.info("Wrote #{outfile}")
      end
    end
  end
end

LOGGER.info("DONE - all results are in output directory: #{result_dir}")
