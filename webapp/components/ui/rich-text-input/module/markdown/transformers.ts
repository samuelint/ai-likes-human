import { TRANSFORMERS } from '@lexical/markdown';
import { HR } from './hr.transformer';
import { CODE } from './code.transformer';


export const _TRANSFORMERS = [CODE, ...TRANSFORMERS, HR];