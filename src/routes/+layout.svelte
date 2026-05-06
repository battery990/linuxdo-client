<script>
  import { page } from "$app/stores";
  import { Home, Bookmark, Search } from "lucide-svelte";
  import "../app.css";

  let { children } = $props();

  const navItems = [
    { path: "/", icon: Home, label: "首页" },
    { path: "/bookmarks", icon: Bookmark, label: "收藏" },
    { path: "/search", icon: Search, label: "搜索" },
  ];
</script>

<div class="app-shell">
  <main class="content">
    {@render children()}
  </main>

  <nav class="bottom-nav">
    {#each navItems as item}
      <a
        href={item.path}
        class="nav-item"
        class:active={$page.url.pathname === item.path}
      >
        <svelte:component this={item.icon} size={22} />
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>
</div>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    max-width: 600px;
    margin: 0 auto;
  }
  .content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    -webkit-overflow-scrolling: touch;
  }
  .bottom-nav {
    display: flex;
    border-top: 1px solid var(--md-sys-color-surface-container-high);
    background: var(--md-sys-color-surface);
    padding: 6px 0 env(safe-area-inset-bottom, 6px);
  }
  .nav-item {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: 8px 0;
    font-size: 11px;
    color: var(--md-sys-color-on-surface-variant);
    text-decoration: none;
    transition: color 0.15s;
  }
  .nav-item.active {
    color: var(--md-sys-color-primary);
  }
</style>
