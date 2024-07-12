import { ToolbarAction } from '../../toolbar';
import { INSERT_BANNER_COMMAND } from './banner.command';


export function InsertBannerAction() {
  return (
    <ToolbarAction
      action={{
        command: INSERT_BANNER_COMMAND,
        payload: undefined
      }}
    >
      Banner
    </ToolbarAction>
  );
}


export function BannerToolbarActions() {
  return (
    <>
      <InsertBannerAction />
    </>
  );
}
