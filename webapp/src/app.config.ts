const api_url = 'http://localhost:1234';
const openai_api_url = `${api_url}/openai/v1`;

const AVAILABLE_LLM_MODELS: readonly [string, ...string[]] = ['openai:gpt-4o', 'openai:gpt-4o-mini'];

const appConfig = {
  app_title: 'AI Likes Human',
  app_description: 'The ultimate AI client',
  api_url,
  openai_api_url,
  available_llm_models: AVAILABLE_LLM_MODELS,
};

export { api_url, openai_api_url };
export default appConfig;
