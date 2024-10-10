use anyhow::{anyhow, Result};
use futures::Stream;
use std::{pin::Pin, sync::Arc};

use futures::StreamExt;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use llama_cpp_2::{context::LlamaContext, token::LlamaToken};

use super::options::RunOptions;
use super::{model_context_factory::ModelContextFactory, model_factory::ModelFactory};

#[derive(Clone)]
pub struct PredictOptions {
    pub top_k: i32,
    pub temperature: f32,
    pub f16_kv: bool,
}

impl Default for PredictOptions {
    fn default() -> Self {
        Self {
            top_k: 90,
            temperature: 0.7,
            f16_kv: true,
        }
    }
}

pub type LlamaCppStream<'a> = Pin<Box<dyn Stream<Item = Result<String>> + 'a>>;

pub struct AsyncLLamaCPP {
    model_factory: Arc<dyn ModelFactory>,
    context_factory: Arc<dyn ModelContextFactory>,
}

impl AsyncLLamaCPP {
    pub fn new(
        model_factory: Arc<dyn ModelFactory>,
        context_factory: Arc<dyn ModelContextFactory>,
    ) -> Self {
        Self {
            model_factory,
            context_factory,
        }
    }

    pub async fn invoke(
        &self,
        model_path: &str,
        options: &RunOptions,
        prompt: &str,
    ) -> Result<String> {
        let mut stream = self.stream(model_path, options, prompt);

        let mut responses: Vec<String> = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(e) => {
                    return Err(anyhow!(e));
                }
            };

            responses.push(chunk);
        }

        Ok(responses.join(""))
    }

    pub fn stream<'s>(
        &self,
        model_path: &str,
        options: &'s RunOptions,
        prompt: &'s str,
    ) -> LlamaCppStream<'s> {
        let prompt_and_response_length = 32; // the length of the prompt + output in tokens
        let model_path = model_path.to_string();
        let model_factory = self.model_factory.clone();
        let context_factory = self.context_factory.clone();

        let s = async_stream::stream! {
            let model = model_factory.create(&model_path).unwrap();
            let mut ctx = context_factory.create(&model).unwrap();

            let tokens_list = match Self::generate_tokens(&model, prompt) {
                Ok(tokens_list) => tokens_list,
                Err(err) => {
                    yield Err(err.into());
                    return;
                }
            };

            match Self::assert_tokens(&ctx, &tokens_list, prompt_and_response_length) {
                Ok(_) => {},
                Err(err) => {
                    yield Err(err.into());
                    return;
                }
            };

            let mut batch = match Self::create_batch(tokens_list, &options) {
                Ok(batch) => batch,
                Err(err) => {
                    yield Err(err.into());
                    return;
                }
            };

            match ctx.decode(&mut batch) {
                Ok(_) => {}
                Err(err) => {
                    yield Err(err.into());
                    return;
                }
            }
            let mut decoder = encoding_rs::UTF_8.new_decoder();
            let mut n_cur = batch.n_tokens();

            while n_cur <= prompt_and_response_length {
                // sample the next token
                {
                    let candidates = ctx.candidates();

                    let candidates_p = LlamaTokenDataArray::from_iter(candidates, false);

                    // sample the most likely token
                    let new_token_id = ctx.sample_token_greedy(candidates_p);

                    // is it an end of stream?
                    if model.is_eog_token(new_token_id) {
                        break;
                    }

                    let output_bytes =
                        match model.token_to_bytes(new_token_id, Special::Tokenize) {
                            Ok(output_bytes) => output_bytes,
                            Err(err) => {
                                yield Err(err.into());
                                return;
                            }
                        };
                    // use `Decoder.decode_to_string()` to avoid the intermediate buffer
                    let mut output_string = String::with_capacity(32);
                    let _decode_result =
                        decoder.decode_to_string(&output_bytes, &mut output_string, false);

                    yield Ok(output_string);

                    batch.clear();
                    match batch.add(new_token_id, n_cur, &[0], true) {
                        Ok(_) => {}
                        Err(err) => {
                            yield Err(err.into());
                            return;
                        }
                    };
                }

                n_cur += 1;

                match ctx.decode(&mut batch) {
                    Ok(_) => {}
                    Err(err) => {
                        yield Err(err.into());
                        return;
                    }
                };
            }
        };

        Box::pin(s)
    }

    fn generate_tokens(model: &LlamaModel, prompt: &str) -> Result<Vec<LlamaToken>> {
        let tokens = model.str_to_token(prompt, AddBos::Always)?;

        Ok(tokens)
    }

    fn create_batch(tokens: Vec<LlamaToken>, options: &RunOptions) -> Result<LlamaBatch> {
        let mut batch = LlamaBatch::new(options.n_batch, 1);

        let last_index: i32 = (tokens.len() - 1) as i32;
        for (i, token) in (0_i32..).zip(tokens.into_iter()) {
            // llama_decode will output logits only for the last token of the prompt
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last)?;
        }

        Ok(batch)
    }

    fn assert_tokens(
        ctx: &LlamaContext,
        tokens: &Vec<LlamaToken>,
        prompt_and_response_length: i32,
    ) -> Result<()> {
        let n_cxt = ctx.n_ctx() as i32;
        let n_kv_req = tokens.len() as i32 + (prompt_and_response_length - tokens.len() as i32);

        if n_kv_req > n_cxt {
            return Err(anyhow!("n_kv_req > n_ctx, the required kv cache size is not big enough either reduce n_len or increase n_ctx").into());
        }

        if tokens.len() >= usize::try_from(prompt_and_response_length).unwrap() {
            return Err(anyhow!("the prompt is too long, it has more tokens than n_len").into());
        }

        Ok(())
    }
}
