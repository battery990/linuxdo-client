import { writable } from "svelte/store";

export const topics = writable([]);
export const loading = writable(false);
export const currentPage = writable(0);
export const selectedCategory = writable(null);

export async function fetchTopics(page = 0, category = null) {
  loading.set(true);
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const data = await invoke("get_topics", { page, category });
    if (page === 0) {
      topics.set(data.topic_list.topics);
    } else {
      topics.update((prev) => [...prev, ...data.topic_list.topics]);
    }
    currentPage.set(page);
    return data;
  } finally {
    loading.set(false);
  }
}
