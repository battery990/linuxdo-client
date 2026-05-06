<script>
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import TopicCard from "$lib/components/TopicCard.svelte";
  import CategoryFilter from "$lib/components/CategoryFilter.svelte";
  import { topics, loading, fetchTopics, selectedCategory } from "$lib/stores/topics.js";
  import { RefreshCw } from "lucide-svelte";

  let refreshing = $state(false);
  let loadingMore = $state(false);

  async function loadPage(page = 0) {
    await fetchTopics(page, $selectedCategory);
  }

  async function handleRefresh() {
    refreshing = true;
    await loadPage(0);
    refreshing = false;
  }

  async function handleCategoryChange(slug) {
    $selectedCategory = slug;
    await loadPage(0);
  }

  async function handleScroll(e) {
    const el = e.target;
    if (loadingMore || $loading) return;
    if (el.scrollTop + el.clientHeight >= el.scrollHeight - 200) {
      loadingMore = true;
      const nextPage = Math.floor($topics.length / 30);
      await loadPage(nextPage);
      loadingMore = false;
    }
  }

  onMount(() => loadPage(0));
</script>

<div class="page" onscroll={handleScroll}>
  <header class="page-header">
    <h1>linux.do</h1>
    <button class="refresh-btn" onclick={handleRefresh} disabled={refreshing || $loading}>
      <RefreshCw size={18} class:spinning={refreshing} />
    </button>
  </header>

  <CategoryFilter
    selected={$selectedCategory}
    onchange={handleCategoryChange}
  />

  {#if $loading && $topics.length === 0}
    <div class="loading">
      <div class="spinner"></div>
      <span>加载中...</span>
    </div>
  {:else}
    <div class="topic-list">
      {#each $topics as topic (topic.id)}
        <TopicCard
          {topic}
          onclick={() => goto(`/topic/${topic.id}`)}
        />
      {/each}

      {#if loadingMore}
        <div class="loading-more">
          <div class="spinner"></div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .page {
    height: 100%;
    overflow-y: auto;
  }
  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px 0;
  }
  .page-header h1 {
    font-size: 20px;
    font-weight: 700;
  }
  .refresh-btn {
    display: flex;
    padding: 8px;
    color: var(--md-sys-color-on-surface-variant);
    border-radius: 50%;
  }
  .refresh-btn:hover {
    background: var(--md-sys-color-surface-container);
  }
  .refresh-btn:disabled {
    opacity: 0.5;
  }
  :global(.spinning) {
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .loading, .loading-more {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 40px;
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
</style>
