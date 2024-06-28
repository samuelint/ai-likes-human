import '@testing-library/jest-dom';
import 'web-streams-polyfill/polyfill';
import { TextEncoder, TextDecoder } from 'util';


Object.assign(global, { TextDecoder, TextEncoder });