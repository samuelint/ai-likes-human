const api_url = 'http://localhost:1234';
const openai_api_url = `${api_url}/openai/v1`;

const AVAILABLE_LLM_MODELS: readonly [string, ...string[]] = ['openai:gpt-4o', 'openai:gpt-4o-mini'];
const LLM_API_KEYS_KEYS: readonly [string, ...string[]] = ['OPENAI_API_KEY', 'ANTHROPIC_API_KEY'];

const appConfig = {
  app_title: 'AI Likes Human',
  app_description: 'The ultimate AI client',
  api_url,
  openai_api_url,
  available_llm_models: AVAILABLE_LLM_MODELS,
};

export { api_url, openai_api_url, LLM_API_KEYS_KEYS };
export default appConfig;
