export function avatarUrl(template, size = 48) {
  if (!template) return "";
  const url = template.replace("{size}", size.toString());
  if (url.startsWith("http")) return url;
  return `https://linux.do${url}`;
}
