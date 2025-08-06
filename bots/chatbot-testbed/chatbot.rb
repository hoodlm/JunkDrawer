require 'json'
require 'curb'
require 'logger'
require 'pry'

LOGGER = Logger.new($stderr)

class OllamaClient
  def initialize(model_options)
    @model_options = model_options
  end

  def prompt(message)
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
      LOGGER.debug(response)
      return response["response"]
    else
      LOGGER.error(result)
      return "ERROR: #{result}"
    end
  end
end

class Character
  def initialize(character_name, player_name, llm_client, system_prompt, character_prompt)
    @character_name = character_name
    @player_name = player_name
    @llm_client = llm_client
    @context = ""
    initialize_context(system_prompt, character_prompt)
  end

  def initialize_context(system_prompt, character_prompt)
    @context = %Q(
      #{system_prompt}

      Here is a profile of your character, #@character_name:
      <profile>
      #{character_prompt}
      </profile>
    ).strip!
  end

  def human_says(content)
    @context << "\n#@player_name: #{content}"
    content
  end

  def next_dialog
    prompt = @context.clone
    prompt << "INSTRUCTIONS: Write {{char}}'s next line of dialog"
    prompt.gsub!("{{char}}", @character_name)
    prompt.gsub!("{{human}}", @player_name)
    dialog = @llm_client.prompt(prompt)
    # The LLM usually prefixes the character name + colon, but add it if not:
    unless dialog =~ /^#@character_name:/
      dialog = "#@character_name: #{dialog}"
    end
    @context << "\n#{dialog}"
    dialog
  end

  def chatlog
    @context
  end
end

llm_client = OllamaClient.new(model: "mistral-nemo:12b")
system_prompt = %Q(
This is a fictional conversation between a character {{char}} and a human {{human}}.
You will be contributing the dialog and emoted actions for {{char}}.
Put dialog in quotation marks and emoted actions in *asterisks*.
You should only ever speak or act as {{char}}.
NEVER write dialog or actions on behalf of {{human}}.
)
character_prompt = %Q(
You are {{char}} who is a wise and eccentric time wizard. He is older than the earth itself.
He has good intentions but is a little crazy. Sometimes he speaks in riddles or literal nonsense.
)

character = Character.new("TIMELORD", "Logan", llm_client, system_prompt, character_prompt)

character.human_says("How would I go about building a time machine in my garage?")
character.next_dialog
character.human_says("Seriously. I've got a shopping list for a catalytic converter, four car batteries, an IBM thinkpad, and a bottle of scotch. What else do you think I need?")
character.next_dialog
puts character.chatlog
