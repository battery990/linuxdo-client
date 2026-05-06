<script>
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import PostItem from "$lib/components/PostItem.svelte";
  import { toggleBookmark, checkBookmarked } from "$lib/stores/bookmarks.js";
  import { ArrowLeft, Bookmark, BookmarkCheck, ExternalLink } from "lucide-svelte";

  let topicId = $derived(Number($page.params.id));
  let topic = $state(null);
  let posts = $state([]);
  let users = $state([]);
  let loading = $state(true);
  let isBookmarked = $state(false);
  let bookmarkLoading = $state(false);

  onMount(async () => {
    try {
      const data = await invoke("get_topic_detail", { topicId });
      topic = data;
      posts = data.post_stream?.posts ?? [];
      users = [
        ...(data.participants ?? []).map((p) => ({ id: p.id, username: p.username, avatar_template: "" })),
      ];
      isBookmarked = await checkBookmarked(topicId);
    } catch (e) {
      console.error("Failed to load topic:", e);
    } finally {
      loading = false;
    }
  });

  async function handleBookmark() {
    if (bookmarkLoading) return;
    bookmarkLoading = true;
    try {
      isBookmarked = await toggleBookmark(topicId, topic?.title ?? "");
    } finally {
      bookmarkLoading = false;
    }
  }

  function openInBrowser() {
    window.open(`https://linux.do/t/${topicId}`, "_blank");
  }
</script>

<div class="detail-page">
  <header class="detail-header">
    <button class="back-btn" onclick={() => goto("/")}>
      <ArrowLeft size={20} />
    </button>
    <div class="header-actions">
      <button class="action-btn" onclick={handleBookmark}>
        {#if isBookmarked}
          <BookmarkCheck size={20} />
        {:else}
          <Bookmark size={20} />
        {/if}
      </button>
      <button class="action-btn" onclick={openInBrowser}>
        <ExternalLink size={20} />
      </button>
    </div>
  </header>

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <span>加载中...</span>
    </div>
  {:else if topic}
    <h1 class="topic-title">{topic.fancy_title || topic.title}</h1>

    <div class="posts-list">
      {#each posts as post (post.id)}
        <PostItem {post} {users} />
      {/each}
    </div>
  {:else}
    <div class="error">加载失败，请稍后重试</div>
  {/if}
</div>

<style>
  .detail-page {
    height: 100%;
    overflow-y: auto;
  }
  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    position: sticky;
    top: 0;
    background: var(--md-sys-color-surface);
    z-index: 10;
  }
  .back-btn, .action-btn {
    display: flex;
    padding: 8px;
    color: var(--md-sys-color-on-surface);
    border-radius: 50%;
  }
  .back-btn:hover, .action-btn:hover {
    background: var(--md-sys-color-surface-container);
  }
  .header-actions {
    display: flex;
    gap: 4px;
  }

  .topic-title {
    font-size: 18px;
    font-weight: 600;
    line-height: 1.4;
    padding: 0 16px 14px;
  }

  .loading, .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 60px 16px;
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
</style>
