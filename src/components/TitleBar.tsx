import { getBridge } from '../bridge';

export function TitleBar() {
  return (
    <div id="titlebar">
      <span id="titlebar-text"> Musicli v2.1</span>
      <div id="titlebar-btns">
        <button id="btn-minimize" title="Minimize" onClick={() => getBridge().minimize()}>
          ─
        </button>
        <button id="btn-close" title="Close" onClick={() => getBridge().close()}>
          x
        </button>
      </div>
    </div>
  );
}
