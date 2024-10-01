
import { AssistantResponseBuilder, NonFunctionProperties } from './assistant-response-builder';


type AssistantErrorMockProps = NonFunctionProperties<ErrorMock>;

export class ErrorMock extends AssistantResponseBuilder {
  public status?: number = 500;
  public errorMessage?: string = 'Internal Error';
  public url_match?: string;
  public callCount?: number = 0;

  public constructor(props: AssistantErrorMockProps = {}) {
    super();
    Object.assign(this, props);
  }

  public doesMatch(url: string): boolean {
    if (this.url_match) {
      return url.includes(this.url_match);
    }

    return true;
  }

  public getResponse(): Response {
    this.callCount = (this.callCount || 0) + 1;

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