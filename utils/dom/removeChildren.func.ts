export default function removeChildren(el: HTMLElement): void {
  while (el?.firstChild) {
    el.removeChild(el.firstChild);
  }
}
