import { writable } from "svelte/store";

export const bookmarks = writable([]);

export async function loadBookmarks() {
  const { invoke } = await import("@tauri-apps/api/core");
  const data = await invoke("get_bookmarks");
  bookmarks.set(data);
}

export async function toggleBookmark(topicId, title) {
  const { invoke } = await import("@tauri-apps/api/core");
  const isBookmarked = await invoke("is_bookmarked", { topicId });
  if (isBookmarked) {
    await invoke("remove_bookmark", { topicId });
  } else {
    await invoke("add_bookmark", { topicId, title });
  }
  return !isBookmarked;
}

export async function checkBookmarked(topicId) {
  const { invoke } = await import("@tauri-apps/api/core");
  return await invoke("is_bookmarked", { topicId });
}
