<script>
  import { Search, X } from "lucide-svelte";

  let { value = $bindable(""), onsubmit, onclear } = $props();

  function handleKeydown(e) {
    if (e.key === "Enter") {
      onsubmit?.(value);
    }
  }

  function handleClear() {
    value = "";
    onclear?.();
  }
</script>

<div class="search-bar">
  <Search size={18} class="search-icon" />
  <input
    type="text"
    placeholder="搜索帖子..."
    bind:value
    onkeydown={handleKeydown}
  />
  {#if value}
    <button class="clear-btn" onclick={handleClear}>
      <X size={16} />
    </button>
  {/if}
</div>

<style>
  .search-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 10px 16px;
    padding: 8px 14px;
    background: var(--md-sys-color-surface-container);
    border-radius: 28px;
  }
  input {
    flex: 1;
    border: none;
    background: none;
    font-size: 14px;
    color: var(--md-sys-color-on-surface);
    outline: none;
  }
  input::placeholder {
    color: var(--md-sys-color-on-surface-variant);
  }
  .clear-btn {
    display: flex;
    padding: 4px;
    color: var(--md-sys-color-on-surface-variant);
    border-radius: 50%;
  }
  .clear-btn:hover {
    background: var(--md-sys-color-surface-container-high);
  }
</style>
