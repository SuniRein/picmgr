export function downloadImage(title: string, url: string) {
  const link = document.createElement('a');
  link.download = title;
  link.href = url;
  link.click();
}
