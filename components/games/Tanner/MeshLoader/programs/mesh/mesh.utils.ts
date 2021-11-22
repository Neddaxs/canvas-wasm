export const loadImage = (url: string, callback: () => void) => {
  const image = new Image();
  image.onload = () => {
    callback();
  };
  image.src = url;
};
