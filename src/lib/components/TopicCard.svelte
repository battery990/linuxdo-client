<script>
  import { timeAgo } from "$lib/utils/time.js";
  import { avatarUrl } from "$lib/utils/avatar.js";
  import { MessageSquare, Eye } from "lucide-svelte";

  let { topic, onclick } = $props();
</script>

<button class="topic-card" {onclick}>
  <div class="topic-header">
    {#if topic.last_poster}
      <img
        class="avatar"
        src={avatarUrl(topic.last_poster.avatar_template, 48)}
        alt={topic.last_poster.username}
        loading="lazy"
      />
    {/if}
    <span class="author">{topic.last_poster?.username ?? "匿名"}</span>
    <span class="dot">·</span>
    <span class="time">{timeAgo(topic.last_posted_at ?? topic.created_at)}</span>
  </div>

  <h3 class="topic-title">{topic.fancy_title || topic.title}</h3>

  <div class="topic-meta">
    <span class="stats">
      <MessageSquare size={14} />
      {topic.posts_count - 1}
    </span>
    <span class="stats">
      <Eye size={14} />
      {topic.views}
    </span>
    {#if topic.pinned}
      <span class="pin-badge">置顶</span>
    {/if}
  </div>
</button>

<style>
  .topic-card {
    display: block;
    width: 100%;
    text-align: left;
    padding: 14px 16px;
    border-bottom: 1px solid var(--md-sys-color-surface-container-high);
    transition: background-color 0.15s;
  }
  .topic-card:hover {
    background-color: var(--md-sys-color-surface-container-low);
  }
  .topic-card:active {
    background-color: var(--md-sys-color-surface-container);
  }

  .topic-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 6px;
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }
  .avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    object-fit: cover;
  }
  .dot {
    opacity: 0.5;
  }

  .topic-title {
    font-size: 15px;
    font-weight: 500;
    line-height: 1.4;
    color: var(--md-sys-color-on-surface);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    margin-bottom: 8px;
  }

  .topic-meta {
    display: flex;
    align-items: center;
    gap: 14px;
  }
  .stats {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }
  .pin-badge {
    font-size: 11px;
    padding: 1px 6px;
    border-radius: 4px;
    background: var(--md-sys-color-primary-container);
    color: var(--md-sys-color-on-primary-container);
  }
</style>
