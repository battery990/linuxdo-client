<script>
  import { timeAgo } from "$lib/utils/time.js";
  import { avatarUrl } from "$lib/utils/avatar.js";
  import { Heart } from "lucide-svelte";

  let { post, users } = $props();

  let user = $derived(users.find((u) => u.id === post.user_id));
</script>

<article class="post-item">
  <div class="post-header">
    <span class="floor">#{post.post_number}</span>
    {#if user}
      <img
        class="avatar"
        src={avatarUrl(user.avatar_template, 48)}
        alt={user.username}
        loading="lazy"
      />
      <span class="username">{user.username}</span>
    {/if}
    <span class="time">{timeAgo(post.created_at)}</span>
  </div>

  <div class="post-content">
    {@html post.cooked}
  </div>

  {#if post.like_count > 0}
    <div class="post-footer">
      <span class="likes">
        <Heart size={14} />
        {post.like_count}
      </span>
    </div>
  {/if}
</article>

<style>
  .post-item {
    padding: 14px 16px;
    border-bottom: 1px solid var(--md-sys-color-surface-container-high);
  }

  .post-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
    font-size: 13px;
    color: var(--md-sys-color-on-surface-variant);
  }
  .floor {
    font-weight: 600;
    color: var(--md-sys-color-primary);
    min-width: 28px;
  }
  .avatar {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    object-fit: cover;
  }
  .username {
    font-weight: 500;
    color: var(--md-sys-color-on-surface);
  }
  .time {
    margin-left: auto;
    font-size: 12px;
  }

  .post-content {
    font-size: 14px;
    line-height: 1.7;
    overflow-wrap: break-word;
  }
  .post-content :global(img) {
    max-width: 100%;
    border-radius: 8px;
  }
  .post-content :global(pre) {
    background: var(--md-sys-color-surface-container);
    padding: 12px;
    border-radius: 8px;
    overflow-x: auto;
    font-size: 13px;
  }
  .post-content :global(blockquote) {
    border-left: 3px solid var(--md-sys-color-outline);
    padding-left: 12px;
    margin: 8px 0;
    color: var(--md-sys-color-on-surface-variant);
  }
  .post-content :global(a) {
    color: var(--md-sys-color-primary);
  }

  .post-footer {
    margin-top: 8px;
  }
  .likes {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--md-sys-color-on-surface-variant);
  }
</style>
