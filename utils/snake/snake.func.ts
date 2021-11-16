import removeChildren from '../dom/removeChildren.func';

const BACKGROUND_COLOR = '#588A38';

export default function snake(root: HTMLDivElement): () => void {
  removeChildren(root);

  const canvas = document.createElement('canvas');
  const ctx = canvas.getContext('2d');
  canvas.style.position = 'absolute';
  root.appendChild(canvas);

  const state = {
    clicks: 0,
    ctx: ctx,
    aspect: 1,
  };

  window.onresize = function (): void {
    const { offsetWidth, offsetHeight } = root;

    if (offsetWidth > offsetHeight) {
      canvas.width = offsetHeight;
      canvas.height = offsetHeight;
      canvas.style.left = `${(offsetWidth - offsetHeight) / 2}px`;
      canvas.style.top = '';
    } else {
      canvas.width = offsetWidth;
      canvas.height = offsetWidth;
      canvas.style.top = `${(offsetHeight - offsetWidth) / 2}px`;
      canvas.style.left = '';
    }

    state.aspect = canvas.offsetWidth === 0 ? 1 : canvas.offsetWidth / 100;
    render();
  };

  canvas.onclick = function (): void {
    state.clicks++;
    render();
  };

  window.onresize(null);
  function render() {
    state.ctx.fillStyle = BACKGROUND_COLOR;
    state.ctx.fillRect(0, 0, ctx.canvas.offsetWidth, ctx.canvas.offsetHeight);

    state.ctx.fillStyle = 'black';
    state.ctx.fillText(
      String(state.clicks),
      5 * state.aspect,
      3 * state.aspect
    );

    state.ctx.fillRect(
      5 * state.aspect,
      5 * state.aspect,
      5 * state.aspect,
      5 * state.aspect
    );
  }

  return function cleanup() {
    // cleanup, this will be called on unmount
    removeChildren(root);
    window.removeEventListener('onresize', window.onresize);
  };
}
