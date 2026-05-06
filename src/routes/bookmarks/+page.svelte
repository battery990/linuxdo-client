<script>
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { bookmarks, loadBookmarks } from "$lib/stores/bookmarks.js";
  import { Bookmark } from "lucide-svelte";

  onMount(() => loadBookmarks());
</script>

<div class="bookmarks-page">
  <header class="page-header">
    <h1>收藏</h1>
  </header>

  {#if $bookmarks.length === 0}
    <div class="empty">
      <Bookmark size={48} />
      <p>暂无收藏</p>
      <p class="hint">在帖子详情页点击收藏按钮</p>
    </div>
  {:else}
    <div class="bookmark-list">
      {#each $bookmarks as bm (bm.id)}
        <button class="bookmark-item" onclick={() => goto(`/topic/${bm.topic_id}`)}>
          <div class="bm-title">{bm.title}</div>
          <div class="bm-time">{bm.saved_at}</div>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .bookmarks-page {
    height: 100%;
    overflow-y: auto;
  }
  .page-header {
    padding: 14px 16px;
  }
  .page-header h1 {
    font-size: 20px;
    font-weight: 700;
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 80px 16px;
    color: var(--md-sys-color-on-surface-variant);
  }
  .empty p {
    font-size: 16px;
  }
  .hint {
    font-size: 13px !important;
    opacity: 0.7;
  }

  .bookmark-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 14px 16px;
    border-bottom: 1px solid var(--md-sys-color-surface-container-high);
    transition: background-color 0.15s;
  }
  .bookmark-item:hover {
    background: var(--md-sys-color-surface-container-low);
  }
  .bm-title {
    font-size: 15px;
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .bm-time {
    margin-top: 4px;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }
</style>
