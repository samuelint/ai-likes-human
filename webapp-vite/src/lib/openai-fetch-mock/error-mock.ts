
import { AssistantResponseBuilder, NonFunctionProperties } from './assistant-response-builder';


type AssistantErrorMockProps = NonFunctionProperties<ErrorMock>;

export class ErrorMock extends AssistantResponseBuilder {
  public status?: number = 500;
  public errorMessage?: string = 'Internal Error';

  public constructor(props: AssistantErrorMockProps = {}) {
    super();
    Object.assign(this, props);
  }

  public doesMatch(): boolean {
    return true;
  }

  public getResponse(): Response {
    const errorMessage = this.errorMessage;
    return {
      ok: false,
      status: this.status,
      headers: new Map([['content-type', 'text/event-stream']]),
      bodyUsed: false,
      body: {
        getReader() {

          return {
            read() {
              return Promise.resolve(errorMessage);
            },
            releaseLock() {},
            cancel() {},
          };
        },
      },
      text: () => Promise.resolve(errorMessage),
    } as unknown as Response;
  }
}