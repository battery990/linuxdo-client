<script>
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import SearchBar from "$lib/components/SearchBar.svelte";
  import TopicCard from "$lib/components/TopicCard.svelte";
  import { Clock, Trash2 } from "lucide-svelte";

  let query = $state("");
  let results = $state([]);
  let loading = $state(false);
  let searched = $state(false);
  let history = $state([]);

  onMount(async () => {
    try {
      history = await invoke("get_search_history");
    } catch (e) {
      console.error("Failed to load search history:", e);
    }
  });

  async function handleSearch(q) {
    const searchText = q ?? query;
    if (!searchText.trim()) return;
    query = searchText;
    loading = true;
    searched = true;
    try {
      const data = await invoke("search_topics", { query: searchText });
      results = data.topics ?? [];
      await invoke("add_search_history", { query: searchText });
      history = await invoke("get_search_history");
    } catch (e) {
      console.error("Search failed:", e);
      results = [];
    } finally {
      loading = false;
    }
  }
</script>

<div class="search-page">
  <SearchBar
    bind:value={query}
    onsubmit={handleSearch}
    onclear={() => {
      results = [];
      searched = false;
    }}
  />

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <span>搜索中...</span>
    </div>
  {:else if searched}
    {#if results.length === 0}
      <div class="empty">没有找到相关帖子</div>
    {:else}
      <div class="result-count">找到 {results.length} 个结果</div>
      {#each results as topic (topic.id)}
        <TopicCard {topic} onclick={() => goto(`/topic/${topic.id}`)} />
      {/each}
    {/if}
  {:else}
    {#if history.length > 0}
      <div class="history-section">
        <div class="section-header">
          <span class="section-title">搜索历史</span>
        </div>
        {#each history as item (item.id)}
          <button
            class="history-item"
            onclick={() => {
              query = item.query;
              handleSearch(item.query);
            }}
          >
            <Clock size={16} />
            <span>{item.query}</span>
          </button>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .search-page {
    height: 100%;
    overflow-y: auto;
  }
  .loading, .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 40px 16px;
    color: var(--md-sys-color-on-surface-variant);
    font-size: 14px;
  }
  .spinner {
    width: 28px;
    height: 28px;
    border: 3px solid var(--md-sys-color-surface-container-high);
    border-top-color: var(--md-sys-color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  .result-count {
    padding: 6px 16px;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }

  .history-section {
    padding: 0 16px;
  }
  .section-header {
    padding: 12px 0 8px;
  }
  .section-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--md-sys-color-on-surface-variant);
  }
  .history-item {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 10px 0;
    color: var(--md-sys-color-on-surface);
    font-size: 14px;
    border-bottom: 1px solid var(--md-sys-color-surface-container-high);
  }
  .history-item:hover {
    color: var(--md-sys-color-primary);
  }
</style>
