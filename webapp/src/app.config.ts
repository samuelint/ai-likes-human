const api_url = 'http://localhost:1234';
const openai_api_url = `${api_url}/openai/v1`;

const appConfig = {
  app_title: 'AI Likes Human',
  app_description: 'The ultimate AI client',
  api_url,
  openai_api_url,
};

export { api_url, openai_api_url };
export default appConfig;
