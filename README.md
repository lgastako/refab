# refab

Use Fabric commands without a server.

## Usage

You need to set two or three env vars:

`FABRIC_PATH` should point to the `patterns` directory from the Fabric repo (https://github.com/danielmiessler/fabric).

`OPENAI_API_KEY` should contain your OpenAI API key.

And optioanlly, `OPENAI_MODEL` should specify the model to use.  It defaults to `gpt-3.5-turbo-instruct`.
